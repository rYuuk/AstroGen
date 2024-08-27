use bevy::prelude::{App, Bundle, Changed, Color, Component, Entity, Event, EventWriter, JustifyContent, Name, NodeBundle, Plugin, Query, Update, Val, With};
use bevy::reflect::{DynamicStruct, Reflect, Typed, TypeInfo};
use sickle_ui::prelude::{LabelConfig, SetBackgroundColorExt, SetJustifyContentExt, SetWidthExt, Slider, SliderConfig, UiColumnExt, UiContainerExt, UiLabelExt, UiSliderExt};
use sickle_ui::ui_builder::UiBuilder;
use crate::settings::simple_noise_settings::SimpleNoiseSettings;

pub struct SimpleNoisePlugin;

#[derive(Event)]
pub struct SimpleNoiseSettingsChanged(pub SimpleNoiseSettings);

impl Plugin for SimpleNoisePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SimpleNoiseSettingsChanged>()
            .add_systems(Update, get_value_changed);
    }
}

#[derive(Component, Debug, Default)]
struct ValueChanged;

#[derive(Component, Debug, Default)]
pub struct SimpleNoiseSettingWidget {
    settings: SimpleNoiseSettings,
}

impl SimpleNoiseSettingWidget {
    pub fn get_labels(&self) -> Vec<String> {
        let mut labels: Vec<String> = vec![];

        let TypeInfo::Struct(type_info) = SimpleNoiseSettings::type_info() else {
            panic!("expected struct");
        };

        for field_name in type_info.field_names() {
            let name = field_name.to_string();
            labels.push(name.to_string());
        }

        labels
    }

    fn frame() -> impl Bundle {
        (Name::new("Simple Noise Setting Widget"), NodeBundle::default())
    }
}

pub trait SimpleNoiseWidgetExt {
    fn create_simple_noise_setting_widget(&mut self, spawn_children: impl FnOnce(&mut UiBuilder<Entity>)) -> UiBuilder<Entity>;
}

impl SimpleNoiseWidgetExt for UiBuilder<'_, Entity> {
    fn create_simple_noise_setting_widget(&mut self, spawn_children: impl FnOnce(&mut UiBuilder<Entity>)) -> UiBuilder<Entity> {
        let widget = SimpleNoiseSettingWidget::default();
        let labels = widget.get_labels();
        let mut builder = self.container((SimpleNoiseSettingWidget::frame(), widget), spawn_children);
        builder.column(|column| {
            column.label(LabelConfig::from("Simple Noise Settings"));
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
    mut widget_query: Query<&mut SimpleNoiseSettingWidget>,
    mut simple_noise_setting_changed: EventWriter<SimpleNoiseSettingsChanged>,
)
{
    for slider_bar in query.iter_mut() {
        for mut widget in widget_query.iter_mut() {
            let field = slider_bar.config().clone().label.unwrap();
            let mut patch = DynamicStruct::default();
            patch.insert(field, slider_bar.value());
            widget.settings.apply(&patch);

            simple_noise_setting_changed.send(SimpleNoiseSettingsChanged(widget.settings.clone()));
        }
    }
}