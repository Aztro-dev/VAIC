use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_transform_gizmo::TransformGizmoPlugin;

pub struct PlacingPlugin;

impl Plugin for PlacingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPickingPlugins.build(),
            // .disable::<DebugPickingPlugin>(),
            TransformGizmoPlugin::default(),
        ))
        .add_systems(Update, (snap_to_closest, spawn_on_p));
    }
}

fn spawn_on_p(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::P) {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(StandardMaterial { ..default() }),
                ..default()
            },
            bevy_mod_picking::PickableBundle::default(),
            bevy_mod_picking::backends::raycast::RaycastPickTarget::default(),
            bevy_transform_gizmo::GizmoTransformable,
        ));
    }
}
const SNAP_DIST: f32 = 0.1;
const SNAP_ROT: f32 = std::f32::consts::PI / 24.0;

fn snap_to_closest(
    mut movable_query: Query<&mut Transform, With<bevy_transform_gizmo::GizmoTransformable>>,
) {
    for mut transform in movable_query.iter_mut() {
        transform.translation.x = transform.translation.x - transform.translation.x % SNAP_DIST;
        transform.translation.y = transform.translation.y - transform.translation.y % SNAP_DIST;
        transform.translation.z = transform.translation.z - transform.translation.z % SNAP_DIST;
        let to_euler = transform.rotation.to_euler(EulerRot::XYZ);
        transform.rotation = Quat::from_euler(
            EulerRot::XYZ,
            to_euler.0 - to_euler.0 % SNAP_ROT,
            to_euler.1 - to_euler.1 % SNAP_ROT,
            to_euler.2 - to_euler.2 % SNAP_ROT,
        );
        println!(
            "{}, {:?}",
            transform.translation,
            transform.rotation.to_euler(EulerRot::XYZ)
        );
    }
}
