use crate::cursor::CursorDelta;

use super::ConstrainState;
use bevy::{prelude::*, ui::RelativeCursorPosition};
use bevy_round_ui::prelude::*;

pub struct ConstraintUiPlugin;

impl Plugin for ConstraintUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MovingWindowState>()
            .add_systems(OnEnter(ConstrainState::Constraining), spawn_ui)
            .add_systems(
                Update,
                (
                    track_moving_window_state,
                    move_window.run_if(in_state(MovingWindowState::Moving)),
                )
                    .run_if(in_state(ConstrainState::Constraining)),
            )
            .add_systems(OnExit(ConstrainState::Constraining), despawn_ui);
    }
}

#[derive(Component)]
struct ConstraintUiWindow;

#[derive(Component)]
struct ConstraintUiTitleBar;

fn spawn_ui(mut commands: Commands, mut materials: ResMut<Assets<RoundUiMaterial>>) {
    let window_width = 125.0;
    let window_height = 250.0;
    commands
        .spawn((
            MaterialNodeBundle {
                style: Style {
                    width: Val::Percent(12.5),
                    height: Val::Percent(25.0),
                    left: Val::Percent(20.0),
                    top: Val::Percent(10.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                material: materials.add(RoundUiMaterial {
                    border_radius: RoundUiBorder::all(crate::ui::UI_RADIUS).into(),
                    background_color: Color::hex("666666").unwrap(),
                    size: Vec2::new(window_width, window_height),
                    ..default()
                }),
                ..default()
            },
            ConstraintUiWindow,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            top: Val::ZERO,
                            width: Val::Percent(100.0),
                            height: Val::Percent(10.0),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::hex("888888").unwrap()),
                        ..default()
                    },
                    ConstraintUiTitleBar {},
                ))
                .insert(RelativeCursorPosition::default());
        });
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
enum MovingWindowState {
    Moving,
    #[default]
    NotMoving,
}

fn track_moving_window_state(
    curr_moving_window_state: Res<State<MovingWindowState>>,
    mut next_curr_moving_window_state: ResMut<NextState<MovingWindowState>>,
    mouse_buttons: Res<Input<MouseButton>>,
    title_constraint_ui_query: Query<&RelativeCursorPosition, With<ConstraintUiTitleBar>>,
) {
    if *curr_moving_window_state == MovingWindowState::NotMoving {
        if title_constraint_ui_query.get_single().unwrap().mouse_over()
            && mouse_buttons.just_pressed(MouseButton::Left)
        {
            next_curr_moving_window_state.set(MovingWindowState::Moving);
        }
    } else {
        if !mouse_buttons.pressed(MouseButton::Left) {
            next_curr_moving_window_state.set(MovingWindowState::NotMoving);
        }
    }
}

fn move_window(
    mut window_constraint_ui_query: Query<
        (&mut Style, &Transform, &Node),
        With<ConstraintUiWindow>,
    >,
    window_query: Query<&Window>,
    cursor_delta: Res<CursorDelta>,
) {
    let mut bruh = window_constraint_ui_query.get_single_mut().unwrap();

    let window = window_query.get_single().unwrap();
    println!(
        "Cursor delta: {:?}, Window Translation: {:?}, Cursor Pos: {:?}",
        cursor_delta.current_delta_as_percentage(window.clone()),
        bruh.1.translation.x - bruh.2.size().x / 2.0,
        window.cursor_position().unwrap_or(Vec2::ZERO)
    );

    // FOR NEXT TIME:
    // bruh.1.translation.x - bruh.2.size().x / 2.0 == window.cursor_position()
    let left = cursor_delta
        .previous_position_as_percentage(window.clone())
        .x
        .clamp(0.0, 100.0 - 100.0 * (*bruh.2).size().x / window.width());

    let right = cursor_delta
        .previous_position_as_percentage(window.clone())
        .y
        .clamp(0.0, 100.0 - 100.0 * (*bruh.2).size().y / window.height());

    (*bruh.0).left = Val::Percent(left);
    (*bruh.0).top = Val::Percent(right);
}

fn despawn_ui(
    mut commands: Commands,
    constraint_ui_query: Query<Entity, With<ConstraintUiWindow>>,
) {
    commands
        .entity(constraint_ui_query.get_single().unwrap())
        .despawn_recursive();
}
