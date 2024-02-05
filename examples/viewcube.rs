use bevy::{
    prelude::*,
    render::view::RenderLayers
};
use bevy_panorbit_camera::{
    PanOrbitCamera,
    PanOrbitCameraPlugin
};
use bevy_mod_picking::prelude::*;

use bevy_viewcube::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(DefaultPickingPlugins)
        .add_plugins(BevyViewCubePlugin{use_powerful_viewcube:true})
        .add_systems(Startup, setup)
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
        PanOrbitCamera {
            allow_upside_down: true,
            ..Default::default()
        },
        // Need set camera layer, or viewcube would be selected on this camera.
        RenderLayers::layer(0),
    ));
    commands.spawn(DirectionalLightBundle {
        ..Default::default()
    });
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_rotation_x(std::f32::consts::PI)),
        ..Default::default()
    });

    // Trident
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add((BevyTridentAxis::default()).into()),
            material: materials.add(StandardMaterial::default()),
            ..Default::default()
        },
    ));
}
