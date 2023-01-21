# **`wpp`**
[![Crates.io](https://img.shields.io/crates/v/wpp.svg)](https://crates.io/crates/wpp) 
[![docs.rs](https://docs.rs/wpp/badge.svg)](https://docs.rs/wpp/)

Flexible and reusable post-processing effects for [`wgpu`](https://wgpu.rs/).

---

The name **`wpp`** is an acronym for **`wgpu` post-processing** and it basically does what it says. This library will provide a collection of post-processing effects that you can easily integrate into your rendering pipeline.

## Features
* All effects are implemented in [`wgsl`](https://www.w3.org/TR/WGSL/) and pure [Rust](https://www.rust-lang.org/) only
* Integrates easily into an existing rendering pipeline
* Small footprint and API surface
* No dependencies apart from [`wgpu`](https://crates.io/crates/wgpu)

## Limitations
* Currently no support for multisampled textures

## Available Effects / Cargo Features
| Name | Description |
| --- | --- |
| `grayscale` | A grayscale post-processing effect mainly used for API testing. |

By default all effects are enabled. However you can selectively enable a subset of available effects by using `default-features = false` and then enabling the desired effect(s) manually using the above cargo features.

## Development
Whenever a change is made, the following commands should be (successfully) run before commiting:
1. `cargo test`
2. `cargo clippy`
3. `cargo fmt`

## License

All code in this repository is dual-licensed under either:

* MIT License ([LICENSE-MIT](docs/LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](docs/LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.