use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl Size {
    pub fn product(&self) -> usize {
        return self.x * self.y * self.z;
    }
}

#[derive(Debug, Clone)]
pub struct Volume<T: Copy + Display> {
    pub size: Size, // Dimensions as a tuple
    pub data: Vec<T>,
}

impl<T: Copy + Display> Volume<T> {
    pub fn new(size: Size, initial_value: T) -> Self {
        let data = vec![initial_value; size.product()];
        Volume { size, data }
    }

    // Helper method to calculate the linear index
    pub fn index(&self, x: usize, y: usize, z: usize) -> usize {
        (z * self.size.y + y) * self.size.x + x
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
    pub fn print(&self, length: usize) {
        let mut indent = " ".to_string();
        for z in 0..self.size.z {
            for y in (0..self.size.y).rev() {
                print!("{}", indent);
                for x in 0..self.size.x {
                    let idx = self.index(x, y, z);
                    let value = format!("{:>1$.1$}", self.data[idx].to_string(), length);
                    print!("{} ", value);
                }
                println!(); // End of a row
            }
            println!(); // Separate layers
            indent.push_str("  ");
        }
    }
}
