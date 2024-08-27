use bevy::asset::AssetContainer;
use bevy::prelude::*;
use bevy::reflect::{DynamicStruct, Typed, TypeInfo};
use bevy::utils::HashMap;
use bevy_easy_compute::prelude::AppComputeWorker;
use rand::rngs::StdRng;
use rand::SeedableRng;
use sickle_ui::prelude::*;
use crate::simple_noise_settings::SimpleNoiseSettings;
use crate::{RngSeed, SimpleComputeWorker};
use crate::utils::PRNG;

pub struct SimpleNoisePlugin;

impl Plugin for SimpleNoisePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, get_value_changed);
    }
}

#[derive(Component, Debug, Default)]
struct ValueChanged;

#[derive(Component, Debug, Default)]
pub struct SimpleNoiseSettingWidget {
    settings: SimpleNoiseSettings,
    labels: HashMap<String, f32>,
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
        let mut widget = SimpleNoiseSettingWidget::default();
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
    mut compute_worker: ResMut<AppComputeWorker<SimpleComputeWorker>>,
    mut widget_query: Query<&mut SimpleNoiseSettingWidget>,
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
            
             compute_worker.write_slice("noise_params_shape", &noise_params);
            compute_worker.execute();
        }
    }
}