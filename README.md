# VarInt for Serde

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
This create provides `serde` (de)serialization functions for integer types (u8, i8, u16, ...).
Either manually, or using a `serde` _attribute tag_, you can conventiently override
the functions used by your (de)serializer for specified integer fields (as in the example above). The
[var-int encoding](https://developers.google.com/protocol-buffers/docs/encoding)
is provided by crate `integer-encoding`.


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