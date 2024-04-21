use bevy::prelude::{Component, Entity, Reflect, Resource, Vec3};
use bevy::render::primitives::Aabb;

#[derive(Component, Debug, Copy, Clone, Reflect)]
pub struct Particle {
    pub position: Vec3,
    pub velocity: Vec3,
    pub mass: f32,
}

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

#[derive(Component, Debug, Copy, Clone, Reflect)]
pub struct Spring {
    pub particle_a: Entity,
    pub particle_b: Entity,
    pub rest_length: f32,
    pub stiffness: f32,
    pub damping: f32,
}
