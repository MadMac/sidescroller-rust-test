#[macro_use]
extern crate serde_derive;

extern crate amethyst;
extern crate rand;

use amethyst::core::transform::TransformBundle;
use amethyst::input::InputBundle;
use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage};

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

    use crate::sidescroller::Sidescroller;

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
            .with_pass(DrawFlat2D::new()),
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
