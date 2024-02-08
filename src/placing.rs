use crate::actions::{Action, ActionList};
use bevy::prelude::*;
use bevy_mod_raycast::prelude::*;

use crate::constraints::ConstrainState;

use crate::ui::editor::handle::ModelHandles;

pub struct PlacingPlugin;

impl Plugin for PlacingPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<PlacingState>()
            .add_event::<PlacingEvent>()
            .add_systems(
                Update,
                spawn_event.run_if(not(in_state(ConstrainState::Constraining))),
            )
            .add_systems(
                Update,
                (placing, stop_placing_mode).run_if(
                    in_state(PlacingState::Placing)
                        .and_then(not(in_state(ConstrainState::Constraining))),
                ),
            )
            .add_systems(OnExit(PlacingState::Placing), despawn_placing);
    }
}

#[derive(Debug)]
pub struct PlacedPart {
    pub name: String,
    pub entity: Entity,
}

/// Takes in path to model
#[derive(Event)]
pub struct PlacingEvent(pub String, pub Handle<Scene>);

#[derive(Component, Debug)]
pub struct PartName(pub String);

fn spawn_event(
    mut event_reader: EventReader<PlacingEvent>,
    mut placing_state: ResMut<NextState<PlacingState>>,
    mut commands: Commands,
) {
    for event in event_reader.read() {
        let new_position = Vec3::new(0.0, -10000.0, 0.0); // Out of the camera's view lmfao
        let name = &event.0;
        let handle = event.1.clone();

        commands.spawn((
            SceneBundle {
                scene: handle.clone(),
                transform: Transform::from_translation(new_position),
                ..default()
            },
            CurrentlyPlacing,
            PartName(name.clone()),
        ));
        placing_state.set(PlacingState::Placing);
    }
}

#[derive(Component)]
pub struct CurrentlyPlacing;

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum PlacingState {
    Placing,
    #[default]
    NotPlacing,
    PlacingDisabled,
}

#[derive(Component)]
pub struct Part;

const PLACING_RADIUS: f32 = 30.0;

fn placing(
    mut commands: Commands,
    mut placing_query: Query<(&mut Transform, &PartName, Entity), With<CurrentlyPlacing>>,
    placed_query: Query<Entity, With<Part>>,
    cursor_ray: Res<CursorRay>,
    mut raycast: Raycast,
    mouse: Res<Input<MouseButton>>,
    mut action_list: ResMut<ActionList>,
    mut event_writer: EventWriter<PlacingEvent>, // To spawn multiple parts
    model_handles: Res<ModelHandles>,
    mut add_constraints_event: EventWriter<crate::constraints::AddConstraintsEvent>,
) {
    for (mut transform, name, entity) in placing_query.iter_mut() {
        if mouse.just_pressed(MouseButton::Left) {
            commands.entity(entity).remove::<CurrentlyPlacing>();
            commands.entity(entity).insert(Part);
            add_constraints_event.send(crate::constraints::AddConstraintsEvent(entity));
            let part_name = (*name).0.clone();
            event_writer.send(PlacingEvent(
                part_name.clone(),
                crate::ui::editor::handle::get_model_handle(
                    part_name.clone(),
                    (*model_handles).clone(),
                )
                .clone(),
            ));
            action_list.0.push(
                PlacedPart {
                    name: part_name.clone(),
                    entity,
                }
                .into(),
            );
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
            if intersection_array.is_empty() {
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
    mut action_list: ResMut<ActionList>,
    mut commands: Commands,
    mut placing_query: Query<Entity, With<CurrentlyPlacing>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        placing_state.set(PlacingState::NotPlacing);
        action_list.0.push(Action::PlaceHolder);
        for part in placing_query.iter_mut() {
            commands.entity(part).despawn_recursive();
        }
    }
}

fn despawn_placing(
    mut commands: Commands,
    currently_placing: Query<Entity, With<CurrentlyPlacing>>,
) {
    for entity in currently_placing.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
