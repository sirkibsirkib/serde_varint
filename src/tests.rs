
use std::{
	convert,
	thread,
	time,
	io,
	net::{
		TcpStream,
		TcpListener,
	},
};

macro_rules! bench {
	($reps:expr, $work:expr) => {{
		let start = time::Instant::now();
		for _ in 0..$reps {
			($work)();
		}
		start.elapsed()
	}}
}

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

macro_rules! mutate {
	($struct:ident) => {{
		$struct.x += 2390;
		$struct.y = -$struct.y + if $struct.y < 0 {-32} else {32};
	}}
}

#[test]
fn benchmark() {
	let mut sink = io::sink();
	let num_runs = 200_000;

	let mut t = AllVarInt {x:0, y:0};
	let time_0 = time::Instant::now();
	for _ in 0..num_runs {
		mutate!(t);
		bincode::serialize_into(&mut sink, &t).unwrap();
	}
	let took_0 = time_0.elapsed();

	let mut t: NoVarInt = t.into();
	let time_1 = time::Instant::now();
	for _ in 0..num_runs {
		mutate!(t);
		bincode::serialize_into(&mut sink, &t).unwrap();
	}
	let took_1 = time_1.elapsed();
	let prop = dur_proportion(took_0, took_1);
	println!("PORP {:?}", prop);
	assert!(prop > 1.0);
	assert!(prop < 8.0);
}

#[test]
fn bench_ser_large() {
	let mut buf = [0u8; 64];
	let num_runs = 1_000_000;
	let varint = AllVarInt {x:9999999927364762743, y:-17722};
	let fixint: NoVarInt = varint.into();

	let (t_varint, t_fixint) = (
		bench!(num_runs, || {
			bincode::serialize_into(&mut buf[..], &varint).unwrap();
		}),
		bench!(num_runs, || {
			bincode::serialize_into(&mut buf[..], &fixint).unwrap();
		}),
	);
	println!("ser large {:?} {:?} {}", t_varint, t_fixint, dur_proportion(t_varint, t_fixint));
}

#[test]
fn bench_ser_small() {
	let mut buf = [0u8; 64];
	let num_runs = 1_000_000;
	let varint = AllVarInt {x:21, y:7};
	let fixint: NoVarInt = varint.into();

	let (t_varint, t_fixint) = (
		bench!(num_runs, || {
			bincode::serialize_into(&mut buf[..], &varint).unwrap();
		}),
		bench!(num_runs, || {
			bincode::serialize_into(&mut buf[..], &fixint).unwrap();
		}),
	);
	println!("ser small {:?} {:?} {}", t_varint, t_fixint, dur_proportion(t_varint, t_fixint));
}

#[test]
fn bench_de_small() {
	let num_runs = 1_000_000;
	let varint = AllVarInt {x:21, y:7};
	let fixint: NoVarInt = varint.into();
	let a = bincode::serialize(&varint).unwrap();
	let b = bincode::serialize(&fixint).unwrap();

	let (t_varint, t_fixint) = (
		bench!(num_runs, || {
			bincode::deserialize_from::<_, AllVarInt>(&a[..]).unwrap()
		}),
		bench!(num_runs, || {
			bincode::deserialize_from::<_, NoVarInt>(&b[..]).unwrap()
		}),
	);
	println!("de small{:?} {:?} {}", t_varint, t_fixint, dur_proportion(t_varint, t_fixint));
}

#[test]
fn bench_de_large() {
	let num_runs = 1_000_000;
	let varint = AllVarInt {x:9999999927364762743, y:-17722};
	let fixint: NoVarInt = varint.into();
	let a = bincode::serialize(&varint).unwrap();
	let b = bincode::serialize(&fixint).unwrap();

	let (t_varint, t_fixint) = (
		bench!(num_runs, || {
			bincode::deserialize_from::<_, AllVarInt>(&a[..]).unwrap()
		}),
		bench!(num_runs, || {
			bincode::deserialize_from::<_, NoVarInt>(&b[..]).unwrap()
		}),
	);
	println!("de large {:?} {:?} {}", t_varint, t_fixint, dur_proportion(t_varint, t_fixint));
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



// fn bench(reps: usize, work: FnMut()) -> time::Duration {
// 	let start = time::Instant::now();
// 	for _ in 0..reps {
// 		work();
// 	}
// 	start.elapsed()
// }

fn dur_conv(a: time::Duration) -> u64 {
	a.as_secs() as u64 * (1000*1000*1000) + a.subsec_nanos() as u64 
}

fn dur_proportion(a: time::Duration, b: time::Duration) -> f32 {
	dur_conv(a) as f32 / dur_conv(b) as f32
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
