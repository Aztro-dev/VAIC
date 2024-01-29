use crate::{move_objects::GizmoOptions, move_objects::GizmoResult, settings::Settings};
use bevy::prelude::*;
use egui_gizmo::GizmoMode;
use smooth_bevy_cameras::controllers::orbit::ControlState;

use bevy_egui::egui;
use core::f32::consts::TAU;
use egui::{pos2, Align2, Color32, FontId, Ui};

// TODO: Make this an actual UI
pub fn change_gizmo_mode(
    keyboard: Res<Input<KeyCode>>,
    mut options: ResMut<GizmoOptions>,
    settings: Res<Settings>,
) {
    if keyboard.just_pressed(KeyCode::T) {
        options.gizmo_mode = match options.gizmo_mode {
            GizmoMode::Rotate => GizmoMode::Translate,
            GizmoMode::Translate => GizmoMode::Rotate,
            _ => GizmoMode::Translate, // Erm what the freak
        }
    }
    if settings.get_control_state() == ControlState::PCMode {
        options.precision_snap = !keyboard.pressed(KeyCode::ShiftLeft);
    } else {
        options.precision_snap = !keyboard.pressed(KeyCode::L);
    }
}

pub fn show_gizmo_status(ui: &Ui, response: GizmoResult, window_size: Vec2) {
    let length = Vec3::from(response.value).length();

    let degrees = length.to_degrees();

    let text = match response.mode {
        GizmoMode::Rotate => format!("{:.1}°, {:.2} rad", degrees % 360.0, length % TAU,),

        GizmoMode::Translate => format!(
            "dX: {:.2}, dY: {:.2}, dZ: {:.2}",
            response.value[0], response.value[1], response.value[2]
        ),

        GizmoMode::Scale => format!("Scale mode might not work as intended!"),
    };

    let rect = ui.clip_rect();

    ui.painter().text(
        pos2(rect.right() - 10.0, rect.bottom() - 10.0),
        Align2::RIGHT_BOTTOM,
        text,
        FontId {
            size: window_size.length() / 50.0,
            ..default()
        },
        Color32::WHITE,
    );
}
