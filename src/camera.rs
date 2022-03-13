use bevy::{
    math::Vec3,
    pbr::{AmbientLight, PointLight, PointLightBundle},
    prelude::{Commands, PerspectiveCameraBundle, Transform},
};
use smooth_bevy_cameras::controllers::orbit::{OrbitCameraBundle, OrbitCameraController};

pub fn camera_system(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        // color: Color::ORANGE_RED,
        brightness: 2.0,
        ..Default::default()
    });
    // commands.spawn_bundle(PointLightBundle {
    //     transform: Transform::from_xyz(4.0, 8.0, 4.0),
    //     point_light: PointLight {
    //         // shadows_enabled: true,
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // });
    commands.spawn_bundle(OrbitCameraBundle::new(
        OrbitCameraController::default(),
        PerspectiveCameraBundle::default(),
        Vec3::new(-0.5, 1.0, 1.0),
        Vec3::new(0., 0., 0.),
    ));
    // commands.spawn_bundle(PerspectiveCameraBundle {
    //     transform: Transform::from_translation(Vec3::new(0., 2.5, -2.))
    //         .looking_at(Vec3::ZERO, Vec3::Y),
    //     ..Default::default()
    // });
}
