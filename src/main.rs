use bevy::{
    prelude::{App, Msaa},
    DefaultPlugins,
};
use field::field_system;

mod field;
mod mesh;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_system(field_system)
        .run();
}
