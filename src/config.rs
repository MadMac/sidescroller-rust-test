use std::path::Path;
use amethyst::config::Config;

#[derive(Debug, Deserialize, Serialize)]
pub struct MapConfig {
	pub map_path: String,
}

impl Default for MapConfig {
	fn default() -> Self {
		MapConfig {
			map_path: String::from("/"),
		}
	}
}


#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GeneralConfig {
	
    pub map: MapConfig,

}
