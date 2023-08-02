use bitcoin::types::errors::Errors;
use crate::finite_field::FieldElement;
use num_bigint::{BigInt, ToBigInt};
use core::ops::Add;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
enum PointValue {
    Point{x: FieldElement, y: FieldElement},
    Infinity
}

#[derive(Clone, Debug, PartialEq)]
struct EllipticCurve {
    a: FieldElement,
    b: FieldElement
}

impl EllipticCurve {
    fn new(a: FieldElement, b: FieldElement) -> Self {
        EllipticCurve { a, b }
    }
}
#[derive(Clone, Debug, PartialEq)]
struct Point {
    point: PointValue,
    elliptic_curve: EllipticCurve
}

#[allow(dead_code)]
impl Point {
    fn new_point(x: FieldElement, y: FieldElement, a: FieldElement, b: FieldElement) -> Result<Self, Errors> {
        // Checks if point is included in the curve y2 = x3 + ax + b
        if y.pow(&2.to_bigint().unwrap()) != x.pow(&3.to_bigint().unwrap()) + a.clone() * x.clone() + b.clone() {
            return Err(Errors::InvalidPoint);
        }

        Ok(Point{point: PointValue::Point{x, y}, elliptic_curve: EllipticCurve::new(a, b)})
        }

    fn new_infinity(a: FieldElement, b: FieldElement) -> Self {
        Point{point: PointValue::Infinity, elliptic_curve: EllipticCurve::new(a, b)}
    }
}

#[cfg(test)]
mod point_tests {
    use num_bigint::ToBigInt;

    use super::*;

    #[test]
    fn test_create_ec_field_valid_point() {
        let prime = 223.to_bigint().unwrap();
        let a = FieldElement::new(0.to_bigint().unwrap(), prime.clone());
        let b = FieldElement::new(7.to_bigint().unwrap(), prime.clone());

        let x1 = FieldElement::new(192.to_bigint().unwrap(), prime.clone()); 
        let y1 = FieldElement::new(105.to_bigint().unwrap(), prime.clone()); 

        // Valid points (192, 105), (17, 56), (1, 193)
        assert!(Point::new_point(x1, y1, a.clone(), b.clone()).is_ok());

        let x1 = FieldElement::new(200.to_bigint().unwrap(), prime.clone()); 
        let y1 = FieldElement::new(119.to_bigint().unwrap(), prime.clone()); 

        // Invalid points  (200, 119), (42, 99)
        assert_eq!(Point::new_point(x1, y1, a.clone(), b.clone()), Err(Errors::InvalidPoint));
    }
}