use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera,
    #[resource] turn_state: &mut TurnState
) {
    if let Some(key) = key {
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

        if delta.x != 0 || delta.y != 0 {
            // Query the set of player components with write access
            // to their Point component.
            let mut players = <&mut Point>::query().filter(component::<Player>());

            // Run the query and iterate over the results
            players.iter_mut(ecs).for_each(|position| {
                // Calculate the new destination
                let destination = *position + delta;
                // Update the players Point position if it is a valid move
                if map.can_enter_tile(destination) {
                    *position = destination;
                    camera.on_player_move(destination);
                    *turn_state = TurnState::PlayerTurn;
                }
            })
        }
    }
}
