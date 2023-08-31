use bevy::prelude::*;
use belly::prelude::*;

mod part_picker;
use part_picker::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BellyPlugin).add_systems(Startup, (spawn_2d_cam, spawn_part_picker));
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
