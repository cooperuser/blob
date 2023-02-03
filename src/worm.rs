use std::{collections::VecDeque, f32::consts::PI};

use bevy::prelude::*;

use crate::{physics::*, brain::{CTRNN, UpdateFlux}, TimeTracker, WormSettings};

const DRAG_NODE: f32 = 0.0;
const DRAG_EDGE: f32 = 1.0;
const SCALE: f32 = 0.5;

const SPRING_SOFT: f32 = 5.0 * 7.5;
const SPRING_HARD: f32 = 5.0 * 7.5;
const SPRING_SKELETON: f32 = 5.0 * 7.5;

#[derive(Debug)]
pub struct Segment<T> {
    index: usize,
    center: T,
    left: T,
    right: T
}

#[derive(Component)]
pub struct CyclicalMapping;
#[derive(Component)]
pub struct RegionalMapping;
#[derive(Component)]
pub struct ManualControl;
#[derive(Component)]
pub struct FrequencyMapping {
    pub frequency: f32,
    pub phase: f32
}
#[derive(Component)]
pub struct Neurons(pub Vec<f32>);

#[derive(Component)]
pub struct SpringHidden;

#[derive(Component)]
pub struct WormController {
    func: fn(f32, f32, f32) -> f32,
    pub segments: Vec<Segment<Entity>>
}

#[derive(Component)]
pub struct Control {
    pub index: i32,
    pub side: f32
}

#[derive(Component)]
pub struct Index(usize);

fn gen_segments(num_segments: i32) -> Vec<Segment<Vec3>> {
    let offset = 0.5;
    (0..num_segments).map(|i| Segment {
        index: i as usize,
        center: Vec3::new(-i as f32 - 1.0, 0.0, 0.0),
        left: Vec3::new(-i as f32 - offset, -offset, 0.0),
        right: Vec3::new(-i as f32 - offset, offset, 0.0),
    }).collect()
}

pub fn worm_builder(
    num_segments: usize,
    position: Vec3,
    commands: &mut Commands,
    controller: fn(f32, f32, f32) -> f32,
    neurons: usize
) -> Entity {
    let ctrnn = CTRNN::trained_ctrnn();
    let voltages = ctrnn.init_voltage();

    let mut parts = vec![];
    let parent_id = commands.spawn((
        Transform::default().with_translation(position),
        GlobalTransform::default(),
        VisibilityBundle::default(),
        CTRNN {
            ctrnn,
            voltages,
            output_history: VecDeque::new(),
            flux_history: vec![],
            activity_history: vec![],
            fitness_history: vec![],
            fitness_sum: vec![],
            avg_fitness_sum: vec![]
        },
        UpdateFlux,
        Neurons(vec![0.0; neurons])
    )).with_children(|parent| {
        let head = parent.spawn((
            Position::new(Vec3::ZERO),
            Force::default(),
            Mass(1.0),
            Drag(DRAG_NODE)
        )).id();

        let entities: Vec<Segment<Entity>> = gen_segments(num_segments as i32 + 1).iter()
            .map(|seg| Segment {
                index: seg.index,
                center: parent.spawn((
                    Position::new(seg.center * SCALE),
                    Force::default(),
                    Mass(1.0),
                    Drag(DRAG_NODE),
                    Index(seg.index)
                )).id(),
                left: parent.spawn((
                    Position::new(seg.left * SCALE),
                    Force::default(),
                    Mass(1.0),
                    Drag(DRAG_NODE)
                )).id(),
                right: parent.spawn((
                    Position::new(seg.right * SCALE),
                    Force::default(),
                    Mass(1.0),
                    Drag(DRAG_NODE)
                )).id(),
            }).collect();

        parent.spawn(Spring { a: entities[0].left, b: head, constant: SPRING_SOFT, length: 1.0 * SCALE });
        parent.spawn(Spring { a: entities[0].center, b: head, constant: SPRING_SKELETON, length: 1.0 * SCALE });
        parent.spawn(Spring { a: entities[0].right, b: head, constant: SPRING_SOFT, length: 1.0 * SCALE });
        parent.spawn(Spring { a: entities[0].center, b: entities[0].left, constant: SPRING_SOFT, length: 1.0 * SCALE });
        parent.spawn(Spring { a: entities[0].center, b: entities[0].right, constant: SPRING_SOFT, length: 1.0 * SCALE });

        for i in 1..num_segments + 1 {
            let new = &entities[i as usize];
            let old = &entities[(i - 1) as usize];

            parent.spawn(Spring { a: new.center, b: old.center, constant: SPRING_SKELETON, length: 1.0 * SCALE });
            parent.spawn(Spring { a: new.center, b: new.left, constant: SPRING_SOFT, length: 1.0 * SCALE });
            parent.spawn(Spring { a: new.center, b: new.right, constant: SPRING_SOFT, length: 1.0 * SCALE });
            parent.spawn(Spring { a: new.left, b: old.center, constant: SPRING_SOFT, length: 1.0 * SCALE });
            parent.spawn(Spring { a: new.right, b: old.center, constant: SPRING_SOFT, length: 1.0 * SCALE });
            parent.spawn((
                Spring { a: new.left, b: new.right, constant: SPRING_SOFT, length: 2.0 * SCALE },
                SpringHidden
            ));
            parent.spawn((
                Spring { a: new.left, b: old.left, constant: SPRING_HARD, length: 1.0 * SCALE },
                Control { index: i as i32, side: -1.0 },
                Drag(DRAG_EDGE)
            ));
            parent.spawn((
                Spring { a: new.right, b: old.right, constant: SPRING_HARD, length: 1.0 * SCALE },
                Control { index: i as i32, side: 1.0 },
                Drag(DRAG_EDGE)
            ));
        }

        parts = entities;
    }).id();

    commands.entity(parent_id).insert(
        WormController { func: controller, segments: parts }
    );

    parent_id
}

