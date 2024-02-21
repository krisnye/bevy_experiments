use bevy::prelude::*;
use bevy_egui::*;
use super::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, system); // .run_if(state_exists_and_equals(STATE)))
    }
}

fn setup() {
    println!("Main Menu Setup");
}

fn system(mut contexts: EguiContexts, mut app_state: ResMut<NextState<AppState>>) {
    let context = contexts.ctx_mut();

    egui::Window::new("Start Menu").show(context, |ui| {
        if ui.button("Triangle").clicked() {
            // Changes app state to "Game"
            app_state.set(AppState::Triangle);
        }
        if ui.button("Shapes").clicked() {
            // Changes app state to "Game"
            app_state.set(AppState::Shapes);
        }
        if ui.button("Physics Blocks").clicked() {
            // Changes app state to "Game"
            app_state.set(AppState::PhysicsBlocks);
        }

    });
}
