use crate::ui::PickerState;
use bevy::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_transform_gizmo::TransformGizmoPlugin;

mod part_placer;
use part_placer::*;

pub struct PartPlugin;

impl Plugin for PartPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            TransformGizmoPlugin::new(Quat::default()),
            DefaultPickingPlugins,
        ))
        .add_systems(OnEnter(PickerState::PartSelected), spawn_part);
    }
}
