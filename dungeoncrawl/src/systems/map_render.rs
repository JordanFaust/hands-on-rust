use crate::prelude::*;

#[system]
pub fn map_render(#[resource] map: &Map, #[resource] camera: &Camera) {
    // Create new draw batch. This will batch deferred rendering commands
    let mut draw_batch = DrawBatch::new();
    // Set the console targe to the base layer (Layer 0)
    draw_batch.target(0);

    // Build the individual cell glyphs for the current viewport
    for y in camera.top_y..camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            // The point on the full map
            let point = Point::new(x, y);
            // The edges of the current viewport
            let offset = Point::new(camera.left_x, camera.top_y);
            if map.in_bounds(point) {
                // Get the glyph for the current point
                let idx = map_idx(x, y);
                let glyph = match map.tiles[idx] {
                    TileType::Floor => to_cp437('.'),
                    TileType::Wall => to_cp437('#'),
                };
                // Set the individual cell glyph for the position
                draw_batch.set(point - offset, ColorPair::new(WHITE, BLACK), glyph);
            }
        }
    }

    // Submit the completed set of batched render. Set the sort order to 0
    // so that it is rendered first ensuring that it is drawn at the beginning of the
    // render cycle.
    draw_batch.submit(0).expect("Batch error");
}