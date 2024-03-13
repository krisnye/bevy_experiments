use bevy::utils::HashMap;
use crate::physics::*;

pub struct VoxelMaterialLookup {
    pub length: Length,
    pub name_to_id: HashMap<&'static str, MaterialId>,
    pub materials: Vec<VoxelMaterial>,
}

impl VoxelMaterialLookup {
    pub fn new(length: Length) -> VoxelMaterialLookup {
        VoxelMaterialLookup { length, name_to_id: HashMap::new(), materials: Vec::new() }
    }
    pub fn id(&self, name: &'static str) -> MaterialId {
        *self.name_to_id.get(name).unwrap()
    }
    pub fn add(&mut self, mat: PhysicsMaterial) {
        let id = self.materials.len();
        self.name_to_id.insert(mat.name, id as MaterialId);
        self.materials.push(mat.to_voxel_material(self.length));
    }
}