use bevy::prelude::*;
//  used to make enums iterable.
use strum_macros::{EnumIter, Display};
mod triangle;
mod shapes;
mod physics_blocks;
mod model;
mod voxel_editor;
mod model_view_system;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, EnumIter, Display)]
pub enum AppState {
    Triangle,
    Shapes,
    PhysicsBlocks,
    #[default]
    VoxelEditor,
    Model,
    ModelViewSystem
}

pub fn add_systems(app: &mut App) {
    app.add_plugins((
        triangle::TrianglePlugin,
        shapes::ShapesPlugin,
        physics_blocks::PhysicsBlocksPlugin,
        voxel_editor::VoxelEditorPlugin,
        model::ModelPlugin,
        model_view_system::ModelViewSystemPluginGroup,
    ));
}
