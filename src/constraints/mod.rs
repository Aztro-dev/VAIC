use crate::placing::PlacingState;
use bevy::prelude::*;

mod ui;
use ui::ConstraintUiPlugin;

mod add_constraints;
pub use add_constraints::AddConstraintsEvent;
use add_constraints::*;

mod handle_constraints;
pub use handle_constraints::ConstraintEvent;
use handle_constraints::*;

pub struct ConstraintPlugin;

impl Plugin for ConstraintPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ConstrainState>()
            .add_event::<AddConstraintsEvent>()
            .add_event::<ConstraintEvent>()
            .add_plugins(ConstraintUiPlugin)
            .add_systems(Update, check_for_c)
            .add_systems(
                Update,
                (exit_constrain, handle_constraint_event, select_constraints)
                    .run_if(in_state(ConstrainState::Constraining)),
            )
            .add_systems(OnEnter(ConstrainState::Constraining), show_constraints)
            .add_systems(OnExit(ConstrainState::Constraining), hide_constraints)
            .add_systems(
                Update,
                add_constraints_event.run_if(in_state(crate::placing::PlacingState::Placing)),
            );
    }
}

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum ConstrainState {
    Constraining,
    #[default]
    NotConstraining,
}

#[derive(Component)]
pub struct ConstrainComponent;

#[derive(Default, Clone, Copy, Debug)]
pub struct ConstraintData {
    pub transform: Transform,
}

fn check_for_c(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut placing_state: ResMut<NextState<PlacingState>>,
    mut constrain_state: ResMut<NextState<ConstrainState>>,
) {
    if keyboard.just_pressed(KeyCode::KeyC) {
        placing_state.set(PlacingState::PlacingDisabled);
        constrain_state.set(ConstrainState::Constraining);
    }
}

fn exit_constrain(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut placing_state: ResMut<NextState<PlacingState>>,
    mut constrain_state: ResMut<NextState<ConstrainState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        placing_state.set(PlacingState::NotPlacing);
        constrain_state.set(ConstrainState::NotConstraining);
    }
}

fn show_constraints(mut constraints_query: Query<&mut Visibility, With<ConstrainComponent>>) {
    for mut visibility in constraints_query.iter_mut() {
        *visibility = Visibility::Visible;
    }
}

fn hide_constraints(mut constraints_query: Query<&mut Visibility, With<ConstrainComponent>>) {
    for mut visibility in constraints_query.iter_mut() {
        *visibility = Visibility::Hidden;
    }
}