fn worm_control_system(
    worms: Query<(&WormController, &CTRNN), (
        Without<CyclicalMapping>,
        Without<RegionalMapping>,
        Without<FrequencyMapping>
    )>,
    mut nodes: Query<(&Parent, &mut Spring, &Control)>,
    time: Res<TimeTracker>
) {
    for (parent, mut spring, control) in nodes.iter_mut() {
        if let Ok((worm, _ctrnn)) = worms.get(parent.get()) {
            spring.length = (worm.func)(
                time.0,
                control.index as f32,
                control.side
            );
        }
    }
}

fn cyclical_neuron_mapping(
    worms: Query<(&WormController, &Neurons), With<CyclicalMapping>>,
    mut springs: Query<(&Parent, &mut Spring, &Control)>,
) {
    for (parent, mut spring, control) in springs.iter_mut() {
        if let Ok((_worm, neurons)) = worms.get(parent.get()) {
            let outputs = neurons.0.clone();
            let index = control.index - 1;
            let index = index % outputs.len() as i32;
            let value = outputs[index as usize] as f32 - 0.5;
            spring.length = 0.5 + value * 0.5 * control.side;
        }
    }
}

fn regional_neuron_mapping(
    worms: Query<(&WormController, &Neurons), With<RegionalMapping>>,
    mut springs: Query<(&Parent, &mut Spring, &Control)>,
) {
    for (parent, mut spring, control) in springs.iter_mut() {
        if let Ok((worm, neurons)) = worms.get(parent.get()) {
            let outputs = neurons.0.clone();
            let len = (worm.segments.len() - 1) as f32;
            let neurons = outputs.len() as f32;
            let index = (control.index - 1) as f32;
            let index = (index / len * neurons).floor();
            let value = outputs[index as usize] as f32 - 0.5;
            spring.length = 0.5 + value * 0.5 * control.side;
        }
    }
}

fn frequency_neuron_mapping(
    worms: Query<(&WormController, &CTRNN, &FrequencyMapping)>,
    mut springs: Query<(&Parent, &mut Spring, &Control)>,
    time: Res<TimeTracker>
) {
    for (parent, mut spring, control) in springs.iter_mut() {
        if let Ok((_worm, _ctrnn, fm)) = worms.get(parent.get()) {
            let phase = control.index as f32 * std::f32::consts::PI / fm.phase;
            let u = (-time.0 * 60.0 / fm.frequency + phase).sin() * control.side;
            spring.length = 0.5 + u * 0.2
        }
    }
}

