use super::AppState;
use bevy::app::PluginGroupBuilder;
use bevy::math::primitives;
use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};
use bevy_panorbit_camera::{PanOrbitCamera};
use rand::{Rng};
use std::f32::consts::*;
use crate::utils::mesh_builder::MeshBuilder;


/**
Lessons learned:
- drawing adjacent PBR meshes with the same material is faster than with alternating materials.
- drawing a single large mesh is much faster than drawing many small meshes
    - 1 million cube meshes -> 430ms frame time 2 fps
    - 1 large equivalent mesh -> 15ms frame time 75 fps
**/

#[derive(Component)]
struct CleanupFlag;

struct ManyCubesPlugin;

const STATE: AppState = AppState::ManyCubes;

impl Plugin for ManyCubesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(STATE), setup)
            .add_systems(Update, system.run_if(in_state(STATE)))
            .add_systems(OnExit(STATE), cleanup);
    }
}

pub struct ManyCubesPluginGroup;

impl PluginGroup for ManyCubesPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(ManyCubesPlugin)
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    //  lights
    commands
        .spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                ..default()
            },
            // This is a relatively small scene, so use tighter shadow
            // cascade bounds than the default for better quality.
            // We also adjusted the shadow map to be larger since we're
            // only using a single cascade.
            cascade_shadow_config: CascadeShadowConfigBuilder {
                // num_cascades: 1,
                // maximum_distance: 1.6,
                ..default()
            }
            .into(),
            ..default()
        })
        .insert(CleanupFlag);

    let count = 250;

    //  camera
    commands
        .spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(
                    170.0 * count as f32 / 1000.,
                    430.0 * count as f32 / 1000.,
                    1040.0 * count as f32 / 1000.,
                )
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
                ..default()
            },
            EnvironmentMapLight {
                diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
                specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
                intensity: 250.0,
            },
        ))
        .insert(PanOrbitCamera::default())
        .insert(CleanupFlag);

    let mesh = meshes.add(Mesh::from(primitives::Cuboid {
        half_size: Vec3::splat(0.5),
    }));
    let material1 = materials.add(Color::YELLOW);
    let material2 = materials.add(Color::BLACK);

    let stripe_thickness = 10; // Thickness of each stripe
    let width = count;
    let height = 1;
    let depth = count;

    const SINGLE_MESH: bool = true;
    if !SINGLE_MESH {
        for x in 0..width {
            for y in 0..height {
                for z in 0..depth {
                    let wave_height = (x as f32 / 20.0).sin() * 10.0
                        + (z as f32 / 30.0).cos() * 40.0 * count as f32 / 1000.; // Adjust multiplier for amplitude
                    let diagonal_position = (x + z) / stripe_thickness; // Calculate diagonal stripe index

                    commands
                        .spawn(PbrBundle {
                            mesh: mesh.clone(),
                            material: if diagonal_position % 2 == 0 {
                                material1.clone()
                            } else {
                                material2.clone()
                            },
                            transform: Transform::from_xyz(
                                x as f32 - width as f32 / 2.,
                                wave_height,
                                z as f32 - depth as f32 / 2.,
                            ),
                            ..Default::default()
                        })
                        .insert(CleanupFlag);
                }
            }
        }
    }

    //  BEGIN CREATE A SINGLE VERY LARGE MESH
    if SINGLE_MESH {
        let mut mesh_builder = MeshBuilder::create();

        for x in 0..width {
            for z in 0..depth {
                let height = if rand::thread_rng().gen_range(0..1000) <= 2 {
                    5 + rand::thread_rng().gen_range(0..15)
                } else {
                    1
                };
                let wave_height = (x as f32 / 20.0).sin() * 10.0
                    + (z as f32 / 30.0).cos() * 40.0 * count as f32 / 1000.; // Adjust multiplier for amplitude
                let diagonal_position = (x + z) / stripe_thickness; // Calculate diagonal stripe index
                mesh_builder.add_cube(
                    Vec3::new(
                        x as f32 - width as f32 / 2.,
                        wave_height,
                        z as f32 - depth as f32 / 2.,
                    ),
                    if diagonal_position % 2 == 0 {
                        Color::RED
                    } else {
                        Color::BLACK
                    },
                );
                if height > 1 {
                    mesh_builder.add_tree(
                        Vec3::new(
                            x as f32 - width as f32 / 2.,
                            wave_height,
                            z as f32 - depth as f32 / 2.,
                        ),
                        height,
                    );
                }
            }
        }

        let mesh_handle = mesh_builder.to_mesh(meshes);
        let material_handle = materials.add(Color::WHITE);

        // add the big mesh
        commands
            .spawn(PbrBundle {
                mesh: mesh_handle,
                material: material_handle,
                // transform: Transform::from_scale(Vec3::splat(count as f32 / 10.)),
                ..Default::default()
            })
            .insert(CleanupFlag);
    }
}

fn system(time: Res<Time>, mut query: Query<&mut Transform, With<DirectionalLight>>) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            time.elapsed_seconds() * PI / 5.0,
            -FRAC_PI_4,
        );
    }
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<CleanupFlag>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
