use bevy::prelude::*;
use bevy_egui::*;
use super::{AppState, utils};
use strum::IntoEnumIterator; // Needed for AppState.iter()

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
        for state in AppState::iter() {
            let s = state.to_string();
            let s = utils::string_utils::pascal_case_to_readable(&s);
            if ui.button(s).clicked() {
                app_state.set(state);
            }
        }
    });
}
