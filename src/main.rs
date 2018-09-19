extern crate amethyst;
extern crate rand;

use amethyst::core::transform::TransformBundle;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::input::InputBundle;
use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawSprite, Pipeline, RenderBundle, Stage};

#[macro_use]
extern crate log;
extern crate log4rs;

mod sidescroller;
mod systems;

fn main() -> Result<(), amethyst::Error> {
    // amethyst::start_logger(Default::default());
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    use sidescroller::Sidescroller;

    let binding_path = format!(
        "{}/resources/bindings_config.ron",
        env!("CARGO_MANIFEST_DIR")
    );

    let path = format!(
        "{}/resources/display_config.ron",
        env!("CARGO_MANIFEST_DIR")
    );

    let asset_path = format!("{}", env!("CARGO_MANIFEST_DIR"));

    let config = DisplayConfig::load(&path);

    let input_bundle =
        InputBundle::<String, String>::new().with_bindings_from_file(binding_path)?;

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawSprite::new()),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderBundle::new(pipe, Some(config))
                .with_sprite_sheet_processor()
                .with_sprite_visibility_sorting(&["transform_system"]),
        )?.with_bundle(input_bundle)?
        .with(systems::ActorSystem, "actor_system", &["transform_system"])
        .with(systems::PlayerSystem, "player_system", &["input_system"])
        .with(systems::EnemySystem, "enemy_system", &["actor_system"])
        .with(systems::GravitySystem, "gravity_system", &["actor_system"]);
    let mut game = Application::new(asset_path, Sidescroller, game_data)?;
    game.run();
    Ok(())
}

#[derive(Debug, Clone)]
pub struct Actor {
    pub width: f32,
    pub height: f32,
    pub v_velocity: f32,
    pub standing: bool,
    pub spawn: (f32, f32),
}

impl Actor {
    fn new(x: f32, y: f32) -> Actor {
        Actor {
            width: 32.0,
            height: 32.0, 
            v_velocity: 5.0,
            standing: false,
            spawn: (x, y),
        }
    }
}

pub struct Player {
    
}

impl Player {
    fn new() -> Player {
        Player {
            
        }
    }
}

#[derive(Debug, Clone)]
pub struct GameMap {
    pub width: usize,
    pub height: usize,
    pub tile_size: usize,
    pub layers: Vec<MapLayer>,
    pub actors: Vec<Actor>,
}

impl GameMap {
    fn new(width: usize, height: usize) -> GameMap {
        GameMap {
            width: width,
            height: height,
            layers: Vec::new(),
            tile_size: 32,
            actors: Vec::new(),
        }
    }

    fn push(&mut self, map_layer: MapLayer) {
        self.layers.push(map_layer);
    }

    fn add_actor(&mut self, actor: Actor) {
        self.actors.push(actor); 
    }
}

#[derive(Debug, Clone)]
pub struct MapLayer {
    pub tiles: Vec<Vec<u32>>,
}

impl MapLayer {
    fn new(tiles: Vec<Vec<u32>>) -> MapLayer {
        MapLayer { tiles: tiles }
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

impl Component for Actor {
    type Storage = DenseVecStorage<Self>;
}

pub struct Enemy {
    
}

impl Enemy {
    fn new() -> Enemy {
        Enemy {
            
        }
    }
}

impl Component for Enemy {
    type Storage = DenseVecStorage<Self>;
}
