use super::{CursedItem, EquipmentChanged, Equipped, InBackpack, Name, WantsToRemoveItem};
use specs::prelude::*;

pub struct ItemRemoveSystem {}

impl<'a> System<'a> for ItemRemoveSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, WantsToRemoveItem>,
        WriteStorage<'a, Equipped>,
        WriteStorage<'a, InBackpack>,
        ReadStorage<'a, CursedItem>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, EquipmentChanged>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut wants_remove, mut equipped, mut backpack, cursed, names, mut dirty) =
            data;

        for (entity, to_remove) in (&entities, &wants_remove).join() {
            if cursed.get(to_remove.item).is_some() {
                crate::gamelog::Logger::new()
                    .append("You cannot remove ")
                    .append(names.get(to_remove.item).unwrap().name.clone())
                    .append(", it is cursed")
                    .log();
            } else {
                equipped.remove(to_remove.item);
                backpack
                    .insert(to_remove.item, InBackpack { owner: entity })
                    .expect("Unable to insert backpack");
            }
            dirty
                .insert(entity, EquipmentChanged {})
                .expect("Unable to insert");
        }

        wants_remove.clear();
    }
}
