mod vector;
mod physics;

use vector::Vector;
use physics::{Position, Force, Mass, VerletIntegration, Spring, SpringMassSystem, ForceInitializer, Locked};
use specs::{World, WorldExt, Builder, Component, NullStorage};
use specs::{System, ReadStorage, Join};
use specs::DispatcherBuilder;

#[derive(Default)]
pub struct DeltaTime(f32);

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Log;

struct DebugLog;
impl<'a> System<'a> for DebugLog {
    type SystemData = (ReadStorage<'a, Log>, ReadStorage<'a, Position>);

    fn run(&mut self, (log, positions): Self::SystemData) {
        for (position, _) in (&positions, &log).join() {
            println!("{:?}", &position.now);
        }
    }
}

fn main() {
    let mut world = World::new();
    world.register::<Locked>();
    world.register::<Log>();
    world.register::<Position>();
    world.register::<Force>();
    world.register::<Mass>();
    world.register::<Spring>();
    world.insert(DeltaTime(0.05));

    let a = world.create_entity()
        .with(Position::new(Vector::default()))
        .with(Force(Vector::default()))
        .with(Mass(1.0))
        .build();

    let b = world.create_entity()
        .with(Position::new(Vector::new(1.0, 0.0)))
        .with(Force(Vector::zero()))
        .with(Mass(1.0))
        .with(Log)
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
