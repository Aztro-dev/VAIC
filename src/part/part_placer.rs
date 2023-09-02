use bevy::prelude::*;

pub fn spawn_part(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::hex("FFFFFF").unwrap().into()),
            ..Default::default()
        },
        bevy_mod_picking::PickableBundle::default(),
        bevy_mod_picking::backends::raycast::RaycastPickTarget::default(),
        bevy_transform_gizmo::GizmoTransformable,
    ));
}
