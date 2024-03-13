use crate::physics::voxel_material_lookup::VoxelMaterialLookup;
use crate::physics::*;

fn calculate_heat_transfer_voxel(
    from_mat: &VoxelMaterial,
    from_temp: Temperature,
    to_mat: &VoxelMaterial,
    to_temp: Temperature,
) -> HeatTransferRate {
    if from_mat.mass == 0.0 || to_mat.mass == 0.0 {
        return 0.0;
    }
    let thermal_resistance = from_mat.thermal_resistance + to_mat.thermal_resistance;
    let temp_diff = from_temp - to_temp;
    let heat_transfer_rate = temp_diff / thermal_resistance;
    heat_transfer_rate
}

pub fn calculate_heat_transfer_volume(
    material: &Volume<MaterialId>,
    temperature: &Volume<Temperature>,
    heat: &mut Volume<HeatTransferRate>,
    lookup: &VoxelMaterialLookup,
) {
    let size = &material.size;
    for z in 0 .. size.z {
        for y in 0 .. size.y {
            for x in 0 .. size.x {
                let to_index = heat.index(x, y, z);    // we know this is linear, could increment.
                let to_mat = &lookup.materials[material.data[to_index] as usize];
                let to_temp = temperature.data[to_index];
                let mut heat_transfer_rate = 0.0;
                let mut add_heat = |from_index: usize| {
                    let from_mat = &lookup.materials[material.data[from_index] as usize];
                    let from_temp = temperature.data[from_index];
                    heat_transfer_rate += calculate_heat_transfer_voxel(from_mat, from_temp, to_mat, to_temp);
                };
                //  only add heat for values which are not on the boundaries
                if x > 0 { add_heat(to_index - 1); }
                if x + 1 < size.x { add_heat(to_index + 1); }
                if y > 0 { add_heat(to_index - size.x); }
                if y + 1 < size.y { add_heat(to_index + size.x); }
                if z > 0 { add_heat(to_index - size.x * size.y); }
                if z + 1 < size.z { add_heat(to_index + size.x + size.y); }
                heat.set(x, y, z, heat_transfer_rate);
            }
        }
    }
}

pub fn apply_heat_to_volume(
    material: &Volume<MaterialId>,
    temperature: &mut Volume<Temperature>,
    heat: &Volume<HeatTransferRate>,
    material_lookup: &VoxelMaterialLookup,
    time: Time,
) {
    for i in 0 .. temperature.data.len() {
        let mat = material_lookup.materials[material.data[i] as usize];
        //  heat is power, power * time = energy.
        let heat_energy = heat.data[i] * time;
        //  energy / heat capacity = temperature change in kelvin.
        let temp_change = heat_energy  / mat.heat_capacity;
        temperature.data[i] += temp_change;
    }
}
