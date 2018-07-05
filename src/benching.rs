// THIS CODE IS CURRENTLy UNUSED


fn dur_conv(a: time::Duration) -> u64 {
	a.as_secs() as u64 * (1000*1000*1000) + a.subsec_nanos() as u64 
}

fn dur_proportion(a: time::Duration, b: time::Duration) -> f32 {
	dur_conv(a) as f32 / dur_conv(b) as f32
}


macro_rules! mutate {
	($struct:ident) => {{
		$struct.x += 2390;
		$struct.y = -$struct.y + if $struct.y < 0 {-32} else {32};
	}}
}

macro_rules! bench {
	($reps:expr, $work:expr) => {{
		let start = time::Instant::now();
		for _ in 0..$reps {
			($work)();
		}
		start.elapsed()
	}}
}

// #[test]
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

// #[test]
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

// #[test]
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

// #[test]
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
