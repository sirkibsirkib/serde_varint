extern crate bincode;
extern crate integer_encoding;
extern crate itertools;
extern crate serde;
#[macro_use]
extern crate serde_derive;


mod serde_varint;

pub use serde_varint::{
	serialize,
	deserialize,
};

#[cfg(test)]
mod tests;