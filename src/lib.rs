mod trident;
mod viewcube;

use std::f32::consts::PI;
const PI_2:f32 = PI / 2.0;
const PI_4:f32 = PI / 4.0;
const PI_4_3:f32 = PI / 4.0 * 3.0;

#[derive(Component)]
pub(crate) struct SmallView;

/// Add this to target pan_orbit_camera.
/// panic when not set any ViewcubeBinding
#[derive(Component)]
pub struct ViewcubeBinding;

use bevy::{
    prelude::*,
    render::{view::RenderLayers, camera::ClearColorConfig},
};

/// The function `create_small_view` creates a small 3D camera view with a directional light in Rust
/// using the Bevy game engine.
pub(crate) fn create_small_view(mut commands: bevy::ecs::system::Commands) {
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
            transform: Transform::from_xyz(0.6, 0.6, 4.0).looking_at(Vec3::new(0.6, 0.6, 0.6), Vec3::Y),
            ..default()
        },
        RenderLayers::layer(13),
        SmallView,
    )).with_children(|builder| {
        builder.spawn((
            DirectionalLightBundle {
            ..Default::default()
            },
            RenderLayers::layer(13),
        ));
    });
}

pub mod prelude {
    pub use crate::trident::{
        BevyTridentArrow,
        BevyTridentAxis,
        BevyTridentCone
    };
    pub use crate::viewcube::BevyViewCubePlugin;
    pub use crate::ViewcubeBinding;
}