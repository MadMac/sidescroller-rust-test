extern crate amethyst;

use amethyst::core::transform::TransformBundle;
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;
use amethyst::renderer::{
    DisplayConfig, DrawFlat, Event, Pipeline, PosNormTex, RenderBundle, Stage, VirtualKeyCode, PosTex
};

mod sidescroller;

fn main() -> Result<(), amethyst::Error> {
    amethyst::start_logger(Default::default());

    use sidescroller::Sidescroller;

    let path = format!(
        "{}/resources/display_config.ron",
        env!("CARGO_MANIFEST_DIR")
    );
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat::<PosTex>::new()),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)))?
        .with_bundle(TransformBundle::new())?;
    let mut game = Application::build("./", Sidescroller)?.build(game_data)?;
    game.run();
    Ok(())
}
