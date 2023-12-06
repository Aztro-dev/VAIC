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
            )
            .add_systems(Startup, test_constraints)
            .add_systems(OnEnter(ConstrainState::Constraining), show_constraints)
            .add_systems(OnExit(ConstrainState::Constraining), hide_constraints);
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

fn test_constraints(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            visibility: Visibility::Hidden,
            mesh: meshes.add(Mesh::from(shape::Torus {
                radius: 0.5,
                ring_radius: 0.02,
                ..default()
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::RED,
                ..default()
            }),
            ..default()
        },
        ConstrainComponent {},
    ));
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
