// Rust sokoban
// main.rs

use ggez::{
    conf,
    event::{self, KeyCode, KeyMods},
    graphics::Image,
    timer, Context, GameResult,
};
use specs::{RunNow, World, WorldExt};
use std::{collections::HashMap, path};

mod audio;
mod components;
mod constants;
mod entities;
mod event_system;
mod events;
mod map;
mod resources;
mod systems;

use crate::audio::initialize_sounds;
use crate::components::register_components;
use crate::event_system::EventSystem;
use crate::map::load_map;
use crate::resources::{InputQueue, Time, register_resources};
use crate::systems::{GameplayStateSystem, InputSystem, RenderingSystem};

struct Game {
    world: World,
    image_cache: HashMap<String, Image>,
}

impl event::EventHandler<ggez::GameError> for Game {
    fn update(&mut self, context: &mut Context) -> GameResult {
        // Run input system
        {
            let mut is = InputSystem {};
            is.run_now(&self.world);
        }

        // Run gameplay state system
        {
            let mut gss = GameplayStateSystem {};
            gss.run_now(&self.world);
        }

        // Get and update time resource
        {
            let mut time = self.world.write_resource::<Time>();
            time.delta += timer::delta(context);
        }

        // Run event system
        {
            let mut es = EventSystem { context };
            es.run_now(&self.world);
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        // Render game entities
        {
            let mut rs = RenderingSystem {
                context,
                image_cache: &mut self.image_cache,
            };
            rs.run_now(&self.world);
        }

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _context: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        println!("Key pressed: {:?}", keycode);

        let mut input_queue = self.world.write_resource::<InputQueue>();
        input_queue.keys_pressed.push_back(keycode);
    }
}

// Initialize the level
pub fn initialize_level(world: &mut World) {
    const LEVELS: [&str; 6] = [
        // Tutorial default level
        "
        N N W W W W W W
        W W W . . . . W
        W . . . BB . . W
        W . . RB . . . W
        W . P . . . . W
        W . . . . RS . W
        W . . BS . . . W
        W . . . . . . W
        W W W W W W W W
        ",
        // Easy level III: Unsolved
        "
        W W W W W N
        W BS . . W W
        W P BB BB . W
        W W . . . W
        N W W . . W
        N N W W BS W
        N N N W W W
        ",
        // Easy level II
        "
        W W W W N N
        W . . W W W
        W . RB RB . W
        W RS RS RS . W
        W . P RB . W
        W . . . W W
        W W W W W N
        ",
        // Hard level II: Unsolved
        "
        W W W W N N N
        W . . W W W W
        W . RS . RS . W
        W . RB RB W P W
        W W . . . . W
        N W W W W W W
        ",
        // Easy level
        "
        N W W W N
        N W BS W W
        W W BB P W
        W BS BB . W
        W W W W W
        ",
        // Difficult level
        "
        N N W W W W W N
        W W W . . . W N
        W BS P BB . . W N
        W W W . BB BS W N
        W BS W W BB . W N
        W . W . BS . W W
        W BB . BBBS BB BB BS W
        W . . . BS . . W
        W W W W W W W W
        ",
    ];
    const MAP: &str = LEVELS[1];

    load_map(world, MAP);
}

pub fn main() -> GameResult {
    let mut world = World::new();
    register_components(&mut world);
    register_resources(&mut world);
    initialize_level(&mut world);

    // Create a game context and event loop
    let context_builder = ggez::ContextBuilder::new("rust_sokoban", "sokoban")
        .window_setup(conf::WindowSetup::default().title("Rust Sokoban!"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (mut context, event_loop) = context_builder.build()?;
    initialize_sounds(&mut world, &mut context);

    // Create the game state
    let game = Game {
        world,
        image_cache: HashMap::new(),
    };
    // Run the main event loop
    event::run(context, event_loop, game)
}
