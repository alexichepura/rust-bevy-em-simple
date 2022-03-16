use bevy::{
    math::{Quat, Vec3},
    pbr::{PbrBundle, StandardMaterial},
    prelude::{
        shape, AssetServer, Assets, BuildChildren, Color, Commands, Component, Mesh, Res, ResMut,
        Transform,
    },
};
use parry3d::shape::{Cone, Cylinder};

use crate::mesh::bevy_mesh;

#[derive(Component)]
pub struct Arrow;

pub fn field_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    _asset_server: Res<AssetServer>,
) {
    // commands.spawn_bundle(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Plane { size: 200.0 })),
    //     material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    //     ..Default::default()
    // });

    let current: f32 = 10.0;
    let current_vec = Vec3::new(0.0, 0.2, 0.0);
    let current_vec_norm = current_vec.normalize();

    let wire_cylinder = Cylinder::new(0.2, 0.002);
    let wire_mesh = bevy_mesh(wire_cylinder.to_trimesh(10));
    commands.spawn_bundle(PbrBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..Default::default()
        },
        mesh: meshes.add(wire_mesh),
        material: materials.add(Color::rgb(0.5, 0.1, 0.5).into()),
        ..Default::default()
    });

    for x in 0..3 {
        for y in 0..3 {
            for z in 0..3 {
                let color = Color::rgb(0.1, 0.5, 0.1);
                let cylinder = Cylinder::new(0.02, 0.002);
                let cylinder_mesh = bevy_mesh(cylinder.to_trimesh(10));
                let (a_x, a_y, a_z) = (0.1 * x as f32, 0.1 * y as f32, 0.1 * z as f32);
                let arrow_translation = Vec3::new(a_x, a_y, a_z);
                let arrow_translation_norm = arrow_translation.normalize();
                let arrow_cross = current_vec_norm.cross(arrow_translation_norm);
                let arrow_quat = Quat::from_rotation_arc(arrow_cross, Vec3::Y);
                let _arrow = commands
                    .spawn_bundle(PbrBundle {
                        transform: Transform {
                            translation: arrow_translation,
                            rotation: arrow_quat,
                            ..Default::default()
                        },
                        mesh: meshes.add(cylinder_mesh),
                        material: materials.add(color.into()),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        let cone = Cone::new(0.01, 0.005);
                        let cone_mesh = bevy_mesh(cone.to_trimesh(10));
                        parent.spawn_bundle(PbrBundle {
                            transform: Transform {
                                translation: Vec3::new(0.0, 0.02, 0.0),
                                ..Default::default()
                            },
                            mesh: meshes.add(cone_mesh),
                            material: materials.add(color.into()),
                            ..Default::default()
                        });
                    })
                    .insert(Arrow);
            }
        }
    }
}
