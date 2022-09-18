mod vector;
mod physics;

use vector::Vector;
use physics::{Position, Force, Mass, VerletIntegration, Spring, SpringMassSystem, ForceInitializer};
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
        println!("");
    }
}

fn main() {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Force>();
    world.register::<Mass>();
    world.register::<Spring>();
    world.insert(DeltaTime(0.05));

    let a = world.create_entity()
        .with(Position::new(Vector::new(-0.5, 0.0)))
        .with(Force(Vector::zero()))
        .with(Mass(1.0))
        .build();

    let b = world.create_entity()
        .with(Position::new(Vector::new(0.5, 0.0)))
        .with(Force(Vector::zero()))
        .with(Mass(1.0))
        .build();

    let _s1 = world.create_entity()
        .with(Spring { a, b, constant: 10.0, length: 2.0 })
        .build();

    let mut dispatcher = DispatcherBuilder::new()
        .with(ForceInitializer, "force_initializer", &[])
        .with(SpringMassSystem, "spring_mass_system", &["force_initializer"])
        .with(VerletIntegration, "verlet_integration", &["spring_mass_system"])
        .with(DebugLog, "debug_log", &["verlet_integration"])
        .build();

    for _ in 0..20 {
        dispatcher.dispatch(&mut world);
    }
    world.maintain();
}
