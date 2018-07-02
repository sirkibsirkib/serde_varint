use std;
use std::thread;

use std::net::{
	TcpStream,
	TcpListener,
};


use super::*;


#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Data {
	#[serde(with = "super")]
	x: u32,
	#[serde(with = "super")]
	y: i16,
}

#[test]
fn main() {
	let [mut a, mut b] = tcp_pipe();
	let datum = Data{x: 9999, y: -50};
	bincode::serialize_into(&mut a, &datum).unwrap();
	assert_eq!(
		datum,
		bincode::deserialize_from::<_, Data>(&mut b).unwrap(),
	);
}

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