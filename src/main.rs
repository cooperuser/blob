use std::f32::consts::PI;

use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_pancam::*;
use bevy_prototype_debug_lines::*;

mod physics;
mod vector;
mod worm;
mod grid;
mod brain;
mod ui;

use grid::draw_grid;
use physics::*;
use worm::WormController;

pub const HISTORY_LENGTH: usize = 500;

#[derive(Component)]
struct Log;

#[derive(Resource, Default)]
pub struct TimeTracker(f32);

#[derive(Resource, Default)]
pub struct WormSettings {
    frequency: f32,
    phase: f32,
    neurons: usize
}

fn setup(mut commands: Commands, worm_settings: Res<WormSettings>) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection { scale: 0.02, ..default() },
        transform: Transform::default().with_translation(Vec3::Z),
        ..default()
    }).insert(PanCam::default());

    fn default_control(p: f32, t: f32, time: f32, index: f32, side: f32) -> f32 {
        let phase = index * PI / p;
        let u = (-time * 60.0 / t + phase).sin() * side;
        0.5 + u * 0.2
    }

    let worm = worm::worm_builder(12, Vec3::ZERO, &mut commands, |time, index, side| {
        // default_control(3.0, 50.0, time, index, side)
        default_control(6.0, 200.0, time, index, side)
    }, worm_settings.neurons);
    // commands.entity(worm).insert(worm::ManualControl);
    commands.entity(worm).insert(worm::CyclicalMapping);
    // commands.entity(worm).insert(worm::RegionalMapping);
    // commands.entity(worm).insert(worm::FrequencyMapping {
    //     frequency: worm_settings.frequency,
    //     phase: worm_settings.phase,
    // });
    // commands.entity(worm)
        // .insert(worm::CyclicalMapping)
        // .insert(brain::UpdateFlux)
        // .insert(brain::LogCTRNN)
    // ;

    // worm::worm_builder(15, Vec3::new(0., -4., 0.), &mut commands, |time, index, side| {
    //     default_control(6.0, 250.0, time, index, side)
    // });

    // worm::worm_builder(15, Vec3::new(0., 4., 0.), &mut commands, |time, index, side| {
    //     default_control(6.0, 200.0, time, index, side)
    // });
}

fn sync_points(
    mut query: Query<(Entity, &Position, Option<&mut Transform>)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity, pos, transform) in query.iter_mut() {
        if let Some(mut transform) = transform {
            transform.translation.x = pos.now.x;
            transform.translation.y = pos.now.y;
            transform.translation.z = 1.0;
        } else {
            commands.entity(entity).insert(MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
                transform: Transform::default().with_scale(Vec3::splat(0.2)),
                material: materials.add(ColorMaterial::from(Color::BLACK)),
                ..default()
            });
        }
    }
}

fn sync_edges_cyclical(
    worms: Query<(&WormController, &worm::Neurons), With<worm::CyclicalMapping>>,
    query: Query<(&Parent, &Spring, Option<&worm::Control>)>,
    transforms: Query<&GlobalTransform>,
    mut lines: ResMut<DebugLines>
) {
    for (parent, spring, control) in query.iter() {
        if let Ok((worm, neurons)) = worms.get(parent.get()) {
            let a = match transforms.get(spring.a) {
                Ok(t) => t.translation(),
                Err(_) => Vec3::ZERO
            };
            let b = match transforms.get(spring.b) {
                Ok(t) => t.translation(),
                Err(_) => Vec3::ZERO
            };
            let color = match control {
                // _ => Color::BLACK
                Some(control) => match (control.index - 1) % neurons.0.len() as i32 {
                    0 => Color::RED,
                    1 => Color::ORANGE,
                    2 => Color::YELLOW,
                    3 => Color::GREEN,
                    4 => Color::BLUE,
                    5 => Color::PURPLE,
                    _ => Color::BLACK
                },
                None => Color::BLACK,
            };
            lines.line_colored(a, b, 0., color);
        }
    }
}

fn sync_edges_regional(
    worms: Query<(&WormController, &worm::Neurons), With<worm::RegionalMapping>>,
    query: Query<(&Parent, &Spring, Option<&worm::Control>)>,
    transforms: Query<&GlobalTransform>,
    mut lines: ResMut<DebugLines>
) {
    for (parent, spring, control) in query.iter() {
        if let Ok((worm, neurons)) = worms.get(parent.get()) {
            let a = match transforms.get(spring.a) {
                Ok(t) => t.translation(),
                Err(_) => Vec3::ZERO
            };
            let b = match transforms.get(spring.b) {
                Ok(t) => t.translation(),
                Err(_) => Vec3::ZERO
            };
            let len = worm.segments.len();
            let color = match control {
                // _ => Color::BLACK
                Some(control) => match (control.index - 1) / (len / neurons.0.len()) as i32 {
                    0 => Color::RED,
                    1 => Color::ORANGE,
                    2 => Color::YELLOW,
                    3 => Color::GREEN,
                    4 => Color::BLUE,
                    5 => Color::PURPLE,
                    _ => Color::BLACK
                },
                None => Color::BLACK,
            };
            lines.line_colored(a, b, 0., color);
        }
    }
}

fn increment_time(mut time: ResMut<TimeTracker>) {
    time.0 += 1.0 / 60.0;
}

fn log_output_and_exit(
    time: Res<TimeTracker>,
    mut exit: EventWriter<AppExit>,
    positions: Query<&Position>
) {
    if time.0 >= 600.0 {
        let mut total = Vec3::default();
        let mut count = 0;
        for pos in positions.iter() {
            total += pos.now;
            count += 1;
        }
        println!("{}", total.x / count as f32);
        exit.send(AppExit);
    }
}

fn logger(positions: Query<&Position, With<Log>>) {
    for pos in positions.iter() {
        println!("{:?}", pos.now);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let frequency = 0.1;
    let frequency: f32 = match args.get(1) {
        Some(num) => num.parse().unwrap_or(frequency),
        None => frequency,
    };
    let phase = 2.0 * PI / 6.0;
    let phase: f32 = match args.get(2) {
        Some(num) => num.parse().unwrap_or(phase),
        None => phase,
    };
    let neurons = 6;
    let neurons: usize = match args.get(3) {
        Some(num) => num.parse().unwrap_or(neurons),
        None => neurons,
    };
    let nogui = match args.last() {
        Some(text) => if text == "--nogui" { true } else { false },
        None => false,
    };

    let mut app = App::new();
    let using_gui = !nogui;
    if using_gui {
        // app.insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        app.insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                window: WindowDescriptor {
                    fit_canvas_to_parent: true,
                    ..default()
                },
                ..default()
            }))
            .add_system(bevy::window::close_on_esc)
            // .add_plugin(WorldInspectorPlugin::new())
            .add_plugin(PanCamPlugin::default())
            .add_plugin(DebugLinesPlugin::with_depth_test(true))
            .add_plugin(ui::UIPlugin)
            // .add_system(draw_grid.before(sync_edges))
            .add_system(sync_points)
            .add_system(sync_edges_cyclical)
            .add_system(sync_edges_regional);
    } else {
        app.add_plugins(MinimalPlugins);
    }

    app
        .insert_resource(TimeTracker(0.0))
        .insert_resource(WormSettings { frequency, phase, neurons })
        .add_system(increment_time)
        .add_system(log_output_and_exit)
        .add_plugin(physics::PhysicsPlugin)
        .add_plugin(worm::WormPlugin)
        .add_plugin(brain::BrainPlugin)
        .add_startup_system(setup)
        .add_system(logger)
        .run();
}
