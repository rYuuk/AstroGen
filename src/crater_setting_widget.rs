use bevy::asset::AssetContainer;
use bevy::prelude::*;
use bevy::reflect::{DynamicStruct, Typed, TypeInfo};
use bevy::utils::HashMap;
use bevy_easy_compute::prelude::AppComputeWorker;
use sickle_ui::prelude::*;
use crate::{RngSeed, SimpleComputeWorker};
use crate::crater_settings::CraterSettings;

pub struct CraterSettingPlugin;

impl Plugin for CraterSettingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, get_value_changed);
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
            // .font("fonts/monofonto_rg.otf".to_string())
            .width(Val::Percent(100.));
        builder
    }
}

fn get_value_changed(
    mut query: Query<&mut Slider, (With<ValueChanged>, Changed<Slider>)>,
    mut compute_worker: ResMut<AppComputeWorker<SimpleComputeWorker>>,
    mut widget_query: Query<&mut CraterSettingWidget>,
    mut seed: ResMut<RngSeed>,
)
{
    for mut sliderBar in query.iter_mut() {
        for mut widget in widget_query.iter_mut() {
            let field = sliderBar.config().clone().label.unwrap();
            let a = field.clone();
            let mut patch = DynamicStruct::default();
            patch.insert(field, sliderBar.value());
            widget.settings.apply(&patch);

            let nmu = widget.settings.get_num_craters() as u32;
            let craters = widget.settings.get_craters(seed.0);

            let mut craters_centre: Vec<Vec3> = vec![];
            let mut craters_radius: Vec<f32> = vec![];
            let mut craters_floor_height: Vec<f32> = vec![];
            let mut craters_smoothness: Vec<f32> = vec![];

            for crater in craters.clone() {
                // println!("{} {} {} {} {} {} {}", a, craters.len().clone(), nmu, crater.centre, crater.radius, crater.floor_height, crater.smoothness);
                craters_centre.push(crater.centre);
                craters_radius.push(crater.radius);
                craters_floor_height.push(crater.floor_height);
                craters_smoothness.push(crater.smoothness);
            }

            compute_worker.write_slice("num_craters", &[widget.settings.get_num_craters() as u32]);
            compute_worker.write_slice("rim_steepness", &[widget.settings.get_rim_steepness()]);
            compute_worker.write_slice("rim_width", &[widget.settings.get_rim_width()]);
            compute_worker.write_slice("craters_centre", &craters_centre);
            compute_worker.write_slice("craters_radius", &craters_radius);
            compute_worker.write_slice("craters_floor_height", &craters_floor_height);
            compute_worker.write_slice("craters_smoothness", &craters_smoothness);

            compute_worker.execute();
        }
    }
}