use bevy::prelude::*;

mod flycam;
mod third_person_camera;

use third_person_camera::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((flycam::NoCameraPlayerPlugin, ThirdPersonCameraPlugin))
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, toggle_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(5.0, 10.0, 5.0).looking_at(Vec3::ZERO, Vec3::ZERO),
            ..default()
        },
        ThirdPersonCamera {
            zoom: Zoom::new(0.01, 999.0),
            ..default()
        },
        flycam::FlyCam,
    ));
}

fn toggle_camera(keys: Res<Input<KeyCode>>, mut cameras_query: Query<&mut ThirdPersonCamera>) {
    if keys.just_pressed(KeyCode::O) {
        let mut third_person_camera = cameras_query.get_single_mut().unwrap();
        third_person_camera.enabled = !third_person_camera.enabled;
    }
}
