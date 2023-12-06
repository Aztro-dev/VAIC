use crate::placing::PlacingState;
use bevy::prelude::*;

pub struct ConstraintPlugin;

impl Plugin for ConstraintPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<ConstrainState>()
            .add_systems(Update, check_for_c)
            .add_systems(
                Update,
                exit_constrain.run_if(in_state(ConstrainState::Constraining)),
            );
    }
}

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum ConstrainState {
    Constraining,
    #[default]
    NotConstraining,
}

fn check_for_c(
    keyboard: Res<Input<KeyCode>>,
    mut placing_state: ResMut<NextState<PlacingState>>,
    mut constrain_state: ResMut<NextState<ConstrainState>>,
) {
    if keyboard.just_pressed(KeyCode::C) {
        placing_state.set(PlacingState::PlacingDisabled);
        constrain_state.set(ConstrainState::Constraining);
    }
}

fn exit_constrain(
    keyboard: Res<Input<KeyCode>>,
    mut placing_state: ResMut<NextState<PlacingState>>,
    mut constrain_state: ResMut<NextState<ConstrainState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        placing_state.set(PlacingState::NotPlacing);
        constrain_state.set(ConstrainState::NotConstraining);
    }
}
