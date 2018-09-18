use amethyst::core::Transform;
use amethyst::ecs::{Join, ReadExpect, System, WriteStorage};

use GameMap;
use Actor;

pub struct ActorSystem;
impl<'s> System<'s> for ActorSystem {
	type SystemData = (
		WriteStorage<'s, Transform>,
		WriteStorage<'s, Actor>,
		ReadExpect<'s, GameMap>,
	);

	fn run(&mut self, (mut transforms, mut actors, game_map): Self::SystemData) {
		for (actor, transform) in (&mut actors, &mut transforms).join() {

			// Avoid out of bounds from map
			if transform.translation[0] <= 1.0 {
				transform.translation[0] = 1.0;
			}

			if transform.translation[0] >= ((&game_map.width - 1) * 32) as f32 {
				transform.translation[0] = ((&game_map.width - 1) * 32) as f32;
			}

			// Calcultate tile coordinates for the actor

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
				&& actor.v_velocity >= 0.0
			{
				// debug!(target: "game_engine", "DOWN COLLIDE");
				actor.v_velocity = 0.0;
				actor.standing = true;
				transform.translation[1] = (&game_map.height * &game_map.tile_size) as f32
					- (tile_y * &game_map.tile_size) as f32;
			} else if (collision_layer.tiles[tile_y - 1][tile_x] == 1
				|| collision_layer.tiles[tile_y - 1][tile_x_right] == 1)
				&& (transform.translation[1] + tile_size_as_f32 / 2.0)
					< ((&game_map.height + tile_y) * &game_map.tile_size) as f32
				&& actor.v_velocity < 0.0
			{
				// Upwards
				// debug!(target: "game_engine", "UP COLLIDE");
				actor.v_velocity = 0.0;
				actor.standing = false;
				transform.translation[1] = (&game_map.height * &game_map.tile_size) as f32
					- (tile_y * &game_map.tile_size) as f32;
			} else {
				actor.standing = false;
			}
		}
	}
}
