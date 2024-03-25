use super::{
    spawn_scenes::{spawn_scene, SpawnScenesState},
    Animations,
};
use crate::bevy_app::asset_loader_plugin::MyAssets;
use bevy::{gltf::Gltf, prelude::*};

pub fn spawn_character(
    mut commands: Commands,
    asset_pack: Res<MyAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    mut animations: ResMut<Animations>,
    mut next_state: ResMut<NextState<SpawnScenesState>>,
) {
    let character_id = 0;
    let gltf_handle = asset_pack
        .test_gltf
        .get("scifi_character.glb")
        .expect("to have loaded the glb");

    spawn_scene(
        &mut commands,
        &assets_gltf,
        gltf_handle.clone(),
        Some(&mut animations.0),
        "test_mesh".to_string(),
    );

    next_state.set(SpawnScenesState::AwaitingAnimations)
}
