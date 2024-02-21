use bevy::prelude::*;
use super::AppState;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, system); // .run_if(state_exists_and_equals(STATE)))
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
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
    });
}

fn cleanup() {
    println!("Main Menu cleanup");
}