use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(MovingRandomly)]
#[read_component(Health)]
#[read_component(Player)]
pub fn random_move(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    // Get all entities and their point that have the moving randomly component.
    let mut movers = <(Entity, &Point, &MovingRandomly)>::query();
    // Get the health and current point of all entities that have health.
    let mut positions = <(Entity, &Point, &Health)>::query();

    // Randomly move each mover in a random direction
    movers.iter(ecs).for_each(|(entity, position, _)| {
        let mut rng = RandomNumberGenerator::new();
        let destination = match rng.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        } + *position;

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
    })
}
