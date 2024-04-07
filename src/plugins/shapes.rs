use std::f32::consts::PI;
use bevy::math::*;
use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;
use super::AppState;

pub struct ShapesPlugin;

//  This local state component is added to all entities we create in our system.
//  This makes it easy to query for and despawn all entities with this component on cleanup.
#[derive(Component)]
struct CleanupFlag;

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
    }).insert(CleanupFlag);

    //  camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
            },
        PanOrbitCamera::default()
    )).insert(CleanupFlag);

    // sphere
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(primitives::Sphere { radius: 0.5 })),
        material: materials.add(Color::RED),
        transform: Transform::from_xyz(-2., 1., 0.),
        ..Default::default()
    }).insert(CleanupFlag);

    // torus
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(primitives::Torus { minor_radius: 0.25, major_radius: 0.5 })),
        material: materials.add(Color::ORANGE),
        transform: Transform::from_xyz(0., 1., 0.).with_rotation(Quat::from_rotation_x(PI / 2.0)),
        ..Default::default()
    }).insert(CleanupFlag);

    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(primitives::Cuboid { half_size: Vec3::splat(0.5) })),
        material: materials.add(Color::YELLOW),
        transform: Transform::from_xyz(2., 1., 0.),
        ..Default::default()
    }).insert(CleanupFlag);

    // capsule
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(primitives::Capsule3d { radius: 0.5, half_length: 0.5})),
        material: materials.add(Color::GREEN),
        transform: Transform::from_xyz(-2., -1., 0.),
        ..Default::default()
    }).insert(CleanupFlag);

    // cylinder
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(primitives::Cylinder { radius: 0.5, half_height: 0.5 })),
        material: materials.add(Color::BLUE),
        transform: Transform::from_xyz(0., -1., 0.),
        ..Default::default()
    }).insert(CleanupFlag);

    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(primitives::Plane3d { normal: Direction3d::from_xyz(0., 1., 0.).unwrap() })),
        material: materials.add(Color::VIOLET),
        transform: Transform::from_xyz(2., -1., 0.),
        ..Default::default()
    }).insert(CleanupFlag);
}

fn system() {
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<CleanupFlag>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
