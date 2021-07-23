mod camera;
mod components;
mod map;
mod map_builder;
mod spawners;
mod systems;
mod turn_state;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawners::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
}

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        // Spawn the player within the rendered map
        spawn_player(&mut ecs, map_builder.player_start);
        // Spawn an enemy in each room other then the first room
        map_builder
            .rooms
            .iter()
            .skip(1)
            .map(|room| room.center())
            .for_each(|position| spawn_monster(&mut ecs, &mut rng, position));
        // Add the map as a resource
        resources.insert(map_builder.map);
        // Add the camera as a resource
        resources.insert(Camera::new(map_builder.player_start));
        // Set the default state the waiting input
        resources.insert(TurnState::AwaitingInput);
        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // Clear base layer
        ctx.set_active_console(0);
        ctx.cls();
        // Clear Entity Layer
        ctx.set_active_console(1);
        ctx.cls();
        // Clear HUD Layer
        ctx.set_active_console(2);
        ctx.cls();
        // Add the keyboard state as a resource. This makes the keyboard state
        // available to any system. This replaces the previous keyboard state
        // from the last tick.
        self.resources.insert(ctx.key);
        // Execute the systems for the current state
        let current_state = self.resources.get::<TurnState>().unwrap().clone();
        match current_state {
            TurnState::AwaitingInput => self
                .input_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => self
                .player_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::MonsterTurn => self
                .monster_systems
                .execute(&mut self.ecs, &mut self.resources),
        }
        // TODO: Render Draw Buffer
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        // Specify the dimensions of the console
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        // Specify the size of each character in the font file
        .with_tile_dimensions(32, 32)
        // The directory in which assets and graphics are placed
        .with_resource_path("resources/")
        // The font file to load the the dimensions of each character. Usually the same as
        // tile dimentions.
        .with_font("dungeonfont.png", 32, 32)
        // The font used for the HUD
        .with_font("terminal8x8.png", 8, 8)
        // Add a console using the dimension specified with the named tile graphics file
        // Base Layer
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        // Add a second console with no background so transparency shows through
        // Entity Layer
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        // Add a third console with no background so transparency shows through
        // HUD Layer
        .with_simple_console_no_bg(SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2, "terminal8x8.png")
        .build()?;

    main_loop(context, State::new())
}
