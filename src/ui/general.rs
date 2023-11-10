use super::UIState;
use bevy::prelude::*;

pub struct GeneralPlugin;

impl Plugin for GeneralPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UIState::General), spawn_ui)
            .add_systems(OnExit(UIState::General), despawn_ui);
    }
}

#[derive(Component)]
struct GeneralUIComponent;

fn spawn_ui(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(40.0),
                height: Val::Percent(80.0),
                left: Val::Percent(30.0), // 30% - 40% - 30%
                align_self: AlignSelf::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::hex("444444").unwrap().into()),
            ..default()
        },
        GeneralUIComponent,
    ));
}

fn despawn_ui(mut commands: Commands, mut ui: Query<Entity, With<GeneralUIComponent>>) {
    commands
        .entity(ui.get_single_mut().expect("General UI Not Found!"))
        .despawn_recursive();
}
