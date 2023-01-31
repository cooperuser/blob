use std::{collections::VecDeque, f32::consts::PI};

use bevy::prelude::*;

use crate::{physics::*, brain::{CTRNN, UpdateFlux}, TimeTracker, WormSettings};

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
        let drag_node = 0.0;
        let drag_edge = 1.0;
        let s = 0.5;
        let head = parent.spawn((
            Position::new(Vec3::ZERO),
            Force::default(),
            Mass(1.0),
            Drag(drag_node)
        )).id();

        let entities: Vec<Segment<Entity>> = gen_segments(num_segments as i32 + 1).iter()
            .map(|seg| Segment {
                index: seg.index,
                center: parent.spawn((
                    Position::new(seg.center * s),
                    Force::default(),
                    Mass(1.0),
                    Drag(drag_node),
                    Index(seg.index)
                )).id(),
                left: parent.spawn((
                    Position::new(seg.left * s),
                    Force::default(),
                    Mass(1.0),
                    Drag(drag_node)
                )).id(),
                right: parent.spawn((
                    Position::new(seg.right * s),
                    Force::default(),
                    Mass(1.0),
                    Drag(drag_node)
                )).id(),
            }).collect();

        let soft = 2.0 * 7.5;
        let hard = 7.5;
        let skeleton = 7.5;

        parent.spawn(Spring { a: entities[0].left, b: head, constant: soft, length: 1.0 * s });
        parent.spawn(Spring { a: entities[0].center, b: head, constant: skeleton, length: 1.0 * s });
        parent.spawn(Spring { a: entities[0].right, b: head, constant: soft, length: 1.0 * s });
        parent.spawn(Spring { a: entities[0].center, b: entities[0].left, constant: soft, length: 1.0 * s });
        parent.spawn(Spring { a: entities[0].center, b: entities[0].right, constant: soft, length: 1.0 * s });

        for i in 1..num_segments + 1 {
            let new = &entities[i as usize];
            let old = &entities[(i - 1) as usize];

            parent.spawn(Spring { a: new.center, b: old.center, constant: skeleton, length: 1.0 * s });
            parent.spawn(Spring { a: new.center, b: new.left, constant: soft, length: 1.0 * s });
            parent.spawn(Spring { a: new.center, b: new.right, constant: soft, length: 1.0 * s });
            parent.spawn(Spring { a: new.left, b: old.center, constant: soft, length: 1.0 * s });
            parent.spawn(Spring { a: new.right, b: old.center, constant: soft, length: 1.0 * s });
            parent.spawn((
                Spring { a: new.left, b: old.left, constant: hard, length: 1.0 * s },
                Control { index: i as i32, side: -1.0 },
                Drag(drag_edge)
            ));
            parent.spawn((
                Spring { a: new.right, b: old.right, constant: hard, length: 1.0 * s },
                Control { index: i as i32, side: 1.0 },
                Drag(drag_edge)
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
            let len = worm.segments.len();
            let index = control.index - 1;
            // let index = index / 6 % outputs.len() as i32;
            // TODO: Fix this from panicking when len == 5
            let index = index / (len / outputs.len()) as i32;
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
    positions: Query<&Position>,
    mut commands: Commands
) {
    if let Ok((entity, mut worm)) = worms.get_single_mut() {
        let length = worm.segments.len();
        let last = &worm.segments[length - 1].center;
        let prev = &worm.segments[length - 2].center;
        let last = positions.get(*last).unwrap().now;
        let prev = positions.get(*prev).unwrap().now;
        let diff = prev - last;
        commands.entity(entity).with_children(|parent| {
            // TODO: finish this
        });

        // commands.entity(entity).with_children(|parent| {

        // });
    }
}

fn manually_adjust_neurons(
    mut neurons: Query<&mut Neurons, With<ManualControl>>,
    time: Res<TimeTracker>,
    settings: Res<WormSettings>
) {
    let t = time.0 * 2.0 * PI * settings.frequency;
    // let t = 2.0 * PI * settings.frequency;
    // let t = t / time.0;
    for mut neurons in neurons.iter_mut() {
        let map = |t: f32| -> f32 { t.sin() * 0.4 + 0.5 };
        for n in 0..neurons.0.len() {
            // let offset = settings.phase * n as f32 / neurons.0.len() as f32;
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
