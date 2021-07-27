use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Health)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    // Query the set of player components and their current Point
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());

    if let Some(key) = *key {
        let delta = match key {
            VirtualKeyCode::A => Point::new(-1, 0),
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::D => Point::new(1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::W => Point::new(0, -1),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::S => Point::new(0, 1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };

        // Iterate until you find the player and its intended destionation
        let (player_entity, destination) = players
            .iter(ecs)
            .find_map(|(entity, position)| Some((*entity, *position + delta)))
            .unwrap();

        let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());

        let mut did_something = false;
        // If the player has moved check to see if they are attacking an enemy
        if delta.x != 0 || delta.y != 0 {
            // Track if the player moved or stayed in place
            did_something = true;
            // Filter enemies to entities in the same position as the player just moved.
            // If an entity is in the same position send a message of intent for attacking.
            let mut hit_something = false;
            enemies
                .iter(ecs)
                .filter(|(_, position)| **position == destination)
                .for_each(|(entity, _)| {
                    hit_something = true;

                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: player_entity,
                            victim: *entity,
                        },
                    ));
                });

            // If there are no enemies in the position the player wants to move to send
            // a messag of intent to move.
            if !hit_something {
                // Emit a WantsToMove message with the intended destination
                // NOTE: Push doesn't work for single component insertions so add an empty component
                // with the message.
                commands.push((
                    (),
                    WantsToMove {
                        entity: player_entity,
                        destination,
                    },
                ));
            }
        }

        // If the player didn't move or attack
        if !did_something {
            if let Ok(mut health) = ecs
                .entry_mut(player_entity)
                .unwrap()
                .get_component_mut::<Health>()
            {
                health.current = i32::min(health.max, health.current + 1);
                log(format!("Player healed current: {}", health.current));
            }
        }

        *turn_state = TurnState::PlayerTurn;
    }
}
