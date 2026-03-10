#![doc = include_str!("../README.md")]

use core::fmt;
use core::ops::{Deref, DerefMut};
use core::str::FromStr;

#[cfg(test)]
mod tests;

macro_rules! define_uint {
    ($name:ident, $bits:literal, $limbs:literal) => {
        #[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(feature = "serde", serde(transparent))]
        #[repr(transparent)]
        /// A foreign-friendly compatible unsigned integer from [`ruint::Uint`].
        pub struct $name(pub ruint::Uint<$bits, $limbs>);

        impl $name {
            /// Convert the `Uint` to a padded hex string with leading zeros, ensuring a fixed length of $BITS.
            #[must_use]
            pub fn to_padded_hex_string(&self) -> String {
                format!("{:#0width$x}", self.0, width = $bits / 4 + 2)
            }
        }

        impl Deref for $name {
            type Target = ruint::Uint<$bits, $limbs>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl From<ruint::Uint<$bits, $limbs>> for $name {
            fn from(v: ruint::Uint<$bits, $limbs>) -> Self {
                Self(v)
            }
        }

        impl From<$name> for ruint::Uint<$bits, $limbs> {
            fn from(v: $name) -> Self {
                v.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }

        impl FromStr for $name {
            type Err = ruint::ParseError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                ruint::Uint::from_str(s).map(Self)
            }
        }

        /// Conversion from `Uint` to `String` for lowering to foreign languages.
        ///
        /// Hex-encoded number as strings are used primarily because lowering arrays via the uniffi
        /// boundary is not supported, so allocating a `Vec` already requires heap allocation. Furthermore,
        /// using a `Vec` results in more cumbersome handling on the native side as the native types don't
        /// offer a simple way of parsing from a byte array.
        impl From<$name> for String {
            fn from(v: $name) -> Self {
                format!("{:x}", v.0)
            }
        }

        /// Conversion from `String` to `Uint` for lifting from foreign languages.
        impl TryFrom<String> for $name {
            type Error = ruint::ParseError;

            fn try_from(val: String) -> Result<Self, Self::Error> {
                ruint::Uint::from_str_radix(&val, 16).map(Self)
            }
        }
    };
}

define_uint!(Uint128, 128, 2);
define_uint!(Uint160, 160, 3);
define_uint!(Uint256, 256, 4);
define_uint!(Uint384, 384, 6);
define_uint!(Uint512, 512, 8);
define_uint!(Uint1024, 1024, 16);

/// Register ruint-uniffi types with the consumer's `UniFFI` scaffolding.
///
/// Call this macro once at the crate root, after `uniffi::setup_scaffolding!()`.
/// It registers each wrapper type as a `UniFFI` custom type bridged through
/// hex strings. Uses the consumer's own uniffi version.
///
/// To register only specific types, pass them as arguments:
/// ```ignore
/// ruint_uniffi::register_types!(Uint256, Uint512);
/// ```
#[macro_export]
macro_rules! register_types {
    () => {
        $crate::register_types!(Uint128, Uint160, Uint256, Uint384, Uint512, Uint1024);
    };
    ($($name:ident),+ $(,)?) => {
        $(
            use $crate::$name;
            uniffi::custom_type!($name, String, { remote });
        )+
    };
}
