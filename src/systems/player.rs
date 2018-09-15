use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, System, WriteStorage, ReadExpect};
use amethyst::input::InputHandler;

use Player;
use GameMap;

const MOVEMENT_SCALE: f32 = 3.0;

pub struct PlayerSystem;
impl<'s> System<'s> for PlayerSystem {
	type SystemData = (
		WriteStorage<'s, Transform>,
		WriteStorage<'s, Player>,
		ReadExpect<'s, GameMap>,
		Read<'s, InputHandler<String, String>>,
	);

	fn run(&mut self, (mut transforms, mut players, game_map, input): Self::SystemData) {
		for (player, transform) in (&mut players, &mut transforms).join() {
			let movement = input.axis_value("running");

			if let Some(mv_amount) = movement {
				let scaled_amount = MOVEMENT_SCALE * mv_amount as f32;

				transform.translation[0] += scaled_amount;
			}

			if let Some(is_jumping) = input.action_is_down("jumping") {
				if is_jumping && player.standing {
					player.v_velocity = -600.0;
					transform.translation[1] += 1.0;
				}
			}

			// Avoid out of bounds from map
			if transform.translation[0] <= 1.0 {
				transform.translation[0] = 1.0;
			}

			if transform.translation[0] >= ((&game_map.width-1)*32) as f32 {
				transform.translation[0] = ((&game_map.width-1)*32) as f32;
			}
		}
	}
}
