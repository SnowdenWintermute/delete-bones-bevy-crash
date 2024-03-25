use bevy::{gltf::Gltf, prelude::*, utils::HashMap};

#[derive(Component, Debug)]
pub struct SceneName(pub String);

#[derive(States, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub enum SpawnScenesState {
    #[default]
    Spawning,
    AwaitingAnimations,
    Done,
}

#[derive(Component, Debug)]
pub struct SceneLoaded;

pub fn spawn_scene(
    commands: &mut Commands,
    assets_gltf: &Res<Assets<Gltf>>,
    gltf_handle: Handle<Gltf>,
    animations_option: Option<&mut HashMap<String, Handle<AnimationClip>>>,
    scene_name: String,
) {
    let gltf = assets_gltf
        .get(gltf_handle)
        .expect("to have loaded the glb");

    let entity_commands = commands.spawn((
        SceneBundle {
            scene: gltf.named_scenes["Scene"].clone(),
            ..Default::default()
        },
        SceneName(scene_name),
    ));

    if let Some(animations) = animations_option {
        for named_animation in gltf.named_animations.iter() {
            info!("inserting animation: {}", named_animation.0);
            animations.insert(
                named_animation.0.clone(),
                gltf.named_animations[named_animation.0].clone(),
            );
        }
    }
}
