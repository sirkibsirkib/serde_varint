
use std::{
	convert,
	thread,
	net::{
		TcpStream,
		TcpListener,
	},
};


///////////////////// LIB IMPORT ///////////////////////

use super::*;

/////////////////////// TEST STRUCTS //////////////////


#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
struct AllVarInt {
	#[serde(with = "super")]
	x: u64,
	#[serde(with = "super")]
	y: i16,
}

impl convert::Into<NoVarInt> for AllVarInt {
	fn into(self) -> NoVarInt {
		NoVarInt {x:self.x, y:self.y}
	}
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
struct NoVarInt {
	x: u64,
	y: i16,
}

impl convert::Into<AllVarInt> for NoVarInt {
	fn into(self) -> AllVarInt {
		AllVarInt {x:self.x, y:self.y}
	}
}

///////////////////////////// TESTS ////////////////

#[test]
fn one() {
	let [mut a, b] = tcp_pipe();
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

#[test]
fn comparing() {
	let smaller = AllVarInt {x: 263, y: -2 };
	let larger: NoVarInt = smaller.into(); 

	let smaller_bytes = bincode::serialize(&smaller).unwrap();
	let larger_bytes  = bincode::serialize(&larger).unwrap();

	assert!(smaller_bytes.len() < larger_bytes.len());
}

#[test]
fn json() {
	let a = AllVarInt {x:7234123, y:-2734};
	let json = serde_json::to_string(&a).unwrap();
	let b: AllVarInt = serde_json::from_str(&json).unwrap();
	assert_eq!(a, b);

}

#[test]
fn incomplete() {
	let a = AllVarInt {x: 212363, y: -232 };
	let mut bytes = bincode::serialize(&a).unwrap();
	bytes.pop();
	let res = bincode::deserialize_from::<_,AllVarInt>(&bytes[..]);
	res.unwrap_err();
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
