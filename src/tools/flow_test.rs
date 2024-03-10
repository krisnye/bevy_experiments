use bevy_experiments::physics;
use physics::volume::*;

//  what are we going to simulate, something cool I hope.
//  calculation of what fails/breaks where?

//  -----------------------------------------
//  |   |   |   |   |   |   |   |   |   |   |
//  -----------------------------------------
//  |   |   |   |   |   |   |   |   |   |   |
//  -----------------------------------------
//  |   |   |   |   |   |   |   |   |   |   |
//  -----------------------------------------
//  |   |   |   |   |   |   |   |   |   |   |
//  -----------------------------------------
//  |   |   | X | X | X | X | X |   |   |   |
//  -----------------------------------------
//  |   | X |   | X |   |   |   |   |   |   |
//  -----------------------------------------
//  | X |   |   | X |   |   |   |   |   |   |
//  -----------------------------------------

fn main() {
    let size = (4, 3, 2); // Dimensions for the volume
    let initial_value = 0.0_f32; // Initial value for all cells
    let mut volume = Volume::new(size, initial_value);

    // Populate the volume with some sample data
    for z in 0..size.2 {
        for y in 0..size.1 {
            for x in 0..size.0 {
                let value = ((x + y + z) as f32) * 1.234; // Just a sample calculation
                let idx = volume.index(x, y, z);
                volume.data[idx] = value;
            }
        }
    }

    // Visualize the volume with a specified display length for each element
    volume.print_volume(6);
}
