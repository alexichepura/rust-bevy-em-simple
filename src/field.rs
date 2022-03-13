use bevy::{
    math::Vec3,
    pbr::{PbrBundle, StandardMaterial},
    prelude::{
        shape, AssetServer, Assets, BuildChildren, Color, Commands, Mesh, Res, ResMut, Transform,
    },
};
use parry3d::shape::{Cone, Cylinder};

use crate::mesh::bevy_mesh;

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

    for x in 0..3 {
        for y in 0..3 {
            for z in 0..3 {
                let color = Color::rgb(0.1, 0.5, 0.1);
                let cylinder = Cylinder::new(0.02, 0.002);
                let cylinder_mesh = bevy_mesh(cylinder.to_trimesh(10));
                let _arrow = commands
                    .spawn_bundle(PbrBundle {
                        transform: Transform {
                            translation: Vec3::new(0.1 * x as f32, 0.1 * y as f32, 0.1 * z as f32),
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
                    });
            }
        }
    }
}
