use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, position: Point) {
    log(format!("spawning player at position: {:?}", position));
    ecs.push((
        Player,
        position,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
    ));
}

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, position: Point) {
    log(format!("spawning enemy at position: {:?}", position));
    ecs.push((
        Enemy,
        position,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: match rng.range(0, 4) {
                // Render a ettin
                0 => to_cp437('E'),
                // Render a ogre
                1 => to_cp437('O'),
                // Render a orc
                2 => to_cp437('o'),
                // Redner a goblin
                _ => to_cp437('g'),
            },
        },
        MovingRandomly {},
    ));
}
