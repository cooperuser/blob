use bevy::prelude::*;
use bevy_egui::egui::{self, plot::{Plot, Line, PlotPoints, PlotBounds}, Vec2};

use crate::brain::CTRNN;

fn outputs_graph(mut egui_context: ResMut<bevy_egui::EguiContext>, ctrnns: Query<&CTRNN>) {
    let default = vec![0.0, 0.0, 0.0];
    if let Ok(ctrnn) = ctrnns.get_single() {
        egui::Window::new("Outputs")
            .default_size(Vec2::new(300.0, 300.0))
            .show(egui_context.ctx_mut(), |ui| {
            let line = Line::new(PlotPoints::from_parametric_callback(
                move |t| {
                    let index = (ctrnn.output_history.len() as f64 * t) as usize;
                    let elem = ctrnn.output_history.get(index).unwrap_or(&default);
                    (elem[0], elem[1])
                },
                0.0..1.0,
                100
            )).name("output");
            Plot::new("plot").show(ui, |plot_ui| {
                plot_ui.set_plot_bounds(PlotBounds::from_min_max([0.0, 0.0], [1.0, 1.0]));
                plot_ui.line(line);
            })
        });
    }
}

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_egui::EguiPlugin);
        app.add_system(outputs_graph);
    }
}
