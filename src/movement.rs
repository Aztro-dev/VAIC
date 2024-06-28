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
            // .add_systems(Update, (switch_control_state, switch_projection))
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
            // OrbitCameraBundle::new(
            //     OrbitCameraController {
            //         mouse_rotate_sensitivity: settings.rotate_sensitivity,
            //         mouse_translate_sensitivity: settings.translate_sensitivity,
            //         mouse_wheel_zoom_sensitivity: settings.zoom_sensitivity,
            //         control_state: settings.get_control_state(),
            //         ..default()
            //     },
            //     Vec3::new(-7.0, 7.5, 10.0),
            //     Vec3::new(0.0, 0.0, 0.0),
            //     Vec3::Y,
            EditorCam {
                sensitivity: Sensitivity {
                    orbit: settings.rotate_sensitivity,
                    zoom: settings.zoom_sensitivity,
                },
                ..default()
            }, // Step 2: add camera controller component to any cameras
            EnvironmentMapLight {
                // Unrelated to camera controller, needed for lighting:
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
}*/
