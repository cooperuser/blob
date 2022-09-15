use crate::DeltaTime;
use crate::vector::Vector;
use specs::{Component, VecStorage};
use specs::{System, ReadStorage, WriteStorage, Read, Join};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub now: Vector,
    pub last: Vector,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Force {
    pub vector: Vector,
    pub magnitude: f32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Mass(pub(crate) f32);

pub struct VerletIntegration;
impl<'a> System<'a> for VerletIntegration {
    type SystemData = (
        Read<'a, DeltaTime>,
        ReadStorage<'a, Mass>,
        ReadStorage<'a, Force>,
        WriteStorage<'a, Position>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (dt, mass, force, mut pos) = data;
        let dt = dt.0;

        for (mass, force, pos) in (&mass, &force, &mut pos).join() {
            let last = pos.now;
            let a = force.vector * force.magnitude / mass.0 * dt * dt;
            pos.now += pos.now - pos.last + a;
            pos.last = last;
        }
    }
}
