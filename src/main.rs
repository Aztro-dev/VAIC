use bevy::prelude::*;
use bevy::window::{PresentMode, WindowTheme};
use bevy_fps_counter::FpsCounterPlugin;
use bevy_infinite_grid::{
    InfiniteGrid, InfiniteGridBundle, InfiniteGridPlugin, InfiniteGridSettings,
};
use bevy_mod_picking::low_latency_window_plugin;
use bevy_mod_raycast::prelude::*;

mod placing;
use placing::PlacingEvent;
use placing::PlacingPlugin;

mod movement;
use movement::MovementPlugin;

mod ui;
use ui::UIPlugin;

mod settings;
use settings::SettingsPlugin;

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
                        window_theme: Some(WindowTheme::Dark),
                        ..default()
                    }),
                    close_when_requested: true,
                    ..default()
                })
                .set(low_latency_window_plugin()),
            PlacingPlugin,
            InfiniteGridPlugin,
            MovementPlugin,
            UIPlugin,
            SettingsPlugin,
            DefaultRaycastingPlugin,
            FpsCounterPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, toggle_grid_visibility)
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
            shadow_color: None,
            ..default()
        },
        visibility: Visibility::Visible,
        ..default()
    });
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(
                shape::Plane {
                    size: 1_000_000.0, // Basically infinite if you really think about it
                    ..default()
                }
                .into(),
            ),
            transform: Transform::from_xyz(0.0, -0.1, 0.0),
            material: materials.add(Color::rgba(1.0, 0.0, 0.0, 0.0).into()),
            visibility: Visibility::Visible,
            ..default()
        },
        placing::Part {},
        RaycastMesh::<()>::default(), // Make this mesh ray cast-able;
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
