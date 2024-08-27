use bevy::prelude::{App, Bundle, Changed, Color, Component, Entity, Event, EventWriter, JustifyContent, Name, NodeBundle, Plugin, Query, Update, Val, With};
use bevy::reflect::{DynamicStruct, Reflect, Typed, TypeInfo};
use sickle_ui::prelude::{LabelConfig, SetBackgroundColorExt, SetJustifyContentExt, SetWidthExt, Slider, SliderConfig, UiColumnExt, UiContainerExt, UiLabelExt, UiSliderExt};
use sickle_ui::ui_builder::UiBuilder;
use crate::ridge_noise_settings::RidgeNoiseSettings;

pub struct RidgeNoisePlugin;

#[derive(Event)]
pub struct RidgeNoiseSettingsChanged(pub RidgeNoiseSettings, pub String);

impl Plugin for RidgeNoisePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<RidgeNoiseSettingsChanged>()
            .add_systems(Update, get_value_changed);
    }
}

#[derive(Component, Debug, Default)]
struct ValueChanged;

#[derive(Component, Debug, Default)]
pub struct RidgeNoiseSettingWidget {
    settings: RidgeNoiseSettings,
    suffix: String,
}

impl RidgeNoiseSettingWidget {
    pub fn get_labels(&self) -> Vec<String> {
        let mut labels: Vec<String> = vec![];

        let TypeInfo::Struct(type_info) = RidgeNoiseSettings::type_info() else {
            panic!("expected struct");
        };

        for field_name in type_info.field_names() {
            let name = field_name.to_string();
            labels.push(name.to_string());
        }

        labels
    }

    fn frame() -> impl Bundle {
        (Name::new("Ridge Noise Setting Widget"), NodeBundle::default())
    }
}

pub trait RidgeNoiseSettingWidgetExt {
    fn create_ridge_noise_setting_widget(&mut self, spawn_children: impl FnOnce(&mut UiBuilder<Entity>), suffix: String) -> UiBuilder<Entity>;
}

impl RidgeNoiseSettingWidgetExt for UiBuilder<'_, Entity> {
    fn create_ridge_noise_setting_widget(&mut self, spawn_children: impl FnOnce(&mut UiBuilder<Entity>), suffix: String) -> UiBuilder<Entity> {
        let mut widget = RidgeNoiseSettingWidget::default();
        let labels = widget.get_labels();
        widget.suffix = suffix;
        let mut builder = self.container((RidgeNoiseSettingWidget::frame(), widget), spawn_children);
        builder.column(|column| {
            column.label(LabelConfig::from("Ridge Noise Settings"));
            for name in labels {
                column.slider(SliderConfig::horizontal(
                    name,
                    0.,
                    20.0,
                    0.,
                    true,
                ))
                    .insert(ValueChanged);
            }
        })
            .style()
            .justify_content(JustifyContent::FlexStart)
            .background_color(Color::srgb(0.3, 0.3, 0.3))
            .width(Val::Percent(100.));
        builder
    }
}

fn get_value_changed(
    mut query: Query<&mut Slider, (With<ValueChanged>, Changed<Slider>)>,
    mut widget_query: Query<&mut RidgeNoiseSettingWidget>,
    mut ridge_noise_settings_changed: EventWriter<RidgeNoiseSettingsChanged>,
)
{
    for slider_bar in query.iter_mut() {
        for mut widget in widget_query.iter_mut() {
            let field = slider_bar.config().clone().label.unwrap();
            let mut patch = DynamicStruct::default();
            patch.insert(field, slider_bar.value());
            widget.settings.apply(&patch);

            ridge_noise_settings_changed.send(RidgeNoiseSettingsChanged(widget.settings.clone(),widget.suffix.to_string()));
        }
    }
}