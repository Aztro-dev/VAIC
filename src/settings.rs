use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use smooth_bevy_cameras::controllers::orbit::ControlState;
use std::fs::File;
use std::io::Read;
use std::io::Write;

#[derive(Resource, Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Settings {
    pub control_state: u8,
    pub rotate_sensitivity: Vec2,
    pub translate_sensitivity: Vec2,
    pub zoom_sensitivity: f32,
}

impl Settings {
    pub fn get_control_state(&self) -> ControlState {
        match self.control_state {
            0 => ControlState::PCMode,
            1 => ControlState::TrackPadMode,
            _ => ControlState::PCMode,
        }
    }

    pub fn set_control_state(&mut self, control_state: ControlState) {
        match control_state {
            ControlState::PCMode => {
                self.control_state = 0;
            }
            ControlState::TrackPadMode => {
                self.control_state = 1;
            }
        }
        self.write_changes();
    }

    fn write_changes(&mut self) {
        if let Ok(mut file) = File::create(".settings.ron") {
            let settings_string = ron::ser::to_string(self).unwrap();
            file.write_all(settings_string.as_bytes())
                .expect("Couldn't write to file in load_or_create_settings_file");
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            control_state: 0,
            rotate_sensitivity: Vec2::splat(0.8),
            translate_sensitivity: Vec2::splat(0.2),
            zoom_sensitivity: 1.0,
        }
    }
}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Settings>()
            .add_systems(PreStartup, load_or_create_settings_file);
    }
}

fn load_or_create_settings_file(mut settings: ResMut<Settings>) {
    if let Ok(mut file) = File::open(".settings.ron") {
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let bruh: ron::error::SpannedResult<Settings> = ron::from_str(contents.as_str());
        *settings = bruh.expect("Couldn't convert to Settings in load_or_create_settings_file");
    } else {
        let mut file = File::create(".settings.ron")
            .expect("Couldn't create .settings.ron file in load_or_create_settings_file");
        let settings_string = ron::ser::to_string(&(*settings)).unwrap();
        file.write_all(settings_string.as_bytes())
            .expect("Couldn't write to file in load_or_create_settings_file");
    }
}
