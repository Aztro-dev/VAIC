use bevy::prelude::*;
use bevy::window::*;
use bevy_infinite_grid::*;

mod movement;
use movement::MovementPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("333333").unwrap()))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "VAIC".into(),
                fit_canvas_to_parent: true,
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((InfiniteGridPlugin, MovementPlugin))
        .add_systems(Startup, (create_light, spawn_grid))
        .add_systems(Update, close_on_esc)
        .run();
}

pub fn create_light(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0,
    });
}

fn spawn_grid(mut commands: Commands) {
    commands.spawn(InfiniteGridBundle {
        grid: InfiniteGrid {
            // shadow_color: None,
            ..default()
        },
        ..default()
    });
}
