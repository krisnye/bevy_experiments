use bevy::prelude::*;
//  used to make enums iterable.
use strum_macros::{Display, EnumIter};
mod model;
mod model_view_system;
mod physics_blocks;
mod shapes;
mod triangle;
mod voxel_editor;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, EnumIter, Display)]
pub enum AppState {
    Triangle,
    Shapes,
    PhysicsBlocks,
    #[default]
    VoxelEditor,
    Model,
    ModelViewSystem,
}

pub fn add_systems(app: &mut App) {
    app.add_plugins((
        triangle::TrianglePlugin,
        shapes::ShapesPlugin,
        physics_blocks::PhysicsBlocksPlugin,
        voxel_editor::VoxelEditorPluginGroup,
        model::ModelPlugin,
        model_view_system::ModelViewSystemPluginGroup,
    ));
}
