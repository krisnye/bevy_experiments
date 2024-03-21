mod cursor_plugin;
pub mod common;

use super::AppState;
use bevy::app::PluginGroupBuilder;
use bevy::math::*;
use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use common::*;
use cursor_plugin::*;

//  This local state component is added to all entities we create in our system.
//  This makes it easy to query for and despawn all entities with this component on cleanup.
#[derive(Component)]
struct CleanupFlag;

const STATE: AppState = AppState::VoxelEditor;
struct VoxelEditorPlugin;

impl Plugin for VoxelEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(STATE), setup)
            .add_systems(OnExit(STATE), cleanup);
    }
}

pub struct VoxelEditorPluginGroup;

impl PluginGroup for VoxelEditorPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(VoxelEditorPlugin)
            .add(PanOrbitCameraPlugin)
            .add(CursorPlugin { state: STATE })
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0,
    });
    commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 2_000_000.,
                range: 100.,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(3.0, 12.0, 2.0),
            ..default()
        })
        .insert(CleanupFlag);
    // Camera
    commands
        .spawn(Camera3dBundle {
            camera: Camera {
                ..default()
            },
            transform: Transform::from_xyz(-4.0, 18.0, 18.0),
            ..default()
        })
        .insert(PanOrbitCamera::default())
        .insert(CleanupFlag);
    // Infinite Flat Plane
    commands
        .spawn((PbrBundle {
            mesh: meshes.add(Mesh::from(primitives::Plane3d {
                normal: Direction3d::Y,
            })),
            //  transform the plane slightly down beneath the y origin.
            transform: Transform::from_scale(Vec3::splat(1000.)).with_translation(Vec3::new(0.0, -0.05, 0.0)),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
            ..default()
        },))
        .insert(CleanupFlag);

    let cube_mesh = meshes.add(Mesh::from(primitives::Cuboid {
        half_size: Vec3::splat(0.5),
    }));
    let material_red = materials.add(Color::rgb(0.8, 0.7, 0.6));
    let material_black = materials.add(Color::rgb(0.4, 0.2, 0.3));

    // Cube
    for x in 0 ..LENGTH {
        for z in 0 ..LENGTH {
            let y = -1;
            commands
                .spawn((PbrBundle {
                    mesh: cube_mesh.clone(),
                    material: if (x + z) % 2 == 0 {
                        material_red.clone()
                    } else {
                        material_black.clone()
                    },
                    transform: Bounds::new(x as f32, y as f32, z as f32, 1.0, 1.0, 1.0).to_unit_cube_transform(),
                    ..default()
                }))
                .insert(CleanupFlag);
        }
    }
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<CleanupFlag>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}