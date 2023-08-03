use bitcoin::types::errors::Errors;
use crate::finite_field::FieldElement;
use num_bigint::{BigInt, ToBigInt};
use core::ops::Add;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
enum PointValue {
    Point(FieldElement,FieldElement),
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

        Ok(Point{point: PointValue::Point(x, y), elliptic_curve: EllipticCurve::new(a, b)})
        }

    fn new_infinity(a: FieldElement, b: FieldElement) -> Self {
        Point{point: PointValue::Infinity, elliptic_curve: EllipticCurve::new(a, b)}
    }
}

impl Add<Point> for Point {
    type Output = Self;

    fn add(self, other: Point) -> Self {
        if self.elliptic_curve != other.elliptic_curve {
            panic!("{}", Errors::DifferentCurves);
        }

        let a = self.elliptic_curve.a.clone();
        let b = self.elliptic_curve.b.clone();
        let prime = self.elliptic_curve.a.prime.clone();

        match (self.point.clone(), other.point.clone()) {
            (PointValue::Point(x1,y1), PointValue::Point(x2,y2)) => {
                if x1 == x2 {
                    if y1 == y2 {
                        // Case when P1 == P2

                        // When tanget line is vertical
                        if y1.num == 0.to_bigint().unwrap() {
                            return Point::new_infinity(a, b)
                        }
                        
                        let slope = (FieldElement::new(BigInt::from(3), prime.clone()) * x1.pow(&BigInt::from(2)) + a.clone()) / FieldElement::new(BigInt::from(2), prime.clone()) * y1.clone();
                        let x3 = slope.pow(&BigInt::from(2)) - FieldElement::new(BigInt::from(2), prime.clone())*x1.clone();
                        let y3 = slope*(x1 - x3.clone()) - y1;
                        // This unwrap cannot fail as this functions already recives two valid points.
                        Point::new_point(x3, y3, a, b).unwrap()
                    } else {
                        // Vertical line (same x but different y coordinates)
                        Point::new_infinity(a, b)
                    }
                } else {
                    // Case were x coordinates are differents
                    let slope = (y2 - y1.clone())/(x2.clone() - x1.clone());
                    let x3 = slope.pow(&BigInt::from(2)) - x1.clone() - x2;
                    let y3 = slope*(x1 - x3.clone()) - y1;
                    // This unwrap cannot fail as this functions already recives two valid points.
                    Point::new_point(x3, y3, a, b).unwrap()
                }
            },
            // Handle identity (Infinity point). In case both are Infinity, returns Infinity (self).
            (_, PointValue::Infinity) => self,
            (PointValue::Infinity, _) => other
        }
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


        let valid_points: [(BigInt, BigInt); 3] = [(192.to_bigint().unwrap(), 105.to_bigint().unwrap()), (17.to_bigint().unwrap(), 56.to_bigint().unwrap()), (1.to_bigint().unwrap(), 193.to_bigint().unwrap())];
        let invalid_points: [(BigInt, BigInt); 2] = [(200.to_bigint().unwrap(), 119.to_bigint().unwrap()), (42.to_bigint().unwrap(), 99.to_bigint().unwrap())];

        for (x, y) in valid_points.iter() {
            let x = FieldElement::new(x.clone(), prime.clone());
            let y = FieldElement::new(y.clone(), prime.clone());
            assert!(Point::new_point(x, y, a.clone(), b.clone()).is_ok())
        }

        for (x, y) in invalid_points.iter() {
            let x = FieldElement::new(x.clone(), prime.clone());
            let y = FieldElement::new(y.clone(), prime.clone());
            assert_eq!(Point::new_point(x, y, a.clone(), b.clone()), Err(Errors::InvalidPoint))
        }
    }

    #[test]
    fn test_add_ec_field_points() {
        todo!()
    }
}