use bevy::prelude::*;

mod part_picker;
use part_picker::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<PickerState>()
            .init_resource::<PickerSelect>()
            .add_systems(Startup, (spawn_2d_cam, spawn_part_picker))
            .add_systems(Update, (mouse_scroll, detect_button_click));
    }
}

fn spawn_2d_cam(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera: Camera {
            order: -1,
            ..default()
        },
        ..default()
    });
}

#[derive(States, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Debug, Default)]
pub enum PickerState {
    PartSelected,
    PartDeselected,
    #[default]
    PartNeutral,
}

#[derive(Resource, Default)]
pub struct PickerSelect {
    pub selected: Option<String>,
}
