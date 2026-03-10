# ruint-uniffi

[UniFFI](https://mozilla.github.io/uniffi-rs/) bindings for [ruint](https://docs.rs/ruint) big unsigned integer types. This crate provides convenient aliases that can be easily de-referenced in Rust to `Uint` and lowered to foreign bindings as native big integer types. Note that Swift does not have a native big integer type, so `BigUInt` from the [BigInt](https://github.com/attaswift/BigInt) library is used.

Values cross the FFI boundary as hex strings, mapping to native big integer types on each platform: `BigUInt` on Swift and `BigInteger` on Kotlin.

## Usage

1. Add the dependency `ruint-uniffi` to your `Cargo.toml`
1. Register the types in your UniFFI crate (after `uniffi::setup_scaffolding!()`):
  ```rust
  // Register all types
  ruint_uniffi::register_types!();
  
  // Or pick specific ones
  ruint_uniffi::register_types!(Uint256, Uint512);
  ```
1. Copy the foreign type mappings from [`examples/uniffi.toml`](examples/uniffi.toml) into your project's `uniffi.toml` for the types you need.
