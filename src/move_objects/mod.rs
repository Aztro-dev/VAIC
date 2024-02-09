mod ui;

use core::f32::consts::PI;

use crate::{
    actions::{Action, ActionList},
    constraints::ConstrainState,
    placing::{CurrentlyPlacing, Part, PlacingState},
    settings::Settings,
};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_infinite_grid::InfiniteGrid;
use bevy_mod_raycast::prelude::*;
use egui::{Color32, LayerId};

use egui_gizmo::{Gizmo, GizmoMode, GizmoOrientation, GizmoResult, GizmoVisuals};

pub struct MoveObjectsPlugin;

#[derive(Default, States, Debug, Hash, Eq, Clone, Copy, PartialEq)]
pub enum MoveObjectsState {
    Moving,
    #[default]
    NotMoving,
}

impl Plugin for MoveObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_state::<MoveObjectsState>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    update,
                    select_object.run_if(not(in_state(PlacingState::Placing))),
                    (unselect_object, ui::change_gizmo_mode, delete_object)
                        .run_if(in_state(MoveObjectsState::Moving)),
                )
                    .run_if(in_state(crate::ui::UIState::Editor)),
            );
    }
}
#[derive(Resource)]
struct GizmoOptions {
    gizmo_mode: GizmoMode,
    gizmo_orientation: GizmoOrientation,
    precision_snap: bool,
    snap_angle: f32,
    snap_distance: f32,
    last_result: Option<GizmoResult>,
    custom_highlight_color: bool,
    visuals: GizmoVisuals,
}

fn setup(mut commands: Commands, settings: Res<Settings>) {
    commands.insert_resource(GizmoOptions {
        gizmo_mode: GizmoMode::Translate,
        gizmo_orientation: GizmoOrientation::Global,
        precision_snap: settings.precision_snap,
        snap_angle: PI / 12.0, // 15 degrees
        snap_distance: 0.20,
        last_result: None,
        custom_highlight_color: false,
        visuals: GizmoVisuals {
            x_color: Color32::from_rgb(255, 0, 148),
            y_color: Color32::from_rgb(148, 255, 0),
            z_color: Color32::from_rgb(0, 148, 255),
            s_color: Color32::WHITE,
            highlight_alpha: 2.0,
            gizmo_size: 75.0,
            ..default()
        },
    });
}

#[derive(Component)]
pub struct CurrentlyMoving;

fn update(
    mut contexts: EguiContexts,
    camera_q: Query<(&Camera, &Transform), Without<CurrentlyMoving>>,
    mut target_q: Query<&mut Transform, With<CurrentlyMoving>>,
    mut gizmo_options: ResMut<GizmoOptions>,
    constrain_state: Res<State<ConstrainState>>,
    window: Query<&Window>,
) {
    if *constrain_state == ConstrainState::Constraining {
        return;
    }
    let (projection_matrix, view_matrix) = {
        let (camera, transform) = camera_q.single();
        (
            camera.projection_matrix(),
            transform.compute_matrix().inverse(),
        )
    };

    if target_q.is_empty() {
        return;
    }

    egui::Area::new("Viewport")
        .fixed_pos((0.0, 0.0))
        .show(contexts.ctx_mut(), |ui| {
            ui.with_layer_id(LayerId::background(), |ui| {
                let precise_snap = gizmo_options.precision_snap;

                // Snap angle to use for rotation when snapping is enabled.
                let snap_angle = if precise_snap {
                    gizmo_options.snap_angle
                } else {
                    1.0
                };

                // Snap distance to use for translation when snapping is enabled.
                let snap_distance = if precise_snap {
                    gizmo_options.snap_distance
                } else {
                    0.1
                };

                let visuals = GizmoVisuals {
                    highlight_color: if gizmo_options.custom_highlight_color {
                        gizmo_options.visuals.highlight_color
                    } else {
                        None
                    },
                    ..gizmo_options.visuals
                };

                let model_matrix = target_q.single_mut().compute_matrix();

                let gizmo = Gizmo::new("Move Objects Gizmo")
                    .view_matrix(view_matrix.to_cols_array_2d().into())
                    .projection_matrix(projection_matrix.to_cols_array_2d().into())
                    .model_matrix(model_matrix.to_cols_array_2d().into())
                    .mode(gizmo_options.gizmo_mode)
                    .orientation(gizmo_options.gizmo_orientation)
                    .snapping(precise_snap)
                    .snap_angle(snap_angle)
                    .snap_distance(snap_distance)
                    .visuals(visuals);

                gizmo_options.last_result = gizmo.interact(ui);

                if let Some(gizmo_response) = gizmo_options.last_result {
                    let mut target_transform = target_q.single_mut();

                    // We have to do some manual translation because of a new update in the
                    // egui-gizmo dependency.
                    target_transform.translation = Vec3::new(
                        gizmo_response.translation.x,
                        gizmo_response.translation.y,
                        gizmo_response.translation.z,
                    );
                    target_transform.rotation = Quat::from_array(*gizmo_response.rotation.as_ref());
                    target_transform.scale = Vec3::new(
                        gizmo_response.scale.x,
                        gizmo_response.scale.y,
                        gizmo_response.scale.z,
                    );

                    let window = window.get_single().unwrap();

                    let window_size = Vec2::new(
                        window.resolution.physical_width() as f32,
                        window.resolution.physical_height() as f32,
                    );

                    ui::show_gizmo_status(ui, gizmo_response, window_size);
                }
            });
        });
}

