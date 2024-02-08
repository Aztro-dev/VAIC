use crate::constraints::ConstraintEvent;
use crate::placing::PlacedPart;
use bevy::prelude::*;

#[derive(Resource)]
pub struct ActionList(pub Vec<Action>);

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
