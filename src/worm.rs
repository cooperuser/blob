use crate::physics::{Position, Force, Mass, Spring, Drag, Control};
use crate::vector::Vector;

use bevy::prelude::*;

struct Segment<T> {
    center: T,
    left: T,
    right: T
}

pub fn worm_builder(
    mut commands: Commands
) {
    let num_segments = 16;
    let s = 0.5;
    let drag_node = 0.0;
    let drag_edge = 1.0;

    let head = commands.spawn((
        Position::new(Vector::zero()),
        Force(Vector::zero()),
        Mass(1.0),
        Drag(drag_node)
    )).id();

    let entities: Vec<Segment<Entity>> = gen_segments(num_segments).iter()
        .map(|seg| Segment {
            center: commands.spawn((
                Position::new(seg.center * s),
                Force(Vector::zero()),
                Mass(1.0),
                Drag(drag_node)
            )).id(),
            left: commands.spawn((
                Position::new(seg.left * s),
                Force(Vector::zero()),
                Mass(1.0),
                Drag(drag_node)
            )).id(),
            right: commands.spawn((
                Position::new(seg.right * s),
                Force(Vector::zero()),
                Mass(1.0),
                Drag(drag_node)
            )).id(),
        }).collect();

    let soft = 7.5;
    let hard = 7.5;
    let skeleton = 7.5;

    commands.spawn(Spring { a: entities[0].left, b: head, constant: soft, length: 1.0 * s });
    commands.spawn(Spring { a: entities[0].center, b: head, constant: skeleton, length: 1.0 * s });
    commands.spawn(Spring { a: entities[0].right, b: head, constant: soft, length: 1.0 * s });
    commands.spawn(Spring { a: entities[0].center, b: entities[0].left, constant: soft, length: 1.0 * s });
    commands.spawn(Spring { a: entities[0].center, b: entities[0].right, constant: soft, length: 1.0 * s });

    for i in 1..num_segments {
        let new = &entities[i as usize];
        let old = &entities[(i - 1) as usize];

        commands.spawn(Spring { a: new.center, b: old.center, constant: skeleton, length: 1.0 * s });
        commands.spawn(Spring { a: new.center, b: new.left, constant: soft, length: 1.0 * s });
        commands.spawn(Spring { a: new.center, b: new.right, constant: soft, length: 1.0 * s });
        commands.spawn(Spring { a: new.left, b: old.center, constant: soft, length: 1.0 * s });
        commands.spawn(Spring { a: new.right, b: old.center, constant: soft, length: 1.0 * s });
        commands.spawn((
            Spring { a: new.left, b: old.left, constant: hard, length: 1.0 * s },
            Control { index: i, side: -1.0 },
            Drag(drag_edge)
        ));
        commands.spawn((
            Spring { a: new.right, b: old.right, constant: hard, length: 1.0 * s },
            Control { index: i, side: 1.0 },
            Drag(drag_edge)
        ));
    }
}

fn gen_segments(num_segments: i32) -> Vec<Segment<Vector>> {
    let offset = 0.5;
    (0..num_segments).map(|i| Segment {
        center: Vector::new(-i as f32 - 1.0, 0.0),
        left: Vector::new(-i as f32 - offset, -offset),
        right: Vector::new(-i as f32 - offset, offset),
    }).collect()
}
