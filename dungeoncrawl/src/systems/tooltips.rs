use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(Health)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn tooltips(ecs: &SubWorld, #[resource] mouse_pos: &Point, #[resource] camera: &Camera) {
    // Get the list of entities with a point and name component
    let mut positions = <(Entity, &Point, &Name)>::query();
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).nth(0).unwrap();

    // Calculate the map position
    let offset = Point::new(camera.left_x, camera.top_y);
    let map_position = *mouse_pos + offset;

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);

    // For any entity that the mouse is currently hovering over, get and display the name and health
    // of the entity. If the health is not available display the name.
    positions
        .iter(ecs)
        .filter(|(_, position, _)| {
            **position == map_position && player_fov.visible_tiles.contains(&position)
        })
        .for_each(|(entity, _, name)| {
            // The mouse position is in coordinates that align with the mosters layer.
            // The tooltip layer is four times larger - multiply the mouse position by four
            // to get the screen position for the tooltip layer.
            let screen_position = *mouse_pos * 4;
            let display =
                if let Ok(health) = ecs.entry_ref(*entity).unwrap().get_component::<Health>() {
                    format!("{}: {} hp", &name.value, health.current)
                } else {
                    name.value.clone()
                };
            draw_batch.print(screen_position, &display);
        });

    draw_batch.submit(10100).expect("Batch error");
}
