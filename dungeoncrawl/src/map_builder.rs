use crate::prelude::*;

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    // The map building built
    pub map: Map,
    // The List of rooms represented by the Rect structure making up each room
    pub rooms: Vec<Rect>,
    // The location at which the player enters the map
    pub player_start: Point,
    // The location of the Amulet of Yala
    pub amulet_start: Point,
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut mb = Self {
            map: Map::new(),
            rooms: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        };

        // Mark all tiles in the map as walls
        mb.fill(TileType::Wall);
        // Randomly build a set of rooms in the map
        log(format!("building rooms count={}", NUM_ROOMS));
        mb.build_random_rooms(rng);
        // Build corridors connecting the rooms
        log("building corridors");
        mb.build_corridors(rng);
        // Mark the player start area as the center of the first room
        mb.player_start = mb.rooms[0].center();
        // Use a Dijkstra Map (Flow Map) to find ther furthest position
        // from the player starting point.
        let dijkstra_map = DijkstraMap::new(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &vec![mb.map.point2d_to_index(mb.player_start)],
            &mb.map,
            1024.0,
        );
        const UNREACHABLE: &f32 = &f32::MAX;
        mb.amulet_start = mb.map.index_to_point2d(
            dijkstra_map
                .map
                .iter()
                .enumerate()
                .filter(|(_, distance)| *distance < UNREACHABLE)
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .unwrap()
                .0,
        );
        mb
    }

    // Fill the map with the given TileType
    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    /*
     * Given a random generator with a set seed, randomly generate a set
     * of rooms that are not intersecting and create the floors within each room.
     */
    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            // Build the room
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );

            // Check to see if it overlaps with other rooms
            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }

            log(format!("generated room with no overlap {:?}", room));

            // Build out the floor within the room and add it to the list
            // of generated rooms.
            if !overlap {
                room.for_each(|position| {
                    if position.x > 0
                        && position.x < SCREEN_WIDTH
                        && position.y > 0
                        && position.y < SCREEN_HEIGHT
                    {
                        let idx = map_idx(position.x, position.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });
                self.rooms.push(room)
            }
        }
    }

    /*
     * Create a virtical tunnel between two points on a map
     */
    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor
            }
        }
    }

    /*
     * Create a horizontal tunnel between two points on a map
     */
    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        // Sort rooms by their center point before allocating corridors. This
        // makes it more likely that corridors will connect adjacent rooms and not
        // snake across the whole map.
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        // Iterate through the set of rooms and connect each room with the previous room.
        // Skip the first enumerated room so that previous is a valid index.
        // The rooms are connected from their center points moving left to right based on the
        // centered x coordinate of the room.
        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();

            // Randomly dig the horizontal and then virtical parts of the corridor,
            // or vice versa.
            if rng.range(0, 2) == 1 {
                log(format!(
                    "connecting rooms horizontal then virtical a: {:?} b: {:?}",
                    prev, new
                ));
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel(prev.y, new.y, new.x)
            } else {
                log(format!(
                    "connecting rooms virtical then horizontal a: {:?} b: {:?}",
                    prev, new
                ));
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel(prev.x, new.x, new.y);
            }
        }
    }
}
