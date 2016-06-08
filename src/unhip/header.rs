use std::fmt;
use super::super::util;
use super::super::ustr::Ustr;

pub struct HeaderDate {
	pub period:   Ustr,
	pub day_name: Ustr,
	pub month:    Ustr,
	pub day_num:  Ustr,
	pub time:     Ustr,
	pub year:     Ustr
}

impl fmt::Display for HeaderDate {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{1} {2} {3}, {5} {0} at {4}",
		self.period, self.day_name, self.month, self.day_num, self.time, self.year)
	}
}

pub struct HeaderData {
	pub version: u32,
	pub flags: Vec<u8>,
	pub date: HeaderDate,
	pub platform:  Ustr,
	pub langauge:  Ustr,
	pub format:    Ustr,
	pub game_name: Ustr
}

impl fmt::Display for HeaderData {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,
"HIP Version {0} created on {2}
Flags: {1:?}
Game is \"{3}\" for {4} {5} {6}
"
		, self.version, self.flags, self.date, self.game_name,
		self.platform, self.format, self.langauge)
	}
}

fn parse_version(data:&[u8]) -> u32 {
	util::from_u8array::<u32>(&data[0..4])
}

fn parse_date(data:&[u8]) -> HeaderDate {
	HeaderDate {
		period:   Ustr::from_u8(&data[0..4]),
		day_name: Ustr::from_u8(&data[4..8]),
		month:    Ustr::from_u8(&data[8..12]),
		day_num:  Ustr::from_u8(&data[12..15]),
		time:     Ustr::from_u8(&data[15..24]),
		year:     Ustr::from_u8(&data[24..28])
	}
}

pub fn parse_header(data: &[u8]) -> HeaderData {
	// Parse data
	if &data[0..4] != "PVER".as_bytes() {
		panic!("No PVER version!");
	}
	let version_len = util::from_u8array::<usize>(&data[4..8]);
	let version = parse_version(&data[8..8+version_len]);

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
	//TODO: Figure out wtf the pcnt data even is...???

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
	let mut platform_data:Vec<&[u8]> = Vec::new();
	let mut pval = 0u8;
	let mut platpos = 8usize;
	for i in 8..8+platform_len {
		let val = data[i];
		if val == 0 && pval == 0 {
			platform_data.push(&data[platpos..i-1]);
			platpos = i + 1;
		}
		pval = val;
	}
	// Platform information, can be GC, BX, or PS
	let platform = platform_data[0];
	// Language, for some reason this is actually 'Gamecube'
	let langauge = platform_data[1];
	// Format, probably NTSC
	let format = platform_data[2];
	// Actual name of game
	let game_name = platform_data[3];
	HeaderData {
		version: version,
		flags: flags,
		date: date,
		platform:  Ustr::from_u8(platform ),
		langauge:  Ustr::from_u8(langauge ),
		format:    Ustr::from_u8(format   ),
		game_name: Ustr::from_u8(game_name)
	}
}