fn select_object(
    mut commands: Commands,
    cursor_ray: Res<CursorRay>,
    mut raycast: Raycast,
    placed_query: Query<Entity, (With<Part>, Without<CurrentlyPlacing>)>,
    grid_query: Query<Entity, With<InfiniteGrid>>,
    mut target_query: Query<Entity, With<CurrentlyMoving>>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut moving_state: ResMut<NextState<MoveObjectsState>>,
    constrain_state: Res<State<ConstrainState>>,
) {
    if !mouse_buttons.just_pressed(MouseButton::Left) {
        return;
    }

    if placed_query.is_empty() {
        return;
    }

    if *constrain_state == ConstrainState::Constraining {
        return;
    }

    let cursor_ray = **cursor_ray;
    if cursor_ray.is_none() {
        return;
    }
    let cursor_ray = cursor_ray.unwrap();

    let intersection_array = &raycast.cast_ray(
        cursor_ray,
        &RaycastSettings {
            filter: &|filter_entity| {
                return !placed_query.contains(filter_entity) // Idk why, but this works
                    && !grid_query.contains(filter_entity);
            },
            ..default()
        },
    );

    if intersection_array.is_empty() {
        return;
    }

    for entity in target_query.iter_mut() {
        commands.entity(entity).remove::<CurrentlyMoving>();
    }

    let entity = intersection_array[0].0;

    commands.entity(entity).insert(CurrentlyMoving);
    moving_state.set(MoveObjectsState::Moving);
}

fn unselect_object(
    mut commands: Commands,
    mut target_query: Query<Entity, With<CurrentlyMoving>>,
    mut moving_state: ResMut<NextState<MoveObjectsState>>,
    keyboard: Res<Input<KeyCode>>,
) {
    if !keyboard.just_pressed(KeyCode::Escape) {
        return;
    }
    moving_state.set(MoveObjectsState::NotMoving);
    if let Some(entity) = target_query.get_single_mut().ok() {
        commands.entity(entity).remove::<CurrentlyMoving>();
    }
}

fn delete_object(
    mut commands: Commands,
    target_query: Query<(Entity, &Name, &Transform), With<CurrentlyMoving>>,
    mut moving_state: ResMut<NextState<MoveObjectsState>>,
    keyboard: Res<Input<KeyCode>>,
    mut action_list: ResMut<ActionList>,
) {
    if !keyboard.just_pressed(KeyCode::X) {
        return;
    }
    moving_state.set(MoveObjectsState::NotMoving);
    if let Some(target) = target_query.get_single().ok() {
        let entity = target.0;
        let part_name = target.1;
        let transform = target.2;
        action_list.0.push(Action::Deleted(
            entity,
            part_name.to_string().clone(),
            transform.clone(),
        ));
        commands.entity(entity).despawn_recursive();
    }
}
