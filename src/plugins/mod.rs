use bevy::diagnostic::{DiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
//  used to make enums iterable.
use strum_macros::{Display, EnumIter};
use crate::utils::fps_display::FPSDisplayPluginGroup;

mod model;
mod model_view_system;
mod physics_blocks;
mod shapes;
mod triangle;
mod voxel_editor;
mod many_cubes;
mod physics_new;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, EnumIter, Display)]
pub enum AppState {
    Triangle,
    Shapes,
    PhysicsBlocks,
    VoxelEditor,
    Model,
    ModelViewSystem,
    ManyCubes,
    #[default]
    PhysicsNew,
}

pub fn add_systems(app: &mut App) {
    app.add_plugins((
        FPSDisplayPluginGroup,
        triangle::TrianglePlugin,
        shapes::ShapesPlugin,
        physics_blocks::PhysicsBlocksPlugin,
        voxel_editor::VoxelEditorPluginGroup,
        model::ModelPlugin,
        model_view_system::ModelViewSystemPluginGroup,
        many_cubes::ManyCubesPluginGroup,
        physics_new::PhysicsNewPluginGroup,
    ));
}
