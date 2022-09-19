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

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Mass(pub(crate) f32);

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
