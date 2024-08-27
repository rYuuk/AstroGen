﻿use bevy::asset::AssetContainer;
use bevy::prelude::{App, Bundle, Changed, Color, Component, Entity, Event, EventWriter, JustifyContent, Name, NodeBundle, Plugin, Query, Update, Val, With};
use bevy::reflect::{DynamicStruct, Reflect};
use bevy::utils::HashMap;
use sickle_ui::prelude::{LabelConfig, SetBackgroundColorExt, SetJustifyContentExt, SetWidthExt, Slider, SliderConfig, UiColumnExt, UiContainerExt, UiLabelExt, UiSliderExt};
use sickle_ui::ui_builder::UiBuilder;
use crate::crater_settings::CraterSettings;

pub struct CraterSettingPlugin;

#[derive(Event)]
pub struct CraterSettingsChanged(pub CraterSettings);

impl Plugin for CraterSettingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<CraterSettingsChanged>()
            .add_systems(Update, get_value_changed);
    }
}

#[derive(Component, Debug, Default)]
struct ValueChanged;

#[derive(Component, Debug, Default)]
pub struct CraterSettingWidget {
    settings: CraterSettings,
    labels: HashMap<String, f32>,
}

impl CraterSettingWidget {
    fn frame() -> impl Bundle {
        (Name::new("Crater Setting Widget"), NodeBundle::default())
    }
}

pub trait CraterSettingWidgetExt {
    fn create_crater_setting_widget(&mut self, spawn_children: impl FnOnce(&mut UiBuilder<Entity>)) -> UiBuilder<Entity>;
}

impl CraterSettingWidgetExt for UiBuilder<'_, Entity> {
    fn create_crater_setting_widget(&mut self, spawn_children: impl FnOnce(&mut UiBuilder<Entity>)) -> UiBuilder<Entity> {
        let mut widget = CraterSettingWidget::default();
        let mut builder = self.container((CraterSettingWidget::frame(), widget), spawn_children);
        builder.column(|column| {
            column.label(LabelConfig::from("Crater Settings"));
            column.slider(SliderConfig::horizontal(
                "num_craters".to_string(),
                0.,
                1000.0,
                3.,
                true,
            ))
                .insert(ValueChanged);
            column.slider(SliderConfig::horizontal(
                "crater_size_min".to_string(),
                0.,
                2.,
                0.01,
                true,
            ))
                .insert(ValueChanged);
            column.slider(SliderConfig::horizontal(
                "crater_size_max".to_string(),
                0.,
                2.,
                0.1,
                true,
            ))
                .insert(ValueChanged);
            column.slider(SliderConfig::horizontal(
                "rim_steepness".to_string(),
                0.,
                2.,
                0.2,
                true,
            ))
                .insert(ValueChanged);
            column.slider(SliderConfig::horizontal(
                "rim_width".to_string(),
                0.,
                5.,
                1.6,
                true,
            ))
                .insert(ValueChanged);
            column.slider(SliderConfig::horizontal(
                "smooth_min".to_string(),
                0.,
                1.,
                0.4,
                true,
            ))
                .insert(ValueChanged);
            column.slider(SliderConfig::horizontal(
                "smooth_max".to_string(),
                0.,
                2.,
                1.5,
                true,
            ))
                .insert(ValueChanged);
            column.slider(SliderConfig::horizontal(
                "size_distribution".to_string(),
                0.,
                2.,
                0.4,
                true,
            ))
                .insert(ValueChanged);
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
    mut widget_query: Query<&mut CraterSettingWidget>,
    mut crater_settings_changed: EventWriter<CraterSettingsChanged>,
)
{
    for mut sliderBar in query.iter_mut() {
        for mut widget in widget_query.iter_mut() {
            let field = sliderBar.config().clone().label.unwrap();
            let mut patch = DynamicStruct::default();
            patch.insert(field, sliderBar.value());
            widget.settings.apply(&patch);

            crater_settings_changed.send(CraterSettingsChanged(widget.settings.clone()));
        }
    }
}