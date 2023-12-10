use super::ConstrainState;
use bevy::{input::mouse::MouseMotion, prelude::*, ui::RelativeCursorPosition};
use bevy_round_ui::prelude::*;

pub struct ConstraintUiPlugin;

impl Plugin for ConstraintUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ConstrainState::Constraining), spawn_ui)
            .add_systems(
                Update,
                move_window.run_if(in_state(ConstrainState::Constraining)),
            )
            .add_systems(OnExit(ConstrainState::Constraining), despawn_ui);
    }
}

#[derive(Component)]
struct ConstraintUiComponent;

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
            ConstraintUiComponent,
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

fn move_window(
    mouse_buttons: Res<Input<MouseButton>>,
    title_constraint_ui_query: Query<&RelativeCursorPosition, With<ConstraintUiTitleBar>>,
    mut window_constraint_ui_query: Query<(&mut Style, &Transform), With<ConstraintUiComponent>>,
    mut mouse_position: EventReader<MouseMotion>,
) {
    if !mouse_buttons.pressed(MouseButton::Left) {
        return;
    }

    if !title_constraint_ui_query.get_single().unwrap().mouse_over() {
        return;
    }

    let mut bruh = window_constraint_ui_query.get_single_mut().unwrap();
    for motion in mouse_position.read() {
        (*bruh.0).left = Val::Px(motion.delta.x);
        (*bruh.0).top = Val::Px(motion.delta.y);
    }
}

fn despawn_ui(
    mut commands: Commands,
    constraint_ui_query: Query<Entity, With<ConstraintUiComponent>>,
) {
    commands
        .entity(constraint_ui_query.get_single().unwrap())
        .despawn_recursive();
}
