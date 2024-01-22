use bevy::prelude::*;

#[derive(Resource)]
pub struct MostRecentSave(pub std::time::SystemTime);

impl MostRecentSave {
    pub fn set(&mut self, time: std::time::SystemTime) {
        self.0 = time;
    }

    pub fn time_since_last_save(&self) -> Option<u64> {
        if self.0 == std::time::SystemTime::UNIX_EPOCH {
            return None;
        }
        return Some(
            std::time::SystemTime::now()
                .duration_since(self.0)
                .ok()?
                .as_secs(),
        );
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
        return Self {
            timer: Timer::new(std::time::Duration::from_secs(1), TimerMode::Repeating),
        };
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
