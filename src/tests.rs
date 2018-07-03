
use std::{
	thread,
	net::{
		TcpStream,
		TcpListener,
	},
};

///////////////////// LIB IMPORT ///////////////////////

use super::*;

/////////////////////// TESTS //////////////////


#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct AllVarInt {
	#[serde(with = "super")]
	x: u64,
	#[serde(with = "super")]
	y: i16,
}

#[test]
fn one() {
	let [mut a, mut b] = tcp_pipe();
	let datum = AllVarInt{x: 9999, y: -50};
	bincode::serialize_into(&mut a, &datum).unwrap();
	assert_eq!(
		datum,
		bincode::deserialize_from::<_, AllVarInt>(&b).unwrap(),
	);
}

#[test]
fn varying_byte_size() {
	let mut datum = AllVarInt { x: 121, y: -50 };
	let mut bytes = bincode::serialize(&datum).unwrap();
	let datum2:AllVarInt = bincode::deserialize_from(&bytes[..]).unwrap();
	assert_eq!(&datum, &datum2);
	assert_eq!(bytes.len(), 2);
	datum.x = 73427834389843;
	datum.y = -23621;
	bytes = bincode::serialize(&datum).unwrap();
	let datum2:AllVarInt = bincode::deserialize_from(&bytes[..]).unwrap();
	assert_eq!(&datum, &datum2);
	assert!(bytes.len() > 2);
}

//////////////////// AUX ////////////////////

fn tcp_pipe() -> [TcpStream; 2] {
	for port in 200..=std::u16::MAX {
		let addr = format!("127.0.0.1:{}", port);
		if let Ok(listener) = TcpListener::bind(&addr) {
			let handle = thread::spawn(move || {
				listener.accept().unwrap().0
			});
			return [
				TcpStream::connect(&addr).unwrap(),
				handle.join().unwrap(),
			];
		}
	}
	panic!("NO PORTS LEFT!")
}
