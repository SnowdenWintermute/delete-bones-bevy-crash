use bevy::{prelude::*, utils::HashMap};

pub fn collect_hierarchy(
    all_entities_with_children: &Query<&Children>,
    names: &Query<&Name>,
    root_entity: &Entity,
    collected: &mut HashMap<String, Entity>,
) {
    if let Ok(name) = names.get(*root_entity) {
        collected.insert(format!("{}", name), *root_entity);

        if let Ok(children) = all_entities_with_children.get(*root_entity) {
            for child in children {
                collect_hierarchy(all_entities_with_children, names, child, collected)
            }
        }
    }
}
