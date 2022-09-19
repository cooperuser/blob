mod vector;
mod physics;
mod worm;

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

    worm::builder(&mut world, 3);

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
