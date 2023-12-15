use bevy::prelude::*;
use bevy_mod_raycast::prelude::*;

use super::{ui::CurrentConstraintOperation, ConstrainComponent, ConstraintData};

#[derive(Event, Debug)]
pub struct ConstraintEvent {
    pub constraints: [ConstraintData; 2],
    /// The Entity at position one is the entity that will move
    pub parents: [Entity; 2],
}

pub fn handle_constraint_event(
    // mut commands: Commands,
    mut events: EventReader<ConstraintEvent>,
    mut transform_query: Query<&mut Transform, With<crate::placing::Part>>,
) {
    for event in events.read() {
        let mut transform = transform_query.get_mut(event.parents[0]).unwrap();
        let displacement =
            event.constraints[0].transform.translation - event.constraints[1].transform.translation;
        (*transform).translation += displacement;
        println!("{displacement}");
    }
}

pub fn select_constraints(
    mut constrain_events: EventWriter<ConstraintEvent>,
    constraints_query: Query<(Entity, &Handle<StandardMaterial>), With<ConstrainComponent>>,
    mut current_constraint_operation: ResMut<CurrentConstraintOperation>,
    constraint_data_query: Query<&Transform, With<ConstrainComponent>>,
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
                    if let Ok(transform) = constraint_data_query.get(intersection.0) {
                        current_constraint_operation.constraints[0] = Some(ConstraintData {
                            transform: *transform,
                        });
                        if let Ok(parent) = parent_query.get(intersection.0) {
                            // No if-let && :(
                            current_constraint_operation.parents[0] = Some(parent.get());
                        }
                    }
                } else if current_constraint_operation.constraints[1].is_none() {
                    if let Ok(transform) = constraint_data_query.get(intersection.0) {
                        current_constraint_operation.constraints[1] = Some(ConstraintData {
                            transform: *transform,
                        });
                        if let Ok(parent) = parent_query.get(intersection.0) {
                            // No if-let && :(
                            current_constraint_operation.parents[1] = Some(parent.get());
                            constrain_events.send((*current_constraint_operation).into());
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
