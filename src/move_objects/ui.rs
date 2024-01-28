use crate::move_objects::GizmoOptions;
use crate::settings::Settings;
use bevy::prelude::*;
use egui_gizmo::GizmoMode;

// TODO: Make this an actual UI
pub fn change_gizmo_mode(
    keyboard: Res<Input<KeyCode>>,
    mut options: ResMut<GizmoOptions>,
    mut settings: ResMut<Settings>,
) {
    if keyboard.just_pressed(KeyCode::T) {
        options.gizmo_mode = match options.gizmo_mode {
            GizmoMode::Rotate => GizmoMode::Translate,
            GizmoMode::Translate => GizmoMode::Rotate,
            _ => GizmoMode::Translate, // Erm what the freak
        }
    }

    if keyboard.pressed(KeyCode::L) {
        settings.precision_snap = true;
    }
}
