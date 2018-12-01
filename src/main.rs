#[macro_use]
extern crate serde_derive;

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

mod config;
mod sidescroller;
mod systems;
mod game_data;

use config::GeneralConfig;
use game_data::CustomGameDataBuilder;

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

    let general_path = format!(
        "{}/resources/general_config.ron",
        env!("CARGO_MANIFEST_DIR")
    );

    let asset_path = format!("{}", env!("CARGO_MANIFEST_DIR"));

    let general_config = GeneralConfig::load(&general_path);

    let display_config = DisplayConfig::load(&path);

    let input_bundle =
        InputBundle::<String, String>::new().with_bindings_from_file(binding_path)?;

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawSprite::new()),
    );

    let game_data = CustomGameDataBuilder::default()
        .with_running(systems::PlayerSystem, "player_system", &[])
        .with_running(systems::ActorSystem, "actor_system", &["player_system"])
        .with_running(systems::EnemySystem, "enemy_system", &["actor_system"])
        .with_running(systems::GravitySystem, "gravity_system", &["actor_system"])
        .with_base_bundle(TransformBundle::new())?
        .with_base_bundle(
            RenderBundle::new(pipe, Some(display_config))
                .with_sprite_sheet_processor()
                .with_sprite_visibility_sorting(&["transform_system"]),
        )?.with_base_bundle(input_bundle)?;
        
    let mut game = Application::build(asset_path, Sidescroller)?
        .with_resource(general_config.map)
        .build(game_data)?;
    game.run();
    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
pub enum ActorType {
    ENEMY,
    PLAYER,
    NOTYPE,
}

#[derive(Debug, Clone)]
pub struct Actor {
    pub width: f32,
    pub height: f32,
    pub v_velocity: f32,
    pub standing: bool,
    pub spawn: (f32, f32),
    pub actor_type: ActorType,
}

impl Actor {
    fn new(x: f32, y: f32, actor_type: ActorType) -> Actor {
        Actor {
            width: 32.0,
            height: 32.0,
            v_velocity: 5.0,
            standing: false,
            spawn: (x, y),
            actor_type: actor_type,
        }
    }
}

pub struct Player {}

impl Player {
    fn new() -> Player {
        Player {}
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

    fn get_player(&self) -> &Actor {
        for actor in &self.actors {
            if actor.actor_type == ActorType::PLAYER {
                return actor;
            }
        }

        panic!("Couldn't find player data in the gamemap data");
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

pub struct Enemy {}

impl Enemy {
    fn new() -> Enemy {
        Enemy {}
    }
}

impl Component for Enemy {
    type Storage = DenseVecStorage<Self>;
}
