use bevy::prelude::*;

mod part_picker;
use part_picker::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_2d_cam, spawn_part_picker))
            .add_systems(Update, mouse_scroll);
    }
}

fn spawn_2d_cam(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera: Camera {
            order: -1,
            ..default()
        },
        ..default()
    });
}
