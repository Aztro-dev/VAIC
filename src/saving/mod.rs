use bevy::prelude::*;

use crate::ui;

mod save_timer;
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
                )
                    .run_if(in_state(ui::UIState::Editor)),
            );
    }
}

fn check_for_save_key(
    mut save_event_writer: EventWriter<SaveEvent>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::S) && keyboard.pressed(KeyCode::ControlLeft) {
        save_event_writer.send(SaveEvent {});
    }
}
