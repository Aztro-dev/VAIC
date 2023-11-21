use crate::placing::PlacingEvent;
use bevy::prelude::*;

pub fn get_parts() -> Vec<String> {
    return vec![String::from("Duck"), String::from("Cube")];
}

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut placing_event: EventWriter<PlacingEvent>,
    mut recently_placed: ResMut<crate::placing::RecentlyPlaced>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                let name = get_model_name(text.sections[0].value.as_str());

                let formatted = format!("models/{name}#Scene0");

                recently_placed.0 = Some(formatted.clone());

                placing_event.send(PlacingEvent(formatted.clone()));
                *color = Color::hex("AAAAAA").unwrap().into();
            }
            Interaction::Hovered => {
                *color = Color::hex("999999").unwrap().into();
            }
            Interaction::None => {
                *color = Color::hex("777777").unwrap().into();
            }
        }
    }
}

fn get_model_name(text: &str) -> &str {
    match text {
        "Duck" => "low_poly_duck.glb",
        "Cube" => "cube.glb",
        _ => "duck",
    }
}
