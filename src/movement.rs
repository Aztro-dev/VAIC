use bevy::prelude::*;
use bevy_infinite_grid::GridShadowCamera;
use bevy_transform_gizmo::GizmoPickSource;
use smooth_bevy_cameras::controllers::orbit::OrbitCameraBundle;
use smooth_bevy_cameras::controllers::orbit::OrbitCameraController;
use smooth_bevy_cameras::controllers::orbit::OrbitCameraPlugin;
use smooth_bevy_cameras::LookTransformPlugin;

use crate::settings::Settings;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((LookTransformPlugin, OrbitCameraPlugin::default()))
            .add_systems(Update, switch_control_state)
            .add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands, settings: Res<Settings>) {
    commands.spawn(Camera3dBundle::default()).insert((
        GizmoPickSource::default(),
        GridShadowCamera,
        OrbitCameraBundle::new(
            OrbitCameraController {
                mouse_rotate_sensitivity: settings.rotate_sensitivity,
                mouse_translate_sensitivity: settings.translate_sensitivity,
                mouse_wheel_zoom_sensitivity: settings.zoom_sensitivity,
                control_state: settings.get_control_state(),
                ..default()
            },
            Vec3::new(-7.0, 7.5, 10.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::Y,
        ),
    ));
}

fn switch_control_state(
    mut query: Query<&mut OrbitCameraController>,
    keyboard: Res<Input<KeyCode>>,
    mut settings: ResMut<Settings>,
) {
    if !keyboard.just_pressed(KeyCode::P) {
        return;
    }
    if let Ok(mut controller) = query.get_single_mut() {
        controller.toggle_control_state();
        settings.set_control_state(controller.control_state);
    }
}
