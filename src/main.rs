extern crate amethyst;

use amethyst::core::transform::TransformBundle;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::input::InputBundle;
use amethyst::prelude::*;
use amethyst::renderer::{
    DisplayConfig, DrawFlat, Event, Pipeline, PosNormTex, PosTex, RenderBundle, Stage,
    VirtualKeyCode,
};

mod sidescroller;
mod systems;

fn main() -> Result<(), amethyst::Error> {
    amethyst::start_logger(Default::default());

    use sidescroller::Sidescroller;

    let binding_path = format!(
        "{}/resources/bindings_config.ron",
        env!("CARGO_MANIFEST_DIR")
    );

    let path = format!(
        "{}/resources/display_config.ron",
        env!("CARGO_MANIFEST_DIR")
    );
    let config = DisplayConfig::load(&path);

    let input_bundle = InputBundle::<String, String>::new().with_bindings_from_file(binding_path)?;

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat::<PosTex>::new()),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)))?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::PlayerSystem, "player_system", &["input_system"]);
    let mut game = Application::build("./", Sidescroller)?.build(game_data)?;
    game.run();
    Ok(())
}

pub struct Player {
    pub width: f32,
    pub height: f32,
}

impl Player {
    fn new() -> Player {
        Player {
            width: 32.0,
            height: 32.0,
        }
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}
