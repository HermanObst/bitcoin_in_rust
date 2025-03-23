#![allow(dead_code)]
use crate::elliptic_curve::{
    finite_field::FieldElement, traits::Point, weierstrass_field_point::WeierstrassCurve,
};
use num_bigint::BigInt;
use num_traits::Num;
use once_cell::sync::Lazy;

// Bitcoin secp256k1 prime = 2**256 - 2**32 - 977
const SECP256K1_PRIME_HEX: &str =
    "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F";

// TODO: Implement BigInt as an array. This would let it be known at compile time, thus removing the need of lazy
static SECP256K1_PRIME: Lazy<BigInt> =
    Lazy::new(|| BigInt::from_str_radix(SECP256K1_PRIME_HEX, 16).unwrap());
static SECP256K1_A: Lazy<FieldElement> =
    Lazy::new(|| FieldElement::new(0.into(), SECP256K1_PRIME.clone()));
static SECP256K1_B: Lazy<FieldElement> =
    Lazy::new(|| FieldElement::new(7.into(), SECP256K1_PRIME.clone()));
static SECP256K1_CURVE: Lazy<WeierstrassCurve> = Lazy::new(|| WeierstrassCurve {
    a: SECP256K1_A.clone(),
    b: SECP256K1_B.clone(),
});


const SECP256K1_X_GENERATOR_HEX: &str =
    "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798";
const SECP256K1_Y_GENERATOR_HEX: &str =
    "483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8";
const SECP256K1_ORDER_HEX: &str =
    "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141";

static SECP256K1_GX: Lazy<FieldElement> = Lazy::new(|| {
    FieldElement::new(
        BigInt::from_str_radix(SECP256K1_X_GENERATOR_HEX, 16).unwrap(),
        SECP256K1_PRIME.clone(),
    )
});
static SECP256K1_GY: Lazy<FieldElement> = Lazy::new(|| {
    FieldElement::new(
        BigInt::from_str_radix(SECP256K1_Y_GENERATOR_HEX, 16).unwrap(),
        SECP256K1_PRIME.clone(),
    )
});
static SECP256K1_ORDER: Lazy<BigInt> =
    Lazy::new(|| BigInt::from_str_radix(SECP256K1_ORDER_HEX, 16).unwrap());

static SECP256K1_GENERATOR: Lazy<Point<WeierstrassCurve>> = Lazy::new(|| {
    Point::<WeierstrassCurve>::new_point(&SECP256K1_CURVE, &SECP256K1_GX, &SECP256K1_GY).unwrap()
});

#[cfg(test)]
mod elliptic_curve_tests {
    use num_bigint::ToBigInt;

    use super::*;

    #[test]
    fn test_multiply_secp256k1_curve_generator_by_order_returns_infinity() {
        assert_eq!(
            Point::<WeierstrassCurve>::new_infinity(&SECP256K1_CURVE),
            SECP256K1_GENERATOR.clone() * SECP256K1_ORDER.clone()
        )
    }

    #[test]
    fn test_multiply_secp256k1_curve_generator_by_order_plus1_returns_circles_back() {
        assert_eq!(
            SECP256K1_GENERATOR.clone(),
            SECP256K1_GENERATOR.clone() * (SECP256K1_ORDER.clone() + 1.to_bigint().unwrap())
        )
    }
}
