use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(MovingRandomly)]
pub fn random_move(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    // Get all entities and their point that have the moving randomly component.
    let mut movers = <(Entity, &Point, &MovingRandomly)>::query();

    // Randomly move each mover in a random direction
    movers.iter_mut(ecs).for_each(|(entity, position, _)| {
        let mut rng = RandomNumberGenerator::new();
        let destination = match rng.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        } + *position;

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
    })
}
