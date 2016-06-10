use std::fmt;
use super::super::util;
use super::super::ustr::Ustr;

pub struct HeaderDate {
	pub timestamp: u32,
	pub date: Ustr
}

impl fmt::Display for HeaderDate {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.date)
	}
}

pub struct HeaderData {
	pub version: u32,
	pub version_compatible: u32,
	pub version_client_major: u16,
	pub version_client_minor: u16,
	pub flags: Vec<u8>,
	pub date: HeaderDate,
	pub platform:      Ustr,
	pub platform_name: Ustr,
	pub langauge:      Ustr,
	pub format:        Ustr,
	pub game_name:     Ustr,
	original_data:     Vec<u8>
}

impl fmt::Display for HeaderData {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,
"HIP Version {0:?} created on {2}
Flags: {1:?}
Game is {3:?} for {4:?} {5:?} {6:?} {7:?}
",
		self.version, self.flags, self.date, self.game_name,
		self.platform, self.format, self.langauge, self.platform_name)
	}
}

fn parse_date(data:&[u8]) -> HeaderDate {
	HeaderDate {
		timestamp:util::from_u8array::<u32>(&data[0..4]),
		date: Ustr::from_u8(&data[4..28])
	}
}

pub fn parse_header(data: &[u8]) -> HeaderData {
	let original = data.to_vec();
	// Parse data
	if &data[0..4] != "PVER".as_bytes() {
		panic!("No PVER version!");
	}
	let version_len = util::from_u8array::<usize>(&data[4..8]);
	let version = util::from_u8array::<u32>(&data[8..12]);
	let version_client_major = util::from_u8array::<u16>(&data[12..14]);
	let version_client_minor = util::from_u8array::<u16>(&data[14..16]);
	let version_compatible = util::from_u8array::<u32>(&data[16..20]);

	// Parse flags
	let data = &data[version_len+8..];
	if &data[0..4] != "PFLG".as_bytes() {
		panic!("No PFLG flags!");
	}
	let flags_len = util::from_u8array::<usize>(&data[4..8]);
	let flags = data[8..8+flags_len].to_vec();

	// Parse Count
	let data = &data[8+flags_len..];
	if &data[0..4] != "PCNT".as_bytes() {
		panic!("No PCNT count!");
	}
	let count_len = util::from_u8array::<usize>(&data[4..8]);
	//PCNT data: not necessary for header to load these
	// 0..4 is number of files
	// 4..8 is size of largest file
	// 8..12 is size of largest layer
	// 12..16 is size of largest virtual file

	// Parse Date
	let data = &data[8+count_len..];
	if &data[0..4] != "PCRT".as_bytes() {
		panic!("No PCRT creation date!");
	}
	let date_len = util::from_u8array::<usize>(&data[4..8]);
	let date = parse_date(&data[8..8+date_len]);

	// Parse modification Date
	let data = &data[8+date_len..];
	if &data[0..4] != "PMOD".as_bytes() {
		panic!("No PMOD modification date!");
	}
	let mod_date_len = util::from_u8array::<usize>(&data[4..8]);
	//ignore this its actually nothing

	// Parse platform ( the real stuff )
	let data = &data[8+mod_date_len..];
	if &data[0..4] != "PLAT".as_bytes() {
		panic!("No PLAT platform information!");
	}
	let platform_len = util::from_u8array::<usize>(&data[4..8]);
	let platform_data = data[8..8+platform_len]
		.split(|val| *val == 0)
		.filter(|val| !val.is_empty()).collect::<Vec<&[u8]>>();

	// Platform information, can be GC, BX, or PS
	let platform = platform_data[0];
	// Language, for some reason this is actually 'Gamecube'
	let platform_name = platform_data[1];
	// Format, probably NTSC
	let format = platform_data[2];
	// Language
	let langauge = platform_data[3];
	// Actual name of game
	let game_name = platform_data[4];
	HeaderData {
		version: version,
		version_compatible: version_compatible,
		version_client_major: version_client_major,
		version_client_minor: version_client_minor,
		flags: flags,
		date: date,
		platform:      Ustr::from_u8(platform     ),
		platform_name: Ustr::from_u8(platform_name),
		langauge:      Ustr::from_u8(langauge     ),
		format:        Ustr::from_u8(format       ),
		game_name:     Ustr::from_u8(game_name    ),
		original_data: original
	}
}
