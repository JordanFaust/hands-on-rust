use crate::prelude::*;

mod chasing;
mod combat;
mod end_turn;
mod entity_render;
mod fov;
mod hud;
mod map_render;
mod movement;
mod player_input;
mod random_move;
mod tooltips;

pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        // Watch for player input
        .add_system(player_input::player_input_system())
        // Run the field of view system. This is required here to ensure the
        // field of view is calculated at the beginning of a game. There
        // should not be a dirty field of view after that point within this
        // schedule.
        .add_system(fov::fov_system())
        // Flush changes made as part of player input
        .flush()
        // Render the map after the player movement
        .add_system(map_render::map_render_system())
        // Render the entities on the map after player movement
        .add_system(entity_render::entity_render_system())
        // Render the player HUD
        .add_system(hud::hud_system())
        // Render the tooltips
        .add_system(tooltips::tooltips_system())
        .build()
}

pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        // Process Combat Messages (Player has already sent movement and attacking intent)
        .add_system(combat::combat_system())
        // Flush and process combat
        .flush()
        // Process any movement intents
        .add_system(movement::movement_system())
        // Flush and process any movement
        .flush()
        // Recalculate field of view in case player moved
        .add_system(fov::fov_system())
        // Flush and process field of view updates
        .flush()
        // Render the map after any collisions
        .add_system(map_render::map_render_system())
        // Render the entities ontop of the map
        .add_system(entity_render::entity_render_system())
        // Render the player HUD
        .add_system(hud::hud_system())
        // Render the tooltips
        .add_system(tooltips::tooltips_system())
        // Call the end turn system to handle turn state transition
        .add_system(end_turn::end_turn_system())
        .build()
}

pub fn build_monster_scheduler() -> Schedule {
    Schedule::builder()
        // Add random movement to entities tagged with MovingRandomly
        .add_system(random_move::random_move_system())
        // Add pathfinding to monsters
        .add_system(chasing::chasing_system())
        // Flush any changes made as a result of a random movement
        .flush()
        // Process Combat Messages (after random movement from monsters)
        .add_system(combat::combat_system())
        .flush()
        // Process any movement intents
        .add_system(movement::movement_system())
        // Flush and process any movement
        .flush()
        // Recalculate field of view for any monsters that have moved
        .add_system(fov::fov_system())
        // Flush field of view changes
        .flush()
        // Render the map after any collisions
        .add_system(map_render::map_render_system())
        // Render the entities ontop of the map
        .add_system(entity_render::entity_render_system())
        // Render the player HUD
        .add_system(hud::hud_system())
        // Render the tooltips
        .add_system(tooltips::tooltips_system())
        // Call the end turn system to handle turn state transition
        .add_system(end_turn::end_turn_system())
        .build()
}
