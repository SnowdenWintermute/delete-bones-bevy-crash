use super::{
    collect_hierarchy::collect_hierarchy,
    find_child_with_name_containing::find_child_with_name_containing, spawn_scenes::SceneName,
};
use bevy::input::prelude::*;
use bevy::{prelude::*, utils::hashbrown::HashMap};

pub fn delete_bones_on_spacebar_keypress(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    scene_query: Query<Entity, With<SceneName>>,
    all_entities_with_children: Query<&Children>,
    names: Query<&Name>,
) {
    if keys.just_pressed(KeyCode::Space) {
        for scene_entity in scene_query.iter() {
            if let Some(root_bone) = find_child_with_name_containing(
                &all_entities_with_children,
                &names,
                &scene_entity,
                "Root",
            ) {
                let mut bones: HashMap<String, Entity> = HashMap::new();
                collect_hierarchy(&all_entities_with_children, &names, &root_bone, &mut bones);

                for (bone_name, bone_entity) in bones {
                    commands.entity(bone_entity).despawn();
                    info!("despawning {:?}", bone_name);
                }
            };
        }
    }
}
