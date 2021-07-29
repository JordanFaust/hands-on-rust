use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(AmuletOfYala)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState) {
    let mut player = <(&Health, &Point)>::query().filter(component::<Player>());
    let mut amulet  = <&Point>::query().filter(component::<AmuletOfYala>());
    let current_state = turn_state.clone();
    let mut new_state = match current_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => current_state,
    };

    // Get the position of the amulet
    let amulet_position = amulet.iter(ecs).nth(0).unwrap();

    player.iter(ecs).for_each(|(hp, position)| {
        // If the players health has dropped to zero set the state to game over
        if hp.current < 1 {
            new_state = TurnState::GameOver;
        }

        // If the player has reached the amulet set the state to victory
        if position == amulet_position {
            new_state = TurnState::Victory;
        }
    });


    *turn_state = new_state;
}
