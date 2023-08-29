mod mouse;

use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use mouse::{orbit_mouse, MousePlugin};

pub struct ThirdPersonCameraPlugin;

impl Plugin for ThirdPersonCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MousePlugin).add_systems(
            Update,
            (
                aim.run_if(aim_condition).run_if(enabled),
                sync_player_camera.after(orbit_mouse).run_if(enabled),
                toggle_x_offset
                    .run_if(toggle_x_offset_condition)
                    .run_if(enabled),
                toggle_cursor.run_if(toggle_cursor_condition),
            ),
        );
    }
}

#[derive(Component)]
pub struct ThirdPersonCamera {
    pub enabled: bool,
    pub enabled_key: KeyCode,
    pub aim_enabled: bool,
    pub aim_button: MouseButton,
    pub aim_speed: f32,
    pub aim_zoom: f32,
    pub cursor_lock_toggle_enabled: bool,
    pub cursor_lock_active: bool,
    pub cursor_lock_key: KeyCode,
    pub focus: Vec3,
    pub mouse_sensitivity: f32,
    pub offset_enabled: bool,
    pub offset: Offset,
    pub offset_toggle_enabled: bool,
    pub offset_toggle_key: KeyCode,
    pub offset_toggle_speed: f32,
    pub zoom_enabled: bool,
    pub zoom: Zoom,
    pub zoom_sensitivity: f32,
}

impl Default for ThirdPersonCamera {
    fn default() -> Self {
        ThirdPersonCamera {
            enabled: false,
            enabled_key: KeyCode::O,
            aim_enabled: false,
            aim_button: MouseButton::Right,
            aim_speed: 3.0,
            aim_zoom: 0.7,
            cursor_lock_key: KeyCode::P,
            cursor_lock_toggle_enabled: false,
            focus: Vec3::ZERO,
            cursor_lock_active: true,
            mouse_sensitivity: 1.0,
            offset_enabled: false,
            offset: Offset::new(0.5, 0.4),
            offset_toggle_enabled: false,
            offset_toggle_speed: 5.0,
            offset_toggle_key: KeyCode::E,
            zoom_enabled: true,
            zoom: Zoom::new(1.5, 3.0),
            zoom_sensitivity: 1.0,
        }
    }
}

// Sets the zoom bounds (min & max)
pub struct Zoom {
    pub min: f32,
    pub max: f32,
    radius: f32,
    radius_copy: Option<f32>,
}

impl Zoom {
    pub fn new(min: f32, max: f32) -> Self {
        Self {
            min,
            max,
            radius: 32.0,
            radius_copy: None,
        }
    }
}

// Offset the camera behind the player. For example, an offset value of (0.5, 0.25) will
// place the camera closer the player's right shoulder
pub struct Offset {
    pub offset: (f32, f32),
    offset_copy: (f32, f32),
    is_transitioning: bool,
}

impl Offset {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            offset: (x, y),
            offset_copy: (x, y),
            is_transitioning: false,
        }
    }
}

// The desired target for the third person camera to look at
#[derive(Component)]
pub struct ThirdPersonCameraTarget;

fn sync_player_camera(
    player_q: Query<&Transform, With<ThirdPersonCameraTarget>>,
    mut cam_q: Query<(&mut ThirdPersonCamera, &mut Transform), Without<ThirdPersonCameraTarget>>,
) {
    let Ok(player) = player_q.get_single() else {
        return;
    };
    let Ok((cam, mut cam_transform)) = cam_q.get_single_mut() else {
        return;
    };

    // Calculate the desired camera translation based on focus, radius, and xy_offset
    let rotation_matrix = Mat3::from_quat(cam_transform.rotation);

    // apply the offset if offset_enabled is true
    let mut offset = Vec3::ZERO;
    if cam.offset_enabled {
        offset = rotation_matrix.mul_vec3(Vec3::new(cam.offset.offset.0, cam.offset.offset.1, 0.0));
    }

    let desired_translation =
        cam.focus + rotation_matrix.mul_vec3(Vec3::new(0.0, 0.0, cam.zoom.radius)) + offset;

    // Update the camera translation and focus
    let delta = player.translation - cam.focus;
    cam_transform.translation = desired_translation + delta;
}

// only run aiming logic if `aim_enabled` is true
fn aim_condition(cam_q: Query<&ThirdPersonCamera, With<ThirdPersonCamera>>) -> bool {
    let Ok(cam) = cam_q.get_single() else {
        return false;
    };
    cam.aim_enabled && cam.enabled
}

