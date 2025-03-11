use num_bigint::BigInt;
use num_traits::Num;
use once_cell::sync::Lazy;
use crate::elliptic_curve::{
    finite_field::FieldElement,
    traits::{Coords, EllipticCurve, Point},
};

// secp256k1 prime = 2**256 - 2**32 - 977
const SECP256K1_PRIME_HEX: &str = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F";

// TODO: Implement BifInt as an array. This would let it be known at compile time, thus removing the need of lazy
static SECP256K1_PRIME: Lazy<BigInt> = Lazy::new(|| BigInt::from_str_radix(SECP256K1_PRIME_HEX, 16).unwrap());
static SECP256K1_A: Lazy<FieldElement> = Lazy::new(|| FieldElement::new(0.into(), SECP256K1_PRIME.clone()));
static SECP256K1_B: Lazy<FieldElement> = Lazy::new(|| FieldElement::new(7.into(), SECP256K1_PRIME.clone()));
struct secp256k1curve {}

impl EllipticCurve for secp256k1curve {
    type Field = FieldElement;
    fn a(&self) -> Self::Field {
        SECP256K1_A.clone()
    }

    fn b(&self) -> Self::Field {
        SECP256K1_B.clone()
    }

    fn defining_equation(&self, x: &Self::Field, y: &Self::Field) -> Self::Field {
        y.clone().pow(&2.into()) - x.clone().pow(&3.into()) - self.a() * x.clone() - self.b()
    }

}