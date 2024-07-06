use bevy::{
    app::{App, Plugin, PreUpdate},
    ecs::system::{Query, ResMut, Resource},
    math::Vec2,
    window::Window,
};

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CursorDelta::default())
            .add_systems(PreUpdate, track_cursor);
    }
}

#[derive(Resource, Default, Debug, Clone, Copy)]
pub struct CursorDelta {
    pub previous_position: Vec2,
    pub current_delta: Vec2,
}

impl CursorDelta {
    pub fn current_delta_as_percentage(&self, window: Window) -> Vec2 {
        return Vec2::new(
            100.0 * self.current_delta.x / window.width(),
            100.0 * self.current_delta.y / window.height(),
        );
    }
}

fn track_cursor(mut cursor: ResMut<CursorDelta>, window_query: Query<&Window>) {
    let window = window_query.get_single().unwrap();
    let cursor_pos = window.cursor_position().unwrap_or(cursor.previous_position);

    cursor.current_delta = cursor.previous_position - cursor_pos;
    cursor.previous_position = cursor_pos;
}
