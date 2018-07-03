# VarInt for Serde


`integer-encoding` offers an implementation of variable-length integer encoding,
as described [here](https://developers.google.com/protocol-buffers/docs/encoding).

`serde` is a fantastic, ubiquitious system for (ser)ializing and (de)serializing
structured data.

By using a _serde attribute tag_, you can cause arbitrary integers within your
structs to be var-int-encoded, while leaving the other fields untouched.
This is particularly useful in combination with `bincode`, which will result in
super-minimal bytes for small numbers.

Example:
```rust

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_varint;

#[derive(Serialize, Deserialize)]
struct MyStuff {
    #[serde(with = "serde_varint")]
    x: u64,
    #[serde(with = "serde_varint")]
    y: i32,
    #[serde(with = "serde_varint")]
    z: u16,

    a: f32,
    b: u64,
    c: Box<MyStuff>,
}
```
When serialized, fields `x,y,z` of `MyStuff` will be each encoded as a tuple
of bytes encoded as variable integers, rather than as a u64, i32 and u8 respectively.
`a,b,c` are left alone and serialized however they usually are (even `b` which is var-int encodable)

When used in combination with bincode, assuming the numbers are small enough,
the serialized version of this struct will require just 3 bytes for
fields `a,b,c`, where ordinarily it would require 8+4+2 = 14. 

## Trade-off
Serializing a number using var-int encoding is usually slower than just slapping
the bytes into the writer. For bincode, it seems to be around 2x to 5x slower, 
depending on the length when var-int-encoded.

Since serializing and deserializing these numbers is reasonably fast, this
shouldn't usually be your bottleneck anyway. Just bear it in mind when deciding
whether or not to use this option.

## Var Int for raw numbers
If you want to serialize _just_ some integer (eg: u64), there are two options:
1. Create a newtype struct with the integer as its field, apply the serde attribute
exactly as before.
1. Use the `integer-encoding` crate directly. 