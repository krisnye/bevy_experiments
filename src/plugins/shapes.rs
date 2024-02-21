use bevy::prelude::*;
use crate::plugins::AppState;

pub struct ShapesPlugin;

//  This local state component is added to all entities we create in our system.
//  This makes it easy to query for and despawn all entities with this component on cleanup.
#[derive(Component)]
struct LocalStateFlag;

const STATE : AppState = AppState::Shapes;

impl Plugin for ShapesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(STATE), setup)
            .add_systems(Update, system.run_if(in_state(STATE)))
            .add_systems(OnExit(STATE), cleanup);
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    //  lights
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(1.0, 2.0, 1.0),
        ..default()
    }).insert(LocalStateFlag);

    //  camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }).insert(LocalStateFlag);

    // add a sphere to the end.
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere {
            radius: 0.5,
            sectors: 16,
            stacks: 16,
        })),
        material: materials.add(Color::BEIGE),
        transform: Transform::from_scale(Vec3::splat(0.5)),
        ..Default::default()
    }).insert(LocalStateFlag);
}

fn system() {
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<LocalStateFlag>>) {
    println!("Shapes cleanup");
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
