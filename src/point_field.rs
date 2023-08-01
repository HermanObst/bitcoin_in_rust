use bitcoin::types::errors::Errors;
use crate::finite_field::FieldElement;
use num_bigint::{BigInt, ToBigInt};
use core::ops::Add;

#[allow(dead_code)]
#[derive(Clone, Debug)]
enum Point {
    Point(FieldElement, FieldElement),
    Infinity
}

#[allow(dead_code)]
impl Point {
    fn new_point(x: FieldElement, y: FieldElement) -> Result<Self, Errors> {
        // Checks if point is included in the curve y2 = x3 + ax + b
        if y.pow(2) != x.pow(3) + A * &x + B {
            return Err(Errors::InvalidPoint);
        }

        Ok(Point::<A, B>::Point(x, y))
        }

    fn new_infinity() -> Self {
        Point::<A,B>::Infinity
    }
}

#[cfg(test)]
mod point_tests {
    use num_bigint::ToBigInt;

    use super::*;

    #[test]
    fn test_create_ec_field_valid_point() {
        let prime = 223.to_bigint().unwrap();
        let A = FieldElement::new(0.to_bigint().unwrap(), &prime);
        let B = FieldElement::new(7.to_bigint().unwrap(), &prime); 

        // Valid points (192, 105), (17, 56), (1, 193)
        assert!(Point::new_point(-1.to_bigint().unwrap(),-1.to_bigint().unwrap()).is_ok());
    }
}