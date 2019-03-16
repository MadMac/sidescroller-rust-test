extern crate tiled;

use crate::game_data::CustomGameData;

use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::nalgebra::Vector3;
use amethyst::core::transform::{GlobalTransform, Transform};
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;
use amethyst::renderer::{
	Camera, PngFormat, Projection, Sprite, SpriteRender, SpriteSheet,
	SpriteSheetHandle, Texture, TextureCoordinates, VirtualKeyCode, TextureMetadata,
	SpriteSheetFormat
};
use amethyst::ecs::prelude::{Component, DenseVecStorage};

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use self::tiled::parse;

pub struct Sidescroller;
pub struct Menu;

pub const CAMERA_WIDTH: f32 = 800.0;
pub const CAMERA_HEIGHT: f32 = 600.0;

use crate::config::MapConfig;

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for Menu {
	fn on_start(&mut self, _: StateData<CustomGameData>) {
		debug!(target: "game_engine", "GAME PAUSED!");
	}

	fn handle_event(
		&mut self,
		_: StateData<CustomGameData>,
		event: StateEvent,
	) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
		if let StateEvent::Window(event) = &event {
			if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
				Trans::Quit
			} else if is_key_down(&event, VirtualKeyCode::P) {
				debug!(target: "game_engine", "GAME UNPAUSED!");
				Trans::Pop
			} else {
				Trans::None
			}
		} else {
			Trans::None
		}
	}

	fn update(&mut self, data: StateData<CustomGameData>) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
		data.data.update(&data.world, false); // false to say we should not dispatch running
		Trans::None
	}
}

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for Sidescroller {
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
		event: StateEvent,
	) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
		if let StateEvent::Window(event) = &event {
			if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
				Trans::Quit
			} else if is_key_down(&event, VirtualKeyCode::P) {
				Trans::Push(Box::new(Menu))
			} else {
				Trans::None
			}
		} else {
			Trans::None
		}
	}

	fn update(&mut self, data: StateData<CustomGameData>) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
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
			TextureMetadata::srgb_scale(),
			(),
			&texture_storage,
		)
	};

	let loader = world.read_resource::<Loader>();
	let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();

	loader.load(
		"sprites/player_spritesheet.ron",
		SpriteSheetFormat,
		texture_handle, 
		(),
		&sprite_sheet_store,
	)
	/*
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
	*/
}

// TODO: Better way to handle multiple spritesheet loading
fn load_enemy_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
	let texture_handle = {
		let loader = world.read_resource::<Loader>();
		let texture_storage = world.read_resource::<AssetStorage<Texture>>();
		loader.load(
			"sprites/enemy.png",
			PngFormat,
			TextureMetadata::srgb_scale(),
			(),
			&texture_storage,
		)
	};

	let loader = world.read_resource::<Loader>();
	let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();

	loader.load(
		"sprites/enemy_spritesheet.ron",
		SpriteSheetFormat,
		texture_handle, 
		(),
		&sprite_sheet_store,
	)

	/*
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
	*/
}

fn initialise_camera(world: &mut World) {
	let mut camera_transform = Transform::default();
	camera_transform.set_xyz(0.0, 0.0, 1.0);
	// camera_transform.translation = Vector3::new(0.0, 0.0, 1.0);

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
	let game_map = world.read_resource::<GameMap>().clone();
	let player_data = game_map.get_player();

	let map_height_in_pixels = (game_map.height * game_map.tile_size) as f32;

	let mut player_transform = Transform::default();
	player_transform.set_xyz(player_data.spawn.0, map_height_in_pixels - player_data.spawn.1, 0.1);

	let sprite_render = SpriteRender {
		sprite_sheet: sprite_sheet_handle.clone(),
		sprite_number: 0
	};

	world
		.create_entity()
		.with(sprite_render)
		.with(Actor::new(32.0, 300.0, ActorType::PLAYER))
		.with(Player::new())
		.with(GlobalTransform::default())
		.with(player_transform)
		.build();
}

