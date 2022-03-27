use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::{App, Msaa},
    DefaultPlugins,
};
use bevy_egui::EguiPlugin;
use camera::camera_system;
use dash::{dash_fps_system, dash_fps_update_system};
use field::field_system;
use smooth_bevy_cameras::{controllers::orbit::OrbitCameraPlugin, LookTransformPlugin};
use ui::{ui_change_detection, ui_system, UiState};

mod camera;
mod dash;
mod field;
mod mesh;
mod ui;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .init_resource::<UiState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LookTransformPlugin)
        .add_plugin(EguiPlugin)
        .add_plugin(OrbitCameraPlugin::default())
        .add_startup_system(camera_system)
        .add_startup_system(dash_fps_system)
        .add_startup_system(field_system)
        .add_system(dash_fps_update_system)
        .add_system(ui_system)
        .add_system(ui_change_detection)
        .run();
}
