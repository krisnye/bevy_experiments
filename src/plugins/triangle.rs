use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::mesh::VertexAttributeValues;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::PrimitiveTopology;
use super::AppState;

//  This local state component is added to all entities we create in our system.
//  This makes it easy to query for and despawn all entities with this component on cleanup.
#[derive(Component)]
struct CleanupFlag;

pub struct TrianglePlugin;

const STATE : AppState = AppState::Triangle;

impl Plugin for TrianglePlugin {
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
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }).insert(CleanupFlag);

    // Triangle Mesh
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::Float32x3(vec![
            [0.0, 0.707, 0.0], // Top vertex
            [-0.6, -0.4, 0.0], // Bottom left vertex
            [0.6, -0.4, 0.0],  // Bottom right vertex
        ]),
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        VertexAttributeValues::Float32x3(vec![
            [0.0, 0.0, 1.0], // Normal for top vertex
            [0.0, 0.0, 1.0], // Normal for bottom left vertex
            [0.0, 0.0, 1.0], // Normal for bottom right vertex
        ]),
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_COLOR,
        VertexAttributeValues::Float32x4(vec![
            [1.0, 0.0, 0.0, 1.0], // Color for top vertex
            [0.0, 1.0, 0.0, 1.0], // Color for bottom left vertex
            [0.0, 0.0, 1.0, 1.0], // Color for bottom right vertex
        ]),
    );
    // optionally add indexes. If you don't then non-indexed vertices are drawn.
    mesh.insert_indices(Indices::U32(vec![0, 1, 2])); // Indices for the triangle

    //  meshes not getting cleaned up on exit?  memory leak TODO.

    let mesh_handle = meshes.add(mesh);
    let material_handle = materials.add(Color::WHITE);

    // Spawn the triangle entity
    commands.spawn(PbrBundle {
        mesh: mesh_handle,
        material: material_handle,
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
