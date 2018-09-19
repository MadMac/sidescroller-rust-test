use amethyst::core::Transform;
use amethyst::ecs::{Join, System, WriteStorage};

use rand::prelude::*;

use Actor;
use Enemy;

pub struct EnemySystem;
impl<'s> System<'s> for EnemySystem {
	type SystemData = (
		WriteStorage<'s, Transform>,
		WriteStorage<'s, Enemy>,
		WriteStorage<'s, Actor>,
	);

	fn run(&mut self, (mut transforms, mut enemies, mut actors): Self::SystemData) {
		for (_, actor, transform) in (&mut enemies, &mut actors, &mut transforms).join() {

			let mut rng = thread_rng();

			if actor.standing {
				let x: f64 = rng.gen();

				if x < 0.05 {
					actor.v_velocity = -600.0;
					transform.translation[1] += 1.0;
				}
			}
		}
	}
}
