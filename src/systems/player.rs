use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;
use amethyst::renderer::Camera;

use crate::sidescroller::Actor;
use crate::sidescroller::Player;

const MOVEMENT_SCALE: f32 = 3.0;

pub struct PlayerSystem;
impl<'s> System<'s> for PlayerSystem {
	type SystemData = (
		WriteStorage<'s, Transform>,
		WriteStorage<'s, Player>,
		WriteStorage<'s, Actor>,
		ReadStorage<'s, Camera>,
		Read<'s, InputHandler<String, String>>,
	);

	fn run(
		&mut self,
		(mut transforms, mut players, mut actors, camera, input): Self::SystemData,
	) {
		let movement = input.axis_value("running");
		let mut player_x = 0.0;
		let mut player_y = 0.0;

		for (_, actor, transform) in (&mut players, &mut actors, &mut transforms).join() {
			if let Some(mv_amount) = movement {
				let scaled_amount = MOVEMENT_SCALE * mv_amount as f32;

				transform.translate_x(scaled_amount);
				
			}

			if let Some(is_jumping) = input.action_is_down("jumping") {
				if is_jumping && actor.standing {
					actor.v_velocity = -600.0;
					transform.translate_y(1.0);
				}
			}

			player_x = transform.translation().x;
			player_y = transform.translation().y;

			// debug!(target: "game_engine",
			// 	"Player coordinates: {} {}",
			// 	transform.translation[0], transform.translation[1]
			// );
		}

		for (_, transform) in (&camera, &mut transforms).join() {

			// Place camera view so that the player is in the middle
			transform.set_x(player_x - (800.0/2.0));
			transform.set_y(player_y - (600.0/2.0));
			if transform.translation().x < 0.0 {
				transform.set_x(0.0);
			}

			if transform.translation().y < 0.0 {
				transform.set_y(0.0);
			}
		}
	}
}
