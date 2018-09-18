use amethyst::core::timing::Time;
use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, System, WriteStorage};

use Actor;

pub struct GravitySystem;

impl<'s> System<'s> for GravitySystem {
	type SystemData = (
		WriteStorage<'s, Actor>,
		WriteStorage<'s, Transform>,
		Read<'s, Time>,
	);

	fn run(&mut self, (mut actors, mut locals, time): Self::SystemData) {
		for (actor, local) in (&mut actors, &mut locals).join() {
			local.translation[1] -= actor.v_velocity * time.delta_seconds();
		}
	}
}
