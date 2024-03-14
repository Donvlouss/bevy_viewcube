use bevy::prelude::*;
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
        // bevy_ui debug bug(https://github.com/aevyrie/bevy_mod_picking/issues/317), use default to disable debug ui
        .insert_resource(DebugPickingMode::Normal)
        .add_plugins(BevyViewCubePlugin{use_powerful_viewcube:true})
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn((
        Camera3dBundle {
                        transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PanOrbitCamera {
            allow_upside_down: true,
            ..Default::default()
        },
        ViewcubeBinding,
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
            mesh: meshes.add(BevyTridentAxis::default()),
            material: materials.add(StandardMaterial::default()),
            ..Default::default()
        },
        PickableBundle::default(),
    ));
}
