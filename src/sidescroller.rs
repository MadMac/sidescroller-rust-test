extern crate tiled;

use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::cgmath::{Matrix4, Vector3};
use amethyst::core::transform::{GlobalTransform, Transform};
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;
use amethyst::renderer::{
	Camera, Event, MaterialTextureSet, PngFormat, Projection, Sprite, SpriteRender, SpriteSheet,
	SpriteSheetHandle, Texture, TextureCoordinates, TextureHandle, VirtualKeyCode,
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

impl<'a, 'b> SimpleState<'a, 'b> for Sidescroller {
	fn on_start(&mut self, data: StateData<GameData>) {
		let world = data.world;

		let sprite_sheet_handle = load_sprite_sheet(world);

		initialise_map(world);

		world.register::<Player>();

		initialise_player(world, sprite_sheet_handle);

		initialise_camera(world);
	}

	fn handle_event(
		&mut self,
		_: StateData<GameData>,
		event: StateEvent<()>,
	) -> SimpleTrans<'a, 'b> {
		if let StateEvent::Window(event) = &event {
			if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
				Trans::Quit
			} else {
				Trans::None
			}
		} else {
			Trans::None
		}
	}

	fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans<'a, 'b> {
		data.data.update(&data.world);
		Trans::None
	}
}

fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
	let texture_handle = {
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

	let tex_coords = TextureCoordinates {
		left: 0.0,
		right: 1.0,
		bottom: 0.0,
		top: 1.0,
	};

	let player_sprite = Sprite {
		width: 32.0,
		height: 32.0,
		offsets: [32.0 / 2.0, 32.0 / 2.0],
		tex_coords,
	};

	let texture_id = 1;
	let mut material_texture_set = world.write_resource::<MaterialTextureSet>();
	material_texture_set.insert(texture_id, texture_handle);

	let sprite_sheet = SpriteSheet {
		texture_id,
		sprites: vec![player_sprite],
	};

	let sprite_sheet_handle = {
		let loader = world.read_resource::<Loader>();
		let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
		loader.load_from_data(sprite_sheet, (), &sprite_sheet_store)
	};

	sprite_sheet_handle
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

fn initialise_player(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {
	let mut player_transform = Transform::default();
	player_transform.translation = Vector3::new(16.0, 240.0, 0.0);

	let sprite_render = SpriteRender {
		sprite_sheet: sprite_sheet_handle.clone(),
		sprite_number: 0, // paddle is the first sprite in the sprite_sheet
		flip_horizontal: false,
		flip_vertical: false,
	};

	world
		.create_entity()
		.with(sprite_render)
		.with(Player::new())
		.with(GlobalTransform::default())
		.with(player_transform)
		.build();
}

fn load_tileset_sheet(world: &mut World) -> SpriteSheetHandle {
	let texture_handle = {
		let loader = world.read_resource::<Loader>();
		let texture_storage = world.read_resource::<AssetStorage<Texture>>();
		loader.load(
			"resources/maps/tilesets/map_textures.png",
			PngFormat,
			Default::default(),
			(),
			&texture_storage,
		)
	};

	let tex_coords = TextureCoordinates {
		left: 0.0,
		right: 0.5,
		bottom: 0.0,
		top: 0.5,
	};

	let tileset_sprite = Sprite {
		width: 32.0,
		height: 32.0,
		offsets: [32.0 / 2.0, 32.0 / 2.0],
		tex_coords,
	};

	let texture_id = 0;
	let mut material_texture_set = world.write_resource::<MaterialTextureSet>();
	material_texture_set.insert(texture_id, texture_handle);

	let sprite_sheet = SpriteSheet {
		texture_id,
		sprites: vec![tileset_sprite],
	};

	let sprite_sheet_handle = {
		let loader = world.read_resource::<Loader>();
		let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
		loader.load_from_data(sprite_sheet, (), &sprite_sheet_store)
	};

	sprite_sheet_handle
}

fn initialise_map(world: &mut World) {
	// TODO: Add to config
	let path_to_maps = Path::new("./resources/maps/");

	let map_file = File::open(path_to_maps.join("test1.tmx")).unwrap();
	let reader = BufReader::new(map_file);
	let map = parse(reader).unwrap();

	let tileset_path = &map
		.get_tileset_by_gid(1)
		.unwrap()
		.images
		.get(0)
		.unwrap()
		.source;

	let tileset_sheet_handle = load_tileset_sheet(world);

	for layer in 0..2 {
		let tiles = &map.layers.get(layer).unwrap().tiles;
		for row in 0..tiles.len() {
			for tile in 0..tiles[row].len() {
				let tile_style = (tiles[row][tile] as f32) - 1.0;

				if tile_style == -1.0 {
					continue;
				}

				println!("style: {}", tile_style);

				let mut tile_transform = Transform::default();
				tile_transform.translation =
					Vector3::new(33.0 * (tile as f32), 33.0 * (row as f32), 0.0);

				let tileset_render = SpriteRender {
					sprite_sheet: tileset_sheet_handle.clone(),
					sprite_number: 0, // paddle is the first sprite in the sprite_sheet
					flip_horizontal: false,
					flip_vertical: false,
				};

				world
					.create_entity()
					.with(tileset_render)
					.with(GlobalTransform::default())
					.with(tile_transform)
					.build();
			}
		}
	}

	println!("{:?}", map);
	println!("{:?}", tileset_path);
}
