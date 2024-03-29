use bevy::prelude::*;
use bevy_egui::egui::{self, plot::{Plot, Line, PlotPoints, PlotBounds}, Vec2};

use crate::{brain::CTRNN, worm::Neurons};

fn phase_portrait(mut egui_context: ResMut<bevy_egui::EguiContext>, ctrnns: Query<&CTRNN>) {
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

fn flux_graph(mut egui_context: ResMut<bevy_egui::EguiContext>, ctrnns: Query<&CTRNN>) {
    let default = (0.0, 0.0);
    let weight = (1, 1);
    if let Ok(ctrnn) = ctrnns.get_single() {
        egui::Window::new("Flux")
            .default_size(Vec2::new(300.0, 300.0))
            .show(egui_context.ctx_mut(), |ui| {
                let center = Line::new(PlotPoints::from_parametric_callback(
                    move |t| {
                        if ctrnn.flux_history.len() < 1 { return (t, 0.0) }
                        let index = (ctrnn.output_history.len() as f64 * t) as usize;
                        let len = ctrnn.flux_history.len();
                        let weight = (weight.0.min(len), weight.1.min(len));
                        let elem = ctrnn.flux_history[weight.0][weight.1].get(index).unwrap_or(&default);
                        (t, elem.0)
                    },
                    0.0..1.0,
                    100
                )).name("center");
                let value = Line::new(PlotPoints::from_parametric_callback(
                    move |t| {
                        if ctrnn.flux_history.len() < 1 { return (t, 0.0) }
                        let index = (ctrnn.output_history.len() as f64 * t) as usize;
                        let len = ctrnn.flux_history.len();
                        let weight = (weight.0.min(len), weight.1.min(len));
                        let elem = ctrnn.flux_history[weight.0][weight.1].get(index).unwrap_or(&default);
                        (t, elem.1)
                    },
                    0.0..1.0,
                    100
                )).name("value");
                Plot::new("plot_flux").show(ui, |plot_ui| {
                    // plot_ui.set_plot_bounds(PlotBounds::from_min_max([0.0, 0.0], [1.0, 1.0]));
                    plot_ui.line(center);
                    plot_ui.line(value);
                })
            });
    }
}

fn outputs_graph(mut egui_context: ResMut<bevy_egui::EguiContext>, neurons: Query<&Neurons>) {
    if let Ok(neurons) = neurons.get_single() {
        egui::Window::new("Activity")
            .default_size(Vec2::new(300.0, 300.0))
            .show(egui_context.ctx_mut(), |ui| {
                for neuron in &neurons.0 {
                    ui.add(egui::Slider::new(&mut neuron.clone(), 0.0..=1.0));
                }
            }
            );
    }
}

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_egui::EguiPlugin);
        app.add_system(phase_portrait);
        app.add_system(flux_graph);
        app.add_system(outputs_graph);
    }
}
