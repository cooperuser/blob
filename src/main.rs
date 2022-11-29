use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_pancam::*;
use bevy_prototype_debug_lines::*;

mod physics;
mod vector;
mod worm;
mod grid;

use grid::draw_grid;
use physics::*;

#[derive(Component)]
struct Log;

fn setup(mut commands: Commands) {
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

    worm::worm_builder(15, Vec3::ZERO, &mut commands, |time, index, side| {
        default_control(3.0, 50.0, time, index, side)
    });

    worm::worm_builder(15, Vec3::new(0., -4., 0.), &mut commands, |time, index, side| {
        default_control(6.0, 250.0, time, index, side)
    });

    worm::worm_builder(15, Vec3::new(0., 4., 0.), &mut commands, |time, index, side| {
        default_control(6.0, 200.0, time, index, side)
    });
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
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                ..default()
            });
        }
    }
}

fn sync_edges(
    query: Query<&Spring>,
    transforms: Query<&GlobalTransform>,
    mut lines: ResMut<DebugLines>
) {
    for spring in query.iter() {
        let a = match transforms.get(spring.a) {
            Ok(t) => t.translation(),
            Err(_) => Vec3::ZERO
        };
        let b = match transforms.get(spring.b) {
            Ok(t) => t.translation(),
            Err(_) => Vec3::ZERO
        };
        lines.line(a, b, 0.);
    }
}

fn logger(positions: Query<(&Position, &Force), With<Log>>) {
    for (pos, _) in positions.iter() {
        println!("{:?}", pos.now);
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                fit_canvas_to_parent: true,
                ..default()
            },
            ..default()
        }))
        .add_system(bevy::window::close_on_esc)
        .add_plugin(PanCamPlugin::default())
        // .add_plugin(physics::PhysicsPlugin)
        .add_plugin(physics::PhysicsPlugin)
        .add_plugin(DebugLinesPlugin::with_depth_test(true))
        .add_startup_system(setup)
        // .add_startup_system(worm::worm_builder)
        .add_system(logger)
        .add_system(draw_grid.before(sync_edges))
        .add_system(sync_points)
        .add_system(sync_edges)
        .add_system(worm::worm_control_system)
        .run();
}
