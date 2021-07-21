use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
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

        // Run the query and iterate over the results
        players.iter_mut(ecs).for_each(|(entity, position)| {
            // Calculate the new destination
            let destination = *position + delta;
            // Emit a WantsToMove message with the intended destination
            // NOTE: Push doesn't work for single component insertions so add an empty component
            // with the message.
            commands.push((
                (),
                WantsToMove {
                    entity: *entity,
                    destination,
                },
            ));
        });
        *turn_state = TurnState::PlayerTurn;
    }
}
