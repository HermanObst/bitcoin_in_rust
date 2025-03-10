use num_bigint::BigInt;
use once_cell::sync::Lazy;
use crate::elliptic_curve::{
    finite_field::FieldElement,
    traits::{Coords, EllipticCurve, Point},
};

static SECP256K1_PRIME: Lazy<BigInt> = Lazy::new(|| BigInt::from(2).pow(256) - BigInt::from(2).pow(32) - BigInt::from(977));
static SECP256K1_A: Lazy<FieldElement> = Lazy::new(|| FieldElement::new(0.into(), SECP256K1_PRIME.clone()));
static SECP256K1_B: Lazy<FieldElement> = Lazy::new(|| FieldElement::new(7.into(), SECP256K1_PRIME.clone()));
struct secp256k1 {

}