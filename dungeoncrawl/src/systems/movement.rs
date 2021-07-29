use crate::prelude::*;

/// System that runs for each entity that has the want WantsToMove component. Checks the
/// intended move destination and makes the changes needed if it is a valid movement. If the
/// entity is a Player it also updates its camera.
#[system(for_each)]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn movement(
    entity: &Entity,
    message: &WantsToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(message.destination) {
        // It is safer and more efficient to use commands. Add/Update the entity
        // with the new desired destination
        commands.add_component(message.entity, message.destination);

        if let Ok(entry) = ecs.entry_ref(message.entity) {
            // If the entity has a field of view mark it as dirty
            if let Ok(fov) = entry.get_component::<FieldOfView>() {
                commands.add_component(message.entity, fov.clone_dirty());
            }

            // Check to see if the entity is a Player component
            if entry.get_component::<Player>().is_ok() {
                // The entity exists and is a player, update the players camera information
                camera.on_player_move(message.destination)
            }
        }
    }
    // Remove messages that have been processed for the entity
    commands.remove(*entity)
}
