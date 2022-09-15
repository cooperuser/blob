mod vector;
mod physics;

use vector::Vector;
use physics::{Position, Force, Mass, VerletIntegration};
use specs::{World, WorldExt, Builder};
use specs::{System, ReadStorage, Join};
use specs::DispatcherBuilder;

#[derive(Default)]
pub struct DeltaTime(f32);

struct DebugLog;
impl<'a> System<'a> for DebugLog {
    type SystemData = ReadStorage<'a, Position>;

    fn run(&mut self, data: Self::SystemData) {
        for position in data.join() {
            println!("{:?}", &position.now);
        }
    }
}

fn main() {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Force>();
    world.register::<Mass>();
    world.insert(DeltaTime(0.05));

    world.create_entity()
        .with(Position { now: Vector::zero(), last: Vector::zero() })
        .with(Force { vector: Vector::new(1.0, 0.0), magnitude: 1.0 })
        .with(Mass(1.0))
        .build();

    let mut dispatcher = DispatcherBuilder::new()
        .with(VerletIntegration, "verlet_integration", &[])
        .with(DebugLog, "debug_log", &["verlet_integration"])
        .build();

    for _ in 0..20 {
        dispatcher.dispatch(&mut world);
    }
    world.maintain();
}
