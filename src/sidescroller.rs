extern crate tiled;

use game_data::CustomGameData;

use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::cgmath::Vector3;
use amethyst::core::transform::{GlobalTransform, Transform};
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;
use amethyst::renderer::{
	Camera, MaterialTextureSet, PngFormat, Projection, Sprite, SpriteRender, SpriteSheet,
	SpriteSheetHandle, Texture, TextureCoordinates, VirtualKeyCode,
};

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use self::tiled::parse;

pub struct Sidescroller;

pub const CAMERA_WIDTH: f32 = 800.0;
pub const CAMERA_HEIGHT: f32 = 600.0;

use Actor;
use Enemy;
use Player;

use GameMap;

use MapLayer;

use config::MapConfig;

impl<'a, 'b> State<CustomGameData<'a, 'b>, ()> for Sidescroller {
	fn on_start(&mut self, data: StateData<CustomGameData>) {
		let world = data.world;

		initialise_camera(world);

		let sprite_sheet_handle = load_sprite_sheet(world);
		let enemy_sprite_sheet_handle = load_enemy_sprite_sheet(world);

		initialise_map(world);

		world.register::<Player>();
		world.register::<Actor>();

		initialise_player(world, sprite_sheet_handle);
		initialise_actor(world, enemy_sprite_sheet_handle);
	}

