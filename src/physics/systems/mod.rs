use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Changed, IntoSystemConfigs, Query, Res, Time, Transform, With};
use crate::physics::{Position, Velocity, PhysicsWorld, Gravity, Mass};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, gravitational_acceleration)
            .add_systems(Update, move_position.after(gravitational_acceleration))
            .add_systems(Update, position_transform.after(move_position))
        ;
    }
}

fn move_position(
    time: Res<Time>,
    physics_world: Res<PhysicsWorld>,
    mut query: Query<(&mut Velocity, &mut Position)>,
) {
    let dt = time.delta_seconds();
    let bounds_min = physics_world.bounds.min();
    let bounds_max = physics_world.bounds.max();
    for (mut vel, mut pos) in query.iter_mut() {
        let new_position = pos.0 + vel.0 * dt;

        // Check collision and reflect if necessary, also adjust position based on the collision time
        // X-axis
        if new_position.x <= bounds_min.x || new_position.x >= bounds_max.x {
            let time_to_collision_x = if vel.0.x != 0.0 {
                let bound = if new_position.x <= bounds_min.x { bounds_min.x } else { bounds_max.x };
                (bound - pos.0.x) / vel.0.x
            } else {
                0.0
            };
            vel.0.x = -vel.0.x;
            pos.0.x += vel.0.x * (dt - time_to_collision_x);
        } else {
            pos.0.x = new_position.x;
        }

        // Y-axis
        if new_position.y <= bounds_min.y || new_position.y >= bounds_max.y {
            let time_to_collision_y = if vel.0.y != 0.0 {
                let bound = if new_position.y <= bounds_min.y { bounds_min.y } else { bounds_max.y };
                (bound - pos.0.y) / vel.0.y
            } else {
                0.0
            };
            vel.0.y = -vel.0.y;
            pos.0.y += vel.0.y * (dt - time_to_collision_y);
        } else {
            pos.0.y = new_position.y;
        }

        // Z-axis
        if new_position.z <= bounds_min.z || new_position.z >= bounds_max.z {
            let time_to_collision_z = if vel.0.z != 0.0 {
                let bound = if new_position.z <= bounds_min.z { bounds_min.z } else { bounds_max.z };
                (bound - pos.0.z) / vel.0.z
            } else {
                0.0
            };
            vel.0.z = -vel.0.z;
            pos.0.z += vel.0.z * (dt - time_to_collision_z);
        } else {
            pos.0.z = new_position.z;
        }
    }
}

fn position_transform(time: Res<Time>, mut query: Query<(&Position, &mut Transform), Changed<Position>>) {
    for (position, mut transform) in query.iter_mut() {
        transform.translation = position.0;
    }
}

fn gravitational_acceleration(
    time: Res<Time>,
    physics_world: Res<PhysicsWorld>,
    mut query: Query<(&mut Velocity), With<Gravity>>,
) {
    let acceleration = physics_world.gravity * time.delta_seconds();
    for (mut velocity) in query.iter_mut() {
        velocity.0 += acceleration;
    }
}
