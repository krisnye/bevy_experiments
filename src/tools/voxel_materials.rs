use bevy_experiments::physics::materials;
use bevy_experiments::physics::voxel_material_lookup::VoxelMaterialLookup;

pub fn create_test_materials() -> VoxelMaterialLookup {
    let mut lookup = VoxelMaterialLookup::new(4.0);
    lookup.add(materials::AIR);
    lookup.add(materials::WOOD_HARD);
    lookup.add(materials::WOOD_SOFT);
    lookup.add(materials::IRON);
    lookup.add(materials::ICE);
    lookup.add(materials::DIRT);
    lookup.add(materials::ROCK);
    lookup.add(materials::SAND);
    lookup.add(materials::WATER);
    lookup.add(materials::INFINITE_HEAT_CAPACITY);
    lookup
}