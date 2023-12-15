use bevy::prelude::*;
use bevy_mod_raycast::prelude::*;

use super::{ui::CurrentConstraintOperation, ConstrainComponent, ConstraintData};

#[derive(Event, Debug)]
pub struct ConstraintEvent {
    pub constraints: [ConstraintData; 2],
    /// The Entity at position one (index 0) is the entity that will move
    pub parents: [Entity; 2],
}

pub fn handle_constraint_event(
    mut events: EventReader<ConstraintEvent>,
    mut transform_query: Query<&mut Transform, With<crate::placing::Part>>,
) {
    for event in events.read() {
        let mut transform = transform_query.get_mut(event.parents[0]).unwrap();
        let displacement =
            event.constraints[0].transform.translation - event.constraints[1].transform.translation;
        (*transform).translation += displacement;
        let rotation_diff =
            event.constraints[0].transform.rotation - event.constraints[1].transform.rotation;
        (*transform).rotation = (*transform).rotation - rotation_diff;
        println!("{displacement}");
    }
}

pub fn select_constraints(
    mut constrain_events: EventWriter<ConstraintEvent>,
    constraints_query: Query<(Entity, &Handle<StandardMaterial>), With<ConstrainComponent>>,
    mut current_constraint_operation: ResMut<CurrentConstraintOperation>,
    transform_query: Query<&Transform>,
    parent_query: Query<&Parent>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cursor_ray: Res<CursorRay>,
    mut raycast: Raycast,
    mouse: Res<Input<MouseButton>>,
) {
    if let Some(cursor_ray) = **cursor_ray {
        let intersection_array = &raycast.cast_ray(
            cursor_ray,
            &RaycastSettings {
                filter: &|filter_entity| {
                    if let Ok(_worked) = constraints_query.get(filter_entity) {
                        return true;
                    }
                    return false;
                },
                ..default()
            },
        );
        if !intersection_array.is_empty() {
            let intersection = &intersection_array[0];
            let material = materials
                .get_mut(constraints_query.get(intersection.0).unwrap().1)
                .unwrap();
            if mouse.just_pressed(MouseButton::Left) {
                if current_constraint_operation.constraints[0].is_none() {
                    if let Ok(transform) = transform_query.get(intersection.0) {
                        if let Ok(parent) = parent_query.get(intersection.0) {
                            let parent_entity = parent.get();
                            current_constraint_operation.parents[0] = Some(parent_entity);
                            let parent_transform = *transform_query.get(parent_entity).unwrap();
                            let transform = *transform;
                            current_constraint_operation.constraints[0] = Some(ConstraintData {
                                transform: Transform {
                                    translation: transform.translation
                                        - parent_transform.translation,
                                    rotation: transform.rotation + parent_transform.rotation,
                                    ..default()
                                },
                            });
                        }
                    }
                } else if current_constraint_operation.constraints[1].is_none() {
                    if let Ok(transform) = transform_query.get(intersection.0) {
                        if let Ok(parent) = parent_query.get(intersection.0) {
                            // No if-let && :(
                            let parent_entity = parent.get();
                            current_constraint_operation.parents[1] = Some(parent_entity);
                            let parent_transform = *transform_query.get(parent_entity).unwrap();
                            let transform = *transform;
                            current_constraint_operation.constraints[1] = Some(ConstraintData {
                                transform: Transform {
                                    translation: transform.translation
                                        - parent_transform.translation,
                                    rotation: transform.rotation + parent_transform.rotation,
                                    ..default()
                                },
                            });
                            constrain_events.send((*current_constraint_operation).into());
                            *current_constraint_operation = CurrentConstraintOperation::default();
                        }
                    }
                }
                material.base_color = Color::SEA_GREEN;
            } else {
                material.base_color = Color::BLUE;
            }
        } else {
            for (_entity, handle) in constraints_query.iter() {
                let material = materials.get_mut(handle).unwrap();
                material.base_color = Color::RED;
            }
        }
    }
}
