mod asset_loader_plugin;
mod camera_plugin;
mod collect_hierarchy;
mod delete_bones_on_spacebar_keypress;
mod find_child_with_name_containing;
mod link_animations;
mod run_animations;
mod spawn_character;
mod spawn_scenes;
use self::asset_loader_plugin::AssetLoaderPlugin;
use self::asset_loader_plugin::AssetLoaderState;
use self::camera_plugin::CameraPlugin;
use self::delete_bones_on_spacebar_keypress::delete_bones_on_spacebar_keypress;
use self::link_animations::link_animations;
use self::run_animations::run_animations;
use self::spawn_character::spawn_character;
use self::spawn_scenes::SpawnScenesState;
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::winit::UpdateMode;
use bevy::winit::WinitSettings;
use bevy_panorbit_camera::PanOrbitCameraPlugin;

// RESOURCES
#[derive(Resource, Debug, Default)]
pub struct Animations(pub HashMap<String, Handle<AnimationClip>>);

pub fn bevy_main() {
    App::new()
        .insert_resource(WinitSettings {
            focused_mode: UpdateMode::Continuous,
            unfocused_mode: UpdateMode::Continuous,
        })
        .insert_resource(AssetMetaCheck::Never)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 1000.0,
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#bevy".to_string()),
                ..Default::default()
            }),
            ..default()
        }))
        // custom plugins
        .add_plugins(CameraPlugin)
        .add_plugins(AssetLoaderPlugin)
        // external plugin
        .add_plugins(PanOrbitCameraPlugin)
        // spawning and animating
        .init_state::<SpawnScenesState>()
        .init_resource::<Animations>()
        .add_systems(OnEnter(AssetLoaderState::Done), spawn_character)
        .add_systems(
            OnEnter(SpawnScenesState::AwaitingAnimations),
            link_animations,
        )
        .add_systems(OnEnter(SpawnScenesState::Done), run_animations)
        .add_systems(
            Update,
            delete_bones_on_spacebar_keypress.run_if(in_state(SpawnScenesState::Done)),
        )
        .run();
}
