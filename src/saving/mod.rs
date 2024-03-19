use bevy::prelude::*;

use crate::{actions::ActionList, ui};

pub mod save_timer;
use save_timer::*;

mod save;
use save::*;

pub struct SavingPlugin;

impl Plugin for SavingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SaveEvent>()
            .init_resource::<MostRecentSave>()
            .init_resource::<UpdateSaveCountTimer>()
            .add_systems(
                Update,
                (
                    save_event.run_if(on_event::<SaveEvent>()),
                    time_since_last_save,
                    check_for_save_key,
                    show_icon_on_change,
                )
                    .run_if(in_state(ui::UIState::Editor)),
            );
    }
}

fn check_for_save_key(
    mut save_event_writer: EventWriter<SaveEvent>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut icon_query: Query<&mut Visibility, With<SaveIcon>>,
) {
    if keyboard.just_pressed(KeyCode::KeyS) && keyboard.pressed(KeyCode::ControlLeft) {
        save_event_writer.send(SaveEvent);
        let mut icon_visibility = icon_query.get_single_mut().unwrap();
        *icon_visibility = Visibility::Hidden;
    }
}

#[derive(Component)]
pub struct SaveIcon;

fn show_icon_on_change(
    action_list: Res<ActionList>,
    mut icon_query: Query<&mut Visibility, With<SaveIcon>>,
) {
    if action_list.is_changed() {
        let mut icon_visibility = icon_query.get_single_mut().unwrap();
        *icon_visibility = Visibility::Visible;
    }
}
