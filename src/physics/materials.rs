use crate::physics::types::*;

#[derive(Debug, Clone, Copy)]
pub enum PhysicsPhase {
    Solid,
    Grain,
    Liquid,
    Gas,
}
#[derive(Debug, Clone, Copy)]
pub struct PhysicsMaterial {
    phase: PhysicsPhase,
    specific_heat_capacity: SpecificHeatCapacity,
    thermal_conductivity: ThermalConductivity,
    density: Density,
    viscosity: Viscosity,
}
pub const AIR: PhysicsMaterial = PhysicsMaterial {
    phase: PhysicsPhase::Gas,
    specific_heat_capacity: 1006.0,
    thermal_conductivity: 0.024,
    density:  0.0012,
    viscosity: 0.0181,
};
pub const WATER: PhysicsMaterial = PhysicsMaterial {
    phase: PhysicsPhase::Liquid,
    specific_heat_capacity: 4200.0,
    thermal_conductivity: 0.66,
    density: 0.997,
    viscosity: 1.0,
};

pub const ROCK: PhysicsMaterial = PhysicsMaterial {
    specific_heat_capacity: 800.0,
    thermal_conductivity: 4.0,
    density: 2.65,
    phase: PhysicsPhase::Solid,
    viscosity: f32::INFINITY,
};

pub const ICE: PhysicsMaterial = PhysicsMaterial {
    specific_heat_capacity: 2040.0,
    thermal_conductivity: 2.18,
    density: 0.997,
    phase: PhysicsPhase::Solid,
    viscosity: f32::INFINITY,
};

pub const IRON: PhysicsMaterial = PhysicsMaterial {
    specific_heat_capacity: 460.0,
    thermal_conductivity: 50.0,
    density: 7.874,
    phase: PhysicsPhase::Solid,
    viscosity: f32::INFINITY,
};

pub const DIRT: PhysicsMaterial = PhysicsMaterial {
    specific_heat_capacity: 800.0,
    thermal_conductivity: 0.25,
    density: 1.51,
    phase: PhysicsPhase::Solid,
    viscosity: f32::INFINITY,
};

pub const SAND: PhysicsMaterial = PhysicsMaterial {
    specific_heat_capacity: 830.0,
    thermal_conductivity: 0.2,
    density: 2.1,
    phase: PhysicsPhase::Solid,
    viscosity: f32::INFINITY,
};

pub const WOOD_HARD: PhysicsMaterial = PhysicsMaterial {
    specific_heat_capacity: 2000.0,
    thermal_conductivity: 0.16,
    density: 0.65,
    phase: PhysicsPhase::Solid,
    viscosity: f32::INFINITY,
};

pub const WOOD_SOFT: PhysicsMaterial = PhysicsMaterial {
    specific_heat_capacity: 2300.0,
    thermal_conductivity: 0.12,
    density: 0.49,
    phase: PhysicsPhase::Solid,
    viscosity: f32::INFINITY,
};

pub const INFINITE_HEAT_CAPACITY: PhysicsMaterial = PhysicsMaterial {
    specific_heat_capacity: f32::INFINITY,
    thermal_conductivity: 100.0,
    density: 10.0,
    phase: PhysicsPhase::Solid,
    viscosity: f32::INFINITY,
};

#[derive(Debug, Clone, Copy)]
pub struct VoxelMaterial {
    phase: PhysicsPhase,
    mass: Mass,
    thermal_resistance: ThermalResistance,
    heat_capacity: HeatCapacity,
}

impl PhysicsMaterial {
    fn to_voxel_material(&self, length: Length) -> VoxelMaterial {
        let volume = length * length * length;
        let mass = self.density * volume;
        VoxelMaterial {
            phase: self.phase,
            mass,
            thermal_resistance: 1.0 / (2.0 * self.thermal_conductivity * length),
            heat_capacity: mass * self.specific_heat_capacity,
        }
    }
}