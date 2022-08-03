use crate::vector::Vector;
use specs::{Component, VecStorage, WriteStorage};
use specs::{System, ReadStorage, Join};

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
pub struct Mass {
    pub mass: f32,
}

pub struct VerletIntegration;
impl<'a> System<'a> for VerletIntegration {
    type SystemData = (ReadStorage<'a, Mass>, ReadStorage<'a, Force>, WriteStorage<'a, Position>);

    fn run(&mut self, (mass, force, mut pos): Self::SystemData) {
        for (mass, force, pos) in (&mass, &force, &mut pos).join() {
            let last = pos.now;
            let dt = 0.05;
            let a = force.vector * force.magnitude / mass.mass * dt * dt;
            pos.now += pos.now - pos.last + a;
            pos.last = last;
        }
    }
}
