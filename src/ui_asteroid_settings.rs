use std::ops::RangeInclusive;

use bevy::app::{App, Plugin};
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_egui::egui::{FontId, RichText};

use crate::compute_events::{CraterSettingsChanged, PerturbStrengthChanged, RidgeNoiseSettingsChanged, SimpleNoiseSettingsChanged};
use crate::settings::asteroid_settings::AsteroidSettings;

pub struct UIAsteroidSettings;
#[derive(Event)]
pub struct ExportButtonClicked;


#[derive(Resource)]
struct ValueChanged {
    pub perturb_strength: bool,
    pub crater_settings: bool,
    pub simple_noise_settings: bool,
    pub ridge_noise_settings: bool,
    pub ridge_noise_settings2: bool,
}

impl Default for ValueChanged{
    fn default() -> Self {
        ValueChanged{
            perturb_strength: true,
            crater_settings: true,
            simple_noise_settings: true,
            ridge_noise_settings: true,
            ridge_noise_settings2: true
        }
    }
}

impl Plugin for UIAsteroidSettings {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ValueChanged::default())
            .add_event::<ExportButtonClicked>()
            .add_systems(Update, show_ui);
    }
}

fn show_ui(mut contexts: EguiContexts,
           diagnostic: Res<DiagnosticsStore>,
           mut settings: ResMut<AsteroidSettings>,
           mut commands: Commands,
           mut value_changed: ResMut<ValueChanged>,
           mut status_changed: Local<String>
) {
    if let Some(ctx) = contexts.try_ctx_mut() {
        egui::Window::new("Settings")
            .scroll([false, true])
            .default_height(860.)
            .default_width(420.)
            .show(ctx, |ui| {
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
                            .fill(egui::Color32::from_rgb(99, 181, 74));

                        if ui.add(export_button).clicked() {
                            commands.trigger(ExportButtonClicked);
                            *status_changed = "Saved to asteroid.glb".to_string();
                        }
                        ui.label(&*status_changed);
                    });
                });

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
                ui.add_space(10f32);
                slider(ui, "Perturb Strength", &mut settings.peturb_strength, 0.01f64, 0.0..=1.,&mut value_changed.perturb_strength);
                ui.add_space(10f32);

                if value_changed.perturb_strength
                {
                    commands.trigger(PerturbStrengthChanged(
                            settings.peturb_strength
                    ));
                }
                value_changed.perturb_strength = false;

                let spacing = 20f32;
                let crater_settings = &mut settings.crater_settings;

                egui::CollapsingHeader::new(RichText::new("Crater Settings").font(FontId::proportional(20.0)))
                    .default_open(true)
                    .show(ui, |ui| {
                        slider(ui, "Number of Craters", &mut crater_settings.num_craters, 10f64, 0.0..=2000., &mut value_changed.crater_settings);
                        slider(ui, "Crater size min", &mut crater_settings.crater_size_min, 0.01f64, 0.0..=1., &mut value_changed.crater_settings);
                        slider(ui, "Crater size max", &mut crater_settings.crater_size_max, 0.01f64, 0.1..=1., &mut value_changed.crater_settings);
                        slider(ui, "Rim steepness", &mut crater_settings.rim_steepness, 0.01f64, 0.0..=2., &mut value_changed.crater_settings);
                        slider(ui, "Rim Width", &mut crater_settings.rim_width, 0.01f64, 0.0..=5., &mut value_changed.crater_settings);
                        slider(ui, "Smooth min", &mut crater_settings.smooth_min, 0.01f64, 0.0..=1., &mut value_changed.crater_settings);
                        slider(ui, "Smooth max", &mut crater_settings.smooth_max, 0.01f64, 0.1..=2., &mut value_changed.crater_settings);
                        slider(ui, "Size distribution", &mut crater_settings.size_distribution, 0.01f64, 0.0..=1., &mut value_changed.crater_settings);
                    });
                ui.add_space(spacing);

                if value_changed.crater_settings
                {
                    commands.trigger(CraterSettingsChanged(
                        crater_settings.clone()
                    ));
                }
                value_changed.crater_settings = false;

                let simple_noise_settings = &mut settings.simple_noise_settings;
                egui::CollapsingHeader::new(RichText::new("Simple Noise Settings").font(FontId::proportional(20.0)))
                    .default_open(true)
                    .show(ui, |ui| {
                        slider(ui, "Number of layers", &mut simple_noise_settings.num_layers, 1f64, 0.0..=40., &mut value_changed.simple_noise_settings);
                        slider(ui, "Lacunarity", &mut simple_noise_settings.lacunarity, 0.1f64, 0.0..=5., &mut value_changed.simple_noise_settings);
                        slider(ui, "Persistence", &mut simple_noise_settings.persistence, 0.1f64, 0.0..=5., &mut value_changed.simple_noise_settings);
                        slider(ui, "Scale", &mut simple_noise_settings.scale, 0.1f64, 0.0..=10., &mut value_changed.simple_noise_settings);
                        slider(ui, "Elevation", &mut simple_noise_settings.elevation, 0.1f64, 0.0..=5., &mut value_changed.simple_noise_settings);
                        slider(ui, "Vertical Shift", &mut simple_noise_settings.vertical_shift, 0.1f64, 0.0..=5., &mut value_changed.simple_noise_settings);

                        ui.label("Offset:");
                        offset(ui, &mut simple_noise_settings.offset_x, &mut simple_noise_settings.offset_y, &mut simple_noise_settings.offset_z, &mut value_changed.simple_noise_settings);
                    });
                ui.add_space(spacing);

                if value_changed.simple_noise_settings {
                    commands.trigger(SimpleNoiseSettingsChanged(
                        simple_noise_settings.clone()
                    ));
                }
                value_changed.simple_noise_settings = false;


                let ridge_noise_settings = &mut settings.ridge_noise_settings;
                egui::CollapsingHeader::new(RichText::new("Ridge Noise Settings").font(FontId::proportional(20.0)))
                    .default_open(true)
                    .show(ui, |ui| {
                        slider(ui, "Number of layers", &mut ridge_noise_settings.num_layers, 1f64, 0.0..=40., &mut value_changed.ridge_noise_settings);
                        slider(ui, "Lacunarity", &mut ridge_noise_settings.lacunarity, 0.1f64, 0.0..=5., &mut value_changed.ridge_noise_settings);
                        slider(ui, "Persistence", &mut ridge_noise_settings.persistence, 0.1f64, 0.0..=5., &mut value_changed.ridge_noise_settings);
                        slider(ui, "Scale", &mut ridge_noise_settings.scale, 0.1f64, 0.0..=5., &mut value_changed.ridge_noise_settings);
                        slider(ui, "Power", &mut ridge_noise_settings.power, 0.1f64, 0.0..=5., &mut value_changed.ridge_noise_settings);
                        slider(ui, "Elevation", &mut ridge_noise_settings.elevation, 0.1f64, -5.0..=5., &mut value_changed.ridge_noise_settings);
                        slider(ui, "Gain", &mut ridge_noise_settings.gain, 0.1f64, 0.0..=10., &mut value_changed.ridge_noise_settings);
                        slider(ui, "Vertical Shift", &mut ridge_noise_settings.vertical_shift, 0.1f64, 0.0..=5., &mut value_changed.ridge_noise_settings);
                        slider(ui, "Peak Smoothing", &mut ridge_noise_settings.peak_smoothing, 0.1f64, 0.0..=5., &mut value_changed.ridge_noise_settings);

                        ui.label("Offset:");
                        offset(ui, &mut ridge_noise_settings.offset_x, &mut ridge_noise_settings.offset_y, &mut ridge_noise_settings.offset_z, &mut value_changed.ridge_noise_settings);
                    });
                ui.add_space(20f32);

                if value_changed.ridge_noise_settings {
                    commands.trigger(RidgeNoiseSettingsChanged(
                        ridge_noise_settings.clone(),
                        "".to_string(),
                    ));
                }

                value_changed.ridge_noise_settings = false;

                let ridge_noise_settings2 = &mut settings.ridge_noise_settings2;
                egui::CollapsingHeader::new(RichText::new("Ridge Noise Settings 2").font(FontId::proportional(20.0)))
                    .default_open(true)
                    .show(ui, |ui| {
                        slider(ui, "Number of layers", &mut ridge_noise_settings2.num_layers, 1f64, 0.0..=40., &mut value_changed.ridge_noise_settings2);
                        slider(ui, "Lacunarity", &mut ridge_noise_settings2.lacunarity, 0.1f64, 0.0..=5., &mut value_changed.ridge_noise_settings2);
                        slider(ui, "Persistence", &mut ridge_noise_settings2.persistence, 0.1f64, 0.0..=5., &mut value_changed.ridge_noise_settings2);
                        slider(ui, "Scale", &mut ridge_noise_settings2.scale, 0.1f64, 0.0..=5., &mut value_changed.ridge_noise_settings2);
                        slider(ui, "Power", &mut ridge_noise_settings2.power, 0.1f64, 0.0..=5., &mut value_changed.ridge_noise_settings2);
                        slider(ui, "Elevation", &mut ridge_noise_settings2.elevation, 0.1f64, -5.0..=5., &mut value_changed.ridge_noise_settings2);
                        slider(ui, "Gain", &mut ridge_noise_settings2.gain, 0.1f64, 0.0..=10., &mut value_changed.ridge_noise_settings2);
                        slider(ui, "Vertical Shift", &mut ridge_noise_settings2.vertical_shift, 0.1f64, 0.0..=5., &mut value_changed.ridge_noise_settings2);
                        slider(ui, "Peak Smoothing", &mut ridge_noise_settings2.peak_smoothing, 0.1f64, 0.0..=5., &mut value_changed.ridge_noise_settings2);

                        ui.label("Offset:");
                        offset(ui, &mut ridge_noise_settings2.offset_x, &mut ridge_noise_settings2.offset_y, &mut ridge_noise_settings2.offset_z, &mut value_changed.ridge_noise_settings2);
                    });

                if value_changed.ridge_noise_settings2 {
                    commands.trigger(RidgeNoiseSettingsChanged(
                        ridge_noise_settings2.clone(),
                        "2".to_string(),
                    ));
                }

                value_changed.ridge_noise_settings2 = false;
            });
    }
}
