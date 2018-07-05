# VarInt for Serde

This crate offers functions that change the way integers are encoded for your
`serde` serializer to that of [variable-integer](https://developers.google.com/protocol-buffers/docs/encoding)
encoding, which is usually more _compact_. You can choose which fields do and do
not use these overriding functions.

```rust
// imports go here

#[derive(Serialize, Deserialize)]
struct YourStruct {
    #[serde(with = "serde_varint")]
    x: u64,

    y: u64,
}
```
Using `bincode` as an example, `YourStruct { x:5, y:5 }` is serialized as:
* x --> `[05]`
* y --> `[05,00,00,00,00,00,00,00]`

--------------------------
The crate exposes two functions: `serialize` and `deserialize` which are generic
over integer types {u8, i8, u16, i16, ...}.

If you're using `serde`'s _derive_ macros to serialize your types (I know I often do),
its very conventient to inject the var-int functions just using a serde _attribute tag_
for the desired fields (as in the example above).

Otherwise, the functions can be called manually.

This crate just adapts the low-level, actual encoding provided by crate `integer-encoding`
for use with the serde data model.


## Trade-off
Serializing a number using var-int encoding is slower than just slapping
the bytes into the writer. For bincode, it seems to be around 1x slower extra
for each byte in the variable-encoded form. Since serializing and deserializing
these numbers is reasonably fast, this
shouldn't usually be your bottleneck anyway. Just bear it in mind when deciding
whether or not to use this option.

## Var Int for raw numbers
If you want to serialize _just_ some integer (eg: u64), there are two options:
1. Create a newtype struct with the integer as its field, apply the serde attribute
exactly as before.
1. Use the `integer-encoding` crate directly. 