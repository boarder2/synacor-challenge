use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io;

pub fn load_bin(path: &str) -> Result<Vec<u16>, io::Error> {
	let mut result = Vec::new();
	let mut f = try!(File::open(path));
	let mut keep_reading = true;
	while keep_reading {
		match f.read_u16::<LittleEndian>() {
			Ok(entry) => result.push(entry),
			Err(_) => keep_reading = false, //TODO: Handle errors for real.
		}
	}
	Ok(result)
}

#[cfg(test)]
mod tests {
	use std::fs;
	use std::fs::File;
	use std::io::Write;

	#[test]
	fn test_load() {
		let expected = vec![1];
		let file_path = "./test_load.bin";
		let mut f = File::create(file_path).unwrap();
		let mut buf = [1 as u8, 0];
		f.write_all(&mut buf).unwrap();
		f.flush().unwrap();
		let mem = super::load_bin(file_path).unwrap();
		assert_eq!(expected, mem);
		fs::remove_file(file_path).unwrap();
	}
}