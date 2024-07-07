use bevy::pbr::ClusterConfig;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowTheme};

use bevy_blur_regions::prelude::BlurRegionsPlugin;
use bevy_blur_regions::BlurRegionsCamera;
use bevy_editor_cam::prelude::{EditorCam, EnabledMotion, Sensitivity};
use bevy_framepace::*;
use bevy_infinite_grid::{
    GridShadowCamera, InfiniteGrid, InfiniteGridBundle, InfiniteGridPlugin, InfiniteGridSettings,
};
use bevy_mod_raycast::{prelude::RaycastMesh, DefaultRaycastingPlugin};
// use bevy_fps_counter::FpsCounterPlugin;

mod placing;
use placing::PlacingPlugin;

mod move_objects;
use move_objects::MoveObjectsPlugin;

mod movement;
use movement::MovementPlugin;

mod ui;
use ui::UIPlugin;

mod settings;
use settings::{Settings, SettingsPlugin};

mod constraints;
use constraints::ConstraintPlugin;

mod cursor;
use cursor::CursorPlugin;

mod saving;
use saving::SavingPlugin;

mod actions;
use actions::ActionsPlugin;

mod screenshot;
use screenshot::ScreenshotPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("333333").unwrap()))
        .insert_resource(Msaa::Sample4)
        .add_plugins((
            bevy::prelude::DefaultPlugins.set(bevy::window::WindowPlugin {
                primary_window: Some(Window {
                    title: "VAIC".to_string(),
                    present_mode: PresentMode::AutoVsync,
                    window_theme: Some(WindowTheme::Dark),
                    ..default()
                }),
                close_when_requested: true,
                ..default()
            }),
        ))
        .add_plugins((
            PlacingPlugin,
            MoveObjectsPlugin,
            MovementPlugin,
            UIPlugin,
            SettingsPlugin,
            ConstraintPlugin,
            SavingPlugin,
            ActionsPlugin,
            ScreenshotPlugin,
            CursorPlugin,
            DefaultRaycastingPlugin,
            // FpsCounterPlugin,
            InfiniteGridPlugin,
            BlurRegionsPlugin::default(),
            bevy_framepace::FramepacePlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Startup, set_frame_cap)
        .add_systems(Update, toggle_grid_visibility)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    settings: Res<Settings>,
) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-7.0, 7.5, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert((
            BlurRegionsCamera::default(),
            GridShadowCamera,
            EditorCam {
                sensitivity: Sensitivity {
                    orbit: settings.rotate_sensitivity,
                    zoom: settings.zoom_sensitivity,
                },
                enabled_motion: EnabledMotion {
                    // pan: false,
                    ..default()
                },
                ..default()
            },
            // Doesn't work on my machine for some reason
            EnvironmentMapLight {
                intensity: 1000.0,
                diffuse_map: asset_server.load("environment_maps/diffuse_rgb9e5_zstd.ktx2"),
                specular_map: asset_server.load("environment_maps/specular_rgb9e5_zstd.ktx2"),
            }, // ),
            ClusterConfig::Single,
        ));

    commands.insert_resource(AmbientLight {
        brightness: 1000.0,
        ..default()
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
            mesh: meshes.add(Plane3d::default().mesh().size(1_000_000.0, 1_000_000.0)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            material: materials.add(Color::rgba(1.0, 0.0, 0.0, 0.0)),
            visibility: Visibility::Visible,
            ..default()
        },
        placing::Part {},
        RaycastMesh::<()>::default(), // Make this mesh ray cast-able;
        InfiniteGrid,
    ));
}

fn toggle_grid_visibility(
    mut visibility_query: Query<&mut Visibility, With<InfiniteGrid>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Equal) {
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

fn set_frame_cap(
    mut settings: ResMut<bevy_framepace::FramepaceSettings>,
    _loaded_settings: Res<crate::settings::Settings>,
) {
    // let limit = loaded_settings.fps_cap;
    // if limit == 0.0 {
    settings.limiter = Limiter::Off;
    // } else {
    //     settings.limiter = Limiter::from_framerate(loaded_settings.fps_cap);
    // }
}
