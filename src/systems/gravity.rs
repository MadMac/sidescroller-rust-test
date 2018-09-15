use amethyst::core::timing::Time;
use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadExpect, System, WriteStorage};

use GameMap;
use Player;

pub struct GravitySystem;

impl<'s> System<'s> for GravitySystem {
	type SystemData = (
		WriteStorage<'s, Player>,
		WriteStorage<'s, Transform>,
		ReadExpect<'s, GameMap>,
		Read<'s, Time>,
	);

	fn run(&mut self, (mut players, mut locals, game_map, time): Self::SystemData) {
		for (player, local) in (&mut players, &mut locals).join() {
			local.translation[1] -= player.v_velocity * time.delta_seconds();

			// Calcultate tile coordinates for the player

			let tile_size_as_f32 = &(game_map.tile_size as f32);

			let tile_x = (local.translation[0] / &(game_map.tile_size as f32)).floor() as usize;
			let mut tile_x_right =
				((local.translation[0] + tile_size_as_f32) / tile_size_as_f32).floor() as usize;
			let tile_y = (&game_map.height - 1)
				- ((local.translation[1] - tile_size_as_f32 / 2.0) / tile_size_as_f32).floor()
					as usize;
			let collision_layer = &game_map.layers[1];

			if tile_x_right > &game_map.width - 1 {
				tile_x_right = &game_map.width - 1;
			}

			// Collision system
			if (collision_layer.tiles[tile_y + 1][tile_x] == 1
				|| collision_layer.tiles[tile_y + 1][tile_x_right] == 1)
				&& (local.translation[1] - tile_size_as_f32 / 2.0)
					< ((&game_map.height - tile_y) * &game_map.tile_size) as f32
				&& player.v_velocity >= 0.0
			{
				player.v_velocity = 0.0;
				player.standing = true;
				local.translation[1] = (&game_map.height * &game_map.tile_size) as f32
					- (tile_y * &game_map.tile_size) as f32;
			} else {
				player.v_velocity += 1000.0 * time.delta_seconds();
				player.standing = false;
			}

			println!(
				"Player coordinates: {} {}",
				local.translation[0], local.translation[1]
			);
		}
	}
}
