use bevy::prelude::*;
use bevy_egui::{EguiPlugin};
pub mod triangle;
pub mod shapes;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Triangle,
    Shapes,
}

pub fn add_systems(app: &mut App) {
    app
        .add_plugins(triangle::TrianglePlugin)
        .add_plugins((shapes::ShapesPlugin));
}
