use ustr::Ustr;
use rustc_serialize::json::Json;
use std::collections::BTreeMap;
use util;

/// Structure representing the platform data
#[derive(Debug)]
pub struct Platform {
	/// Short name of the platform (GC)
	pub platform:      Ustr,
	/// Long name of the platform (Gamecube)
	pub platform_name: Ustr,
	/// Language of the game (EN)
	pub language:      Ustr,
	/// Format (NTSC)
	pub format:        Ustr,
	/// Name of the game
	pub game_name:     Ustr,
}

impl Platform {
	/// Create a new Platform object given some data
	/// Takes format "{platform}\x00{platform_name}\x00{format}\x00{language}\x00{game_name}"
	pub fn load(data:&[u8]) -> Platform {
		let platform_data = data
			.split(|val| *val == 0)
			.filter(|val| !val.is_empty()).collect::<Vec<&[u8]>>();
		let platform = platform_data[0];
		let platform_name = platform_data[1];
		let format = platform_data[2];
		let language = platform_data[3];
		let game_name = platform_data[4];

		Platform {
			platform:      Ustr::from_u8(platform),
			platform_name: Ustr::from_u8(platform_name),
			language:      Ustr::from_u8(language),
			format:        Ustr::from_u8(format),
			game_name:     Ustr::from_u8(game_name)
		}
	}

	/// Create a Json object from Platform data
	/// Json object has the following format:
	/// ```json
	/// {
	///     "platformshort": string,
	///     "platformlong": string,
	///     "language": string,
	///     "format": string,
	///     "name": string
	/// }
	/// ```
	pub fn to_json(&self) -> Json {
		let mut datamap = BTreeMap::new();
		datamap.insert("platformshort".to_string(), Json::String(self.platform.to_string()));
		datamap.insert("platformlong".to_string(), Json::String(self.platform_name.to_string()));
		datamap.insert("language".to_string(), Json::String(self.language.to_string()));
		datamap.insert("format".to_string(), Json::String(self.format.to_string()));
		datamap.insert("name".to_string(), Json::String(self.game_name.to_string()));
		Json::Object(datamap)
	}

	/// Create a Platform object from a Json object
	pub fn from_json(data: &Json) -> Platform {
		Platform {
			platform:      Ustr::from_str(data.find("platformshort").unwrap().as_string().unwrap()),
			platform_name: Ustr::from_str(data.find("platformlong") .unwrap().as_string().unwrap()),
			language:      Ustr::from_str(data.find("language")     .unwrap().as_string().unwrap()),
			format:        Ustr::from_str(data.find("format")       .unwrap().as_string().unwrap()),
			game_name:     Ustr::from_str(data.find("name")         .unwrap().as_string().unwrap())
		}
	}

	/// Create a Vec<u8> from a Platform object
	pub fn to_vec(&self) -> Vec<u8> {
		let mut data:Vec<u8> = Vec::new();
		data.extend_from_slice(&self.platform.data);
		data.append(&mut vec![0;2]);
		data.extend_from_slice(&self.platform_name.data);
		data.append(&mut vec![0;2]);
		data.extend_from_slice(&self.format.data);
		data.append(&mut vec![0;2]);
		data.extend_from_slice(&self.language.data);
		data.append(&mut vec![0;1]);//Only one null here!!!
		data.extend_from_slice(&self.game_name.data);
		data.append(&mut vec![0;2]);

		util::create_chunk(data, b"PLAT")
	}
}
