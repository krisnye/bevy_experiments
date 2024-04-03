use bevy::{prelude::*, utils::HashMap};

pub const LENGTH: u32 = 8;

#[derive(Component, Default, PartialEq, Copy, Clone)]
pub struct Bounds {
    pub position: Vec3,
    pub size: Vec3,
}

impl Bounds {
    pub fn new(x: f32, y: f32, z: f32, width: f32, height: f32, depth: f32) -> Bounds {
        Bounds {
            position: Vec3::new(x, y, z),
            size: Vec3::new(width, height, depth),
        }
    }

    pub fn max(&self) -> Vec3 {
        self.position + self.size
    }

    pub fn to_unit_cube_transform(&self) -> Transform {
        Transform {
            translation: Vec3::new(
                self.position.x + self.size.x / 2.0 - LENGTH as f32 / 2.0,
                self.position.y + self.size.y / 2.0,
                self.position.z + self.size.z / 2.0 - LENGTH as f32 / 2.0,
            ),
            scale: self.size,
            rotation: Quat::IDENTITY,
        }
    }

    pub fn contains(&self, other: &Bounds) -> bool {
        self.position.min(other.position) == self.position
            && self.max().max(other.max()) == self.max()
    }

    pub const LENGTH: Bounds = Bounds {
        position: Vec3::ZERO,
        size: Vec3::splat(LENGTH as f32),
    };

    pub const UNIT: Bounds = Bounds {
        position: Vec3::ZERO,
        size: Vec3::splat(1.0),
    };
}

