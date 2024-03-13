
#[derive(Debug, Clone, Copy)]
pub enum PhysicsPhase {
    Solid,
    Grain,
    Liquid,
    Gas,
}
#[derive(Debug, Clone, Copy)]
pub struct PhysicsMaterial {
    pub name: &'static str,
    pub phase: PhysicsPhase,
    pub specific_heat_capacity: SpecificHeatCapacity,
    pub thermal_conductivity: ThermalConductivity,
    pub density: Density,
    pub viscosity: Viscosity,
}

#[derive(Debug, Clone, Copy)]
pub struct VoxelMaterial {
    pub phase: PhysicsPhase,
    pub mass: Mass,
    pub thermal_resistance: ThermalResistance,
    pub heat_capacity: HeatCapacity,
}

impl PhysicsMaterial {
    pub fn to_voxel_material(&self, length: Length) -> VoxelMaterial {
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

//  Meter
pub type Length = f32;

//  Kelvin
pub type Temperature = f32;

//  g/cm3
pub type Density = f32;

//  Pascals, kg/m sec2
pub type Pressure = f32;

//  Kg
pub type Mass = f32;

//  Centipoise, g/(cm2*s)
pub type Viscosity = f32;

//  Watts / Meter Kelvin
pub type ThermalConductivity = f32;

//  Kelvin / Watt
pub type ThermalResistance = f32;

//  Joules / Kg Kelvin
pub type SpecificHeatCapacity = f32;

//  Joules / Kelvin
pub type HeatCapacity = f32;

//  Joules / Second
pub type Power = f32;

//  Joules / Second
pub type HeatTransferRate = Power;

//  index for looking up material properties.
pub type MaterialId = u32;

//  Seconds
pub type Time = f32;
