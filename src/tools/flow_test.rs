mod voxel_materials;

use bevy_experiments::physics::*;
use bevy_experiments::physics::heat_transfer::{apply_heat_to_volume, calculate_heat_transfer_volume};
use bevy_experiments::physics::test::{fill_volume_with_test_material, fill_with_heat_source_and_sink};
use crate::voxel_materials::create_test_materials;

// To automatically run and rerun this on changes:
//  nodemon -w src -e rs -x "cargo run --bin flow_test"

//  what are we going to simulate, something cool I hope.
//  calculation of what fails/breaks where?

//  -----------------------------------------
//  |   |   |   |   |   |   |   |   |   |   |
//  -----------------------------------------
//  |   |   |   |   |   |   |   |   |   |   |
//  -----------------------------------------
//  |   |   |   |   |   |   |   |   |   |   |
//  -----------------------------------------
//  |   |   |   |   |   |   |   |   |   | X |
//  -----------------------------------------
//  |   |   | X | X | X | X | X |   |   | X |
//  -----------------------------------------
//  |   | X |   | X |   |   |   |   |   | X |
//  -----------------------------------------
//  | X |   |   | X |   |   |   |   |   | X |
//  -----------------------------------------

fn main() {
    let size = Size { x: 5, y: 4, z: 3 };

    let lookup = create_test_materials();
    let mut material: Volume<MaterialId> = Volume::new(size, 0);
    let mut temperature: Volume<Temperature> = Volume::new(size, kelvin::ROOM_TEMPERATURE);
    fill_volume_with_test_material(&mut material, &lookup);
    fill_with_heat_source_and_sink(&mut material, &mut temperature, &lookup);

    let mut heat: Volume<HeatTransferRate> = Volume::new(size, 0.0);
    let time_delta = 100.0;

    for _i in 0 .. 10000 {
        calculate_heat_transfer_volume(&material, &temperature, &mut heat, &lookup);
        apply_heat_to_volume(&material, &mut temperature, &heat, &lookup, time_delta);
    }

    println!("material\n");
    material.print(10);
    println!("temperature\n");
    temperature.print(10);
    println!("heat\n");
    heat.print(10);

}
