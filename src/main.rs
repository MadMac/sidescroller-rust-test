extern crate amethyst;

use amethyst::core::transform::TransformBundle;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::input::InputBundle;
use amethyst::prelude::*;
use amethyst::renderer::{
    DisplayConfig, DrawFlat, DrawSprite, Event, Pipeline, PosNormTex, PosTex, RenderBundle, Stage,
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
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::PlayerSystem, "player_system", &["input_system"])
        .with(systems::GravitySystem, "gravity_system", &["player_system"]);
    let mut game = Application::new(asset_path, Sidescroller, game_data)?;
    game.run();
    Ok(())
}

pub struct Player {
    pub width: f32,
    pub height: f32,
    pub v_velocity: f32,
    pub standing: bool,
}

impl Player {
    fn new() -> Player {
        Player {
            width: 32.0,
            height: 32.0,
            v_velocity: 0.0,
            standing: true,
        }
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}
