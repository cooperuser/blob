use specs::{World, WorldExt, Builder, Entity};

use crate::Log;
use crate::vector::Vector;
use crate::physics::{Position, Force, Mass, Spring, Drag, Control};

struct Segment<T> {
    center: T,
    left: T,
    right: T
}

pub fn builder(world: &mut World, num_segments: i32) {
    let s = 0.5;

    if num_segments < 1 { panic!("Not enough worm segments") }

    let head = world.create_entity()
        .with(Position::default())
        .with(Force::default())
        .with(Drag::default())
        .with(Mass(1.0))
        .with(Log)
        .build();

    let entities: Vec<Segment<Entity>> = gen_segments(num_segments).iter()
        .map(|seg| Segment {
            center: world.create_entity()
                .with(Position::new(seg.center * s))
                .with(Force::default())
                .with(Drag::default())
                .with(Mass(1.0))
                .with(Log)
                .build(),
            left: world.create_entity()
                .with(Position::new(seg.left * s))
                .with(Force::default())
                .with(Drag::default())
                .with(Mass(1.0))
                .with(Log)
                .build(),
            right: world.create_entity()
                .with(Position::new(seg.right * s))
                .with(Force::default())
                .with(Drag::default())
                .with(Mass(1.0))
                .with(Log)
                .build(),
        }).collect();

    let soft = 1.5*5.0;
    let hard = 0.5*10.0;
    let skeleton = 0.5*15.0;

    world.create_entity()
        .with(Spring { a: entities[0].left, b: head, constant: soft, length: 1.0 * s })
        .build();
    world.create_entity()
        .with(Spring { a: entities[0].center, b: head, constant: skeleton, length: 1.0 * s })
        .build();
    world.create_entity()
        .with(Spring { a: entities[0].right, b: head, constant: soft, length: 1.0 * s })
        .build();
    world.create_entity()
        .with(Spring { a: entities[0].center, b: entities[0].left, constant: soft, length: 1.0 * s })
        .build();
    world.create_entity()
        .with(Spring { a: entities[0].center, b: entities[0].right, constant: soft, length: 1.0 * s })
        .build();

    for i in 1..num_segments {
        let new = &entities[i as usize];
        let old = &entities[(i - 1) as usize];

        world.create_entity()
            .with(Spring { a: new.center, b: old.center, constant: skeleton, length: 1.0 * s })
            .with(Drag::default())
            .build();
        world.create_entity()
            .with(Spring { a: new.center, b: new.left, constant: soft, length: 1.0 * s })
            .build();
        world.create_entity()
            .with(Spring { a: new.center, b: new.right, constant: soft, length: 1.0 * s })
            .build();
        world.create_entity()
            .with(Spring { a: new.left, b: old.center, constant: soft, length: 1.0 * s })
            .build();
        world.create_entity()
            .with(Spring { a: new.right, b: old.center, constant: soft, length: 1.0 * s })
            .build();
        world.create_entity()
            .with(Spring { a: new.left, b: old.left, constant: hard, length: 1.0 * s })
            // .with(Drag::default())
            .with(Control { index: i, side: -1.0 })
            .build();
        world.create_entity()
            .with(Spring { a: new.right, b: old.right, constant: hard, length: 1.0 * s })
            // .with(Drag::default())
            .with(Control { index: i, side: 1.0 })
            .build();
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
