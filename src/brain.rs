use ctrnn::RLCTRNN;

use bevy::prelude::*;

#[derive(Component)]
pub struct CTRNN {
    pub ctrnn: ctrnn::RLCTRNN,
    pub voltages: Vec<f64>
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

        ctrnn.add_node();

        ctrnn
    }
}


pub fn ctrnn_update(mut ctrnns: Query<&mut CTRNN>) {
    for mut ctrnn in ctrnns.iter_mut() {
        let voltages = &ctrnn.voltages.clone();
        ctrnn.voltages = ctrnn.ctrnn.update(0.05, voltages, vec![]);
    }
}
