use bevy::prelude::*;

use chrono::prelude::DateTime;
use chrono::Local;

use crate::placing;
use crate::saving::save_timer;

#[derive(Event)]
pub struct SaveEvent;

pub fn save_event(
    part_query: Query<(&Transform, &placing::PartName), With<placing::Part>>,
    mut most_recent_save: ResMut<save_timer::MostRecentSave>,
    mut update_save_count_timer: ResMut<crate::saving::UpdateSaveCountTimer>,
) {
    for (transform, part_name) in part_query.iter() {
        println!("{:?} {:?}", part_name, transform);
    }
    most_recent_save.set(std::time::SystemTime::now());
    update_save_count_timer
        .timer
        .set_duration(std::time::Duration::from_secs(1));
    let datetime = DateTime::<Local>::from(most_recent_save.0);
    // Formats the combined date and time with the specified format string.
    let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
    println!("Save at {:?}", timestamp_str);
}
