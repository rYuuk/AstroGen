use bevy::asset::AssetContainer;
use bevy::prelude::*;
use bevy::reflect::{DynamicStruct, Typed, TypeInfo};
use bevy::utils::HashMap;
use bevy_easy_compute::prelude::AppComputeWorker;
use rand::prelude::StdRng;
use rand::SeedableRng;
use sickle_ui::prelude::*;
use crate::{RngSeed, SimpleComputeWorker};
use crate::ridge_noise_settings::RidgeNoiseSettings;
use crate::utils::PRNG;

pub struct RidgeNoisePlugin;

impl Plugin for RidgeNoisePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, get_value_changed);
    }
}

#[derive(Component, Debug, Default)]
struct ValueChanged;

#[derive(Component, Debug, Default)]
pub struct RidgeNoiseSettingWidget {
    settings: RidgeNoiseSettings,
    labels: HashMap<String, f32>,
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
    mut compute_worker: ResMut<AppComputeWorker<SimpleComputeWorker>>,
    mut widget_query: Query<&mut RidgeNoiseSettingWidget>,
    mut seed: ResMut<RngSeed>,
)
{
    for mut sliderBar in query.iter_mut() {
        for mut widget in widget_query.iter_mut() {
            let field = sliderBar.config().clone().label.unwrap();
            let mut patch = DynamicStruct::default();
            patch.insert(field, sliderBar.value());
            widget.settings.apply(&patch);
            
            let master_seed = seed.0;
            let  prng = PRNG{
                seed: master_seed,
                rng: StdRng::seed_from_u64(master_seed)
            };
            
            let noise_params = widget.settings.get_noise_params(prng);
            let prefix = "noise_params_";
            let suffix = &widget.suffix;
            let param_name = &format!("{}{}", prefix, suffix);
            
            compute_worker.write_slice(param_name, &noise_params);
            compute_worker.execute();
        }
    }
}