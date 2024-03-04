# extensions-rs

[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/travisbaars/extensions-rs/integrate.yml?branch=main&logo=github)](https://github.com/travisbaars/extensions-rs/actions)
[![Crates.io Version](https://img.shields.io/crates/v/extensions-rs)](https://crates.io/crates/extensions-rs)
[![docs.rs](https://img.shields.io/docsrs/extensions-rs)](https://docs.rs/extensions-rs)

A collection of file extension types in Rust.

The idea behind this crate is to give a simple way of handling file extension types.

## Installation

Simply add `extensions-rs` to the dependencies your `Cargo.toml` file:

```toml
[dependencies]
extensions-rs = "0.2.0"
```

Or use the `cargo add` command:

```bash
cargo add extensions-rs
```

## Examples

#### Conversion to `Extension` type:

```rust
use extensions_rs::Extension;
use extensions_rs::Image;

assert_eq!("png", Extension::to_str(Extension::Image(Image::ExtPNG)));
```

#### Simple conversion, `&str` to `Image` type:

```rust
use extensions_rs::ext::Image;

assert_eq!(Image::ExtJPG, Image::from("jpg"))
```

#### Validate extension:

```rust
use extensions_rs::utils::Validate;

assert_eq!(true, Validate::check_str("jpg"))
```

## Todo

- [ ] Improve extension coverage
  - [ ] Add text extensions
  - [ ] Add video extensions
  - [ ] Add archive extensions
  - [ ] Add programming extensions
  - [ ] Add document extensions
- [ ] Improve documentation
- [ ] Implement `to_str` for `Image`
- [ ] Add feature to take a whole `path` or `string` and convert it to correct type
- [ ] Remove unnecessary `async' funcion/methods. Possibly transition to alternate async options.
