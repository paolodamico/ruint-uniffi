# ruint-uniffi

[`UniFFI`](https://mozilla.github.io/uniffi-rs/) bindings for [ruint](https://docs.rs/ruint) big unsigned integer types. This crate provides convenient aliases that can be easily de-referenced in Rust to `Uint` and lowered to foreign bindings as native big integer types. Note that Swift does not have a native big integer type, so `BigUInt` from the [BigInt](https://github.com/attaswift/BigInt) library is used.

Values cross the FFI boundary as hex strings, mapping to native big integer types on each platform: `BigUInt` on Swift and `BigInteger` on Kotlin.

## Usage

1. Add the dependency `ruint-uniffi` to your `Cargo.toml`
1. Register the types in your `UniFFI` crate (after `uniffi::setup_scaffolding!()`):
  ```rust,ignore
  // Register all types
  ruint_uniffi::register_types!();

  // Or pick specific ones
  ruint_uniffi::register_types!(Uint256, Uint512);
  ```
1. Copy the foreign type mappings from [`examples/uniffi.toml`](examples/uniffi.toml) into your project's `uniffi.toml` for the types you need.

## Foreign language examples

Given Rust functions exposed via `UniFFI`:

```rust,ignore
#[uniffi::export]
fn total_supply() -> Uint256 { /* ... */ }

#[uniffi::export]
fn transfer(to: String, amount: Uint256) -> bool { /* ... */ }
```

### Swift

`Uint256` maps to `BigUInt` from the [BigInt](https://github.com/attaswift/BigInt) package:

```swift
import BigInt
import MyRustLib

let supply: BigUInt = totalSupply()
print("Total supply: \(supply)")

let amount = BigUInt(2).power(128)
let ok = transfer(to: "0xabcd", amount: amount)
```

### Kotlin

`Uint256` maps to `java.math.BigInteger`:

```kotlin
import my_rust_lib.*
import java.math.BigInteger

val supply: BigInteger = totalSupply()
println("Total supply: $supply")

val amount = BigInteger.TWO.pow(128)
val ok = transfer(to = "0xabcd", amount = amount)
```
