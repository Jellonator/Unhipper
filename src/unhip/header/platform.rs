use ustr::Ustr;
use rustc_serialize::json::Json;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Platform {
	pub platform:      Ustr,
	pub platform_name: Ustr,
	pub language:      Ustr,
	pub format:        Ustr,
	pub game_name:     Ustr,
}

impl Platform {
	pub fn load(data:&[u8]) -> Platform {
		let platform_data = data
			.split(|val| *val == 0)
			.filter(|val| !val.is_empty()).collect::<Vec<&[u8]>>();

		// Platform information, can be GC, BX, or PS
		let platform = platform_data[0];
		// Language, for some reason this is actually 'Gamecube'
		let platform_name = platform_data[1];
		// Format, probably NTSC
		let format = platform_data[2];
		// Language
		let language = platform_data[3];
		// Actual name of game
		let game_name = platform_data[4];

		Platform {
			platform:      Ustr::from_u8(platform),
			platform_name: Ustr::from_u8(platform_name),
			language:      Ustr::from_u8(language),
			format:        Ustr::from_u8(format),
			game_name:     Ustr::from_u8(game_name)
		}
	}

	pub fn to_json(&self) -> Json {
		let mut datamap = BTreeMap::new();
		datamap.insert("platformshort".to_string(), Json::String(self.platform.to_string()));
		datamap.insert("platformlong".to_string(), Json::String(self.platform_name.to_string()));
		datamap.insert("language".to_string(), Json::String(self.language.to_string()));
		datamap.insert("format".to_string(), Json::String(self.format.to_string()));
		datamap.insert("name".to_string(), Json::String(self.game_name.to_string()));
		Json::Object(datamap)
	}

	pub fn from_json(data: &Json) -> Platform {
		Platform {
			platform:      Ustr::from_str(data.find("platformshort").unwrap().as_string().unwrap()),
			platform_name: Ustr::from_str(data.find("platformlong") .unwrap().as_string().unwrap()),
			language:      Ustr::from_str(data.find("language")     .unwrap().as_string().unwrap()),
			format:        Ustr::from_str(data.find("format")       .unwrap().as_string().unwrap()),
			game_name:     Ustr::from_str(data.find("name")         .unwrap().as_string().unwrap())
		}
	}
}
