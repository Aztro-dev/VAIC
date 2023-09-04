use bevy::prelude::*;
use bevy::window::*;
use bevy_infinite_grid::*;
use bevy_mod_raycast::*;

mod movement;
mod part;
mod ui;

#[derive(Reflect)]
pub struct RaycastSet;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("333333").unwrap()))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "VAIC".into(),
                fit_canvas_to_parent: true,
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            InfiniteGridPlugin,
            movement::MovementPlugin,
            ui::UiPlugin,
            part::PartPlugin,
            DefaultRaycastingPlugin::<RaycastSet>::default(),
        ))
        .add_systems(Startup, (create_light, spawn_grid))
        .add_systems(Update, (close_on_esc, print_intersections::<RaycastSet>))
        .add_systems(
            First,
            update_raycast.before(RaycastSystem::BuildRays::<RaycastSet>),
        )
        .run();
}

fn create_light(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0,
    });
}

fn spawn_grid(mut commands: Commands) {
    commands
        .spawn(InfiniteGridBundle {
            grid: InfiniteGrid {
                fadeout_distance: 500.0,
                ..default()
            },
            ..default()
        })
        .insert(RaycastMesh::<RaycastSet>::default());
}

fn update_raycast(
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<&mut RaycastSource<RaycastSet>>,
) {
    // Grab the most recent cursor event if it exists:
    let Some(cursor_moved) = cursor.iter().last() else {
        return;
    };
    for mut pick_source in &mut query {
        pick_source.cast_method = RaycastMethod::Screenspace(cursor_moved.position);
    }
}
