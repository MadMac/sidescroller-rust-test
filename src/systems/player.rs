use amethyst::core::timing::Time;
use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadExpect, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;
use amethyst::renderer::Camera;

use Actor;
use GameMap;
use Player;

const MOVEMENT_SCALE: f32 = 3.0;

pub struct PlayerSystem;
impl<'s> System<'s> for PlayerSystem {
	type SystemData = (
		WriteStorage<'s, Transform>,
		WriteStorage<'s, Player>,
		WriteStorage<'s, Actor>,
		ReadStorage<'s, Camera>,
		ReadExpect<'s, GameMap>,
		Read<'s, InputHandler<String, String>>,
		Read<'s, Time>,
	);

	fn run(
		&mut self,
		(mut transforms, mut players, mut actors, camera, game_map, input, time): Self::SystemData,
	) {
		let movement = input.axis_value("running");
		let mut player_x = 0.0;
		let mut player_y = 0.0;

		for (player, actor, transform) in (&mut players, &mut actors, &mut transforms).join() {
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

			player_x = transform.translation[0];
			player_y = transform.translation[1];

			// debug!(target: "game_engine",
			// 	"Player coordinates: {} {}",
			// 	transform.translation[0], transform.translation[1]
			// );
		}

		for (_, transform) in (&camera, &mut transforms).join() {

			// Place camera view so that the player is in the middle
			transform.translation[0] = player_x - (800.0/2.0);
			transform.translation[1] = player_y - (600.0/2.0);
			if transform.translation[0] < 0.0 {
				transform.translation[0] = 0.0;
			}

			if transform.translation[1] < 0.0 {
				transform.translation[1] = 0.0;
			}
		}
	}
}
