use amethyst::core::timing::Time;
use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadExpect, System, WriteStorage};
use amethyst::input::InputHandler;

use GameMap;
use Player;

const MOVEMENT_SCALE: f32 = 3.0;

pub struct PlayerSystem;
impl<'s> System<'s> for PlayerSystem {
	type SystemData = (
		WriteStorage<'s, Transform>,
		WriteStorage<'s, Player>,
		ReadExpect<'s, GameMap>,
		Read<'s, InputHandler<String, String>>,
		Read<'s, Time>,
	);

	fn run(&mut self, (mut transforms, mut players, game_map, input, time): Self::SystemData) {
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

			if transform.translation[0] >= ((&game_map.width - 1) * 32) as f32 {
				transform.translation[0] = ((&game_map.width - 1) * 32) as f32;
			}

			// TODO: Generalize coliision for every actor
			// TODO: Refactor?
			// Calcultate tile coordinates for the player

			let tile_size_as_f32 = &(game_map.tile_size as f32);

			let mut tile_x = (transform.translation[0] / &(game_map.tile_size as f32)).floor() as usize;
			let mut tile_x_right = (((transform.translation[0]) + tile_size_as_f32)
				/ tile_size_as_f32)
				.floor() as usize;
			let tile_y = (&game_map.height - 1)
				- ((transform.translation[1] - tile_size_as_f32 / 2.0) / tile_size_as_f32).floor()
					as usize;

			let collision_layer = &game_map.layers[1];

			if tile_x_right > &game_map.width - 1 {
				tile_x_right = &game_map.width - 1;
			}

			// Collision system
			if collision_layer.tiles[tile_y][tile_x_right] == 1 {
				transform.translation[0] = ((tile_x_right-1) * &(game_map.tile_size)) as f32;
				// debug!(target: "game_engine", "RIGHT COLLIDE");
			}

			if collision_layer.tiles[tile_y][tile_x] == 1 {
				transform.translation[0] = ((tile_x+1) * &(game_map.tile_size)) as f32;
				// debug!(target: "game_engine", "LEFT COLLIDE");
			}

			tile_x = ((transform.translation[0]+1.0) / &(game_map.tile_size as f32)).floor() as usize;
			tile_x_right = (((transform.translation[0]) + tile_size_as_f32- 1.0)
				/ tile_size_as_f32)
				.floor() as usize;

			// Downward
			if (collision_layer.tiles[tile_y + 1][tile_x] == 1
				|| collision_layer.tiles[tile_y + 1][tile_x_right] == 1)
				&& (transform.translation[1] - tile_size_as_f32 / 2.0)
					< ((&game_map.height - tile_y) * &game_map.tile_size) as f32
				&& player.v_velocity >= 0.0
			{
				// debug!(target: "game_engine", "DOWN COLLIDE");
				player.v_velocity = 0.0;
				player.standing = true;
				transform.translation[1] = (&game_map.height * &game_map.tile_size) as f32
					- (tile_y * &game_map.tile_size) as f32;
			} else if (collision_layer.tiles[tile_y - 1][tile_x] == 1
				|| collision_layer.tiles[tile_y - 1][tile_x_right] == 1)
				&& (transform.translation[1] + tile_size_as_f32 / 2.0)
					< ((&game_map.height + tile_y) * &game_map.tile_size) as f32
				&& player.v_velocity < 0.0
			{
				// Upwards
				// debug!(target: "game_engine", "UP COLLIDE");
				player.v_velocity = 0.0;
				player.standing = false;
				transform.translation[1] = (&game_map.height * &game_map.tile_size) as f32
					- (tile_y * &game_map.tile_size) as f32;
			} else {
				player.v_velocity += 1000.0 * time.delta_seconds();
				player.standing = false;
			}

			// debug!(target: "game_engine",
			// 	"Player coordinates: {} {}",
			// 	transform.translation[0], transform.translation[1]
			// );
			// debug!(target: "game_engine","{} {}", tile_y, tile_x);
		}
	}
}
