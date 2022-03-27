use std::ops::{Add, Mul, Sub};

use bevy::{
    math::{Quat, Vec3},
    pbr::{PbrBundle, StandardMaterial},
    prelude::{
        AssetServer, Assets, BuildChildren, Color, Commands, Component, Mesh, Res, ResMut,
        Transform,
    },
};
// use nalgebra::Point3;
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
    let mut straight_wire_parts: Vec<(Vec3, Vec3)> = vec![];

    for i in 1..5 {
        straight_wire_parts.push((
            Vec3::new(0.0, 0.01 * i as f32, 0.0),
            Vec3::new(0.0, 0.01, 0.0),
        ));
    }

    for i in 1..5 {
        straight_wire_parts.push((
            Vec3::new(0.0, 0.05, 0.01 * i as f32),
            Vec3::new(0.0, 0.0, 0.01),
        ));
    }

    let arrow_init_direction = Vec3::Y;

    for (wire_coord, current_vec) in straight_wire_parts.iter() {
        let current_vec_length = current_vec.length();
        let wire_cylinder = Cylinder::new(current_vec_length / 2., 0.001);
        let wire_mesh = bevy_mesh(wire_cylinder.to_trimesh(10));
        commands.spawn_bundle(PbrBundle {
            transform: Transform {
                translation: *wire_coord,
                rotation: Quat::from_rotation_arc(arrow_init_direction, current_vec.normalize()),
                ..Default::default()
            },
            mesh: meshes.add(wire_mesh),
            material: materials.add(Color::rgb(0.5, 0.1, 0.5).into()),
            ..Default::default()
        });
    }

    let shift = 0.01;
    let n = 5;
    for x in -n..n {
        for y in 0..10 {
            for z in -n..n {
                let color = Color::rgb(0.1, 0.5, 0.1);
                let (a_x, a_y, a_z) = (shift * x as f32, shift * y as f32, shift * z as f32);
                let arrow_coord = Vec3::new(a_x, a_y, a_z);
                let mut arrow_cross = Vec3::new(0., 0., 0.);

                for (wire_coord, current_vec) in straight_wire_parts.iter() {
                    let current_vec_norm = current_vec.normalize();
                    let current_vec_length = current_vec.length();

                    let arrow_translation = Vec3::new(a_x, a_y, a_z).sub(*wire_coord);
                    let arrow_translation_norm = arrow_translation.normalize();
                    let part_current_vec_norm = current_vec_norm.cross(arrow_translation_norm);
                    let arrow_tr_length = arrow_translation.length();
                    let arrow_biot_savart_b_magnitude =
                        current_vec_length / arrow_tr_length.powi(2);

                    arrow_cross =
                        arrow_cross.add(part_current_vec_norm.mul(arrow_biot_savart_b_magnitude));
                }
                let arrow_quat =
                    Quat::from_rotation_arc(arrow_init_direction, arrow_cross.normalize());
                let arrow_scale = 0.00001 * arrow_cross.length();

                let cylinder = Cylinder::new(arrow_scale * 10., arrow_scale);
                let cylinder_mesh = bevy_mesh(cylinder.to_trimesh(10));

                let _arrow = commands
                    .spawn_bundle(PbrBundle {
                        transform: Transform {
                            translation: arrow_coord,
                            rotation: arrow_quat,
                            ..Default::default()
                        },
                        mesh: meshes.add(cylinder_mesh),
                        material: materials.add(color.into()),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        let cone = Cone::new(arrow_scale * 5., arrow_scale * 3.0);
                        let cone_mesh = bevy_mesh(cone.to_trimesh(10));
                        parent.spawn_bundle(PbrBundle {
                            transform: Transform {
                                translation: Vec3::new(0.0, arrow_scale * 10., 0.0),
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
