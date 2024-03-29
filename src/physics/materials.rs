use crate::physics::{PhysicsMaterial, PhysicsPhase};

pub const AIR: PhysicsMaterial = PhysicsMaterial {
    name: "Air",
    phase: PhysicsPhase::Gas,
    specific_heat_capacity: 1006.0,
    thermal_conductivity: 0.024,
    density:  0.0012,
    viscosity: 0.0181,
};
pub const WATER: PhysicsMaterial = PhysicsMaterial {
    name: "Water",
    phase: PhysicsPhase::Liquid,
    specific_heat_capacity: 4200.0,
    thermal_conductivity: 0.66,
    density: 0.997,
    viscosity: 1.0,
};
pub const ROCK: PhysicsMaterial = PhysicsMaterial {
    name: "Rock",
    specific_heat_capacity: 800.0,
    thermal_conductivity: 4.0,
    density: 2.65,
    phase: PhysicsPhase::Solid,
    viscosity: f32::INFINITY,
};
pub const ICE: PhysicsMaterial = PhysicsMaterial {
    name: "Water",
    specific_heat_capacity: 2040.0,
    thermal_conductivity: 2.18,
    density: 0.997,
    phase: PhysicsPhase::Solid,
    viscosity: f32::INFINITY,
};
pub const IRON: PhysicsMaterial = PhysicsMaterial {
    name: "Iron",
    specific_heat_capacity: 460.0,
    thermal_conductivity: 50.0,
    density: 7.874,
    phase: PhysicsPhase::Solid,
    viscosity: f32::INFINITY,
};
pub const DIRT: PhysicsMaterial = PhysicsMaterial {
    name: "Dirt",
    specific_heat_capacity: 800.0,
    thermal_conductivity: 0.25,
    density: 1.51,
    phase: PhysicsPhase::Solid,
    viscosity: f32::INFINITY,
};
pub const SAND: PhysicsMaterial = PhysicsMaterial {
    name: "Sand",
    specific_heat_capacity: 830.0,
    thermal_conductivity: 0.2,
    density: 2.1,
    phase: PhysicsPhase::Solid,
    viscosity: f32::INFINITY,
};
pub const WOOD_HARD: PhysicsMaterial = PhysicsMaterial {
    name: "Hardwood",
    specific_heat_capacity: 2000.0,
    thermal_conductivity: 0.16,
    density: 0.65,
    phase: PhysicsPhase::Solid,
    viscosity: f32::INFINITY,
};
pub const WOOD_SOFT: PhysicsMaterial = PhysicsMaterial {
    name: "Softwood",
    specific_heat_capacity: 2300.0,
    thermal_conductivity: 0.12,
    density: 0.49,
    phase: PhysicsPhase::Solid,
    viscosity: f32::INFINITY,
};
pub const INFINITE_HEAT_CAPACITY: PhysicsMaterial = PhysicsMaterial {
    name: "Infinite Heat Sink",
    specific_heat_capacity: f32::INFINITY,
    thermal_conductivity: 100.0,
    density: 10.0,
    phase: PhysicsPhase::Solid,
    viscosity: f32::INFINITY,
};
