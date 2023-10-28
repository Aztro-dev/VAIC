use bevy::prelude::*;
use bevy::window::*;
use bevy_infinite_grid::*;
use bevy_mod_picking::low_latency_window_plugin;

mod placing;
use placing::PlacingPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("333333").unwrap()))
        .insert_resource(Msaa::Sample4)
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
        ))
        .add_plugins(InfiniteGridPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-7.0, 7.5, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        bevy_mod_picking::backends::raycast::RaycastPickCamera::default(),
        bevy_transform_gizmo::GizmoPickSource::default(),
    ));

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0,
    });

    commands.spawn(InfiniteGridBundle {
        grid: InfiniteGrid {
            fadeout_distance: 500.0,
            ..default()
        },
        ..default()
    });
}
