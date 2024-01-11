use bevy::prelude::*;

use chrono::prelude::DateTime;
use chrono::Local;

use crate::saving::save_timer;

#[derive(Event)]
pub struct SaveEvent;

pub fn save_event(mut most_recent_save: ResMut<save_timer::MostRecentSave>) {
    most_recent_save.set(std::time::SystemTime::now());
    let datetime = DateTime::<Local>::from(most_recent_save.0);
    // Formats the combined date and time with the specified format string.
    let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
    println!("Save at {:?}", timestamp_str);
}
