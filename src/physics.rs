use std::f32::consts::PI;

use crate::DeltaTime;
use crate::vector::Vector;
use specs::{Component, VecStorage, Entity, NullStorage};
use specs::{System, ReadStorage, WriteStorage, Read, Join};

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Locked;

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub now: Vector,
    pub last: Vector,
}

impl Position {
    pub fn new(pos: Vector) -> Self { Self { now: pos, last: pos } }
}

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct Force(pub(crate) Vector);

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Spring {
    pub a: Entity,
    pub b: Entity,
    pub constant: f32,
    pub length: f32,
}

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct Control {
    pub index: i32,
    pub side: f32
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Mass(pub(crate) f32);

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Drag(pub(crate) f32);

impl Default for Drag {
    fn default() -> Self { Self(1.0) }
}

pub struct ForceInitializer;
impl<'a> System<'a> for ForceInitializer {
    type SystemData = WriteStorage<'a, Force>;

    fn run(&mut self, mut forces: Self::SystemData) {
        for force in (&mut forces).join() {
            force.0 = Vector::zero();
        }
    }
}

pub struct SpringMassSystem;
impl<'a> System<'a> for SpringMassSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Spring>,
        WriteStorage<'a, Force>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (positions, springs, mut forces) = data;
        for spring in springs.join() {
            let diff = {
                let a = positions.get(spring.a).unwrap();
                let b = positions.get(spring.b).unwrap();
                a.now - b.now
            };
            let dist = diff.magnitude();

            let x = spring.length - dist;
            let f = -spring.constant * x / diff.magnitude();

            let force_a = forces.get_mut(spring.a).unwrap();
            force_a.0 -= diff * f;
            let force_b = forces.get_mut(spring.b).unwrap();
            force_b.0 += diff * f;
        }
    }
}

pub struct LinearDragSystem;
impl<'a> System<'a> for LinearDragSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Spring>,
        ReadStorage<'a, Drag>,
        WriteStorage<'a, Force>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (positions, springs, drags, mut forces) = data;
        for (spring, drag) in (&springs, &drags).join() {
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
            let force_a = forces.get_mut(spring.a).unwrap();
            force_a.0 -= normal * force;
            let force_b = forces.get_mut(spring.b).unwrap();
            force_b.0 -= normal * force;
        }
    }
}

pub struct PointDragSystem;
impl<'a> System<'a> for PointDragSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Drag>,
        WriteStorage<'a, Force>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (positions, drags, mut forces) = data;
        let density = 1.0;
        let area = 1.0;
        for (pos, force) in (&positions, &mut forces).join() {
            let v = pos.now - pos.last;
            let v_sq = v.sqr_magnitude();
            let f = 2000.0 * density * area * v_sq;
            force.0 -= v.normalized() * f;
        }
    }
}

pub struct VerletIntegration;
impl<'a> System<'a> for VerletIntegration {
    type SystemData = (
        Read<'a, DeltaTime>,
        ReadStorage<'a, Mass>,
        ReadStorage<'a, Force>,
        ReadStorage<'a, Locked>,
        WriteStorage<'a, Position>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (dt, mass, force, locked, mut pos) = data;
        let dt = dt.0;

        for (mass, force, pos, _) in (&mass, &force, &mut pos, !&locked).join() {
            let last = pos.now;
            let a = force.0 / mass.0 * dt * dt;
            pos.now += pos.now - pos.last + a;
            pos.last = last;
        }
    }
}

pub struct ControlSystem(pub f32);
impl<'a> System<'a> for ControlSystem {
    type SystemData = (
        ReadStorage<'a, Control>,
        WriteStorage<'a, Spring>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (control, mut springs) = data;
        self.0 += 1.0;
        let t = self.0 / 100.0;


        for (spring, control) in (&mut springs, &control).join() {
            let phase = control.index as f32 * PI / 6.0;
            spring.length = 0.5 + (t + phase * control.side).sin() * 0.25;
        }
    }
}
