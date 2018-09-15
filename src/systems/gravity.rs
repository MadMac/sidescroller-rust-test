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
			let tile_x = (local.translation[0] / 32.0).floor() as usize;
			let mut tile_x_right = ((local.translation[0]+32.0) / 32.0).floor() as usize;
			let tile_y = 19 - ((local.translation[1] - 16.0) / 32.0).floor() as usize;
			let collision_layer = &game_map.layers[1];

			if tile_x_right > 19 {
				tile_x_right = 19;
			}

			// Collision system
			if (collision_layer.tiles[tile_y + 1][tile_x] == 1 || collision_layer.tiles[tile_y + 1][tile_x_right] == 1)
				&& (local.translation[1] - 16.0) < ((20-tile_y) * 32) as f32
				&& player.v_velocity >= 0.0
			{
				player.v_velocity = 0.0;
				player.standing = true;
				local.translation[1] = (20 * 32) as f32 - (tile_y * 32) as f32;
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
