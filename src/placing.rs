use bevy::prelude::*;
use bevy_mod_raycast::prelude::*;

use crate::constraints::{ConstrainState, ConstraintData, ConstraintEvent};

use crate::ui::editor::handle::ModelHandles;

pub struct PlacingPlugin;

impl Plugin for PlacingPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<PlacingState>()
            .add_event::<PlacingEvent>()
            .insert_resource(ActionList(vec![]))
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
            .add_systems(OnExit(PlacingState::Placing), despawn_placing)
            .add_systems(
                Update,
                undo_move.run_if(in_state(crate::ui::UIState::Editor)),
            );
    }
}

#[derive(Debug)]
pub struct PlacedPart {
    pub name: String,
    pub entity: Entity,
}

#[derive(Debug, Clone)]
pub enum Action {
    Placed(String, Entity),
    Constrained(ConstraintEvent),
    Deleted(String, Transform),
    PlaceHolder,
}

impl Action {
    pub fn is_placed(&self) -> bool {
        match self {
            Self::Placed(_, _) => true,
            _ => false,
        }
    }
    pub fn is_constrained(&self) -> bool {
        match self {
            Self::Constrained(_) => true,
            _ => false,
        }
    }
    pub fn is_deleted(&self) -> bool {
        match self {
            Self::Deleted(_, _) => true,
            _ => false,
        }
    }
    pub fn is_placeholder(&self) -> bool {
        match self {
            Self::PlaceHolder => true,
            _ => false,
        }
    }
}

impl From<PlacedPart> for Action {
    fn from(value: PlacedPart) -> Self {
        Action::Placed(value.name, value.entity)
    }
}

impl Into<PlacedPart> for Action {
    fn into(self) -> PlacedPart {
        match self {
            Action::Placed(name, entity) => PlacedPart {
                name: name.clone(),
                entity,
            },
            _ => panic!(),
        }
    }
}

impl From<ConstraintEvent> for Action {
    fn from(value: ConstraintEvent) -> Self {
        Action::Constrained(value)
    }
}

impl Into<ConstraintEvent> for Action {
    fn into(self) -> ConstraintEvent {
        match self {
            Action::Constrained(constraint_data) => constraint_data,
            _ => panic!(),
        }
    }
}
#[derive(Resource)]
pub struct ActionList(pub Vec<Action>);

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

fn undo_move(
    mut commands: Commands,
    mut action_list: ResMut<ActionList>,
    mut transform_query: Query<&mut Transform, With<crate::placing::Part>>,
    keyboard: Res<Input<KeyCode>>,
    mut refresh_parts_list_writer: EventWriter<crate::ui::editor::parts_list::RefreshPartsList>,
) {
    if keyboard.pressed(KeyCode::ControlLeft) && keyboard.just_pressed(KeyCode::Z) {
        if action_list.0.is_empty() {
            return;
        }
        let mut last_action = &action_list.0[action_list.0.len() - 1];
        let mut last_action_index = action_list.0.len();
        for (index, curr_action) in action_list.0.iter().enumerate().rev() {
            if !curr_action.is_placeholder() {
                last_action = curr_action;
                last_action_index = index;
                break;
            }
        }
        if last_action_index >= action_list.0.len() {
            return;
        }
        match *last_action {
            Action::Placed(_, entity) => {
                commands.entity(entity).despawn_recursive();
            }
            Action::Constrained(constraint_event) => {
                let previous_transform = constraint_event.constraints[1];
                let mut transform = transform_query
                    .get_mut(constraint_event.parents[1])
                    .unwrap();
                *transform = previous_transform.transform;
            }
            _ => {
                println!("Not handled {:?} yet!", last_action);
                return;
            }
        }

        action_list.0.remove(last_action_index);

        let mut index_list = vec![];
        for (index, curr_action) in action_list.0.iter().enumerate().rev() {
            if curr_action.is_placeholder() {
                index_list.push(index);
            }
        }
        for index in index_list.iter() {
            action_list.0.remove(*index);
        }
        action_list.0.push(Action::PlaceHolder);

        refresh_parts_list_writer.send(crate::ui::editor::parts_list::RefreshPartsList);
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
