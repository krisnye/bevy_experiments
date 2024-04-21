use std::collections::HashMap;

pub const SIZE: usize = 16;

pub struct SparseVolumeChunk {
    pub data: [u8; SIZE * SIZE * SIZE],
}

pub struct SparseVolume {
    pub chunks: HashMap<(u32, u32, u32), SparseVolumeChunk>,
}
