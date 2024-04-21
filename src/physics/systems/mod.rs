use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Changed, Entity, IntoSystemConfigs, Query, Res, Time, Transform, Vec3, With};
use crate::physics::{PhysicsWorld, Gravity, Spring, Particle};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, gravitational_acceleration)
            .add_systems(Update, move_position.after(gravitational_acceleration))
            .add_systems(Update, spring_system.after(move_position))
            .add_systems(Update, position_transform.after(move_position))
        ;
    }
}

fn move_position(
    time: Res<Time>,
    physics_world: Res<PhysicsWorld>,
    mut query: Query<&mut Particle>,
) {
    let dt = time.delta_seconds();
    let bounds_min = physics_world.bounds.min();
    let bounds_max = physics_world.bounds.max();
    for mut particle in query.iter_mut() {
        let mut pos = particle.position;
        let mut vel = particle.velocity;
        let new_position = pos + vel * dt;

        // Check collision and reflect if necessary, also adjust position based on the collision time
        // X-axis
        if new_position.x <= bounds_min.x || new_position.x >= bounds_max.x {
            let time_to_collision_x = if vel.x != 0.0 {
                let bound = if new_position.x <= bounds_min.x { bounds_min.x } else { bounds_max.x };
                (bound - pos.x) / vel.x
            } else {
                0.0
            };
            vel.x = -vel.x;
            pos.x += vel.x * (dt - time_to_collision_x);
        } else {
            pos.x = new_position.x;
        }

        // Y-axis
        if new_position.y <= bounds_min.y || new_position.y >= bounds_max.y {
            let time_to_collision_y = if vel.y != 0.0 {
                let bound = if new_position.y <= bounds_min.y { bounds_min.y } else { bounds_max.y };
                (bound - pos.y) / vel.y
            } else {
                0.0
            };
            vel.y = -vel.y;
            pos.y += vel.y * (dt - time_to_collision_y);
        } else {
            pos.y = new_position.y;
        }

        // Z-axis
        if new_position.z <= bounds_min.z || new_position.z >= bounds_max.z {
            let time_to_collision_z = if vel.z != 0.0 {
                let bound = if new_position.z <= bounds_min.z { bounds_min.z } else { bounds_max.z };
                (bound - pos.z) / vel.z
            } else {
                0.0
            };
            vel.z = -vel.z;
            pos.z += vel.z * (dt - time_to_collision_z);
        } else {
            pos.z = new_position.z;
        }

        particle.position = pos;
        particle.velocity = vel;
    }
}

fn position_transform(mut query: Query<(&Particle, &mut Transform), Changed<Particle>>) {
    for (particle, mut transform) in query.iter_mut() {
        transform.translation = particle.position;
    }
}

fn gravitational_acceleration(
    time: Res<Time>,
    physics_world: Res<PhysicsWorld>,
    mut query: Query<&mut Particle, With<Gravity>>,
) {
    let acceleration = physics_world.gravity * time.delta_seconds();
    for mut particle in query.iter_mut() {
        particle.velocity += acceleration;
    }
}

fn spring_system(
    time: Res<Time>,
    mut query: Query<&mut Particle>,
    spring_query: Query<&Spring>,
) {
    let delta_seconds = time.delta_seconds();
    let mut forces = std::collections::HashMap::<Entity, Vec3>::new();

    for spring in spring_query.iter() {
        let [mut particle_a, mut particle_b] = query.get_many_mut([spring.particle_a, spring.particle_b]).unwrap();
        let displacement = particle_b.position - particle_a.position;
        let distance = displacement.length();
        let force_magnitude = spring.stiffness * (distance - spring.rest_length) - spring.damping * (particle_b.velocity - particle_a.velocity).dot(displacement/distance);
        let force = displacement.normalize() * force_magnitude;
        *forces.entry(spring.particle_a).or_insert(Vec3::ZERO) += force;
        *forces.entry(spring.particle_b).or_insert(Vec3::ZERO) -= force;
    }

    for (entity, force) in forces {
        if let Ok(mut particle) = query.get_mut(entity) {
            let acceleration = force / particle.mass;
            particle.velocity += acceleration * delta_seconds;
        }
    }
}