use bevy::prelude::*;

mod pause;
use pause::PausePlugin;

mod settings;
use settings::SettingsPlugin;

pub struct UIPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum UIState {
    #[default]
    MainMenu, // Main Menu (duh)
    None,     // In editor
    Pause,    // Toggling escape
    Settings, // For changing sens, keybindings, etc.
}

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<UIState>()
            .add_systems(Update, handle_esc)
            .add_plugins((PausePlugin, SettingsPlugin));
    }
}

fn handle_esc(
    keyboard: Res<Input<KeyCode>>,
    mut ui_state: ResMut<NextState<UIState>>,
    current_state: Res<State<UIState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            UIState::None => {
                ui_state.set(UIState::Pause);
            }
            UIState::Pause => {
                ui_state.set(UIState::None);
            }
            _ => {
                ui_state.set(UIState::None);
            }
        }
    }
}
