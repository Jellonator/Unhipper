use super::super::util;

pub struct LayerData {

}

pub fn parse_layer(data: &[u8]) -> LayerData {
	let realdata = &data[0..data.len()-12];
	let metadata = &data[realdata.len()..];

	println!("Layer: ");
	for i in 0..realdata.len()/4 {
		let val = util::from_u8array::<u32>(&data[i*4..(i+1)*4]);
		print!("0x{:X}, ", val);
	}
	println!("");

	//println!("{:?}", String::from_utf8_lossy(&metadata[0..4]));
	LayerData {}
}
