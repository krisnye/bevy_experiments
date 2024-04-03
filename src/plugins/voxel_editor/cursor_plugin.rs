use super::{common::*, AppState, CleanupFlag};
use bevy::math::*;
use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;

#[derive(Component, Copy, Clone, PartialEq)]
pub struct Cursor;

#[derive(Component, Copy, Clone, PartialEq)]
pub struct Selection {
    pub start: Vec3,
    pub end: Vec3,
}

impl Selection {
    fn to_bounds(&self) -> Bounds {
        //  create a bounds from these positions using min/max
        let min = self.start.min(self.end);
        let max = self.start.max(self.end);
        Bounds {
            position: min,
            size: max - min + Vec3::splat(1.0),
        }
    }

    fn min(&self) -> Vec3 { self.start.min(self.end) }
    fn max(&self) -> Vec3 { self.start.max(self.end) }
}

pub struct CursorPlugin {
    pub state: AppState,
}

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.state), setup).add_systems(
            Update,
            (keyboard_cursor_handling, on_bounds_updated).run_if(in_state(self.state)),
        );
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    //  Cursor
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(primitives::Cuboid {
                half_size: Vec3::splat(0.5),
            })),
            material: materials.add(Color::rgba(0.5, 1.0, 0.5, 0.7)),
            transform: Transform::from_xyz(0.0, 1.5, 0.0),
            ..default()
        },
        Selection { start: Vec3::ZERO, end: Vec3::ZERO },
        Cursor,
        NotShadowCaster,
        CleanupFlag,
    ));
    //  Selection
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(primitives::Cuboid {
                half_size: Vec3::splat(0.5),
            })),
            material: materials.add(Color::rgba(0.5, 0.5, 1.0, 0.5)),
            transform: Transform::from_xyz(0.0, 1.5, 0.0),
            ..default()
        },
        Selection { start: Vec3::ZERO, end: Vec3::ZERO },
        NotShadowCaster,
        CleanupFlag,
    ));
}

fn on_bounds_updated(mut query: Query<(&Selection, &mut Transform), Changed<Selection>>) {
    for (selection, mut transform) in &mut query {
        transform.set_if_neq(selection.to_bounds().to_unit_cube_transform());
    }
}

fn get_keyboard_direction(keyboard_input: &Res<ButtonInput<KeyCode>>) -> Vec3 {
    let mut direction = Vec3::ZERO;
    if keyboard_input.just_pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    } else if keyboard_input.just_pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    } else if keyboard_input.just_pressed(KeyCode::KeyA)
        || keyboard_input.just_pressed(KeyCode::ArrowLeft)
    {
        direction.x -= 1.0;
    } else if keyboard_input.just_pressed(KeyCode::KeyD)
        || keyboard_input.just_pressed(KeyCode::ArrowRight)
    {
        direction.x += 1.0;
    } else if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        direction.z -= 1.0;
    } else if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        direction.z += 1.0;
    }
    direction
}

fn keyboard_cursor_handling(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Selection, Option<&Cursor>)>,
) {
    let direction = get_keyboard_direction(&keyboard_input);
    let shift = keyboard_input.pressed(KeyCode::ShiftLeft) || keyboard_input.pressed(KeyCode::ShiftRight);
    let sup = keyboard_input.pressed(KeyCode::SuperLeft) || keyboard_input.pressed(KeyCode::SuperRight);
    let space = keyboard_input.just_pressed(KeyCode::Space);
    let mut move_delta = Vec3::ZERO;
    if space {
        let mut height = LENGTH as f32;
        for (selection, cursor) in query.iter() {
            height = height.min(selection.min().y);
        }
        move_delta.y = -height;
    }
    for (mut selection, cursor) in query.iter_mut() {
        let initial = selection.clone();
        if (direction != Vec3::ZERO) {
            if sup {
                selection.start += direction;
                selection.end += direction;
            }
            else {
                let position = selection.end + direction;
                selection.end = position;
                if !shift {
                    selection.start = position;
                }
            }
        }
        selection.start += move_delta;
        selection.end += move_delta;

        //  check that we are still within bounds or else revert
        if !Bounds::LENGTH.contains(&selection.to_bounds()) {
            selection.start = initial.start;
            selection.end = initial.end;
        }
        if cursor.is_some() {
            //     the start will always match end
            selection.start = selection.end;
        }
    }
}
