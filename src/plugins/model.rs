use bevy::prelude::*;
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
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(1.0, 2.0, 1.0),
        ..default()
    }).insert(CleanupFlag);

    //  camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 2.0).looking_at(Vec3::new(0.0, 0.5, 0.0), Vec3::Y),
        ..default()
    }).insert(CleanupFlag);

    //  load the flight helmet scene
    commands.spawn(SceneBundle {
        scene: asset_server.load("models/FlightHelmet/FlightHelmet.gltf#Scene0"),
        ..default()
    }).insert(CleanupFlag);
}

fn system() {
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<CleanupFlag>>) {
    println!("Model cleanup");
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
