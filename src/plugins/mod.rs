use bevy::prelude::*;
pub mod triangle;
pub mod shapes;
mod physics_blocks;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    Triangle,
    Shapes,
    #[default]
    PhysicsBlocks,
}

pub fn add_systems(app: &mut App) {
    app
        .add_plugins(triangle::TrianglePlugin)
        .add_plugins(shapes::ShapesPlugin)
        .add_plugins(physics_blocks::PhysicsBlocksPlugin);
}
