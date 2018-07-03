
extern crate integer_encoding;
extern crate itertools;
extern crate serde;


mod serde_varint;

pub use serde_varint::{
	serialize,
	deserialize,
};

#[cfg(test)]
#[macro_use]
extern crate serde_derive;

#[cfg(test)]
extern crate bincode;

#[cfg(test)]
extern crate serde_json;

#[cfg(test)]
mod tests;