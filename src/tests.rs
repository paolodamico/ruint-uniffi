use crate::{Uint1024, Uint128, Uint160, Uint256, Uint384, Uint512};
use proptest::proptest;

macro_rules! roundtrip_tests {
    ($name:ident, $type:ty, $bits:literal, $limbs:literal) => {
        mod $name {
            use super::*;

            fn lower(v: &$type) -> String {
                String::from(*v)
            }

            fn try_lift(s: &str) -> Result<$type, Box<dyn std::error::Error>> {
                <$type>::try_from(s.to_owned()).map_err(Into::into)
            }

            #[test]
            fn hex_roundtrip_zero() {
                let v = <$type>::default();
                let hex = lower(&v);
                assert_eq!(hex, "0");
                let back = try_lift(&hex).unwrap();
                assert_eq!(back, v);
            }

            #[test]
            fn hex_roundtrip_one() {
                let inner = ruint::Uint::<$bits, $limbs>::from(1u64);
                let v = <$type>::from(inner);
                let hex = lower(&v);
                assert_eq!(hex, "1");
                let back = try_lift(&hex).unwrap();
                assert_eq!(back, v);
            }

            #[test]
            fn hex_roundtrip_max() {
                let inner = ruint::Uint::<$bits, $limbs>::MAX;
                let v = <$type>::from(inner);
                let hex = lower(&v);
                let back = try_lift(&hex).unwrap();
                assert_eq!(back, v);
            }

            #[test]
            fn hex_roundtrip_power_of_two() {
                let inner = ruint::Uint::<$bits, $limbs>::from(1u64) << 64;
                let v = <$type>::from(inner);
                let hex = lower(&v);
                let back = try_lift(&hex).unwrap();
                assert_eq!(back, v);
            }

            #[test]
            fn padded_hex_zero() {
                let v = <$type>::default();
                let hex = v.to_padded_hex_string();
                let expected_len = $bits / 4 + 2;
                assert_eq!(hex.len(), expected_len);
                assert!(hex.starts_with("0x"));
                assert!(hex[2..].chars().all(|c| c == '0'));
            }

            #[test]
            fn padded_hex_one() {
                let v = <$type>::from(ruint::Uint::<$bits, $limbs>::from(1u64));
                let hex = v.to_padded_hex_string();
                let expected_len = $bits / 4 + 2;
                assert_eq!(hex.len(), expected_len);
                assert!(hex.ends_with('1'));
                assert!(hex[2..expected_len - 1].chars().all(|c| c == '0'));
            }

            #[test]
            fn padded_hex_max() {
                let v = <$type>::from(ruint::Uint::<$bits, $limbs>::MAX);
                let hex = v.to_padded_hex_string();
                let expected_len = $bits / 4 + 2;
                assert_eq!(hex.len(), expected_len);
                assert!(hex[2..].chars().all(|c| c == 'f'));
            }

            #[test]
            fn error_invalid_hex() {
                assert!(try_lift("zzzz").is_err());
            }

            #[test]
            fn error_overflow() {
                let max_hex = lower(&<$type>::from(ruint::Uint::<$bits, $limbs>::MAX));
                let overflow = format!("1{max_hex}");
                assert!(try_lift(&overflow).is_err());
            }

            proptest! {
                #[test]
                fn prop_value_roundtrip(limb: u64) {
                    let inner = ruint::Uint::<$bits, $limbs>::from(limb);
                    let v = <$type>::from(inner);
                    let hex = lower(&v);
                    let back = try_lift(&hex).unwrap();
                    assert_eq!(back, v);
                }

                #[test]
                fn prop_hex_roundtrip(limb: u64) {
                    let inner = ruint::Uint::<$bits, $limbs>::from(limb);
                    let hex = format!("{inner:x}");
                    let lifted = try_lift(&hex).unwrap();
                    let lowered = lower(&lifted);
                    assert_eq!(lowered, hex);
                }
            }
        }
    };
}

roundtrip_tests!(uint128, Uint128, 128, 2);
roundtrip_tests!(uint160, Uint160, 160, 3);
roundtrip_tests!(uint256, Uint256, 256, 4);
roundtrip_tests!(uint384, Uint384, 384, 6);
roundtrip_tests!(uint512, Uint512, 512, 8);
roundtrip_tests!(uint1024, Uint1024, 1024, 16);
