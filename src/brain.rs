use std::collections::VecDeque;

use ctrnn::RLCTRNN;

use bevy::prelude::*;

use crate::HISTORY_LENGTH;

#[derive(Component)]
pub struct UpdateFlux;
#[derive(Component)]
pub struct LogCTRNN;

#[derive(Component)]
pub struct CTRNN {
    pub ctrnn: ctrnn::RLCTRNN,
    pub voltages: Vec<f64>,
    pub output_history: VecDeque<Vec<f64>>,
    pub flux_history: Vec<Vec<VecDeque<(f64, f64)>>>
}

impl CTRNN {
    pub fn get_outputs(&self) -> Vec<f64> {
        self.ctrnn.get_outputs(&self.voltages)
    }

    pub fn trained_ctrnn() -> RLCTRNN {
        let mut ctrnn = ctrnn::RLCTRNN::new(2);
        ctrnn
            .set_bias(0, -2.75)
            .set_bias(1, -1.75)
            .set_weight(0, 0, 4.5)
            .set_weight(0, 1, -1.0)
            .set_weight(1, 0, 1.0)
            .set_weight(1, 1, 4.5);

        for from in 0..ctrnn.count {
            ctrnn.biases[from].range_period = 6.0..12.0;
            for to in 0..ctrnn.count {
                ctrnn.weights[to][from].range_period = 6.0..12.0;
            }
        }

        ctrnn.add_node();

        ctrnn
    }
}


fn ctrnn_update(mut ctrnns: Query<&mut CTRNN>) {
    for mut ctrnn in ctrnns.iter_mut() {
        let voltages = &ctrnn.voltages.clone();
        ctrnn.voltages = ctrnn.ctrnn.update(0.05, voltages, vec![]);
    }
}

fn ctrnn_history(mut ctrnns: Query<&mut CTRNN>) {
    for mut ctrnn in ctrnns.iter_mut() {
        let outputs = ctrnn.get_outputs();
        ctrnn.output_history.push_back(outputs);
        if ctrnn.output_history.len() > HISTORY_LENGTH {
            ctrnn.output_history.pop_front();
        }

        for to in 0..ctrnn.ctrnn.count {
            if to >= ctrnn.flux_history.len() {
                ctrnn.flux_history.push(vec![]);
            }
            for from in 0..ctrnn.ctrnn.count {
                if from >= ctrnn.flux_history[to].len() {
                    ctrnn.flux_history[to].push(VecDeque::new());
                }
                let center = ctrnn.ctrnn.weights[to][from].center;
                let value = ctrnn.ctrnn.weights[to][from].get();
                ctrnn.flux_history[to][from].push_back((center, value));
                if ctrnn.flux_history[to][from].len() > HISTORY_LENGTH {
                    ctrnn.flux_history[to][from].pop_front();
                }
            }
        }
    }
}

fn fluctuator_update(mut ctrnns: Query<&mut CTRNN, With<UpdateFlux>>) {
    for mut ctrnn in ctrnns.iter_mut() {
        for from in 0..ctrnn.ctrnn.count {
            for to in 0..ctrnn.ctrnn.count {
                let f = &mut ctrnn.ctrnn.weights[to][from];
                f.update(1.0 / 60.0, 0.0);
            }
        }
    }
}

fn log_ctrnn(ctrnns: Query<&CTRNN, With<LogCTRNN>>) {
    for ctrnn in ctrnns.iter() {
        let outputs = ctrnn.get_outputs();
        println!("{:.3?}", outputs);
    }
}

pub struct BrainPlugin;
impl Plugin for BrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(ctrnn_update);
        app.add_system(ctrnn_history);
        app.add_system(fluctuator_update.before(ctrnn_update));
        app.add_system(log_ctrnn);
    }
}
