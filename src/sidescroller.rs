extern crate tiled;

use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::cgmath::{Matrix4, Vector3};
use amethyst::core::transform::{GlobalTransform, Transform};
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;
use amethyst::renderer::{
	Camera, Event, PngFormat, Projection, Sprite, Texture, TextureHandle, VirtualKeyCode,
	WithSpriteRender,
};

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use self::tiled::parse;

pub struct Sidescroller;

pub const CAMERA_WIDTH: f32 = 640.0;
pub const CAMERA_HEIGHT: f32 = 480.0;

const SPRITESHEET_SIZE: (f32, f32) = (32.0, 32.0);

use Player;

impl<'a, 'b> State<GameData<'a, 'b>> for Sidescroller {
	fn on_start(&mut self, data: StateData<GameData>) {
		let world = data.world;

		let spritesheet = {
			let loader = world.read_resource::<Loader>();
			let texture_storage = world.read_resource::<AssetStorage<Texture>>();
			loader.load(
				"sprites/player.png",
				PngFormat,
				Default::default(),
				(),
				&texture_storage,
			)
		};

		initialise_map(world);

		world.register::<Player>();

		initialise_player(world, spritesheet);

		initialise_camera(world);
	}

	fn handle_event(&mut self, _: StateData<GameData>, event: Event) -> Trans<GameData<'a, 'b>> {
		if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
			Trans::Quit
		} else {
			Trans::None
		}
	}

	fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
		data.data.update(&data.world);
		Trans::None
	}
}

fn initialise_camera(world: &mut World) {
	world
		.create_entity()
		.with(Camera::from(Projection::orthographic(
			0.0,
			CAMERA_WIDTH,
			CAMERA_HEIGHT,
			0.0,
		))).with(GlobalTransform(
			Matrix4::from_translation(Vector3::new(0.0, 0.0, 1.0)).into(),
		)).build();
}

fn initialise_player(world: &mut World, spritesheet: TextureHandle) {
	let sprite = Sprite {
		left: 0.0,
		right: 32.0,
		top: 0.0,
		bottom: 32.0,
	};

	let mut player_transform = Transform::default();
	player_transform.translation = Vector3::new(16.0, 240.0, 0.0);

	world
		.create_entity()
		.with_sprite(&sprite, spritesheet.clone(), SPRITESHEET_SIZE)
		.expect("Failed to add sprite render on left paddle")
		.with(Player::new())
		.with(GlobalTransform::default())
		.with(player_transform)
		.build();
}

fn initialise_map(world: &mut World) {

	// TODO: Add to config
	let path_to_maps = Path::new("./resources/maps/");

	let mut map_file = File::open(path_to_maps.join("test1.tmx")).unwrap();
	let reader = BufReader::new(map_file);
	let map = parse(reader).unwrap();

	let tileset_path = &map.get_tileset_by_gid(1).unwrap().images.get(0).unwrap().source;

	let map_tileset = {
		let loader = world.read_resource::<Loader>();
		let texture_storage = world.read_resource::<AssetStorage<Texture>>();
		loader.load(
			path_to_maps.join(tileset_path).to_str().unwrap(),
			PngFormat,
			Default::default(),
			(),
			&texture_storage,
		)
	};

	println!("{:?}", map);
	println!("{:?}", tileset_path);
}
