use bevy::text::Font;
use bevy::{
    diagnostic::Diagnostics,
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::{AssetServer, Color, Component, TextBundle},
    prelude::{Query, With},
    text::{TextSection, TextStyle},
    ui::{AlignSelf, Style},
};
use bevy::{ecs::system::Res, prelude::Commands};
use bevy::{prelude::Handle, text::Text};

#[derive(Component)]
pub struct FpsText;

pub fn dash_fps_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let bold: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    let medium: Handle<Font> = asset_server.load("fonts/FiraMono-Medium.ttf");
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "FPS: ".to_string(),
                        style: TextStyle {
                            font: bold.clone(),
                            font_size: 60.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: medium.clone(),
                            font_size: 60.0,
                            color: Color::GOLD,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(FpsText);
}

pub fn dash_fps_update_system(
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.sections[1].value = format!("{:.1}", average);
            }
        }
    }
}
