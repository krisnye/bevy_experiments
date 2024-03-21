use bevy::prelude::*;
use bevy::math::*;
use bevy::pbr::ScreenSpaceAmbientOcclusionBundle;
use super::AppState;

//  This local state component is added to all entities we create in our system.
//  This makes it easy to query for and despawn all entities with this component on cleanup.
#[derive(Component)]
struct CleanupFlag;

#[derive(Component)]
struct Cursor;

pub struct VoxelEditorPlugin;

const STATE : AppState = AppState::VoxelEditor;

impl Plugin for VoxelEditorPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Msaa::Off)// turn off multi sample anti aliasing
            //  should this be applied here to all plugins?
            .insert_resource(AmbientLight {
                brightness: 5.0,
                ..default()
            })
            .add_systems(OnEnter(STATE), setup)
            .add_systems(Update, system.run_if(in_state(STATE)))
            .add_systems(Update, keyboard_cursor_handling.run_if(in_state(STATE)))
            .add_systems(OnExit(STATE), cleanup);
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
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1_000_000.,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 8.0),
        ..default()
    }).insert(CleanupFlag);
    // Camera
    commands.spawn(Camera3dBundle {
        camera: Camera {
            hdr: true,
            ..default()
        },
        transform: Transform::from_xyz(-4.0, 6.5, 28.0).looking_at(Vec3::new(0., 8., 0.), Vec3::Y),
        ..default()
    })
        .insert(ScreenSpaceAmbientOcclusionBundle::default())   //  screen space ambient occlusion!
        .insert(CleanupFlag);
    // Infinite Flat Plane
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(primitives::Plane3d { normal: Direction3d::Y })),
            transform: Transform::from_scale(Vec3::splat(1000.)),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
            ..default()
        },
    )).insert(CleanupFlag);

    let cube_mesh = meshes.add(Mesh::from(primitives::Cuboid { half_size: Vec3::splat(0.5)}));
    let material_red = materials.add(Color::rgb(0.8, 0.7, 0.6));
    let material_black = materials.add(Color::rgb(0.4, 0.2, 0.3));

    let size = 1.0;
    // Cube
    for x in -4 .. 4 {
        for y in 0 ..= 0 {
            for z in -4 ..= 4 {
                commands.spawn((
                    PbrBundle {
                        mesh: cube_mesh.clone(),
                        material:  if (x + y + z) % 2 == 0 { material_red.clone() } else { material_black.clone() },
                        transform: Transform::from_xyz(size + x as f32, size / 2. + y as f32, size + z as f32),
                        ..default()
                    },
                )).insert(CleanupFlag);
            }
        }
    }
    //  add a Cursor.
    commands.spawn((
        PbrBundle {
            mesh: cube_mesh.clone(),
            material:  materials.add(Color::WHITE),
            transform: Transform::from_xyz(0.0, 1.5, 0.0),
            ..default()
        },
        Cursor,
        CleanupFlag,
    ));
}

fn system() {
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<CleanupFlag>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn keyboard_cursor_handling(keyboard_input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Transform, With<Cursor>>) {
    for mut transform in query.iter_mut() {
        // Up
        if keyboard_input.just_pressed(KeyCode::KeyW) {
            transform.translation.y += 1.0;
        }
        // Down
        if keyboard_input.just_pressed(KeyCode::KeyS) {
            transform.translation.y -= 1.0;
        }
        // Left
        if keyboard_input.just_pressed(KeyCode::KeyA) || keyboard_input.just_pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= 1.0;
        }
        // Right
        if keyboard_input.just_pressed(KeyCode::KeyD) || keyboard_input.just_pressed(KeyCode::ArrowRight) {
            transform.translation.x += 1.0;
        }
        // Forward
        if keyboard_input.just_pressed(KeyCode::ArrowUp) {
            transform.translation.z -= 1.0;
        }
        // Backward
        if keyboard_input.just_pressed(KeyCode::ArrowDown) {
            transform.translation.z += 1.0;
        }
    }
}
