use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod plugins;
pub mod main_menu;

use plugins::*;

fn main() {
    let mut app = App::new();
    app.insert_state::<AppState>(AppState::Triangle)
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(main_menu::MainMenuPlugin);

    add_systems(&mut app);
    app.run();
}
