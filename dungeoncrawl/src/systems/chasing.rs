use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(ChasingPlayer)]
#[read_component(FieldOfView)]
#[read_component(Health)]
#[read_component(Player)]
pub fn chasing(ecs: &SubWorld, commands: &mut CommandBuffer, #[resource] map: &Map) {
    // Get entities with a point that are chasing the player along with the entities fov
    let mut movers = <(Entity, &Point, &ChasingPlayer, &FieldOfView)>::query();
    // Get the positions of entities with health on the map
    let mut positions = <(Entity, &Point, &Health)>::query();
    // Get the position of the Player
    let mut player = <(&Point, &Player)>::query();

    // Get the current map position of the player
    let player_position = player.iter(ecs).nth(0).unwrap().0;
    let player_idx = map_idx(player_position.x, player_position.y);

    // Generate a flow map to use to hunt a player
    let search_targets = vec![player_idx];
    let dijkstra_map = DijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, &search_targets, map, 1024.0);

    movers.iter(ecs).for_each(|(entity, position, _, fov)| {
        // Only attempt to chase the player if the player is within the field of view
        // of the entity
        if !fov.visible_tiles.contains(&player_position) {
            return;
        }

        let idx = map_idx(position.x, position.y);
        if let Some(destination) = DijkstraMap::find_lowest_exit(&dijkstra_map, idx, map) {
            // Calculate the distance and determine the move the monster should make. Prevent the
            // monster attacking diagonaly by checking the calculated distance
            let distance = DistanceAlg::Pythagoras.distance2d(*position, *player_position);
            let destination = if distance > 1.2 {
                map.index_to_point2d(destination)
            } else {
                *player_position
            };

            let mut attacked = false;
            positions
                .iter(ecs)
                .filter(|(_, target_position, _)| **target_position == destination)
                .for_each(|(victim, _, _)| {
                    // Check to see the entity at the intended destination is a player.
                    // If it is send the message of intent to attack. Track that the entity
                    // attack to determine if it should move into the destination or not.
                    if ecs
                        .entry_ref(*victim)
                        .unwrap()
                        .get_component::<Player>()
                        .is_ok()
                    {
                        commands.push((
                            (),
                            WantsToAttack {
                                attacker: *entity,
                                victim: *victim,
                            },
                        ));
                    }
                    attacked = true;
                });

            // If the destination does not include a player that was attacked or another enemy
            // entity send a message of intent to move to the target destination.
            if !attacked {
                commands.push((
                    (),
                    WantsToMove {
                        entity: *entity,
                        destination,
                    },
                ));
            }
        }
    })
}
