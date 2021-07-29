use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut renderables = <(&Point, &Render)>::query();
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).nth(0).unwrap();

    // Create new draw batch. This will batch deferred rendering commands
    let mut draw_batch = DrawBatch::new();
    // Set the console target to Layer 1
    draw_batch.target(1);
    // Get the camera offset
    let offset = Point::new(camera.left_x, camera.top_y);

    // Query for all entities that are renderable AND within the players fov.
    renderables
        .iter(ecs)
        .filter(|(position, _)| player_fov.visible_tiles.contains(&position))
        .for_each(|(position, render)| {
            draw_batch.set(*position - offset, render.color, render.glyph);
        });

    // Use 5000 as the map may contain 4000 elements. Include an additional offset
    // in case other ui elements are added.
    draw_batch.submit(5000).expect("Batch error");
}
