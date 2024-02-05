use bevy::{
    prelude::*,
    render::{
        view::RenderLayers,
        mesh::shape::UVSphere
    }
};
use bevy_mod_picking::prelude::*;

use crate::generate_viewcube_face;

use super::{
    CubePart,
    ViewcubeHit
};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let center = Vec3::new(0.6, 0.6, 0.6);
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(UVSphere{radius: 0.01, sectors: 1, stacks: 1}.into()),
            material: materials.add(StandardMaterial::default()),
            ..Default::default()
        },
        RenderLayers::layer(13),
        super::ViewcubeCenter,
    )).with_children(|builder| {
        builder.spawn((
            MaterialMeshBundle {
                mesh: meshes.add(crate::prelude::BevyTridentAxis::default().into()),
                material: materials.add(StandardMaterial::default()),
                transform: Transform::from_translation(-center),
                ..Default::default()
            },
            RenderLayers::layer(13),
        ));
        generate_viewcube_simple_face(0.8f32, builder, &mut meshes, &mut materials);
    });
}

pub fn generate_viewcube_simple_face(
    size: f32,
    builder: &mut ChildBuilder,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let plane = shape::Plane {
        size,
        ..Default::default()
    };
    let half = 0.4f32;
    // Right (+X)
    builder.spawn(
        generate_viewcube_face!(
            meshes, materials,
            plane,
            Color::RED,
            Transform::from_matrix(
                Mat4::from_rotation_translation(
                    Quat::from_rotation_z(-crate::PI / 2.0),
                    Vec3::new(half, 0.0, 0.0),
                )),
            ViewcubeHit(CubePart::Right)
        )
    );
    // Left (-X)
    builder.spawn(
        generate_viewcube_face!(
            meshes, materials,
            plane,
            Color::RED,
            Transform::from_matrix(
                Mat4::from_rotation_translation(
                    Quat::from_rotation_z(crate::PI / 2.0),
                    Vec3::new(-half, 0.0, 0.0),
                )),
            ViewcubeHit(CubePart::Left)
        )
    );
    // Top (+Y)
    builder.spawn(
        generate_viewcube_face!(
            meshes, materials,
            plane,
            Color::GREEN,
            Transform::from_matrix(
                Mat4::from_rotation_translation(
                    Quat::from_rotation_x(0.0),
                    Vec3::new(0.0, half,0.0),
                )),
            ViewcubeHit(CubePart::Top)
        )
    );
    // Bottom (-Y)
    builder.spawn(
        generate_viewcube_face!(
            meshes, materials,
            plane,
            Color::GREEN,
            Transform::from_matrix(
                Mat4::from_rotation_translation(
                    Quat::from_rotation_x(crate::PI),
                    Vec3::new(0.0, -half,0.0),
                )),
            ViewcubeHit(CubePart::Bottom)
        )
    );
    // Front (+Z)
    builder.spawn(
        generate_viewcube_face!(
            meshes, materials,
            plane,
            Color::BLUE,
            Transform::from_matrix(
                Mat4::from_rotation_translation(
                    Quat::from_rotation_x(crate::PI / 2.0),
                    Vec3::new(0.0, 0.0, half),
                )),
            ViewcubeHit(CubePart::Front)
        )
    );
    // Back (-Z)
    builder.spawn(
        generate_viewcube_face!(
            meshes, materials,
            plane,
            Color::BLUE,
            Transform::from_matrix(
                Mat4::from_rotation_translation(
                    Quat::from_rotation_x(-crate::PI / 2.0),
                    Vec3::new(0.0, 0.0, -half),
                )),
            ViewcubeHit(CubePart::Back)
        )
    );
}