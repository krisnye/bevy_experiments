use crate::physics::*;
use crate::physics::voxel_material_lookup::VoxelMaterialLookup;

pub fn fill_volume_with_test_material(volume: &mut Volume<MaterialId>, lookup: &VoxelMaterialLookup) {
    let iron = lookup.id("Iron");
    let wood = lookup.id("Hardwood");
    for z in 0 .. volume.size.z {
        let z_edge = (z == 0 || z + 1 == volume.size.z) as i32;
        for y in 0 .. volume.size.y {
            let y_edge = (y == 0 || y + 1 == volume.size.y) as i32;
            for x in 0 .. volume.size.x {
                let x_edge = (x == 0 || x + 1 == volume.size.x) as i32;
                let edges = x_edge + y_edge + z_edge;
                let material_id = if edges >= 2 { iron } else { wood };
                volume.set(x, y, z, material_id);
            }
        }
    }
}

pub fn fill_with_heat_source_and_sink(material: &mut Volume<MaterialId>, temperature: &mut Volume<Temperature>, lookup: &VoxelMaterialLookup) {
    let infinite_heat_capacity = lookup.id("Infinite Heat Sink");
    let hot: usize = material.data.len() - 1;
    let cold: usize = 0;
    material.data[hot] = infinite_heat_capacity;
    material.data[cold] = infinite_heat_capacity;
    temperature.data.fill(kelvin::ROOM_TEMPERATURE);
    temperature.data[hot] = kelvin::TUNGSTEN_MELTING;
    temperature.data[cold] = kelvin::ABSOLUTE_ZERO;
}