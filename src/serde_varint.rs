use std::{
	fmt,
	io,
};

use std::marker::PhantomData as Phant;

use serde::{
	Serializer,
	Deserializer,
	ser::{
		SerializeTuple,
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

/////////////////////////// SERIALIZE ///////////////////////

/// Serialize the given `T:VarInt` value into a tuple of bytes (representing)
/// the encoding provided by crate `integer-encoding`
/// (Google's protobuf encoding).
pub fn serialize<T,S>(t:&T, serializer:S) -> Result<S::Ok, S::Error>
where
	T: VarInt + Copy,
	S: Serializer,
{
	// 1. compute the number of `tuple elements` (number of bytes)
	let space: usize = t.required_space();

	// 2. wrap the serializer's `SerializeTuple` object in our newtype.
    let mut writer = NewtypeWriter {
    	dest: serializer.serialize_tuple(space)?,
    	err: None,
    };

    // 3. exploit the newtype's `io::Write` impl to apply `write_varint`
    match writer.write_varint(*t) {
    	Ok(_) => writer.into_inner().end(),
    	Err(_) => Err(writer.err.unwrap()),
    }
}

// Wrapper for `SerializableTuple` so we can make it implement `io::Write`
struct NewtypeWriter<A> where A: SerializeTuple {
	dest: A,
	err: Option<A::Error>,
}
impl<A> NewtypeWriter<A> where A: SerializeTuple {
	fn into_inner(self) -> A {
		self.dest
	}
}

impl<A> io::Write for NewtypeWriter<A> where A: SerializeTuple {
	fn write(&mut self, bytes: &[u8]) -> Result<usize, io::Error> {
		for byte in bytes.iter() {
	    	if let Err(e) = self.dest.serialize_element(byte) {
	    		self.err = Some(e);
	    		return Err(io::ErrorKind::InvalidData.into());
	    	}
	    }
	    Ok(bytes.len())
	}

	fn flush(&mut self) -> Result<(), io::Error> {
		Ok(())	
	}
}

/////////////////////////// DESERIALIZE ///////////////////////

/// Deerialize the given `T:VarInt` value from a tuple of bytes (representing)
/// the encoding provided by crate `integer-encoding`
/// (Google's protobuf encoding).
pub fn deserialize<'de, D, T>(d:D) -> Result<T, D::Error>
where
	T: VarInt + Copy,
	D: Deserializer<'de>,
{
	// delegate the heavy lifting to the VarIntVisitor
	// `999` limits the number of tuple elements. Visitor will limit itself.
    d.deserialize_tuple(999, VarIntVisitor::<T> {
    	phantom: Phant::default()
    })
}

// Custom visitor that calls `integer-encoding`
struct VarIntVisitor<T> where T: VarInt {
	phantom: Phant<T>,
}

impl<'de,T> Visitor<'de> for VarIntVisitor<T>
where T: VarInt
{
    type Value = T;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a VarInt encoded as a tuple")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where A: SeqAccess<'de>,
    {
    	// visitor will enter here.
    	// wrap the `SeqAccess` object in our newtype to impl `io::Read` on it.
    	let mut t = NewtypeReader {
    		src: &mut seq,
    		phantom: Phant::default(),
    	};
    	// match t.read_varint::<T>() {
    	// 	Ok(o) => Ok(o),
    	// 	Err(e) => Err(e.into()),
    	// }
    	Ok(t.read_varint::<T>().unwrap())
    }
}

// A newtype to enrich a `SeqAccess` object with `io::Read`.
// Need the silly phantom field to stop Rust from complaining about unused `'a`.
struct NewtypeReader<'a, A> where A: SeqAccess<'a> {
	src: A,
	phantom: Phant<&'a ()>,
}

impl<'a, A> io::Read for NewtypeReader<'a, A> where A: SeqAccess<'a> {
	fn read(&mut self, buf:&mut[u8]) -> Result<usize, io::Error> {
		if let Ok(Some(x)) = self.src.next_element::<u8>() {
			buf[0] = x;
			Ok(1)
		} else {	
			Err(io::ErrorKind::UnexpectedEof.into())
		}
	}
}
