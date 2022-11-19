use std::f32::consts::PI;

use crate::vector::Vector;

use bevy::prelude::*;

#[derive(Resource, Default)]
struct DeltaTime(f32);
#[derive(Resource, Default)]
struct Counter(i32);

#[derive(Component)]
pub struct Locked;

#[derive(Component, Debug)]
pub struct Position {
    pub now: Vector,
    pub last: Vector
}

impl Position {
    #[allow(unused)]
    pub fn new(pos: Vector) -> Self { Self { now: pos, last: pos } }
}

#[derive(Component, Debug)]
pub struct Force(pub Vector);

#[allow(unused)]
#[derive(Component, Debug)]
pub struct Spring {
    pub a: Entity,
    pub b: Entity,
    pub constant: f32,
    pub length: f32,
}

#[allow(unused)]
#[derive(Component, Debug)]
pub struct Control {
    pub index: i32,
    pub side: f32
}

#[derive(Component, Debug)]
pub struct Mass(pub f32);

#[derive(Component, Debug)]
pub struct Drag(pub f32);

fn force_resetter(mut query: Query<&mut Force>) {
    for mut force in query.iter_mut() {
        force.0 = Vector::zero();
    }
}

fn spring_mass_system(
    springs: Query<&Spring>,
    positions: Query<&Position>,
    mut forces: Query<&mut Force>
) {
    for spring in springs.iter() {
        let diff = {
            let a = positions.get(spring.a).unwrap();
            let b = positions.get(spring.b).unwrap();
            a.now - b.now
        };
        let dist = diff.magnitude();

        let x = spring.length - dist;
        let f = -spring.constant * x / diff.magnitude();

        let mut force_a = forces.get_mut(spring.a).unwrap();
        force_a.0 -= diff * f;
        let mut force_b = forces.get_mut(spring.b).unwrap();
        force_b.0 += diff * f;
    }
}

fn linear_drag_system(
    positions: Query<&Position>,
    springs: Query<(&Spring, &Drag)>,
    mut forces: Query<&mut Force>
) {
    for (spring, drag) in springs.iter() {
        let a = positions.get(spring.a).unwrap();
        let b = positions.get(spring.b).unwrap();
        let tangent = b.now - a.now;
        let length = tangent.magnitude();
        let normal = Vector::new(tangent.y, -tangent.x);
        let v_a = a.now - a.last;
        let v_b = b.now - b.last;
        let v = (v_a + v_b) / 2.0;
        let dot = Vector::dot(v.normalized(), normal.normalized());

        let force = dot * length * drag.0;
        let mut force_a = forces.get_mut(spring.a).unwrap();
        force_a.0 -= normal * force;
        let mut force_b = forces.get_mut(spring.b).unwrap();
        force_b.0 -= normal * force;
    }
}

fn point_drag_system(
    mut query: Query<(&Position, &mut Force), With<Drag>>,
) {
    let density = 1.0;
    let area = 1.0;
    for (pos, mut force) in query.iter_mut() {
        let v = pos.now - pos.last;
        let v_sq = v.sqr_magnitude();
        let f = 2000.0 * density * area * v_sq;
        force.0 -= v.normalized() * f;
    }
}

fn verlet_integration(
    mut query: Query<(&mut Position, &Mass, &Force), Without<Locked>>,
    dt: Res<DeltaTime>
) {
    let dt = dt.0;
    for (mut pos, mass, force) in query.iter_mut() {
        let last = pos.now;
        let a = force.0 / mass.0 * dt * dt;
        let diff = pos.now - pos.last + a;
        pos.now += diff;
        pos.last = last;
    }
}

fn control_system(
    mut query: Query<(&mut Spring, &Control)>,
    mut counter: ResMut<Counter>
) {
    counter.0 += 1;
    let t = counter.0 as f32 / 50.0;
    for (mut spring, control) in query.iter_mut() {
        let phase = control.index as f32 * PI / 3.0;
        let u = (t + phase).sin() * control.side;
        spring.length = 0.5 + u * 0.2;
    }
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DeltaTime(0.05));
        app.insert_resource(Counter(0));
        app.add_system(force_resetter);
        app.add_system(control_system);
        app.add_system(spring_mass_system.after(force_resetter));
        app.add_system(point_drag_system.after(force_resetter));
        app.add_system(linear_drag_system.after(force_resetter));
        app.add_system(
            verlet_integration
                .after(spring_mass_system)
                .after(point_drag_system)
                .after(linear_drag_system)
        );
    }
}
