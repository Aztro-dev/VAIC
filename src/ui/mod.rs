use bevy::prelude::*;

pub struct UIPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum UIState {
    #[default]
    MainMenu, // Main Menu (duh)
    None,     // In editor
    General,  // Toggling escape
    Settings, // For changing sens, keybindings, etc.
}

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<UIState>().add_systems(Update, handle_esc);
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
                println!("Open General Menu!");
                ui_state.set(UIState::General);
            }
            UIState::General => {
                println!("Close General Menu!");
                ui_state.set(UIState::None);
            }
            _ => {
                println!("Get Out Of Menu!");
                ui_state.set(UIState::None);
            }
        }
    }
}
