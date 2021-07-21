use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    // Create new draw batch. This will batch deferred rendering commands
    let mut draw_batch = DrawBatch::new();
    // Set the console target to Layer 1
    draw_batch.target(1);
    // Get the camera offset
    let offset = Point::new(camera.left_x, camera.top_y);

    // Query for all entities with both the Point and Render components
    // and draw them at their current position taking into account the camera
    // offset.
    <(&Point, &Render)>::query()
        .iter(ecs)
        .for_each(|(position, render)| {
            draw_batch.set(*position - offset, render.color, render.glyph);
        });

    // Use 5000 as the map may contain 4000 elements. Include an additional offset
    // in case other ui elements are added.
    draw_batch.submit(5000).expect("Batch error");
}
