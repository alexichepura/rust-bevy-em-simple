use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::{App, Msaa},
    DefaultPlugins,
};
use camera::camera_system;
use dash::{dash_fps_system, dash_fps_update_system};
use field::field_system;
use smooth_bevy_cameras::{controllers::orbit::OrbitCameraPlugin, LookTransformPlugin};

mod camera;
mod dash;
mod field;
mod mesh;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LookTransformPlugin)
        .add_plugin(OrbitCameraPlugin::default())
        .add_startup_system(camera_system)
        .add_startup_system(dash_fps_system)
        .add_startup_system(field_system)
        .add_system(dash_fps_update_system)
        .run();
}
