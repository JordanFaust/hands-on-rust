use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(MovingRandomly)]
pub fn random_move(ecs: &mut SubWorld, #[resource] map: &Map) {
    // Get the point and moving randomly component of all entities that are
    // moving randomly.
    let mut movers = <(&mut Point, &MovingRandomly)>::query();

    // Randomly move each mover in a random direction
    movers.iter_mut(ecs).for_each(|(position, _)| {
        let mut rng = RandomNumberGenerator::new();
        let destination = match rng.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        } + *position;

        if map.can_enter_tile(destination) {
            *position = destination
        }
    })
}
