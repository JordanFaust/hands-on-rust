use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    /*
    Represent the entire 2 demensional map as a single dimensional vector. Map
    locations of (x, y) will be transformed into vector indices. This is known as "striding".
    This will use row-first encoding of the map.

    A 5x3 map would be represented as

    0  1  2  3  4
    5  6  7  8  9
    10 11 12 13 14

    The index of a tile given it's coordinates is calculated as follows:

    let index = (y * WIDTH) + x;

    You can calculate the reciprocal - the x and y coordinates represented by an index - with:

    let y = index % WIDTH;
    let x = index / WIDTH; <-- Always rounds *DOWN*
    */
    pub tiles: Vec<TileType>,
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        // Render the following to the base layer
        ctx.set_active_console(0);
        for y in camera.top_y..camera.bottom_y {
            for x in camera.left_x..camera.right_x {
                if self.in_bounds(Point::new(x, y)) {
                    let idx = map_idx(x, y);
                    match self.tiles[idx] {
                        TileType::Floor => ctx.set(
                            x - camera.left_x,
                            y - camera.top_y,
                            WHITE,
                            BLACK,
                            to_cp437('.'),
                        ),
                        TileType::Wall => ctx.set(
                            x - camera.left_x,
                            y - camera.top_y,
                            WHITE,
                            BLACK,
                            to_cp437('#'),
                        ),
                    }
                }
            }
        }
    }

    /*
     * Validate that the given point is within the bounds of the map
     */
    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    /*
     * Validate that the player can enter the given tile. Validates that the
     * desired tile is within bounds and is a tile type that can be entered.
     */
    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }

    /*
     * Try and get the index in the Map vector of the given point. Returns an
     * option containing the index or none.
     */
    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            return None;
        }

        Some(map_idx(point.x, point.y))
    }
}
