use crate::constraints::ConstraintEvent;
use crate::placing::PlacedPart;
use bevy::prelude::*;

pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActionList(vec![])).add_systems(
            Update,
            undo_move.run_if(in_state(crate::ui::UIState::Editor)),
        );
    }
}

#[derive(Resource)]
pub struct ActionList(pub Vec<Action>);

#[derive(Debug, Clone)]
pub enum Action {
    Placed(String, Entity),
    Constrained(ConstraintEvent),
    Deleted(Entity, String, Transform),
    PlaceHolder,
}

impl Action {
    pub fn is_placed(&self) -> bool {
        match self {
            Self::Placed(_, _) => true,
            _ => false,
        }
    }
    // pub fn is_constrained(&self) -> bool {
    //     match self {
    //         Self::Constrained(_) => true,
    //         _ => false,
    //     }
    // }
    // pub fn is_deleted(&self) -> bool {
    //     match self {
    //         Self::Deleted(_, _) => true,
    //         _ => false,
    //     }
    // }
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
        match last_action.clone() {
            Action::Placed(_, entity) => {
                commands.entity(entity).despawn_recursive();
            }
            Action::Constrained(constraint_event) => {
                let mut transform = transform_query
                    .get_mut(constraint_event.parents[0])
                    .unwrap();
                let displacement = constraint_event.constraints[1].transform.translation
                    - constraint_event.constraints[0].transform.translation;
                (*transform).translation += displacement;
                (*transform).rotation = constraint_event.constraints[0].transform.rotation;
            }
            Action::Deleted(_, _, _) => {
                println!("Undoing a delete isn't supported yet!");
            }
            Action::PlaceHolder => {
                println!("PlaceHolder");
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
