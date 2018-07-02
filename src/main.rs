use std::io::{
	Write,
	BufWriter,
	BufReader,
	Read,
};
use itertools::Itertools;

extern crate bincode;

extern crate integer_encoding;
extern crate itertools;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use integer_encoding::{
	VarInt,
	VarIntReader,
	VarIntWriter,
};

#[derive(Serialize, Deserialize, Debug)]
struct Data {
	#[serde(serialize_with = "ser_varint")]
	#[serde(deserialize_with = "de_varint")]
	x: u64,
	#[serde(serialize_with = "ser_varint")]
	#[serde(deserialize_with = "de_varint")]
	y: u64,
}

use std::fmt;

use serde::{
	Serializer,
	Deserializer,
	Serialize,
	Deserialize,
	ser::{
		SerializeSeq,
		SerializeMap,
		SerializeStruct,
	},
	de::{
		self,
		Visitor,
		SeqAccess,
	},
};
fn ser_varint<T,S>(t:&T, serializer:S) -> Result<S::Ok, S::Error>
where
	T: VarInt + Copy,
	S: Serializer,
{
	let space: usize = t.required_space();
    let mut buf = [0u8; 8];
    (&mut buf[..]).write_varint(*t).unwrap();
    let mut seq = serializer.serialize_struct("", space)?;
    for (i, byte) in buf[..space].iter().enumerate() {
    	seq.serialize_field("", byte)?;
    }
    seq.end()
}



static EMPTY:  &'static [&'static str] = &["ya"];

fn de_varint<'de, D>(d:D) -> Result<u64, D::Error>
where
	// T: VarInt + Copy,
	D: Deserializer<'de>,
{
    // let mut buf = [0u8; 8];
    d.deserialize_tuple(8, VarIntVisitor)
}


trait MyVarInt : VarInt {

}

use std::io;
use std::marker;
struct VarIntVisitor;

static ALWAYS: &'static u8 = &5;

impl<'de> Visitor<'de> for VarIntVisitor {
    type Value = u64;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a VarInt")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where A: SeqAccess<'de>,
    {
    	let mut tot = 0u8;
    	// let buf = [0u8; 8];
    	// let bytes = Readymagoo{}.read_varint()
    	// let h = marker::PhantomData::default();
    	Ok(Readymagoo{
    		src: &mut seq,
    		phantom: &ALWAYS,
    	}.read_varint().unwrap())
    	// Ok(seq.foo())
    	// {
    	// 	let q = DeserReader { src: &mut seq };
    	// }
    	// if let Ok(Some(x)) = seq.next_element::<u8>() {
    	// 	println!("{} {:?}", tot, x);
    	// 	tot += 1;
    	// }
    	// Ok(0)
    }
}

struct Readymagoo<'a, A> where A: SeqAccess<'a> {
	src: A,
	phantom: &'a u8,
}

impl<'a, A> io::Read for Readymagoo<'a, A> where A: SeqAccess<'a> {
	fn read(&mut self, buf:&mut[u8]) -> Result<usize, io::Error> {
		buf[0] = self.src.next_element::<u8>().unwrap().unwrap();
		// buf[0] = 0x05;
		Ok(1)
	}
}


// trait KindaRead: io::Read {
// 	fn foo(&self) -> u64;
// }

// impl<'de, T> io::Read for SeqAccess<'de, Error=T> {

// }

// impl<'de, A> KindaRead for A where A: SeqAccess<'de> {
// 	fn foo(&self) -> u64 {
// 		23
// 	}
// }

// struct DeserReader<'de, A> where A: SeqAccess<'de> + 'de {
// 	src: &'de A,
// }

// impl<'de, A> io::Read for DeserReader<'de, A> where A: SeqAccess<'de> {
// 	fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
// 		None
// 	}
// }


const BUF_LEN: usize = 32;
fn main() {
	let datum = Data{x: 32, y: 36};

	let mut buf = [1u8; BUF_LEN];
	let value: u32 = 7248;
	let bytes_sent;

	{
		let mut b = BufWriter::new(&mut buf[..]);
		bytes_sent = 0;
		// datum.serialze_into(&mut b);
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
		// let t = reader.read_varint::<u32>();
		// assert_eq!(t.ok(), Some(value));
		// println!("left {:?}", reader.bytes().filter_map(Result::ok).format(","));
		// println!("successfully sent {} over the wire using {} byte(s).", value, bytes_sent);
	}
}
