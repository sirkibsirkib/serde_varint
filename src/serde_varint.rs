use std::{
	self,
	fmt,
	io,
	marker,
};

use serde::{
	self,
	Serializer,
	Deserializer,
	ser::{
		SerializeStruct,
	},
	de::{
		Visitor,
		SeqAccess,
	},
};

use integer_encoding::{
	VarInt,
	VarIntReader,
	VarIntWriter,
};
// use std::convert::From;

pub fn serialize<T,S>(t:&T, serializer:S) -> Result<S::Ok, S::Error>
where
	T: VarInt + Copy,
	S: Serializer,
{
	let space: usize = t.required_space();
    let mut buf = [0u8; 8];
    (&mut buf[..]).write_varint(*t).unwrap();
    let mut seq = serializer.serialize_struct("", space)?;
    for byte in buf[..space].iter() {
    	seq.serialize_field("", byte)?;
    }
    seq.end()
}


pub fn deserialize<'de, D, T>(d:D) -> Result<T, D::Error>
where
	T: VarInt + Copy,
	D: Deserializer<'de>,
{
    d.deserialize_tuple(8, VarIntVisitor::<T> {
    	phantom: marker::PhantomData::default()
    })
}

struct VarIntVisitor<T> where T: VarInt {
	phantom: marker::PhantomData<T>,
}

static ALWAYS: &'static u8 = &5;

impl<'de,T> Visitor<'de> for VarIntVisitor<T>
where T: VarInt
{
    type Value = T;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a VarInt")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where A: SeqAccess<'de>,
    {
    	Ok(Readymagoo{
    		src: &mut seq,
    		phantom: &ALWAYS,
    	}.read_varint::<T>().unwrap())
    }
}

struct Readymagoo<'a, A> where A: SeqAccess<'a> {
	src: A,
	phantom: &'a u8,
}

impl<'a, A> io::Read for Readymagoo<'a, A> where A: SeqAccess<'a> {
	fn read(&mut self, buf:&mut[u8]) -> Result<usize, io::Error> {
		if let Ok(Some(x)) = self.src.next_element::<u8>() {
			buf[0] = x;
			Ok(1)
		} else {	
			Err(io::ErrorKind::InvalidData.into())
		}
	}
}
