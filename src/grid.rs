use crate::vector::Vector;

use bevy::prelude::{Camera, ResMut, Query, GlobalTransform, Vec3, Color};
use bevy_prototype_debug_lines::DebugLines;

pub fn draw_grid(
    camera: Query<(&GlobalTransform, &Camera)>,
    mut lines: ResMut<DebugLines>
) {
    let (transform, camera) = camera.single();
    let min = {
        let t = camera.ndc_to_world(transform, Vec3::new(-1., -1., 0.)).unwrap();
        Vector::new(t.x, t.y)
    };
    let max = {
        let t = camera.ndc_to_world(transform, Vec3::new(1., 1., 0.)).unwrap();
        Vector::new(t.x, t.y)
    };

    if max.x - min.x >= 100.0 { return; }
    for x in min.x as isize ..= max.x as isize {
        let color = match x {
            0 => Color::WHITE,
            _ if x % 4 == 0 => Color::GRAY,
            _ => Color::BLACK
        };
        lines.line_colored(
            Vec3::new(x as f32, min.y, 0.),
            Vec3::new(x as f32, max.y, 0.),
            0.,
            color
        )
    }

    for y in min.y as isize ..= max.y as isize {
        let color = match y {
            0 => Color::WHITE,
            _ if y % 4 == 0 => Color::GRAY,
            _ => Color::BLACK
        };
        lines.line_colored(
            Vec3::new(min.x, y as f32, 0.),
            Vec3::new(max.x, y as f32, 0.),
            0.,
            color
        )
    }
}
