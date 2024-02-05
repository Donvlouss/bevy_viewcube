use bevy::{
    prelude::*,
    render::{
        mesh::{
            shape::UVSphere,
            Indices
        },
        render_resource::PrimitiveTopology,
        view::RenderLayers,
    }
};
use bevy_mod_picking::prelude::*;

use crate::{
    generate_viewcube_face,
    PI_2, PI_4, PI_4_3
};

use super::{
    CubePart,
    simple_viewcube::generate_viewcube_simple_face,
    ViewcubeHit,
};

#[derive(Clone, Copy)]
struct ViewcubeEdge(pub CubePart);

#[derive(Clone, Copy)]
struct ViewcubeCorner(pub CubePart);

impl From<ViewcubeEdge> for Mesh {
    fn from(value: ViewcubeEdge) -> Self {
        let mut width = 0.6f32 / 2.0;
        let mut height = 0.1 * 2f32.sqrt() / 2.0;

        let (q, swap) = match value.0 {
            CubePart::FrontTop => (Quat::from_rotation_x(PI_4), false),
            CubePart::FrontBottom => (Quat::from_rotation_x(PI_4_3), false),
            CubePart::BackTop => (Quat::from_rotation_x(-PI_4), false),
            CubePart::BackBottom => (Quat::from_rotation_x(-PI_4_3), false),
            CubePart::LeftTop => (Quat::from_rotation_z(PI_4), true),
            CubePart::LeftBottom => (Quat::from_rotation_z(PI_4_3), true),
            CubePart::RightTop => (Quat::from_rotation_z(-PI_4), true),
            CubePart::RightBottom => (Quat::from_rotation_z(-PI_4_3), true),
            CubePart::FrontLeft => (
                Quat::from_euler(EulerRot::ZYX, PI_2, 0.0, PI_4),
                false
            ),
            CubePart::FrontRight => (
                Quat::from_euler(EulerRot::ZYX, PI_2, 0.0, PI_4_3),
                false
            ),
            CubePart::BackLeft => (
                Quat::from_euler(EulerRot::ZYX, PI_2, 0.0, -PI_4),
                false
            ),
            CubePart::BackRight => (
                Quat::from_euler(EulerRot::ZYX, PI_2, 0.0, -PI_4_3),
                false
            ),
            _ => panic!(),
        };

        if swap {
            (width, height) = (height, width);
        }
        let dist_edge: f32 = (2.0 * 0.4f32.powi(2)).sqrt() - 0.1 * 2f32.sqrt() / 2.0;
        let positions = vec![
            Vec3::new( width, dist_edge,  height),
            Vec3::new( width, dist_edge, -height),
            Vec3::new(-width, dist_edge, -height),
            Vec3::new(-width, dist_edge,  height),
        ].iter().map(|v| q.mul_vec3(*v)).collect::<Vec<_>>();
        
        let uvs = vec![[0f32,0.0],[0.0,1.0],[1.0,1.0],[1.0,0.0]];
        let indices = vec![0,1,2,2,3,0];
        let normal = q.mul_vec3(Vec3::Y);
        let mut normals = vec![];
        normals.resize_with(4, || normal);

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh
    }
}