fn aim(
    mut cam_q: Query<
        (&mut ThirdPersonCamera, &Transform),
        (With<ThirdPersonCamera>, Without<ThirdPersonCameraTarget>),
    >,
    mouse: Res<Input<MouseButton>>,
    mut player_q: Query<&mut Transform, With<ThirdPersonCameraTarget>>,
    time: Res<Time>,
) {
    let Ok((mut cam, cam_transform)) = cam_q.get_single_mut() else {
        return;
    };

    // check if aim button was pressed
    let aim_btn = mouse.pressed(cam.aim_button);

    if aim_btn {
        // rotate player or target to face direction he is aiming
        let Ok(mut player_transform) = player_q.get_single_mut() else {
            return;
        };
        player_transform.look_to(cam_transform.forward(), Vec3::Y);

        let desired_zoom = cam.zoom.min * cam.aim_zoom;

        // radius_copy is used for restoring the radius (zoom) to it's
        // original value after releasing the aim button
        if cam.zoom.radius_copy.is_none() {
            cam.zoom.radius_copy = Some(cam.zoom.radius);
        }

        let zoom_factor =
            (cam.zoom.radius_copy.unwrap() / cam.aim_zoom) * cam.aim_speed * time.delta_seconds();

        // stop zooming in if current radius is less than desired zoom
        if cam.zoom.radius <= desired_zoom || cam.zoom.radius - zoom_factor <= desired_zoom {
            cam.zoom.radius = desired_zoom;
        } else {
            cam.zoom.radius -= zoom_factor;
        }
    } else {
        if let Some(radius_copy) = cam.zoom.radius_copy {
            let zoom_factor = (radius_copy / cam.aim_zoom) * cam.aim_speed * time.delta_seconds();

            // stop zooming out if current radius is greater than original radius
            if cam.zoom.radius >= radius_copy || cam.zoom.radius + zoom_factor >= radius_copy {
                cam.zoom.radius = radius_copy;
                cam.zoom.radius_copy = None;
            } else {
                cam.zoom.radius +=
                    (radius_copy / cam.aim_zoom) * cam.aim_speed * time.delta_seconds();
            }
        }
    }
}

pub fn zoom_condition(cam_q: Query<&ThirdPersonCamera, With<ThirdPersonCamera>>) -> bool {
    let Ok(cam) = cam_q.get_single() else {
        return false;
    };
    return cam.zoom_enabled && cam.enabled;
}

// only run toggle_x_offset if `offset_toggle_enabled` is true
fn toggle_x_offset_condition(cam_q: Query<&ThirdPersonCamera, With<ThirdPersonCamera>>) -> bool {
    let Ok(cam) = cam_q.get_single() else {
        return false;
    };
    cam.offset_toggle_enabled && cam.enabled
}

// inverts the x offset. Example: left shoulder view -> right shoulder view & vice versa
fn toggle_x_offset(
    mut cam_q: Query<&mut ThirdPersonCamera, With<ThirdPersonCamera>>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let Ok(mut cam) = cam_q.get_single_mut() else {
        return;
    };

    // check if toggle btn was pressed
    let toggle_btn = keys.just_pressed(cam.offset_toggle_key);

    if toggle_btn {
        // Switch direction by inverting the offset_flag
        cam.offset.is_transitioning = !cam.offset.is_transitioning;
    }

    // Determine the transition speed based on direction
    let transition_speed = if cam.offset.is_transitioning {
        -cam.offset_toggle_speed
    } else {
        cam.offset_toggle_speed
    };

    // Update the offset based on the direction and time
    cam.offset.offset.0 = (cam.offset.offset.0 + transition_speed * time.delta_seconds())
        .clamp(-cam.offset.offset_copy.0, cam.offset.offset_copy.0);
}

fn toggle_cursor(
    mut cam_q: Query<&mut ThirdPersonCamera>,
    keys: Res<Input<KeyCode>>,
    mut window_q: Query<&mut Window, With<PrimaryWindow>>,
) {
    let Ok(mut cam) = cam_q.get_single_mut() else {
        return;
    };

    if keys.just_pressed(cam.cursor_lock_key) {
        cam.cursor_lock_active = true;
    }

    let mut window = window_q.get_single_mut().unwrap();
    if cam.cursor_lock_active {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
    } else {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }
}

// checks if the toggle cursor functionality is enabled
fn toggle_cursor_condition(cam_q: Query<&ThirdPersonCamera>) -> bool {
    let Ok(cam) = cam_q.get_single() else {
        return true;
    };
    cam.cursor_lock_toggle_enabled && cam.enabled
}

fn enabled(cam_q: Query<&ThirdPersonCamera>) -> bool {
    let Ok(cam) = cam_q.get_single() else {
        return true;
    };
    return cam.enabled;
}