	fn handle_event(
		&mut self,
		_: StateData<CustomGameData>,
		event: StateEvent<()>,
	) -> Trans<CustomGameData<'a, 'b>, ()> {
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

	fn update(&mut self, data: StateData<CustomGameData>) -> Trans<CustomGameData<'a, 'b>, ()> {
		data.data.update(&data.world, true);
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

// TODO: Better way to handle multiple spritesheet loading
fn load_enemy_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
	let texture_handle = {
		let loader = world.read_resource::<Loader>();
		let texture_storage = world.read_resource::<AssetStorage<Texture>>();
		loader.load(
			"sprites/enemy.png",
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

	let enemy_sprite = Sprite {
		width: 32.0,
		height: 32.0,
		offsets: [32.0 / 2.0, 32.0 / 2.0],
		tex_coords,
	};

	let texture_id = 2;
	let mut material_texture_set = world.write_resource::<MaterialTextureSet>();
	material_texture_set.insert(texture_id, texture_handle);

	let sprite_sheet = SpriteSheet {
		texture_id,
		sprites: vec![enemy_sprite],
	};

	let sprite_sheet_handle = {
		let loader = world.read_resource::<Loader>();
		let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
		loader.load_from_data(sprite_sheet, (), &sprite_sheet_store)
	};

	sprite_sheet_handle
}

fn initialise_camera(world: &mut World) {
	let mut camera_transform = Transform::default();
	camera_transform.translation = Vector3::new(0.0, 0.0, 1.0);

	world
		.create_entity()
		.with(Camera::from(Projection::orthographic(
			0.0,
			CAMERA_WIDTH,
			CAMERA_HEIGHT,
			0.0,
		)))
		.with(GlobalTransform::default())
		.with(camera_transform)
		.build();
}

fn initialise_player(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {
	let mut player_transform = Transform::default();
	player_transform.translation = Vector3::new(32.0, 300.0, 0.1);

	let sprite_render = SpriteRender {
		sprite_sheet: sprite_sheet_handle.clone(),
		sprite_number: 0, // paddle is the first sprite in the sprite_sheet
		flip_horizontal: false,
		flip_vertical: false,
	};

	world
		.create_entity()
		.with(sprite_render)
		.with(Actor::new(32.0, 300.0))
		.with(Player::new())
		.with(GlobalTransform::default())
		.with(player_transform)
		.build();
}

fn initialise_actor(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {
	let game_map = world.read_resource::<GameMap>().clone();

	let sprite_render = SpriteRender {
		sprite_sheet: sprite_sheet_handle.clone(),
		sprite_number: 0, // paddle is the first sprite in the sprite_sheet
		flip_horizontal: false,
		flip_vertical: false,
	};

	let map_height_in_pixels = (game_map.height * game_map.tile_size) as f32;

	for actor in &game_map.actors {
		let mut actor_transform = Transform::default();
		actor_transform.translation =
			Vector3::new(actor.spawn.0, map_height_in_pixels - actor.spawn.1, 0.1);

		debug!(target: "game_engine", "Spawn actor: {:?}", actor);

		world
			.create_entity()
			.with(sprite_render.clone())
			.with(actor.clone())
			.with(Enemy::new())
			.with(GlobalTransform::default())
			.with(actor_transform)
			.build();
	}
}

fn load_tileset_sheet(
	world: &mut World,
	tileset_path: &String,
	tile_size: f32,
	tile_sheet_width: f32,
) -> SpriteSheetHandle {
	let texture_handle = {
		let loader = world.read_resource::<Loader>();
		let texture_storage = world.read_resource::<AssetStorage<Texture>>();
		loader.load(
			tileset_path.clone().as_str(),
			PngFormat,
			Default::default(),
			(),
			&texture_storage,
		)
	};

	// TODO: Add support for rows in the tile sheet
	let width_ratio = tile_size / tile_sheet_width;
	let tile_amount = tile_sheet_width / tile_size;

	let mut sprite_vec = Vec::new();

	debug!(target: "game_engine", "width_ratio: {:?}", width_ratio);
	debug!(target: "game_engine", "tile_size: {:?}", tile_size);
	debug!(target: "game_engine", "tile_sheet_width: {:?}", tile_sheet_width);

	for i in 0..tile_amount as i32 {
		let tile_coords = TextureCoordinates {
			left: (i as f32) * width_ratio,
			right: (i as f32) * width_ratio + width_ratio,
			bottom: 0.0,
			top: 0.5,
		};
		debug!(target: "game_engine", "tile: {:?}", tile_coords);
		let tile_sprite = Sprite {
			width: tile_size,
			height: tile_size,
			offsets: [tile_size / 2.0, tile_size / 2.0],
			tex_coords: tile_coords,
		};

		sprite_vec.push(tile_sprite.clone());
	}

	let texture_id = 0;
	let mut material_texture_set = world.write_resource::<MaterialTextureSet>();
	material_texture_set.insert(texture_id, texture_handle);

	let sprite_sheet = SpriteSheet {
		texture_id,
		sprites: sprite_vec,
	};

	let sprite_sheet_handle = {
		let loader = world.read_resource::<Loader>();
		let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
		loader.load_from_data(sprite_sheet, (), &sprite_sheet_store)
	};

	sprite_sheet_handle
}

fn initialise_map(world: &mut World) {
	let path_to_maps = PathBuf::from(&world.read_resource::<MapConfig>().map_path);

	debug!(target: "game_engine", "Maps folder: {:?}", path_to_maps);

	let map_file = File::open(path_to_maps.join("test1.tmx")).unwrap();
	let reader = BufReader::new(map_file);
	let map = parse(reader).unwrap();

	debug!(target: "game_engine", "{:?}", map);

	let tileset_path = &map
		.get_tileset_by_gid(1)
		.unwrap()
		.images
		.get(0)
		.unwrap()
		.source;

	debug!(target: "game_engine", "Tileset: {:?}", &map.get_tileset_by_gid(1).unwrap());

	let tile_size = &map.get_tileset_by_gid(1).unwrap().tile_width;
	let tile_sheet_width = &map
		.get_tileset_by_gid(1)
		.unwrap()
		.images
		.get(0)
		.unwrap()
		.width;

	let tileset_path = path_to_maps.join(tileset_path);

	let tileset_sheet_handle = load_tileset_sheet(
		world,
		&tileset_path.clone().into_os_string().into_string().unwrap(),
		tile_size.clone() as f32,
		tile_sheet_width.clone() as f32,
	);

	let map_height = &(map.height as usize);
	let map_width = &(map.width as usize);

	let mut game_map = GameMap::new(map_width.clone(), map_height.clone());

	for layer in 0..2 {
		let tiles = &map.layers.get(layer).unwrap().tiles;

		let tile_layer = MapLayer::new(tiles.clone());

		game_map.push(tile_layer);

		for row in 0..tiles.len() {
			for tile in 0..tiles[row].len() {
				let tile_style = (tiles[row][tile] as i32) - 1;

				if tile_style == -1 {
					continue;
				}

				let mut tile_transform = Transform::default();
				tile_transform.translation = Vector3::new(
					32.0 * (tile as f32),
					32.0 * ((map_height - row) as f32),
					0.0,
				);

				let tileset_render = SpriteRender {
					sprite_sheet: tileset_sheet_handle.clone(),
					sprite_number: (tile_style as usize),
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

	let map_objects = &map.object_groups.get(0).unwrap().objects;

	// TODO: Multiple object layers
	for object in map_objects {
		let enemy = Actor::new(object.x, object.y);
		debug!(target: "game_engine", "{:?}", enemy);
		game_map.add_actor(enemy);
	}

	debug!(target: "game_engine", "{:?}", game_map);
	world.add_resource(game_map);
}
