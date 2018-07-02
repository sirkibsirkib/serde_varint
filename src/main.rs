use std::io::{
	BufWriter,
	BufReader,
};
use itertools::Itertools;

extern crate bincode;

extern crate integer_encoding;
extern crate itertools;
extern crate serde;
#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize, Debug)]
struct Data {
	x: u64,
	#[serde(with = "serde_varint")]
	y: u32,
}


mod serde_varint;


const BUF_LEN: usize = 32;
fn main() {
	let datum = Data{x: 32, y: 36};

	let mut buf = [1u8; BUF_LEN];
	let bytes_sent;

	{
		let mut b = BufWriter::new(&mut buf[..]);
		bytes_sent = 0;
		let s = bincode::serialize_into(&mut b, &datum);
		println!("{:?}", s);
	}
	println!("{:?} // {:?}",
		buf[..bytes_sent].iter().format(","),
		buf[bytes_sent..].iter().format(","));
	{
		let mut reader = BufReader::new(& buf[..]);
		let x = bincode::deserialize_from::<_, Data>(&mut reader);
		println!("got {:?}", x);
	}
}
