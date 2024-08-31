use bevy::app::Update;
use bevy::asset::{ AssetServer, Handle};
use bevy::color::Color;
use bevy::DefaultPlugins;
use bevy::prelude::*;
use bevy::text::TextStyle;
use bevy::ui::{AlignItems, BackgroundColor, BorderRadius, Interaction, JustifyContent, Style, UiRect, Val};
use bevy::utils::default;
use bevy::winit::WinitSettings;
use bevy_easy_compute::prelude::AppComputePlugin;
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
#[cfg(feature = "diagnostics")]
use iyes_perf_ui::entries::PerfUiCompleteBundle;
#[cfg(feature = "diagnostics")]
use iyes_perf_ui::PerfUiPlugin;
use sickle_ui::prelude::*;
use sickle_ui::SickleUiPlugin;
use bevy::gltf;

use compute::asteroid_terrain_generator::AsteroidGeneratorPlugin;
use crate::asteroid_mesh::{AsteroidMeshPlugin};
use crate::gltf_exporter::GlTFExporter;
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
mod gltf_exporter;

#[derive(Resource)]
struct RngSeed(u64);

#[derive(Component)]
struct ExportButton;

fn main() {
    App::new()
        .insert_resource(WinitSettings::desktop_app())
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
        .add_plugins(GlTFExporter)
        .add_diagnostics()
        .add_plugins((MainCameraPlugin, LightPlugin))
        .add_plugins(CraterSettingPlugin)
        .add_plugins(SimpleNoisePlugin)
        .add_plugins(RidgeNoisePlugin)
        .insert_resource(Msaa::Sample8)
        .insert_resource(RngSeed(2))
        .add_event::<ExportButtonClicked>()
        .add_systems(Startup, setup)
        .add_systems(Update, button_system)
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
    asset_server: Res<AssetServer>,
) {
    commands.ui_builder(UiRoot).column(|row| {
        row.create_crater_setting_widget(|_x| {});
        row.create_simple_noise_setting_widget(|_x| {});
        row.create_ridge_noise_setting_widget(|_y| {}, "ridge".to_string());
        row.create_ridge_noise_setting_widget(|_y| {}, "ridge2".to_string());
    })
        .style()
        .margin(UiRect::new(Val::ZERO, Val::ZERO, Val::Percent(1.), Val::ZERO))
        .justify_content(JustifyContent::Stretch)
        .width(Val::Percent(30.));

    commands.spawn((ButtonBundle {
        style: Style {
            width: Val::Px(100.),
            height: Val::Px(40.),
            left: Val::Percent(50.),
            margin: UiRect::new(Val::Px(0.), Val::Px(0.), Val::Px(20.), Val::Px(0.)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: BackgroundColor(Color::srgb(0.45, 0.15, 0.15).into()),
        border_radius: BorderRadius::all(Val::Px(2.)),
        ..default()
    },ExportButton))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Export",
                TextStyle {
                    font: asset_server.load("fonts/monofonto_rg.otf"),
                    font_size: 30.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                },
            ));
        });

    #[cfg(feature = "diagnostics")]
    commands.spawn(PerfUiCompleteBundle::default());
}
#[derive(Event)]
pub struct ExportButtonClicked;

fn button_system(
    mut interaction_query: Query<(&Interaction,  &mut BackgroundColor), (Changed<Interaction>, With<ExportButton>)>,
    mut export_clicked: EventWriter<ExportButtonClicked>
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgb(0.35, 0.75, 0.35));
                export_clicked.send(ExportButtonClicked);
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.25, 0.25, 0.25));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgb(0.45, 0.15, 0.15));
            }
        }
    }
}