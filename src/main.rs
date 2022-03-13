use bevy::{
    prelude::{App, Msaa},
    DefaultPlugins,
};
use camera::camera_system;
use field::field_system;
// use bevy_config_cam::ConfigCam;
use smooth_bevy_cameras::{controllers::orbit::OrbitCameraPlugin, LookTransformPlugin};

mod camera;
mod field;
mod mesh;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        // .add_plugin(ConfigCam)
        .add_plugin(LookTransformPlugin)
        .add_plugin(OrbitCameraPlugin::default())
        .add_system(camera_system)
        .add_system(field_system)
        .run();
}
