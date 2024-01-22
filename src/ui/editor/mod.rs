use crate::constraints::ConstrainState;

use self::top_bar::update_top_bar_timer;

use super::UIState;
use bevy::prelude::*;

mod part_selector;
pub use part_selector::reverse_model_name;
use part_selector::spawn_part_selector;

pub mod parts_list;
use parts_list::spawn_parts_list;

pub mod top_bar;
use top_bar::spawn_top_bar;

pub mod handle;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<parts_list::RefreshPartsList>()
            .add_systems(Startup, handle::load_models_early)
            .add_systems(
                OnEnter(UIState::Editor),
                (spawn_part_selector, spawn_parts_list, spawn_top_bar),
            )
            .add_systems(OnExit(UIState::Editor), despawn_ui)
            .add_systems(
                Update,
                (
                    (
                        part_selector::button_system,
                        parts_list::update_parts_list,
                        parts_list::refresh_parts_list,
                    )
                        .run_if(in_state(ConstrainState::NotConstraining)),
                    update_top_bar_timer,
                )
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
