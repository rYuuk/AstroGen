use bevy::app::App;
use bevy::input::mouse::MouseWheel;
use bevy::math::Vec3;
use bevy::prelude::{
    default, Camera3dBundle, Commands, Component, EventReader, PerspectiveProjection, Plugin,
    Query, Startup, Transform, Update, With,
};
pub struct MainCameraPlugin;

#[derive(Component)]
struct MainCamera;

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_camera)
            .add_systems(Update, zoom_camera);
    }
}

fn initialize_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 9.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            projection: PerspectiveProjection {
                fov: 45.0f32.to_radians(),
                ..default()
            }
            .into(),
            ..default()
        },
        MainCamera,
    ));
}

fn zoom_camera(
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    mut scroll_evr: EventReader<MouseWheel>,
) {
    let scroll = scroll_evr.read().map(|ev| ev.y).sum::<f32>();
    if scroll == 0.0 {
        return;
    }

    for mut transform in camera_query.iter_mut() {
        let mut translation = transform.translation;
        translation.z -= scroll * 0.5;
        translation.z = translation.z.clamp(3.0, 10.0); 
        transform.translation = translation;
    }
}
