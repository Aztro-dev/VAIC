use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_mod_raycast::prelude::*;

pub struct PlacingPlugin;

impl Plugin for PlacingPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<PlacingState>()
            .insert_resource(RecentlyPlaced(None))
            .add_plugins((
                DefaultPickingPlugins
                    .build()
                    .disable::<DebugPickingPlugin>(),
                bevy_transform_gizmo::TransformGizmoPlugin::default(),
            ))
            .add_systems(Update, (snap_to_closest, spawn_event))
            .add_systems(
                Update,
                (placing, stop_placing_mode).run_if(in_state(PlacingState::Placing)),
            );
    }
}

#[derive(Resource)]
struct RecentlyPlaced(Option<String>);

/// Takes in path to model
#[derive(Event)]
pub struct PlacingEvent(pub String);

fn spawn_event(
    mut event_reader: EventReader<PlacingEvent>,
    mut placing_state: ResMut<NextState<PlacingState>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut recently_placed: ResMut<RecentlyPlaced>,
) {
    for event in event_reader.read() {
        let new_position = Vec3::new(0.0, -10000.0, 0.0); // Out of the camera's view lmfao
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
        recently_placed.0 = Some(name.clone());
    }
}

#[derive(Component)]
struct CurrentlyPlacing;

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum PlacingState {
    Placing,
    #[default]
    NotPlacing,
}

#[derive(Component)]
pub struct Part;

const PLACING_RADIUS: f32 = 30.0;

fn placing(
    mut commands: Commands,
    mut placing_query: Query<(&mut Transform, Entity), With<CurrentlyPlacing>>,
    placed_query: Query<Entity, With<Part>>,
    cursor_ray: Res<CursorRay>,
    mut raycast: Raycast,
    mouse: Res<Input<MouseButton>>,
    recently_placed: Res<RecentlyPlaced>,
    mut event_writer: EventWriter<PlacingEvent>,
) {
    for (mut transform, entity) in placing_query.iter_mut() {
        if mouse.just_pressed(MouseButton::Left) {
            commands.entity(entity).remove::<CurrentlyPlacing>();
            commands.get_entity(entity).unwrap().insert(Part {});
            event_writer.send(PlacingEvent(recently_placed.0.clone().unwrap()));
        }
        if let Some(cursor_ray) = **cursor_ray {
            let intersection_array = &raycast.cast_ray(
                cursor_ray,
                &RaycastSettings {
                    filter: &|filter_entity| {
                        if let Ok(_worked) = placed_query.get(filter_entity) {
                            return true;
                        }
                        return false;
                    },
                    ..default()
                },
            );
            if intersection_array.len() == 0 {
                transform.translation = cursor_ray.position(PLACING_RADIUS);
                continue;
            }
            let intersection_data = &intersection_array[0].1;
            if intersection_data.distance() >= PLACING_RADIUS {
                transform.translation = cursor_ray.position(PLACING_RADIUS);
                continue;
            }
            transform.translation = intersection_data.position();
        }
    }
}

fn stop_placing_mode(
    keyboard: Res<Input<KeyCode>>,
    mut placing_state: ResMut<NextState<PlacingState>>,
    mut recently_placed: ResMut<RecentlyPlaced>,
    mut commands: Commands,
    mut placing_query: Query<Entity, With<CurrentlyPlacing>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        placing_state.set(PlacingState::NotPlacing);
        recently_placed.0 = None;
        for part in placing_query.iter_mut() {
            commands.entity(part).despawn_recursive();
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
