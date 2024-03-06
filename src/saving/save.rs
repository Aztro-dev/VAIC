use bevy::prelude::*;

use serde::Serialize;

use std::fs::{File, OpenOptions};
use std::io::Write;

use chrono::prelude::DateTime;
use chrono::Local;

use crate::placing::{self, PartName};
use crate::saving::save_timer;

#[derive(Event)]
pub struct SaveEvent;

#[derive(Serialize, Clone, Debug)]
struct SaveContents {
    pub part_name: String,
    pub transform: (Vec3, Vec4, Vec3),
}

impl From<(&Transform, &PartName)> for SaveContents {
    fn from(value: (&Transform, &PartName)) -> Self {
        let transform = value.0;
        let part_name = value.1;
        Self {
            part_name: part_name.0.clone(),
            transform: (
                transform.translation,
                transform.rotation.to_array().into(),
                transform.scale,
            ),
        }
    }
}

pub fn save_event(
    part_query: Query<(&Transform, &placing::PartName), With<placing::Part>>,
    mut most_recent_save: ResMut<save_timer::MostRecentSave>,
    mut update_save_count_timer: ResMut<crate::saving::UpdateSaveCountTimer>,
) {
    let mut save_contents_arr: Vec<SaveContents> = Vec::new();
    for value in part_query.iter() {
        save_contents_arr.push(SaveContents::from(value));
        println!("{:?} {:?}", value.0, value.1);
    }

    let file_name = "save".to_string();

    if let Ok(mut file) = OpenOptions::new()
        .write(true)
        .open(format!("{file_name}.ron"))
    {
        file.write_all(ron::ser::to_string(&save_contents_arr).unwrap().as_bytes())
            .expect("Couldn't write to file");
    } else {
        let mut file = File::create(format!("{file_name}.ron"))
            .expect(format!("Couldn't create {file_name}.ron file").as_str());
        file.write_all(ron::ser::to_string(&save_contents_arr).unwrap().as_bytes())
            .expect("Couldn't write to file");
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
