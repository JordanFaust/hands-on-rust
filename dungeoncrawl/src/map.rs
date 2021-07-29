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

    /*
     * Represents the set of revealed tiles using the same striding indexing as the set of tiles
     * for the map.
     */
    pub revealed_tiles: Vec<bool>,
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
            revealed_tiles: vec![false; NUM_TILES],
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

    /*
     * Given a point and an intended change in destination return the tile
     * ID or none for the attempted exit from the tile.
     */
    fn valid_exit(&self, location: Point, delta: Point) -> Option<usize> {
        let destination = location + delta;
        if self.in_bounds(destination) {
            if self.can_enter_tile(destination) {
                let idx = self.point2d_to_index(destination);
                Some(idx)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }

    fn in_bounds(&self, point: Point) -> bool {
        self.in_bounds(point)
    }
}

impl BaseMap for Map {
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(idx);

        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) {
            exits.push((idx, 1.0))
        }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }

    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] != TileType::Floor
    }
}
