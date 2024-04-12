use bevy::prelude::{Component, Reflect, Vec3};

#[derive(Component, Debug, Copy, Clone, Reflect)]
pub struct Position(pub Vec3);

#[derive(Component, Debug, Copy, Clone, Reflect)]
pub struct Velocity(pub Vec3);
