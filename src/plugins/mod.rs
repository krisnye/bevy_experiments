use bevy::prelude::*;
//  used to make enums iterable.
use strum_macros::{EnumIter, Display};
pub mod triangle;
pub mod shapes;
mod physics_blocks;
mod model;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, EnumIter, Display)]
pub enum AppState {
    Triangle,
    Shapes,
    #[default]
    PhysicsBlocks,
    Model,
}

pub fn add_systems(app: &mut App) {
    app
        .add_plugins(triangle::TrianglePlugin)
        .add_plugins(shapes::ShapesPlugin)
        .add_plugins(model::ModelPlugin)
        .add_plugins(physics_blocks::PhysicsBlocksPlugin);
}
