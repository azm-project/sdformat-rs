# sdformat-rs
[![crates.io](https://img.shields.io/crates/v/sdformat_rs.svg)](https://crates.io/crates/sdformat_rs)

[SDFormat](http://sdformat.org/) parser using [serde-xml-rs](https://crates.io/crates/serde_xml_rs) for Rust.

## Example

```rust
let sdf = read_file("examples/simple_arm/model.sdf").unwrap();
println!("{:#?}", sdf);
```

## Reference
https://github.com/openrr/urdf-rs
