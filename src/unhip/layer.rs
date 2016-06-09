use super::super::util;
use std::fmt;

pub struct LayerData {
	pub typenum: u32,
	pub uuids: Vec<u32>,
	pub original_data: Vec<u8>
}

#[derive(Copy, Clone)]
pub enum LayerType {
	Def = 0,       //default  ( 0 )
	Texture = 1,   //texture  ( 1 )
	BSP = 2,       //map      ( 2 )
	Model = 3,     //model    ( 3 )
	Animation = 4, //animation( 4 )
	// No type 5
	Sound = 6,     //sram     ( 6 )
	SoundInfo = 7, //sndtoc   ( 7 )
	Cutscene = 8,  //cutscene ( 8 )
	// No type 9
	JspInfo = 10,   //jspinfo  ( 10 )
	Unknown = 255,
}

impl LayerType {
	pub fn to_u32(&self) -> u32 {
		*self as u32
	}
}

impl fmt::Display for LayerType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}",
			match *self {
				LayerType::Def => "default",
				LayerType::Texture => "texture",
				LayerType::BSP => "bsp",
				LayerType::Model => "model",
				LayerType::Animation => "animation",
				LayerType::Sound => "sram",
				LayerType::SoundInfo => "sndtoc",
				LayerType::Cutscene => "cutscene",
				LayerType::JspInfo => "jspinfo",
				LayerType::Unknown => "unkown"
			}
		)
	}
}

pub fn get_layer_type(num: u32) -> LayerType {
	match num {
		0 => LayerType::Def,
		1 => LayerType::Texture,
		2 => LayerType::BSP,
		3 => LayerType::Model,
		4 => LayerType::Animation,
		6 => LayerType::Sound,
		7 => LayerType::SoundInfo,
		8 => LayerType::Cutscene,
		10=> LayerType::JspInfo,
		_ => LayerType::Unknown
	}
}

pub fn parse_layer(data: &[u8]) -> LayerData {
	let mut uuids = Vec::new();

	for i in 0..(data.len()-20)/4 {
		let pos = i * 4 + 8;
		uuids.push(util::from_u8array(&data[pos..4+pos]));
	}

	if uuids.len() != util::from_u8array::<usize>(&data[4..8]) {
		println!("ERR");
	}

	LayerData {
		original_data: data.to_vec(),
		typenum: util::from_u8array::<u32>(&data[0..4]),
		uuids: uuids
	}
}
