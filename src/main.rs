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

fn setup(mut commands: Commands,) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection { scale: 0.01, ..default() },
        transform: Transform::default().with_translation(Vec3::Z),
        ..default()
    }).insert(PanCam::default());
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
                material: materials.add(ColorMaterial::from(Color::RED)),
                ..default()
            });
        }
    }
}

fn sync_edges(
    query: Query<&Spring>,
    positions: Query<&Position>,
    mut lines: ResMut<DebugLines>
) {
    for spring in query.iter() {
        let a = positions.get(spring.a).unwrap().now;
        let b = positions.get(spring.b).unwrap().now;
        lines.line(a.as_vec3(0.), b.as_vec3(0.), 0.);
    }
}

fn logger(positions: Query<(&Position, &Force), With<Log>>) {
    for (pos, _) in positions.iter() {
        println!("{:?}", pos.now);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_system(bevy::window::close_on_esc)
        .add_plugin(PanCamPlugin::default())
        .add_plugin(physics::PhysicsPlugin)
        .add_plugin(DebugLinesPlugin::with_depth_test(true))
        .add_startup_system(setup)
        .add_startup_system(worm::worm_builder)
        .add_system(logger)
        .add_system(sync_points)
        .add_system(sync_edges)
        .add_system(draw_grid)
        .run();
}
