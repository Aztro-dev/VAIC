use std::f32::consts::PI;

use crate::placing::*;
// use crate::ui::editor::Models;
// use bevy::gltf::Gltf;
// use bevy::gltf::GltfMesh;
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
    // asset_server_gltf: Res<Assets<Gltf>>,
    // asset_server_gltf_mesh: Res<Assets<GltfMesh>>,
    // asset_server_mesh: Res<Assets<Mesh>>,
    // models: Res<Models>,
) {
    for event in add_constraints_event_reader.read() {
        let entity = event.0;
        let part_name: &PartName = part_query.get(entity).unwrap();

        let name = crate::ui::editor::part_selector::reverse_model_name(part_name.0.clone());

        // let folder = models.folder.clone();
        let constraints: Vec<ConstraintData> = get_constraint_data(
            name.clone(),
            // TODO: Load constraints from file
            // folder,
            // asset_server_gltf.as_ref(),
            // asset_server_gltf_mesh.as_ref(),
            // asset_server_mesh.as_ref(),
        );

        let mut entity_list: Vec<Entity> = Vec::new();
        for constraint in constraints.iter() {
            let id = commands
                .spawn(PbrBundle {
                    visibility: Visibility::Hidden,
                    mesh: meshes.add(Cylinder {
                        radius: 0.10,
                        half_height: 0.07 / 2.0,
                        ..default()
                    }),
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
        commands.entity(entity).push_children(entity_list_slice);
    }
}

// TODO: Load constraints from file
fn get_constraint_data(
    name: String,
    // folder: Vec<Handle<Gltf>>,
    // asset_server_gltf: &Assets<Gltf>,
    // asset_server_gltf_mesh: &Assets<GltfMesh>,
    // asset_server_mesh: &Assets<Mesh>,
) -> Vec<ConstraintData> {
    // TODO: Load constraints from file
    // let name = String::from_utf8(name.as_bytes()[0..name.len() - "#Scene0".len()].into())
    //     .unwrap_or("Failed utf8 parse".to_string());
    //
    // for asset in folder.iter() {
    //     let path = asset.clone().path().unwrap().to_string();
    //     if path == name.clone() {
    //         if let Some(gltf) = asset_server_gltf.get(asset.clone()) {
    //             for (key, handle) in gltf.named_meshes.iter() {
    //                 // Constraints should start with a capital 'C'
    //                 if key.as_bytes()[0] as char != 'C' {
    //                     continue;
    //                 }
    //                 let gltf_mesh = asset_server_gltf_mesh.get(handle.clone());
    //                 if gltf_mesh.is_none() {
    //                     continue;
    //                 }
    //                 // let gltf_mesh = gltf_mesh.unwrap();
    //
    //                 // let mesh = asset_server_mesh.get(gltf_mesh.primitives[0].mesh.clone());
    //                 //
    //                 // println!("{:?}", mesh);
    //             }
    //         }
    //     } else {
    //         continue;
    //     }
    // }
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
