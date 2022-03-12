use bevy::{
    pbr::{PbrBundle, StandardMaterial},
    prelude::{AssetServer, Assets, Color, Commands, Mesh, Res, ResMut},
};
use parry3d::shape::Cylinder;

use crate::mesh::bevy_mesh;

pub fn field_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    _asset_server: Res<AssetServer>,
) {
    let wheel_cylinder = Cylinder::new(0.1, 0.01);
    let wheel_mesh = bevy_mesh(wheel_cylinder.to_trimesh(100));
    let _wheel = commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(wheel_mesh),
        material: materials.add(Color::rgb(0.1, 0.1, 0.3).into()),
        ..Default::default()
    });
}
