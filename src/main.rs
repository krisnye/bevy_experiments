use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_egui::EguiPlugin;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use bevy_xpbd_3d::prelude::*;

mod plugins;
mod main_menu;
mod utils;
pub mod physics;

use plugins::*;

fn main() {
    let mut app = App::new();

    plugins::init(&mut app);

    // app.insert_resource(WindowDescriptor {
    //     vsync: false, // Disable VSync
    //     ..Default::default()
    // })
    app.insert_state::<AppState>(AppState::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::Immediate,   // max rate so we can measure FPS performance.
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(EguiPlugin)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(main_menu::MainMenuPlugin);

    add_systems(&mut app);
    app.run();
}