fn initialise_actor(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {
	let game_map = world.read_resource::<GameMap>().clone();

	let sprite_render = SpriteRender {
		sprite_sheet: sprite_sheet_handle.clone(),
		sprite_number: 0
	};

	let map_height_in_pixels = (game_map.height * game_map.tile_size) as f32;

	for actor in &game_map.actors {
		if actor.actor_type == ActorType::ENEMY {
			let mut actor_transform = Transform::default();
			actor_transform.set_xyz(actor.spawn.0, map_height_in_pixels - actor.spawn.1, 0.1);

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
			TextureMetadata::srgb_scale(),
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


	let loader = world.read_resource::<Loader>();
	let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();

	loader.load(
		"resources/maps/tilesets/map_textures.ron",
		SpriteSheetFormat,
		texture_handle, 
		(),
		&sprite_sheet_store,
	)


	// let texture_id = 0;
	// let mut material_texture_set = world.write_resource::<MaterialTextureSet>();
	// material_texture_set.insert(texture_id, texture_handle);

	// let sprite_sheet = SpriteSheet {
	// 	texture_id,
	// 	sprites: sprite_vec,
	// };

	// let sprite_sheet_handle = {
	// 	let loader = world.read_resource::<Loader>();
	// 	let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
	// 	loader.load_from_data(sprite_sheet, (), &sprite_sheet_store)
	// };

	// sprite_sheet_handle
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
				tile_transform.set_xyz(
					32.0 * (tile as f32),
					32.0 * ((map_height - row) as f32),
					0.0,
				);

				let tileset_render = SpriteRender {
					sprite_sheet: tileset_sheet_handle.clone(),
					sprite_number: (tile_style as usize)
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

	for object_group in &map.object_groups {
		let map_objects = &object_group.objects;

		for object in map_objects {

			let mut actor_type = ActorType::NOTYPE;

			if object.obj_type == "enemy" {
				actor_type = ActorType::ENEMY;
			} else if object.obj_type == "player" {
				actor_type = ActorType::PLAYER;
			}

			let enemy = Actor::new(object.x, object.y, actor_type);
			debug!(target: "game_engine", "{:?}", enemy);
			game_map.add_actor(enemy);
		}
	}
	
	debug!(target: "game_engine", "{:?}", game_map);

	debug!(target: "game_engine", "PLAYER DATA: {:?}", game_map.get_player());
	world.add_resource(game_map);
}


#[derive(Debug, Clone, PartialEq)]
pub enum ActorType {
    ENEMY,
    PLAYER,
    NOTYPE,
}

#[derive(Debug, Clone)]
pub struct Actor {
    pub width: f32,
    pub height: f32,
    pub v_velocity: f32,
    pub standing: bool,
    pub spawn: (f32, f32),
    pub actor_type: ActorType,
}

impl Actor {
    fn new(x: f32, y: f32, actor_type: ActorType) -> Actor {
        Actor {
            width: 32.0,
            height: 32.0,
            v_velocity: 5.0,
            standing: false,
            spawn: (x, y),
            actor_type: actor_type,
        }
    }
}

pub struct Player {}

impl Player {
    fn new() -> Player {
        Player {}
    }
}

#[derive(Debug, Clone)]
pub struct GameMap {
    pub width: usize,
    pub height: usize,
    pub tile_size: usize,
    pub layers: Vec<MapLayer>,
    pub actors: Vec<Actor>,
}

impl GameMap {
    fn new(width: usize, height: usize) -> GameMap {
        GameMap {
            width: width,
            height: height,
            layers: Vec::new(),
            tile_size: 32,
            actors: Vec::new(),
        }
    }

    fn push(&mut self, map_layer: MapLayer) {
        self.layers.push(map_layer);
    }

    fn add_actor(&mut self, actor: Actor) {
        self.actors.push(actor);
    }

    fn get_player(&self) -> &Actor {
        for actor in &self.actors {
            if actor.actor_type == ActorType::PLAYER {
                return actor;
            }
        }

        panic!("Couldn't find player data in the gamemap data");
    }
}

#[derive(Debug, Clone)]
pub struct MapLayer {
    pub tiles: Vec<Vec<u32>>,
}

impl MapLayer {
    fn new(tiles: Vec<Vec<u32>>) -> MapLayer {
        MapLayer { tiles: tiles }
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

impl Component for Actor {
    type Storage = DenseVecStorage<Self>;
}

pub struct Enemy {}

impl Enemy {
    fn new() -> Enemy {
        Enemy {}
    }
}

impl Component for Enemy {
    type Storage = DenseVecStorage<Self>;
}
