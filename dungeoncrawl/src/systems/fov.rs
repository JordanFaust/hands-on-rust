use crate::prelude::*;

#[system]
#[read_component(Point)]
#[write_component(FieldOfView)]
pub fn fov(ecs: &mut SubWorld, #[resource] map: &Map) {
    // Get the Point and FieldOfView component of all entities
    let mut views = <(&Point, &mut FieldOfView)>::query();

    // Update the field of view for all entities that
    // need their field of view updated
    views
        .iter_mut(ecs)
        .filter(|(_, fov)| fov.is_dirty)
        .for_each(|(position, mut fov)| {
            fov.visible_tiles = field_of_view_set(*position, fov.radius, map);
            fov.is_dirty = false;
        });
}
