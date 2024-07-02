use bevy::input::mouse::MouseMotion;
use bevy::pbr::ClusterConfig;
use bevy::prelude::*;
use bevy_editor_cam::prelude::*;
use bevy_infinite_grid::GridShadowCamera;

use crate::settings::Settings;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultEditorCamPlugins)
            .add_plugins(bevy_mod_picking::DefaultPickingPlugins)
            .add_systems(Update, (switch_projection, update_camera))
            .add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands, settings: Res<Settings>, asset_server: Res<AssetServer>) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-7.0, 7.5, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert((
            GridShadowCamera,
            EditorCam {
                sensitivity: Sensitivity {
                    orbit: settings.rotate_sensitivity,
                    zoom: settings.zoom_sensitivity,
                },
                enabled_motion: EnabledMotion {
                    // pan: false,
                    ..default()
                },
                ..default()
            },
            // Doesn't work on my machine for some reason
            EnvironmentMapLight {
                intensity: 1000.0,
                diffuse_map: asset_server.load("environment_maps/diffuse_rgb9e5_zstd.ktx2"),
                specular_map: asset_server.load("environment_maps/specular_rgb9e5_zstd.ktx2"),
            }, // ),
            ClusterConfig::Single,
        ));
}

/*
fn switch_control_state(
    mut query: Query<&mut OrbitCameraController>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<Settings>,
) {
    if !keyboard.just_pressed(KeyCode::KeyP) {
        return;
    }
    if let Ok(mut controller) = query.get_single_mut() {
        controller.toggle_control_state();
        settings.set_control_state(controller.control_state);
    }
}
*/

fn update_camera(
    mut editor_camera: Query<&mut EditorCam>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut mouse_events: EventReader<MouseMotion>,
    settings: Res<Settings>,
) {
    let mut editor_camera = editor_camera.get_single_mut().unwrap();
    let view_space_anchor = editor_camera.anchor_view_space();
    if keyboard.pressed(KeyCode::ShiftLeft) {
        // editor_camera.enabled_motion.pan = true;
        editor_camera.start_pan(view_space_anchor);
        for event in mouse_events.read() {
            let mut movement = event.delta * 100.0;
            movement.x *= settings.translate_sensitivity.x;
            movement.y *= settings.translate_sensitivity.y;

            // editor_camera.send_screenspace_input(movement);
        }
        editor_camera.end_move();
        // editor_camera.enabled_motion.pan = false;
    }
}

fn switch_projection(
    mut camera_query: Query<&mut Projection, With<Camera3d>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if !keyboard.just_pressed(KeyCode::KeyO) {
        return;
    }
    for mut projection in camera_query.iter_mut() {
        *projection = match *projection {
            Projection::Perspective(_) => Projection::Orthographic(OrthographicProjection {
                scale: 5.5,
                scaling_mode: bevy::render::camera::ScalingMode::FixedVertical(2.0),
                ..default()
            }),
            Projection::Orthographic(_) => Projection::default(),
        }
    }
}
