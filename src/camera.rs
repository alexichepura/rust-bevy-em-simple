use bevy::{
    math::Vec3,
    pbr::AmbientLight,
    prelude::{Commands, PerspectiveCameraBundle, UiCameraBundle},
};
use smooth_bevy_cameras::controllers::orbit::{OrbitCameraBundle, OrbitCameraController};

pub fn camera_system(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands.insert_resource(AmbientLight {
        brightness: 2.0,
        ..Default::default()
    });
    commands.spawn_bundle(OrbitCameraBundle::new(
        OrbitCameraController::default(),
        PerspectiveCameraBundle::default(),
        Vec3::new(0., 0.0, -0.2),
        Vec3::new(0., 0.0, 0.),
    ));
}
