use crate::prelude::*;

mod collisions;
mod end_turn;
mod entity_render;
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
        // Process any movement intents
        .add_system(movement::movement_system())
        // Flush and process any movement
        .flush()
        // Check for collisions with other entities
        .add_system(collisions::collisions_system())
        // Flush any changes made as a result of a collision
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
        // Flush any changes made as a result of a random movement
        .flush()
        // Process any movement intents
        .add_system(movement::movement_system())
        // Flush and process any movement
        .flush()
        // Check for collisions with other entities
        .add_system(collisions::collisions_system())
        // Flush any changes made as a result of a collision
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
