use bevy::prelude::*;

use crate::ui;

pub struct SavingPlugin;

impl Plugin for SavingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SaveEvent>()
            .init_resource::<MostRecentSave>()
            .add_systems(
                Update,
                check_for_save_key.run_if(in_state(ui::UIState::Editor)),
            )
            .add_systems(Update, save_event.run_if(in_state(ui::UIState::Editor)));
    }
}

#[derive(Resource)]
struct MostRecentSave(pub std::time::SystemTime);

impl MostRecentSave {
    pub fn set(&mut self, time: std::time::SystemTime) {
        self.0 = time;
    }

    pub fn time_since_last_save(&self) -> Option<std::time::Duration> {
        if self.0 == std::time::SystemTime::UNIX_EPOCH {
            return None;
        }
        return self.0.duration_since(std::time::SystemTime::now()).ok();
    }
}

impl Default for MostRecentSave {
    fn default() -> Self {
        Self(std::time::SystemTime::UNIX_EPOCH)
    }
}

#[derive(Event)]
struct SaveEvent;

fn check_for_save_key(
    mut save_event_writer: EventWriter<SaveEvent>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::S) && keyboard.pressed(KeyCode::ControlLeft) {
        save_event_writer.send(SaveEvent {});
    }
}

fn save_event(
    mut save_event_reader: EventReader<SaveEvent>,
    mut most_recent_save: ResMut<MostRecentSave>,
) {
    for _ in save_event_reader.read() {
        println!("Save!");
        most_recent_save.set(std::time::SystemTime::now());
        println!("{:?}", most_recent_save.0);
    }
}