fn add_worm_segment(
    mut worms: Query<(Entity, &mut WormController)>,
    positions: Query<(Entity, &Position)>,
    mut commands: Commands,
    keys: Res<Input<KeyCode>>
) {
    if keys.just_pressed(KeyCode::Space) {
        if let Ok((entity, mut worm)) = worms.get_single_mut() {
            let length = worm.segments.len();
            let last = &worm.segments[length - 1].center;
            let prev = &worm.segments[length - 2].center;
            let last = positions.get(*last).unwrap().1.now;
            let prev = positions.get(*prev).unwrap().1.now;
            let diff = prev - last;
            commands.entity(entity).with_children(|parent| {
                let center = last - diff;
                let half = last - diff * 0.5;
                let perp = diff.cross(Vec3::new(0.0, 0.0, 1.0)) / 1.0;
                let seg = Segment {
                    index: length,
                    center,
                    left: Vec3::new(half.x + perp.x, half.y + perp.y, 0.0),
                    right: Vec3::new(half.x - perp.x, half.y - perp.y, 0.0),
                };
                let old = &worm.segments[length - 1];
                let old = Segment {
                    index: old.index,
                    center: positions.get(old.center).unwrap().0,
                    left: positions.get(old.left).unwrap().0,
                    right: positions.get(old.right).unwrap().0
                };
                let new = Segment {
                    index: seg.index,
                    center: parent.spawn((
                        Position::new(seg.center),
                        Force::default(),
                        Mass(1.0),
                        Drag(DRAG_NODE),
                        Index(seg.index)
                    )).id(),
                    left: parent.spawn((
                        Position::new(seg.left),
                        Force::default(),
                        Mass(1.0),
                        Drag(DRAG_NODE),
                    )).id(),
                    right: parent.spawn((
                        Position::new(seg.right),
                        Force::default(),
                        Mass(1.0),
                        Drag(DRAG_NODE),
                    )).id()
                };

                parent.spawn(Spring { a: new.center, b: old.center, constant: SPRING_SKELETON, length: 1.0 * SCALE });
                parent.spawn(Spring { a: new.center, b: new.left, constant: SPRING_SOFT, length: 1.0 * SCALE });
                parent.spawn(Spring { a: new.center, b: new.right, constant: SPRING_SOFT, length: 1.0 * SCALE });
                parent.spawn(Spring { a: new.left, b: old.center, constant: SPRING_SOFT, length: 1.0 * SCALE });
                parent.spawn(Spring { a: new.right, b: old.center, constant: SPRING_SOFT, length: 1.0 * SCALE });
                parent.spawn((
                    Spring { a: new.left, b: new.right, constant: SPRING_SOFT, length: 2.0 * SCALE },
                    SpringHidden
                ));
                parent.spawn((
                    Spring { a: new.left, b: old.left, constant: SPRING_HARD, length: 1.0 * SCALE },
                    Control { index: seg.index as i32, side: -1.0 },
                    Drag(DRAG_EDGE)
                ));
                parent.spawn((
                    Spring { a: new.right, b: old.right, constant: SPRING_HARD, length: 1.0 * SCALE },
                    Control { index: seg.index as i32, side: 1.0 },
                    Drag(DRAG_EDGE)
                ));

                worm.segments.push(new);
            });
        }
    }
}

fn manually_adjust_neurons(
    mut neurons: Query<&mut Neurons, With<ManualControl>>,
    time: Res<TimeTracker>,
    settings: Res<WormSettings>
) {
    let t = time.0 * 2.0 * PI * settings.frequency;
    for mut neurons in neurons.iter_mut() {
        let map = |t: f32| -> f32 { t.sin() * 0.4 + 0.5 };
        for n in 0..neurons.0.len() {
            let offset = settings.phase * n as f32;
            neurons.0[n] = map(t - offset);
        }
    }
}

fn adjust_neurons(
    mut neurons: Query<(&mut Neurons, &CTRNN), Without<ManualControl>>
) {
    for (mut neurons, ctrnn) in neurons.iter_mut() {
        neurons.0 = ctrnn.get_outputs().iter().map(|e| *e as f32).collect();
    }
}

pub struct WormPlugin;
impl Plugin for WormPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(worm_control_system);
        app.add_system(cyclical_neuron_mapping);
        app.add_system(regional_neuron_mapping);
        app.add_system(frequency_neuron_mapping);
        app.add_system(add_worm_segment);
        app.add_system(manually_adjust_neurons);
        app.add_system(adjust_neurons);
    }
}
