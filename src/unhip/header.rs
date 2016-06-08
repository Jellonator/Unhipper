use std::fmt;
use super::super::util;

pub struct HeaderDate {
	period: String,
	day_name: String,
	month: String,
	day_num: String,
	time: String,
	year: String
}

impl fmt::Display for HeaderDate {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{1} {2} {3}, {5} {0} at {4}",
		self.period, self.day_name, self.month, self.day_num, self.time, self.year)
	}
}

pub struct HeaderData {
	version: u32,
	flags: Vec<u8>,
	date: HeaderDate,
	platform: String,
	langauge: String,
	format: String,
	game_name: String
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
		period:   String::from_utf8_lossy(&data[0..4]).to_string(),
		day_name: String::from_utf8_lossy(&data[4..8]).to_string(),
		month:    String::from_utf8_lossy(&data[8..12]).to_string(),
		day_num:  String::from_utf8_lossy(&data[12..15]).to_string(),
		time:     String::from_utf8_lossy(&data[15..24]).to_string(),
		year:     String::from_utf8_lossy(&data[24..28]).to_string()
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
	let platform = platform_data[0];
	let langauge = platform_data[1];
	let format = platform_data[2];
	let game_name = platform_data[3];
	HeaderData {
		version: version,
		flags: flags,
		date: date,
		platform:  String::from_utf8_lossy(platform ).to_string(),
		langauge:  String::from_utf8_lossy(langauge ).to_string(),
		format:    String::from_utf8_lossy(format   ).to_string(),
		game_name: String::from_utf8_lossy(game_name).to_string()
	}
}
