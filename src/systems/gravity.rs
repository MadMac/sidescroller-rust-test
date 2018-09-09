use amethyst::core::timing::Time;
use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};

use Player;

pub struct GravitySystem;

impl<'s> System<'s> for GravitySystem {
	type SystemData = (
		WriteStorage<'s, Player>,
		WriteStorage<'s, Transform>,
		Read<'s, Time>,
	);

	fn run(&mut self, (mut players, mut locals, time): Self::SystemData) {
		for (player, local) in (&mut players, &mut locals).join() {

			println!("y: {}", local.translation[1]);
			if local.translation[1] <= 240.0 && player.v_velocity >= 0.0 {
				player.v_velocity = 0.0;
				player.standing = true;
				println!("COLLIDING");
			} else {
				player.v_velocity += 500.0 * time.delta_seconds();
				player.standing = false; 
				println!("FREEFALL");
			}

			local.translation[1] -= player.v_velocity * time.delta_seconds();
			
		}

	}
}
