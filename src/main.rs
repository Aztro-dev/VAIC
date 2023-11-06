use bevy::prelude::*;
use bevy::window::*;
use bevy_infinite_grid::*;
use bevy_mod_picking::low_latency_window_plugin;
use bevy_mod_picking::prelude::*;

mod placing;
use placing::PlacingEvent;
use placing::PlacingPlugin;

mod movement;
use movement::MovementPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("333333").unwrap()))
        .insert_resource(Msaa::Sample4)
        .add_event::<PlacingEvent>()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "VAIC".into(),
                        fit_canvas_to_parent: true,
                        present_mode: PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .set(low_latency_window_plugin()),
            PlacingPlugin,
            InfiniteGridPlugin,
            MovementPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (close_on_esc, toggle_grid_visibility))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 10.0,
    });

    commands.spawn(InfiniteGridBundle {
        settings: InfiniteGridSettings {
            fadeout_distance: 500.0,
            ..default()
        },
        visibility: Visibility::Visible,
        ..default()
    });
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(
                shape::Plane {
                    size: 1000.0,
                    ..default()
                }
                .into(),
            ),
            material: materials.add(Color::rgba(0.0, 0.0, 0.0, 0.0).into()),
            visibility: Visibility::Visible,
            ..default()
        },
        bevy_mod_picking::backends::raycast::RaycastPickTarget::default(),
        On::<Pointer<Click>>::run(placing::send_place_event),
    ));
}

fn toggle_grid_visibility(
    mut visibility_query: Query<&mut Visibility, With<InfiniteGrid>>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Equals) {
        let mut visibility = visibility_query.get_single_mut().unwrap();
        match *visibility {
            Visibility::Visible => {
                *visibility = Visibility::Hidden;
            }
            _ => {
                *visibility = Visibility::Visible;
            }
        }
    }
}
