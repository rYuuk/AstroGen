use bevy::DefaultPlugins;
use bevy::prelude::{App, Commands, Msaa, PluginGroup, Resource, Startup, Window, WindowPlugin};
use bevy::ui::{JustifyContent, UiRect, Val};
use bevy::utils::default;
use bevy_easy_compute::prelude::AppComputePlugin;
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
#[cfg(feature = "diagnostics")]
use iyes_perf_ui::entries::PerfUiCompleteBundle;
#[cfg(feature = "diagnostics")]
use iyes_perf_ui::PerfUiPlugin;
use sickle_ui::prelude::{SetJustifyContentExt, SetMarginExt, SetWidthExt, UiColumnExt};
use sickle_ui::SickleUiPlugin;
use sickle_ui::ui_builder::{UiBuilderExt, UiRoot};

use compute::asteroid_terrain_generator::AsteroidGeneratorPlugin;
use crate::asteroid_mesh::{AsteroidMeshPlugin};
use crate::light::LightPlugin;
use crate::main_camera::MainCameraPlugin;
use crate::ui_widgets::crater_setting_widget::{CraterSettingPlugin, CraterSettingWidgetExt};
use crate::ui_widgets::ridge_noise_setting_widget::{RidgeNoisePlugin, RidgeNoiseSettingWidgetExt};
use crate::ui_widgets::simple_noise_setting_widget::{SimpleNoisePlugin, SimpleNoiseWidgetExt};

mod compute;
mod settings;
mod ui_widgets;
mod sphere_mesh;
mod asteroid_mesh;
mod utils;
mod main_camera;
mod light;

#[derive(Resource)]
struct RngSeed(u64);

fn main() {
    App::new()
        .add_plugins(EmbeddedAssetPlugin {
            mode: PluginMode::ReplaceDefault
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "AstroGen".into(),
                resolution: (1920., 1080.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(SickleUiPlugin)
        .add_plugins(AppComputePlugin)
        .add_plugins(AsteroidGeneratorPlugin)
        .add_plugins(AsteroidMeshPlugin)
        .add_diagnostics()
        .add_plugins((MainCameraPlugin, LightPlugin))
        .add_plugins(CraterSettingPlugin)
        .add_plugins(SimpleNoisePlugin)
        .add_plugins(RidgeNoisePlugin)
        .insert_resource(Msaa::Sample8)
        .insert_resource(RngSeed(2))
        .add_systems(Startup, setup)
        .run();
}

pub trait AddDiagnostics {
    fn add_diagnostics(&mut self) -> &mut Self;
}

impl AddDiagnostics for App {
    fn add_diagnostics(&mut self) -> &mut Self {
        #[cfg(feature = "diagnostics")]
        {
            self.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
                .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
                .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
                .add_plugins(PerfUiPlugin);
        }
        self
    }
}
fn setup(
    mut commands: Commands,
) {
    commands.ui_builder(UiRoot).column(|row| {
        row.create_crater_setting_widget(|_x| {});
        row.create_simple_noise_setting_widget(|_x| {});
        row.create_ridge_noise_setting_widget(|_y| {}, "ridge".to_string());
        row.create_ridge_noise_setting_widget(|_y| {}, "ridge2".to_string());
    })
        .style()
        .margin(UiRect::new(Val::ZERO, Val::ZERO, Val::Percent(1.), Val::ZERO))
        .justify_content(JustifyContent::FlexStart)
        .width(Val::Percent(30.));

    #[cfg(feature = "diagnostics")]
    commands.spawn(PerfUiCompleteBundle::default());
}