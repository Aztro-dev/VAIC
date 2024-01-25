use crate::{placing::Part, settings::Settings};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_infinite_grid::InfiniteGrid;
use bevy_mod_raycast::prelude::*;
use egui::{pos2, Align2, Color32, FontId, LayerId, Ui};

use egui_gizmo::{
    Gizmo, GizmoMode, GizmoOrientation, GizmoResult, GizmoVisuals, DEFAULT_SNAP_ANGLE,
    DEFAULT_SNAP_DISTANCE,
};

pub struct MoveObjectsPlugin;

impl Plugin for MoveObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (update, select_object).run_if(in_state(crate::ui::UIState::Editor)),
            );
    }
}
#[derive(Resource)]
struct GizmoOptions {
    gizmo_mode: GizmoMode,
    gizmo_orientation: GizmoOrientation,
    last_result: Option<GizmoResult>,
    custom_highlight_color: bool,
    visuals: GizmoVisuals,
}

fn setup(mut commands: Commands) {
    commands.insert_resource(GizmoOptions {
        gizmo_mode: GizmoMode::Rotate,
        gizmo_orientation: GizmoOrientation::Global,
        last_result: None,
        custom_highlight_color: false,
        visuals: GizmoVisuals {
            x_color: Color32::from_rgb(255, 0, 148),
            y_color: Color32::from_rgb(148, 255, 0),
            z_color: Color32::from_rgb(0, 148, 255),
            s_color: Color32::WHITE,
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
    settings: Res<Settings>,
) {
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
                let precise_snap = settings.precision_snap;

                // Snap angle to use for rotation when snapping is enabled.
                // Smaller snap angle is used when shift key is pressed.
                let snap_angle = if precise_snap {
                    DEFAULT_SNAP_ANGLE / 2.0
                } else {
                    DEFAULT_SNAP_ANGLE
                };

                // Snap distance to use for translation when snapping is enabled.
                // Smaller snap distance is used when shift key is pressed.
                let snap_distance = if precise_snap {
                    DEFAULT_SNAP_DISTANCE / 2.0
                } else {
                    DEFAULT_SNAP_DISTANCE
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

                let gizmo = Gizmo::new("My gizmo")
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

                    target_transform.translation = gizmo_response.translation.into();
                    target_transform.rotation = gizmo_response.rotation.into();
                    target_transform.scale = gizmo_response.scale.into();

                    show_gizmo_status(ui, gizmo_response);
                }
            });
        });
}
fn show_gizmo_status(ui: &Ui, response: GizmoResult) {
    let length = Vec3::from(response.value).length();

    let text = match response.mode {
        GizmoMode::Rotate => format!("{:.1}°, {:.2} rad", length.to_degrees(), length),

        GizmoMode::Translate | GizmoMode::Scale => format!(
            "dX: {:.2}, dY: {:.2}, dZ: {:.2}",
            response.value[0], response.value[1], response.value[2]
        ),
    };

    let rect = ui.clip_rect();
    ui.painter().text(
        pos2(rect.right() - 10.0, rect.bottom() - 10.0),
        Align2::RIGHT_BOTTOM,
        text,
        FontId::default(),
        Color32::WHITE,
    );
}

fn select_object(
    mut commands: Commands,
    cursor_ray: Res<CursorRay>,
    mut raycast: Raycast,
    placed_query: Query<Entity, With<Part>>,
    grid_query: Query<Entity, With<InfiniteGrid>>,
    mut target_query: Query<Entity, With<CurrentlyMoving>>,
    mouse_buttons: Res<Input<MouseButton>>,
) {
    if !mouse_buttons.just_pressed(MouseButton::Left) {
        return;
    }

    if placed_query.is_empty() {
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
}
