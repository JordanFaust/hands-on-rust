use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn collisions(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    // Get the players current position
    let mut player_position = Point::zero();
    let mut players = <&Point>::query().filter(component::<Player>());
    players
        .iter(ecs)
        .for_each(|position| player_position = *position);

    // Get the position and entity reference for all enemies
    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());

    // Find any enemies that are overlapping with the players current position
    // and remove them.
    enemies
        .iter(ecs)
        .filter(|(_, position)| **position == player_position)
        .for_each(|(entity, _)| {
            commands.remove(*entity);
        });
}
