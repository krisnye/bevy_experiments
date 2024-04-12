use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Changed, IntoSystemConfigs, Query, Res, Time, Transform};
use crate::physics::{Position, Velocity};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, move_position)
            .add_systems(Update, position_transform.after(move_position))
        ;
    }
}

fn move_position(time: Res<Time>, mut query: Query<(&Velocity, &mut Position)>) {
    for (velocity, mut position) in query.iter_mut() {
        let new_position = position.0 + velocity.0 * time.delta_seconds();
        position.0 = new_position;
    }
}

fn position_transform(time: Res<Time>, mut query: Query<(&Position, &mut Transform), Changed<Position>>) {
    for (position, mut transform) in query.iter_mut() {
        transform.translation = position.0;
    }
}
