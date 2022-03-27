use bevy::prelude::{info, Component, Res, ResMut};
use bevy_egui::{egui, EguiContext};

#[derive(Component, Default, Debug)]
pub struct UiState {
    preset: String,
}

pub fn ui_system(mut egui_context: ResMut<EguiContext>, mut ui_state: ResMut<UiState>) {
    egui::Window::new("Preset").show(egui_context.ctx_mut(), |ui| {
        ui.label(&ui_state.preset);
        if ui.button("straight wire").clicked() {
            ui_state.preset = "straight wire".to_string();
        }
        if ui.button("circle").clicked() {
            ui_state.preset = "circle".to_string();
        }
    });
}

pub fn ui_change_detection(ui_state: Res<UiState>) {
    if ui_state.is_changed() {
        info!("{:?}", ui_state.preset);
    }
}
