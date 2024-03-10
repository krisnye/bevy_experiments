use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_xpbd_3d::prelude::*;

mod plugins;
mod main_menu;
mod utils;
pub mod physics;
pub mod data;

use plugins::*;

fn main() {
    let mut app = App::new();
    app.insert_state::<AppState>(AppState::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(main_menu::MainMenuPlugin);

    add_systems(&mut app);
    app.run();
}
