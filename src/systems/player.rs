use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

use {Player};

const MOVEMENT_SCALE: f32 = 3.0;

pub struct PlayerSystem;
impl<'s> System<'s> for PlayerSystem {
	type SystemData = (
		WriteStorage<'s, Transform>,
		ReadStorage<'s, Player>,
		Read<'s, InputHandler<String, String>>,
	);

	fn run(&mut self, (mut transforms, players, input): Self::SystemData) {
		for (player, transform) in (&players, &mut transforms).join() {
			let movement = input.axis_value("running");

			if let Some(mv_amount) = movement {
				let scaled_amount = MOVEMENT_SCALE * mv_amount as f32;

				transform.translation[0] += scaled_amount;
			}

			if let Some(is_jumping) = input.action_is_down("jumping") {
				if is_jumping {
					println!("JUMP");
				}
			}
		}
	}


}