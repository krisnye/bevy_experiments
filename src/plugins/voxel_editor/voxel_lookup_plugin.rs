use super::{AppState};
use bevy::prelude::*;
use bevy::utils::HashMap;

pub struct VoxelLookupPlugin {
    pub state: AppState,
}

impl Plugin for VoxelLookupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.state), setup).add_systems(
            Update,
            voxel_lookup_system.run_if(in_state(self.state)),
        );
    }
}

fn setup(
    mut commands: Commands
) {
    // Volume Lookup
    commands.insert_resource(VoxelLookup(HashMap::default()));
}

#[derive(Component, PartialEq, Copy, Clone)]
pub struct Voxel;

//  begin volume lookup
#[derive(Resource)]
pub struct VoxelLookup(HashMap<Position, Entity>);

pub fn voxel_lookup_system(
    mut voxel_lookup: ResMut<VoxelLookup>,
    query: Query<(Entity, &Transform), (Changed<Transform>, With<Voxel>)>,
) {
    voxel_lookup.0.clear();
    for (entity, transform) in query.iter() {
        voxel_lookup
            .0
            .insert(Position::from_vec3(transform.translation), entity);
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    x: u32,
    y: u32,
    z: u32,
}

impl Position {
    fn new(x: u32, y: u32, z: u32) -> Self {
        Position { x, y, z }
    }

    // Creates a Position from a Bevy Vec3, rounding to the nearest u32
    fn from_vec3(vec: Vec3) -> Self {
        Position {
            x: vec.x.round() as u32,
            y: vec.y.round() as u32,
            z: vec.z.round() as u32,
        }
    }
}