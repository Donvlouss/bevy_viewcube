mod simple_viewcube;
mod powerful_viewcube;
use bevy::{app::{Plugin, Startup, Update}, ecs::{component::Component, entity::Entity, query::{With, Without}, schedule::IntoSystemConfigs, system::{Commands, Query}}, math::{UVec2, Vec3}, prelude::default, render::camera::Camera, transform::components::Transform, window::Window};
use bevy_panorbit_camera::PanOrbitCamera;

use crate::{PI_2, PI_4, PI_4_3};

#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) enum CubePart {
    // Face
    Front, Back, Left, Right, Top, Bottom,
    // Edge
    FrontTop, FrontBottom, BackTop, BackBottom,
    LeftTop, LeftBottom, RightTop, RightBottom,
    FrontLeft, FrontRight, BackLeft, BackRight,
    // Corner
    FrontLeftTop, FrontLeftBottom, FrontRightTop, FrontRightBottom,
    BackLeftTop, BackLeftBottom, BackRightTop, BackRightBottom,
}

#[derive(Default)]
pub struct BevyViewCubePlugin {
    pub use_powerful_viewcube: bool,
}

impl Plugin for BevyViewCubePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let setup = if self.use_powerful_viewcube {
            powerful_viewcube::setup
        } else {
            simple_viewcube::setup
        };
        app
        .add_systems(Startup, (setup,crate::create_small_view).chain())
        .add_systems(Update, update_view)
        .add_systems(Update, viewcube_hit)
        ;
    }
}

#[derive(Component)]
pub(crate) struct ViewcubeCenter;

#[derive(Component)]
pub(crate) struct ViewcubeHit(pub CubePart);

#[macro_export]
macro_rules! generate_viewcube_face {
    ($meshes:ident, $materials: ident, $part: expr, $color: expr, $transform: expr, $component: expr) => {
        (MaterialMeshBundle {
            mesh: $meshes.add($part.clone().into()),
            material: $materials.add(StandardMaterial::from($color)),
            transform: $transform,
            ..Default::default()
        },
        RenderLayers::layer(13),
        PickableBundle::default(),
        On::<Pointer<Click>>::commands_mut(|event, commands| {
            commands.entity(event.target).insert($component);
        }))
    };
}


pub(crate) fn update_view(
    windows: Query<&Window>,
    mut trident: Query<&mut Transform, With<ViewcubeCenter>>,
    mut camera: Query<&mut Camera, With<crate::SmallView>>,
    orbit_cameras: Query<&Transform, (With<PanOrbitCamera>, Without<ViewcubeCenter>)>,
) {
    let window = windows.single();
    let mut cam = camera.single_mut();
    cam.viewport = Some(bevy::render::camera::Viewport {
        physical_position: UVec2::new(
            0, (window.physical_height() as f32 * 0.6) as u32
        ),
        physical_size: UVec2::new(
            (window.physical_width() as f32 * 0.3) as u32,
            (window.physical_height() as f32 * 0.4) as u32,
        ),
        ..default()
    });
    let mut trident_transform = trident.single_mut();
    let transform = orbit_cameras.single();
    trident_transform.rotation = transform.rotation.inverse();
}

pub(crate) fn viewcube_hit(
    mut commands: Commands,
    entity: Query<(Entity, &ViewcubeHit)>,
    mut camera: Query<&mut PanOrbitCamera>,
) {
    if entity.is_empty() {
        return;
    }
    let (item, dir) = entity.single();
    commands.entity(item).remove::<ViewcubeHit>();

    let (alpha, beta) = match dir.0 {
        CubePart::Right => (PI_2, 0.0),
        CubePart::Left => (-PI_2, 0.0),
        CubePart::Top => (0.0, PI_2),
        CubePart::Bottom => (0.0, -PI_2),
        CubePart::Front => (0.0, 0.0),
        CubePart::Back => (crate::PI, 0.0),
        CubePart::FrontTop => (0.0, PI_4),
        CubePart::FrontBottom => (0.0, -PI_4),
        CubePart::BackTop => (crate::PI, PI_4),
        CubePart::BackBottom => (crate::PI, -PI_4),
        CubePart::LeftTop => (-PI_2, PI_4),
        CubePart::LeftBottom => (-PI_2, -PI_4),
        CubePart::RightTop => (PI_2, PI_4),
        CubePart::RightBottom => (PI_2, -PI_4),
        CubePart::FrontLeft => (-PI_4, 0.0),
        CubePart::FrontRight => (PI_4, 0.0),
        CubePart::BackLeft => (-PI_4_3, 0.0),
        CubePart::BackRight => (PI_4_3, 0.0),
        CubePart::FrontLeftTop => (-PI_4, PI_4),
        CubePart::FrontLeftBottom => (-PI_4, -PI_4),
        CubePart::FrontRightTop => (PI_4, PI_4),
        CubePart::FrontRightBottom => (PI_4, -PI_4),
        CubePart::BackLeftTop => (-PI_4_3, PI_4),
        CubePart::BackLeftBottom => (-PI_4_3, -PI_4),
        CubePart::BackRightTop => (PI_4_3, PI_4),
        CubePart::BackRightBottom => (PI_4_3, -PI_4),
    };

    let mut orbit_camera = camera.single_mut();

    orbit_camera.target_focus = Vec3::ZERO;
    orbit_camera.target_alpha = alpha;
    orbit_camera.target_beta = beta;
}