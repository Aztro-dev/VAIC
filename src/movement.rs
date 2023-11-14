use bevy::prelude::*;
use smooth_bevy_cameras::controllers::orbit::OrbitCameraBundle;
use smooth_bevy_cameras::controllers::orbit::OrbitCameraController;
use smooth_bevy_cameras::controllers::orbit::OrbitCameraPlugin;
use smooth_bevy_cameras::LookTransformPlugin;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((LookTransformPlugin, OrbitCameraPlugin::default()))
            .add_systems(Update, switch_control_state)
            .add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle::default()).insert((
        bevy_transform_gizmo::GizmoPickSource::default(),
        OrbitCameraBundle::new(
            OrbitCameraController {
                mouse_rotate_sensitivity: Vec2::splat(0.8),
                mouse_translate_sensitivity: Vec2::splat(0.2),
                mouse_wheel_zoom_sensitivity: 1.0,
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
) {
    if !keyboard.just_pressed(KeyCode::P) {
        return;
    }
    if let Ok(mut controller) = query.get_single_mut() {
        controller.toggle_control_state();
    }
}
