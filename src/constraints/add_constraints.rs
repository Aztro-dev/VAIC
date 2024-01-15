use std::f32::consts::PI;

use crate::placing::*;
use bevy::prelude::*;

use super::ConstrainComponent;
use super::ConstraintData;

#[derive(Event)]
pub struct AddConstraintsEvent(pub Entity);

pub fn add_constraints_event(
    mut commands: Commands,
    part_query: Query<&PartName, With<Part>>,
    mut add_constraints_event_reader: EventReader<AddConstraintsEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in add_constraints_event_reader.read() {
        let entity = event.0;
        let part_name: &PartName = part_query.component(entity);
        let stripped_name = crate::ui::editor::reverse_model_name(part_name.0.clone());
        let constraints: Vec<ConstraintData> = get_constraint_data(stripped_name.clone());

        let mut entity_list: Vec<Entity> = Vec::new();
        for constraint in constraints.iter() {
            let id = commands
                .spawn(PbrBundle {
                    visibility: Visibility::Hidden,
                    mesh: meshes.add(Mesh::from(shape::Cylinder {
                        radius: 0.10,
                        height: 0.07,
                        resolution: 64,
                        ..default()
                    })),
                    material: materials.add(StandardMaterial {
                        base_color: Color::RED,
                        ..default()
                    }),
                    transform: constraint.transform,
                    ..default()
                })
                .insert(ConstrainComponent {})
                .id();
            entity_list.push(id);
        }
        let entity_list_slice: &[Entity] = &entity_list;
        commands
            .entity(entity)
            .insert_children(0, entity_list_slice);
    }
}

fn get_constraint_data(name: String) -> Vec<ConstraintData> {
    // We do a little hardcoding (teehee)
    return match name.as_str() {
        "2x1 C-Channel" => vec![
            ConstraintData {
                transform: Transform::from_translation(Vec3::new(0.29, 0.035, 0.0)),
                ..default()
            },
            ConstraintData {
                transform: Transform::from_translation(Vec3::new(-0.29, 0.035, 0.0)),
                ..default()
            },
            ConstraintData {
                transform: Transform::from_translation(Vec3::new(0.0, 0.035, -0.29)),
            },
            ConstraintData {
                transform: Transform {
                    translation: Vec3::new(-0.55, 0.365, 0.0),
                    rotation: Quat::from_rotation_z(PI / 2.0),
                    ..default()
                },
            },
            ConstraintData {
                transform: Transform {
                    translation: Vec3::new(0.55, 0.365, 0.0),
                    rotation: Quat::from_rotation_z(PI / 2.0),
                    ..default()
                },
            },
        ],
        "2x25 C-Channel" => vec![
            ConstraintData {
                transform: Transform::from_translation(Vec3::new(0.29, 0.035, 0.0)),
                ..default()
            },
            ConstraintData {
                transform: Transform::from_translation(Vec3::new(-0.29, 0.035, 0.0)),
                ..default()
            },
            ConstraintData {
                transform: Transform::from_translation(Vec3::new(0.0, 0.035, -0.29)),
            },
            ConstraintData {
                transform: Transform {
                    translation: Vec3::new(-0.55, 0.365, 0.0),
                    rotation: Quat::from_rotation_z(PI / 2.0),
                    ..default()
                },
            },
            ConstraintData {
                transform: Transform {
                    translation: Vec3::new(0.55, 0.365, 0.0),
                    rotation: Quat::from_rotation_z(PI / 2.0),
                    ..default()
                },
            },
        ],
        _ => {
            println!("{name} has no constraints!");
            vec![]
        }
    };
}
