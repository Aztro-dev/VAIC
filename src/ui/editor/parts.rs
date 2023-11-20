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
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                let str = get_model_name(text.sections[0].value.as_str());

                placing_event.send(PlacingEvent(format!("models/{str}#Scene0")));
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
        "Cube" => "duck",
        _ => "duck",
    }
}
