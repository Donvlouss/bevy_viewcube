use bevy::{
    prelude::*, 
    render::{
        camera::ClearColorConfig,
        view::RenderLayers,
    },
    math::primitives::Sphere
};
use bevy_panorbit_camera::{
    PanOrbitCamera,
    PanOrbitCameraPlugin
};
use bevy_viewcube::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
                .add_systems(Startup, setup)
        .add_systems(Update, update_view)
        .run();
}

#[derive(Component, Default)]
struct SmallView;

#[derive(Component, Default)]
struct Trident;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            camera: Camera {
                ..default()
            },
            ..default()
        },
        PanOrbitCamera::default(),
    ));
    commands.spawn(DirectionalLightBundle {
        ..Default::default()
    });
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_rotation_x(std::f32::consts::PI)),
        ..Default::default()
    });
    
    // x
    commands.spawn(MaterialMeshBundle{
        mesh: meshes.add(Mesh::from(Sphere { radius: 0.75})),
        material: materials.add(StandardMaterial::from(Color::Rgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 })),
        transform: Transform::from_xyz(5.0, 0.0, 0.0),
        ..Default::default()
    });
    // y
    commands.spawn(MaterialMeshBundle{
        mesh: meshes.add(Mesh::from(Sphere { radius: 0.75})),
        material: materials.add(StandardMaterial::from(Color::Rgba { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 })),
        transform: Transform::from_xyz(0.0, 5.0, 0.0),
        ..Default::default()
    });
    // z
    commands.spawn(MaterialMeshBundle{
        mesh: meshes.add(Mesh::from(Sphere { radius: 0.75})),
        material: materials.add(StandardMaterial::from(Color::Rgba { red: 0.0, green: 0.0, blue: 1.0, alpha: 1.0 })),
        transform: Transform::from_xyz(0.0, 0.0, 5.0),
        ..Default::default()
    });

    // Trident
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(BevyTridentAxis::default()),
            material: materials.add(StandardMaterial::default()),
            ..Default::default()
        },
    ));
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(BevyTridentAxis::default()),
            material: materials.add(StandardMaterial::default()),
            ..Default::default()
        },
        RenderLayers::layer(13),
        Trident,
    ));

    commands.spawn((
        Camera3dBundle{
            camera: Camera {
                order: 1,
                clear_color: ClearColorConfig::None,
                ..Default::default()
            },
            camera_3d: Camera3d {
                depth_load_op: bevy::core_pipeline::core_3d::Camera3dDepthLoadOp::Clear(0.),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        RenderLayers::layer(13),
        SmallView,
    ));

    commands.spawn((
        DirectionalLightBundle {
        ..Default::default()
        },
        RenderLayers::layer(13),
    ));
    commands.spawn((
        DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_rotation_x(std::f32::consts::PI)),
        ..Default::default()
        },
        RenderLayers::layer(13),
    ));

}

fn update_view(
    windows: Query<&Window>,
    mut trident: Query<&mut Transform, With<Trident>>,
    mut camera: Query<&mut Camera, With<SmallView>>,
    orbit_cameras: Query<&Transform, (With<PanOrbitCamera>, Without<Trident>)>,
) {
    let window = windows.single();
    let mut cam = camera.single_mut();
    cam.viewport = Some(bevy::render::camera::Viewport {
        physical_position: UVec2::new(
            0, (window.physical_height() as f32 * 0.9) as u32
        ),
        physical_size: UVec2::new(
            (window.physical_width() as f32 * 0.1) as u32,
            (window.physical_height() as f32 * 0.1) as u32,
        ),
        ..default()
    });
    let mut trident_transform = trident.single_mut();
    let transform = orbit_cameras.single();
    trident_transform.rotation = transform.rotation.inverse();
}