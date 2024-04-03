use std::collections::HashMap;

const SIZE: usize = 16;

struct SparseVolumeChunk {
    pub data: [u8; SIZE * SIZE * SIZE],
}

struct SparseVolume {
    pub chunks: HashMap<(u32, u32, u32), SparseVolumeChunk>,
}
