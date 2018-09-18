use amethyst::core::timing::Time;
use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadExpect, System, WriteStorage};
use amethyst::input::InputHandler;

use GameMap;
use Actor;
use Player;

const MOVEMENT_SCALE: f32 = 3.0;

pub struct PlayerSystem;
impl<'s> System<'s> for PlayerSystem {
	type SystemData = (
		WriteStorage<'s, Transform>,
		WriteStorage<'s, Player>,
		WriteStorage<'s, Actor>,
		ReadExpect<'s, GameMap>,
		Read<'s, InputHandler<String, String>>,
		Read<'s, Time>,
	);

	fn run(&mut self, (mut transforms, mut players, mut actors, game_map, input, time): Self::SystemData) {
		for (player, actor, transform) in (&mut players, &mut actors, &mut transforms).join() {
			let movement = input.axis_value("running");

			if let Some(mv_amount) = movement {
				let scaled_amount = MOVEMENT_SCALE * mv_amount as f32;

				transform.translation[0] += scaled_amount;
			}

			if let Some(is_jumping) = input.action_is_down("jumping") {
				if is_jumping && actor.standing {
					actor.v_velocity = -600.0;
					transform.translation[1] += 1.0;
				}
			}

			// debug!(target: "game_engine",
			// 	"Player coordinates: {} {}",
			// 	transform.translation[0], transform.translation[1]
			// );
		}
	}
}
