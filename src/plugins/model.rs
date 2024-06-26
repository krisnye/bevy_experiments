use bevy::{
    pbr::{CascadeShadowConfigBuilder},
    prelude::*,
};
use std::f32::consts::*;
use bevy_panorbit_camera::PanOrbitCamera;
use super::AppState;

#[derive(Component)]
struct CleanupFlag;

pub struct ModelPlugin;

const STATE : AppState = AppState::Model;

impl Plugin for ModelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(STATE), setup)
            .add_systems(Update, system.run_if(in_state(STATE)))
            .add_systems(OnExit(STATE), cleanup);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    //  lights
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        // This is a relatively small scene, so use tighter shadow
        // cascade bounds than the default for better quality.
        // We also adjusted the shadow map to be larger since we're
        // only using a single cascade.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            num_cascades: 1,
            maximum_distance: 1.6,
            ..default()
        }
        .into(),
        ..default()
    }).insert(CleanupFlag);

    //  camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.7, 0.7, 1.0)
                .looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
            ..default()
        },
        EnvironmentMapLight {
            diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
            specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
            intensity: 250.0,
        },
        PanOrbitCamera::default()
    )).insert(CleanupFlag);

    //  load the flight helmet scene
    commands.spawn(SceneBundle {
        scene: asset_server.load("models/FlightHelmet/FlightHelmet.gltf#Scene0"),
        ..default()
    }).insert(CleanupFlag);
}

fn system(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
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