impl From<ViewcubeCorner> for Mesh {
    fn from(value: ViewcubeCorner) -> Self {
        let edge =0.1 * 2f32.sqrt();
        let a = edge / 2.0;
        let b = a * 3f32.sqrt();

        let d = 1.0 / 3.0f32;

        let (q, c) = match value.0 {
            CubePart::FrontLeftTop => (
                Quat::from_euler(
                EulerRot::YXZ, -PI_4, PI_4, 0.0),
                Vec3::new(-d, d, d)
            ),
            CubePart::FrontLeftBottom => (
                Quat::from_euler(
                EulerRot::YXZ, PI_4_3, -PI_4_3, 0.0),
                Vec3::new(-d, -d, d)
            ),
            CubePart::FrontRightTop => (
                Quat::from_euler(
                EulerRot::YXZ, PI_4, PI_4, 0.0),
                Vec3::new(d, d, d)
            ),
            CubePart::FrontRightBottom => (
                Quat::from_euler(
                EulerRot::YXZ, -PI_4_3, -PI_4_3, 0.0),
                Vec3::new(d, -d, d)
            ),
            CubePart::BackLeftTop => (
                Quat::from_euler(
                EulerRot::YXZ, -PI_4_3, PI_4, 0.0),
                Vec3::new(-d, d, -d)
            ),
            CubePart::BackLeftBottom => (
                Quat::from_euler(
                EulerRot::YXZ, PI_4, -PI_4_3, 0.0),
                Vec3::new(-d, -d, -d)
            ),
            CubePart::BackRightTop => (
                Quat::from_euler(
                EulerRot::YXZ, PI_4_3, PI_4, 0.0),
                Vec3::new(d, d, -d)
            ),
            CubePart::BackRightBottom => (
                Quat::from_euler(
                EulerRot::YXZ, -PI_4, -PI_4_3, 0.0),
                Vec3::new(d, -d, -d)
            ),
            _ => panic!(),
        };
        
        let positions = vec![
            Vec3::new(0.0, 0.0, -b /3.0 * 2.0),
            Vec3::new( -a, 0.0, b / 3.0),
            Vec3::new(  a, 0.0, b / 3.0),
        ].iter().map(|v| q.mul_vec3(*v) + c).collect::<Vec<_>>();

        let uvs = vec![[0f32,0.0],[0.0,1.0],[1.0,1.0]];
        let indices = vec![0u32, 1, 2];
        let normal = q.mul_vec3(Vec3::Y);
        let mut normals = vec![];
        normals.resize_with(3, || normal);

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh
    }
}


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
        generate_viewcube_simple_face(0.6f32, builder, &mut meshes, &mut materials);
        builder.spawn(
            generate_viewcube_face!(
                meshes, materials,
                ViewcubeEdge(CubePart::FrontTop),
                Color::PINK,
                Transform::IDENTITY,
                ViewcubeHit(CubePart::FrontTop)
            )
        );
        builder.spawn(
            generate_viewcube_face!(
                meshes, materials,
                ViewcubeEdge(CubePart::FrontBottom),
                Color::PINK,
                Transform::IDENTITY,
                ViewcubeHit(CubePart::FrontBottom)
            )
        );
        builder.spawn(
            generate_viewcube_face!(
                meshes, materials,
                ViewcubeEdge(CubePart::BackTop),
                Color::PINK,
                Transform::IDENTITY,
                ViewcubeHit(CubePart::BackTop)
            )
        );
        builder.spawn(
            generate_viewcube_face!(
                meshes, materials,
                ViewcubeEdge(CubePart::BackBottom),
                Color::PINK,
                Transform::IDENTITY,
                ViewcubeHit(CubePart::BackBottom)
            )
        );
        builder.spawn(
            generate_viewcube_face!(
                meshes, materials,
                ViewcubeEdge(CubePart::LeftTop),
                Color::PINK,
                Transform::IDENTITY,
                ViewcubeHit(CubePart::LeftTop)
            )
        );
        builder.spawn(
            generate_viewcube_face!(
                meshes, materials,
                ViewcubeEdge(CubePart::LeftBottom),
                Color::PINK,
                Transform::IDENTITY,
                ViewcubeHit(CubePart::LeftBottom)
            )
        );
        builder.spawn(
            generate_viewcube_face!(
                meshes, materials,
                ViewcubeEdge(CubePart::RightTop),
                Color::PINK,
                Transform::IDENTITY,
                ViewcubeHit(CubePart::RightTop)
            )
        );
        builder.spawn(
            generate_viewcube_face!(
                meshes, materials,
                ViewcubeEdge(CubePart::RightBottom),
                Color::PINK,
                Transform::IDENTITY,
                ViewcubeHit(CubePart::RightBottom)
            )
        );
        builder.spawn(
            generate_viewcube_face!(
                meshes, materials,
                ViewcubeEdge(CubePart::FrontLeft),
                Color::PINK,
                Transform::IDENTITY,
                ViewcubeHit(CubePart::FrontLeft)
            )
        );
        builder.spawn(
            generate_viewcube_face!(
                meshes, materials,
                ViewcubeEdge(CubePart::FrontRight),
                Color::PINK,
                Transform::IDENTITY,
                ViewcubeHit(CubePart::FrontRight)
            )
        );
        builder.spawn(
            generate_viewcube_face!(
                meshes, materials,
                ViewcubeEdge(CubePart::BackLeft),
                Color::PINK,
                Transform::IDENTITY,
                ViewcubeHit(CubePart::BackLeft)
            )
        );
        builder.spawn(
            generate_viewcube_face!(
                meshes, materials,
                ViewcubeEdge(CubePart::BackRight),
                Color::PINK,
                Transform::IDENTITY,
                ViewcubeHit(CubePart::BackRight)
            )
        );
        builder.spawn(
            generate_viewcube_face!(
                meshes, materials,
                ViewcubeCorner(CubePart::FrontLeftTop),
                Color::VIOLET,
                Transform::IDENTITY,
                ViewcubeHit(CubePart::FrontLeftTop)
            )
        );
        builder.spawn(
            generate_viewcube_face!(
                meshes, materials,
                ViewcubeCorner(CubePart::FrontLeftBottom),
                Color::VIOLET,
                Transform::IDENTITY,
                ViewcubeHit(CubePart::FrontLeftBottom)
            )
        );
        builder.spawn(
            generate_viewcube_face!(
                meshes, materials,
                ViewcubeCorner(CubePart::FrontRightTop),
                Color::VIOLET,
                Transform::IDENTITY,
                ViewcubeHit(CubePart::FrontRightTop)
            )
        );
        builder.spawn(
            generate_viewcube_face!(
                meshes, materials,
                ViewcubeCorner(CubePart::FrontRightBottom),
                Color::VIOLET,
                Transform::IDENTITY,
                ViewcubeHit(CubePart::FrontRightBottom)
            )
        );
        builder.spawn(
            generate_viewcube_face!(
                meshes, materials,
                ViewcubeCorner(CubePart::BackLeftTop),
                Color::VIOLET,
                Transform::IDENTITY,
                ViewcubeHit(CubePart::BackLeftTop)
            )
        );
        builder.spawn(
            generate_viewcube_face!(
                meshes, materials,
                ViewcubeCorner(CubePart::BackLeftBottom),
                Color::VIOLET,
                Transform::IDENTITY,
                ViewcubeHit(CubePart::BackLeftBottom)
            )
        );
        builder.spawn(
            generate_viewcube_face!(
                meshes, materials,
                ViewcubeCorner(CubePart::BackRightTop),
                Color::VIOLET,
                Transform::IDENTITY,
                ViewcubeHit(CubePart::BackRightTop)
            )
        );
        builder.spawn(
            generate_viewcube_face!(
                meshes, materials,
                ViewcubeCorner(CubePart::BackRightBottom),
                Color::VIOLET,
                Transform::IDENTITY,
                ViewcubeHit(CubePart::BackRightBottom)
            )
        );
    });
}