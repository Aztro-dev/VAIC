use bevy::prelude::*;
use bevy_round_ui::prelude::*;

use crate::{constraints::ConstrainState, move_objects::MoveObjectsState, placing::PlacingState};

mod pause;
use pause::PausePlugin;

mod settings;
use settings::SettingsPlugin;

pub mod editor;
use editor::EditorPlugin;

pub const UI_RADIUS: f32 = 20.0;

pub struct UIPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum UIState {
    #[default]
    MainMenu, // Main Menu (duh)
    Editor,   // In editor
    Pause,    // Toggling escape
    Settings, // For changing sens, keybindings, etc.
}

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<UIState>()
            .add_systems(
                Update,
                handle_esc.run_if(
                    not(in_state(PlacingState::Placing))
                        .and_then(not(in_state(ConstrainState::Constraining)))
                        .and_then(not(in_state(MoveObjectsState::Moving))),
                ),
            )
            .add_plugins((PausePlugin, SettingsPlugin, EditorPlugin, RoundUiPlugin));
    }
}

fn handle_esc(
    keyboard: Res<Input<KeyCode>>,
    mut ui_state: ResMut<NextState<UIState>>,
    current_state: Res<State<UIState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            UIState::Editor => {
                ui_state.set(UIState::Pause);
            }
            _ => {
                ui_state.set(UIState::Editor);
            }
        }
    }
}
