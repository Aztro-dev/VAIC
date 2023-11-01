use bevy::prelude::*;
use bevy_mod_picking::backend::HitData;
use bevy_mod_picking::prelude::*;

pub struct PlacingPlugin;

impl Plugin for PlacingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPickingPlugins
                .build()
                .disable::<DebugPickingPlugin>(),
            bevy_transform_gizmo::TransformGizmoPlugin::default(),
        ))
        .add_systems(Update, (snap_to_closest, spawn_event));
    }
}

#[derive(Event)]
pub struct PlacingEvent(pub HitData);

fn spawn_event(
    mut event_reader: EventReader<PlacingEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in event_reader.iter() {
        let hit = &event.0;
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(StandardMaterial { ..default() }),
                transform: Transform::from_translation(hit.position.unwrap()),
                ..default()
            },
            bevy_mod_picking::PickableBundle::default(),
            bevy_mod_picking::backends::raycast::RaycastPickTarget::default(),
            bevy_transform_gizmo::GizmoTransformable,
        ));
    }
}

pub fn send_place_event(
    mut place_event: EventWriter<PlacingEvent>,
    listener: Listener<Pointer<Click>>,
) {
    let button = listener.button;
    if button != PointerButton::Secondary {
        return;
    }
    let hit = listener.hit.clone();
    println!("{:?}", hit);
    place_event.send(PlacingEvent(hit));
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
    }
}
