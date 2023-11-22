use super::UIState;
use bevy::prelude::*;

mod part_selector;
use part_selector::spawn_part_selector;

mod parts_list;
use parts_list::spawn_parts_list;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(UIState::Editor),
            (spawn_part_selector, spawn_parts_list),
        )
        .add_systems(OnExit(UIState::Editor), despawn_ui)
        .add_systems(
            Update,
            (part_selector::button_system, parts_list::update_parts_list)
                .run_if(in_state(UIState::Editor)),
        );
    }
}

#[derive(Component)]
struct EditorUIComponent;

fn despawn_ui(mut commands: Commands, mut ui: Query<Entity, With<EditorUIComponent>>) {
    for entity in ui.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
}
