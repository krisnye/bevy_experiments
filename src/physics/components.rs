use bevy::prelude::{Component, Reflect, Resource, Vec3};
use bevy::render::primitives::Aabb;

#[derive(Component, Debug, Copy, Clone, Reflect)]
pub struct Position(pub Vec3);

#[derive(Component, Debug, Copy, Clone, Reflect)]
pub struct Velocity(pub Vec3);

#[derive(Resource, Debug, Copy, Clone, Reflect)]
pub struct PhysicsWorld {
    pub bounds: Aabb,
    pub gravity: Vec3,
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        PhysicsWorld {
            bounds: Aabb::from_min_max(Vec3::splat(-8.), Vec3::splat(8.)),
            gravity: Vec3 { x: 0., y: -9.8, z: 0.},
        }
    }
}

//  indicates the Entity is affected by Gravity
#[derive(Component)]
pub struct Gravity;

#[derive(Resource, Debug, Copy, Clone, Reflect)]
pub struct GravitationalAcceleration(Vec3);
