use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Volume<T: Copy + Display> {
    pub size: (usize, usize, usize), // Dimensions as a tuple
    pub data: Vec<T>,
}

impl<T: Copy + Display> Volume<T> {
    pub fn new(size: (usize, usize, usize), initial_value: T) -> Self {
        let (width, height, depth) = size;
        let num_voxels = width * height * depth;
        let data = vec![initial_value; num_voxels];

        Volume { size, data }
    }

    // Helper method to calculate the linear index
    pub fn index(&self, x: usize, y: usize, z: usize) -> usize {
        z * self.size.0 * self.size.1 + y * self.size.0 + x
    }

    // Get a copy of the value at specified coordinates
    pub fn get(&self, x: usize, y: usize, z: usize) -> T {
        let index = self.index(x, y, z);
        self.data[index]
    }

    // Set the value at specified coordinates
    pub fn set(&mut self, x: usize, y: usize, z: usize, value: T) {
        let index = self.index(x, y, z);
        self.data[index] = value;
    }

    // Function to visualize the volume with padding/truncation
    pub fn print_volume(&self, length: usize) {
        let (width, height, depth) = self.size;
        for z in 0..depth {
            for y in 0..height {
                for x in 0..width {
                    let idx = self.index(x, y, z);
                    let value = format!("{:1$.1$}", self.data[idx].to_string(), length);
                    print!("{} ", value);
                }
                println!(); // End of a row
            }
            println!(); // Separate layers
        }
    }
}
