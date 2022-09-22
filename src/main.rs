mod vector;
mod physics;
mod worm;

use physics::{Position, Force, Mass, VerletIntegration, Spring, SpringMassSystem, ForceInitializer, Locked};
use specs::{World, WorldExt, Component, NullStorage};
use specs::{System, ReadStorage, Join};
use specs::DispatcherBuilder;

use crate::physics::{Drag, PointDragSystem};

#[derive(Default)]
pub struct DeltaTime(f32);

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Log;

struct DebugLog(i32);
impl<'a> System<'a> for DebugLog {
    type SystemData = (ReadStorage<'a, Log>, ReadStorage<'a, Position>);

    fn run(&mut self, (log, positions): Self::SystemData) {
        self.0 += 1;
        if self.0 % 10 != 0 {
            return
        }
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
    world.register::<Drag>();
    world.insert(DeltaTime(0.05));

    worm::builder(&mut world, 10);

    let mut dispatcher = DispatcherBuilder::new()
        .with(ForceInitializer, "force_initializer", &[])
        .with(SpringMassSystem, "spring_mass_system", &["force_initializer"])
        .with(PointDragSystem, "point_drag_system", &["force_initializer"])
        .with(VerletIntegration, "verlet_integration", &["spring_mass_system", "point_drag_system"])
        .with(DebugLog(1), "debug_log", &["verlet_integration"])
        .build();

    let stdin = std::io::stdin();
    for _line in stdin.lines() {
        for _ in 0..10 {
            dispatcher.dispatch(&mut world);
        }
    }

    println!("[exited]");
    world.maintain();
}
