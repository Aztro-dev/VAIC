use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_mod_raycast::prelude::*;

pub struct PlacingPlugin;

impl Plugin for PlacingPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<PlacingState>()
            .add_plugins((
                DefaultPickingPlugins
                    .build()
                    .disable::<DebugPickingPlugin>(),
                bevy_transform_gizmo::TransformGizmoPlugin::default(),
            ))
            .add_systems(Update, (snap_to_closest, spawn_event))
            .add_systems(Update, placing.run_if(in_state(PlacingState::Placing)));
    }
}

/// Takes in path to model
#[derive(Event)]
pub struct PlacingEvent(pub String);

const PLACING_RADIUS: f32 = 30.0;

fn spawn_event(
    mut event_reader: EventReader<PlacingEvent>,
    mut placing_state: ResMut<NextState<PlacingState>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for event in event_reader.read() {
        let new_position = Vec3::new(0.0, 0.0, 0.0);
        let name = &event.0;

        let handle = asset_server.load(name);
        commands.spawn((
            SceneBundle {
                scene: handle,
                transform: Transform::from_translation(new_position),
                ..default()
            },
            CurrentlyPlacing {},
        ));
        placing_state.set(PlacingState::Placing);
    }
}

#[derive(Component)]
struct CurrentlyPlacing;

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
enum PlacingState {
    Placing,
    #[default]
    NotPlacing,
}

fn placing(
    mut commands: Commands,
    mut part_query: Query<(&mut Transform, Entity), With<CurrentlyPlacing>>,
    cursor_ray: Res<CursorRay>,
    mut raycast: Raycast,
    mouse: Res<Input<MouseButton>>,
    mut placing_state: ResMut<NextState<PlacingState>>,
) {
    for (mut transform, entity) in part_query.iter_mut() {
        if mouse.just_pressed(MouseButton::Left) {
            commands.entity(entity).remove::<CurrentlyPlacing>();
            placing_state.set(PlacingState::NotPlacing);
        }
        let should_be_picked: &dyn Fn(Entity) -> bool = &(|filter_entity: Entity| -> bool {
            println!("{}, {}", entity.index(), filter_entity.index());
            return entity.index() != filter_entity.index();
        });
        if let Some(cursor_ray) = **cursor_ray {
            let intersection_array = &raycast.cast_ray(
                cursor_ray,
                &RaycastSettings {
                    filter: should_be_picked,
                    ..default()
                },
            );
            let intersection_data = &intersection_array[intersection_array.len() - 1].1;
            let intersection_entity = &intersection_array[intersection_array.len() - 1].0;
            transform.translation = intersection_data.position();
        }
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
    }
}
