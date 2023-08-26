use bevy::prelude::*;
use bevy::window::*;
use bevy_third_person_camera::{ThirdPersonCamera, ThirdPersonCameraPlugin, Zoom};

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
        .add_plugins(ThirdPersonCameraPlugin)
        .add_systems(Startup, (spawn_camera, create_light))
        .add_systems(Update, close_on_esc)
        .run();
}

pub fn create_light(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 120.0,
    });
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle::default(),
        ThirdPersonCamera {
            zoom: Zoom::new(25.0, 30.0),
            ..default()
        },
    ));
}
