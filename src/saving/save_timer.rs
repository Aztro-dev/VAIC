use bevy::prelude::*;

#[derive(Resource)]
pub struct MostRecentSave(pub std::time::SystemTime);

impl MostRecentSave {
    pub fn set(&mut self, time: std::time::SystemTime) {
        self.0 = time;
    }

    pub(crate) fn time_since_last_save(&self) -> Option<u64> {
        if self.0 == std::time::SystemTime::UNIX_EPOCH {
            return None;
        }
        Some(
            std::time::SystemTime::now()
                .duration_since(self.0)
                .ok()?
                .as_secs(),
        )
    }
}

impl ToString for MostRecentSave {
    fn to_string(&self) -> String {
        let most_recent_save = self.time_since_last_save();
        let mut most_recent_save_str = "Not saved yet!".to_string();
        if most_recent_save.is_some() {
            let most_recent_save = most_recent_save.unwrap();
            if most_recent_save == 1 {
                most_recent_save_str = "Saved 1 sec ago".to_string();
            } else if most_recent_save < 60 {
                most_recent_save_str = format!("Saved {} secs ago", most_recent_save);
            } else if most_recent_save >= 60 && most_recent_save < 120 {
                most_recent_save_str = "Saved 1 min ago".to_string();
            } else {
                most_recent_save_str = format!("Saved {} mins ago", most_recent_save / 60);
            }
        }
        most_recent_save_str
    }
}

impl Default for MostRecentSave {
    fn default() -> Self {
        Self(std::time::SystemTime::UNIX_EPOCH)
    }
}

#[derive(Resource)]
pub struct UpdateSaveCountTimer {
    /// Repeating timer
    pub timer: Timer,
}

impl Default for UpdateSaveCountTimer {
    fn default() -> Self {
        Self {
            timer: Timer::new(std::time::Duration::from_secs(1), TimerMode::Repeating),
        }
    }
}

pub fn time_since_last_save(
    mut update_save_count_timer: ResMut<UpdateSaveCountTimer>,
    most_recent_save: Res<MostRecentSave>,
    time: Res<Time>,
) {
    update_save_count_timer.timer.tick(time.delta());

    if update_save_count_timer.timer.finished() {
        let time = most_recent_save.time_since_last_save();

        if time.is_some() {
            if time.unwrap() >= 60 {
                update_save_count_timer
                    .timer
                    .set_duration(std::time::Duration::from_secs(60));
            }
        }
    }
}
