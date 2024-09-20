use std::ops::RangeInclusive;

use bevy::app::{App, Plugin};
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_easy_compute::prelude::AppComputeWorker;
use bevy_egui::{egui, EguiContexts};
use bevy_egui::egui::{FontId, RichText};

use crate::{ExportButtonClicked, RngSeed};
use crate::compute::asteroid_terrain_generator::AsteroidComputeWorker;
use crate::settings::asteroid_settings::AsteroidSettings;
use crate::utils::PRNG;

pub struct UIAsteroidSettings;

impl Plugin for UIAsteroidSettings {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, show_ui);
    }
}

fn show_ui(mut contexts: EguiContexts,
           diagnostic: Res<DiagnosticsStore>,
           mut settings: ResMut<AsteroidSettings>,
           mut compute_worker: ResMut<AppComputeWorker<AsteroidComputeWorker>>,
           seed: ResMut<RngSeed>,
           mut export_clicked: EventWriter<ExportButtonClicked>,
) {
    egui::Window::new("Settings")
        .scroll([false, true])
        .default_height(860.)
        .default_width(420.)
        .show(contexts.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    if let Some(fps) = diagnostic.get(&FrameTimeDiagnosticsPlugin::FPS) {
                        if let Some(value) = fps.smoothed() {
                            ui.label(RichText::new(format!("FPS: {value:.2}")).font(FontId::proportional(20.0)));
                        }
                    }
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(10f32);
                    let export_button = egui::Button::new(
                        RichText::new("Export glb")
                            .strong()
                            .font(FontId::proportional(20.0))
                            .color(egui::Color32::WHITE))
                        .fill(egui::Color32::from_rgb(99, 181, 74)); // Light green color

                    if ui.add(export_button).clicked() {}
                });
            });
            let mut crater_settings_changed = false;
            let mut simple_noise_settings_changed = false;
            let mut ridge_noise_settings_changed = false;
            let mut ridge_noise_settings2_changed = false;

            let slider = |ui: &mut egui::Ui, label: &str, value: &mut f32, step: f64, range: RangeInclusive<f32>, changed: &mut bool| {
                ui.style_mut().spacing.slider_width = 200f32;

                let response = ui.add(
                    egui::Slider::new(value, range)
                        .text(label)
                        .step_by(step)
                        .clamp_to_range(true),
                );
                if response.changed() {
                    *changed = true;
                }
            };

            let drag_value = |ui: &mut egui::Ui, label: &str, value: &mut f32, changed: &mut bool| {
                ui.horizontal(|ui| {
                    ui.label(label);
                    let response = ui.add(
                        egui::DragValue::new(value)
                            .speed(0.01)
                            .fixed_decimals(2)
                            .custom_formatter(|n, _| format!("{:.2}", n))
                            .custom_parser(|s| s.parse::<f64>().ok())
                    );
                    if response.changed() {
                        *changed = true;
                    }
                });
            };

            let offset = |ui: &mut egui::Ui, x: &mut f32, y: &mut f32, z: &mut f32, changed: &mut bool| {
                ui.horizontal(|ui| {
                    drag_value(ui, "X:", x, changed);
                    drag_value(ui, "Y:", y, changed);
                    drag_value(ui, "Z:", z, changed);
                });
            };

            let spacing = 20f32;

            let crater_settings = &mut settings.crater_settings;

            egui::CollapsingHeader::new(RichText::new("Crater Settings").font(FontId::proportional(20.0)))
                .default_open(true)
                .show(ui, |ui| {
                    slider(ui, "Number of Craters", &mut crater_settings.num_craters, 10f64, 0.0..=2000., &mut crater_settings_changed);
                    slider(ui, "Crater size min", &mut crater_settings.crater_size_min, 0.01f64, 0.0..=1., &mut crater_settings_changed);
                    slider(ui, "Crater size max", &mut crater_settings.crater_size_max, 0.01f64, 0.1..=1., &mut crater_settings_changed);
                    slider(ui, "Rim steepness", &mut crater_settings.rim_steepness, 0.01f64, 0.0..=2., &mut crater_settings_changed);
                    slider(ui, "Rim Width", &mut crater_settings.rim_width, 0.01f64, 0.0..=5., &mut crater_settings_changed);
                    slider(ui, "Smooth min", &mut crater_settings.smooth_min, 0.01f64, 0.0..=1., &mut crater_settings_changed);
                    slider(ui, "Smooth max", &mut crater_settings.smooth_max, 0.01f64, 0.1..=2., &mut crater_settings_changed);
                    slider(ui, "Size distribution", &mut crater_settings.size_distribution, 0.01f64, 0.0..=1., &mut crater_settings_changed);
                });
            ui.add_space(spacing);
            crater_settings.crater_size_min = crater_settings.crater_size_min.min(crater_settings.crater_size_max);

            if crater_settings_changed
            {
                let craters = crater_settings.get_craters(seed.0);

                compute_worker.write_slice("num_craters", &[craters.len() as u32]);
                compute_worker.write_slice("rim_steepness", &[crater_settings.get_rim_steepness()]);
                compute_worker.write_slice("rim_width", &[crater_settings.get_rim_width()]);
                compute_worker.write_slice("craters", &craters);
                compute_worker.execute();
            }
            crater_settings_changed = false;

            let simple_noise_settings = &mut settings.simple_noise_settings;
            egui::CollapsingHeader::new(RichText::new("Simple Noise Settings").font(FontId::proportional(20.0)))
                .default_open(true)
                .show(ui, |ui| {
                    slider(ui, "Number of layers", &mut simple_noise_settings.num_layers, 1f64, 0.0..=40., &mut simple_noise_settings_changed);
                    slider(ui, "Lacunarity", &mut simple_noise_settings.lacunarity, 0.1f64, 0.0..=5., &mut simple_noise_settings_changed);
                    slider(ui, "Persistence", &mut simple_noise_settings.persistence, 0.1f64, 0.0..=5., &mut simple_noise_settings_changed);
                    slider(ui, "Scale", &mut simple_noise_settings.scale, 0.1f64, 0.0..=10., &mut simple_noise_settings_changed);
                    slider(ui, "Elevation", &mut simple_noise_settings.elevation, 0.1f64, 0.0..=5., &mut simple_noise_settings_changed);
                    slider(ui, "Vertical Shift", &mut simple_noise_settings.vertical_shift, 0.1f64, 0.0..=5., &mut simple_noise_settings_changed);

                    ui.label("Offset:");
                    offset(ui, &mut simple_noise_settings.offset_x, &mut simple_noise_settings.offset_y, &mut simple_noise_settings.offset_z, &mut simple_noise_settings_changed);
                });
            ui.add_space(spacing);

            if simple_noise_settings_changed {
                let prng = PRNG::new(seed.0);
                let noise_params = simple_noise_settings.get_noise_params(prng);

                compute_worker.write_slice("noise_params_shape", &noise_params);
                compute_worker.execute();
            }
            simple_noise_settings_changed = false;


            let ridge_noise_settings = &mut settings.ridge_noise_settings;
            egui::CollapsingHeader::new(RichText::new("Ridge Noise Settings").font(FontId::proportional(20.0)))
                .default_open(true)
                .show(ui, |ui| {
                    slider(ui, "Number of layers", &mut ridge_noise_settings.num_layers, 1f64, 0.0..=40., &mut ridge_noise_settings_changed);
                    slider(ui, "Lacunarity", &mut ridge_noise_settings.lacunarity, 0.1f64, 0.0..=5., &mut ridge_noise_settings_changed);
                    slider(ui, "Persistence", &mut ridge_noise_settings.persistence, 0.1f64, 0.0..=5., &mut ridge_noise_settings_changed);
                    slider(ui, "Scale", &mut ridge_noise_settings.scale, 0.1f64, 0.0..=5., &mut ridge_noise_settings_changed);
                    slider(ui, "Power", &mut ridge_noise_settings.power, 0.1f64, 0.0..=5., &mut ridge_noise_settings_changed);
                    slider(ui, "Elevation", &mut ridge_noise_settings.elevation, 0.1f64, 0.0..=5., &mut ridge_noise_settings_changed);
                    slider(ui, "Gain", &mut ridge_noise_settings.gain, 0.1f64, 0.0..=10., &mut ridge_noise_settings_changed);
                    slider(ui, "Vertical Shift", &mut ridge_noise_settings.vertical_shift, 0.1f64, 0.0..=5., &mut ridge_noise_settings_changed);
                    slider(ui, "Peak Smoothing", &mut ridge_noise_settings.peak_smoothing, 0.1f64, 0.0..=5., &mut ridge_noise_settings_changed);

                    ui.label("Offset:");
                    offset(ui, &mut ridge_noise_settings.offset_x, &mut ridge_noise_settings.offset_y, &mut ridge_noise_settings.offset_z, &mut ridge_noise_settings_changed);
                });
            ui.add_space(20f32);

            if ridge_noise_settings_changed {
                let prng = PRNG::new(seed.0);
                let noise_params = ridge_noise_settings.get_noise_params(prng);

                compute_worker.write_slice("noise_params_ridge", &noise_params);
                compute_worker.execute();
            }

            ridge_noise_settings_changed = false;

            let ridge_noise_settings2 = &mut settings.ridge_noise_settings2;
            egui::CollapsingHeader::new(RichText::new("Ridge Noise Settings 2").font(FontId::proportional(20.0)))
                .default_open(true)
                .show(ui, |ui| {
                    slider(ui, "Number of layers", &mut ridge_noise_settings2.num_layers, 1f64, 0.0..=40., &mut ridge_noise_settings2_changed);
                    slider(ui, "Lacunarity", &mut ridge_noise_settings2.lacunarity, 0.1f64, 0.0..=5., &mut ridge_noise_settings2_changed);
                    slider(ui, "Persistence", &mut ridge_noise_settings2.persistence, 0.1f64, 0.0..=5., &mut ridge_noise_settings2_changed);
                    slider(ui, "Scale", &mut ridge_noise_settings2.scale, 0.1f64, 0.0..=5., &mut ridge_noise_settings2_changed);
                    slider(ui, "Power", &mut ridge_noise_settings2.power, 0.1f64, 0.0..=5., &mut ridge_noise_settings2_changed);
                    slider(ui, "Elevation", &mut ridge_noise_settings2.elevation, 0.1f64, 0.0..=5., &mut ridge_noise_settings2_changed);
                    slider(ui, "Gain", &mut ridge_noise_settings2.gain, 0.1f64, 0.0..=10., &mut ridge_noise_settings2_changed);
                    slider(ui, "Vertical Shift", &mut ridge_noise_settings2.vertical_shift, 0.1f64, 0.0..=5., &mut ridge_noise_settings2_changed);
                    slider(ui, "Peak Smoothing", &mut ridge_noise_settings2.peak_smoothing, 0.1f64, 0.0..=5., &mut ridge_noise_settings2_changed);

                    ui.label("Offset:");
                    offset(ui, &mut ridge_noise_settings2.offset_x, &mut ridge_noise_settings2.offset_y, &mut ridge_noise_settings2.offset_z, &mut ridge_noise_settings2_changed);
                });

            if ridge_noise_settings2_changed {
                let prng = PRNG::new(seed.0);
                let noise_params = ridge_noise_settings2.get_noise_params(prng);

                compute_worker.write_slice("noise_params_ridge2", &noise_params);
                compute_worker.execute();
            }

            ridge_noise_settings2_changed = false;
        });
}
