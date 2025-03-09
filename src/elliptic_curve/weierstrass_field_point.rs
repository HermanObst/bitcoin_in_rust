use std::ops::{Add, Mul};

use num_bigint::BigInt;
use num_traits::Zero;

use crate::elliptic_curve::{
    finite_field::FieldElement,
    traits::{Coords, EllipticCurve, Point},
};
use crate::types::errors::Errors;

#[derive(Debug, PartialEq, Clone)]
struct WeierstrassCurve {
    a: FieldElement,
    b: FieldElement,
}

impl EllipticCurve for WeierstrassCurve {
    type Field = FieldElement;

    fn a(&self) -> Self::Field {
        self.a.clone()
    }

    fn b(&self) -> Self::Field {
        self.b.clone()
    }
    fn defining_equation(&self, x: &Self::Field, y: &Self::Field) -> Self::Field {
        y.clone().pow(&2.into()) - x.clone().pow(&3.into()) - self.a() * x.clone() - self.b()
    }
}

#[allow(dead_code)]
impl<'a> Point<'a, WeierstrassCurve> {
    fn new_point(
        curve: &'a WeierstrassCurve,
        x: &FieldElement,
        y: &FieldElement,
    ) -> Result<Self, Errors> {
        if curve.defining_equation(x, y) != FieldElement::zero(x.prime()) {
            return Err(Errors::InvalidPoint);
        }

        Ok(Point {
            coords: Coords::Point(x.clone(), y.clone()),
            curve,
        })
    }

    fn new_infinity(curve: &'a WeierstrassCurve) -> Self {
        Point {
            coords: Coords::Infinity,
            curve,
        }
    }
}

impl PartialEq for Point<'_, WeierstrassCurve> {
    fn eq(&self, other: &Self) -> bool {
        match (&self.coords, &other.coords) {
            (Coords::Point(x1, y1), Coords::Point(x2, y2)) => x1 == x2 && y1 == y2,
            (Coords::Infinity, Coords::Infinity) => true,
            _ => false,
        }
    }
}

// TODO: this needs to create new BigInts instances for every sum, although they are fixed
// We could define them outside as constants and use referecnes to them
// Similar, the prime number do not need to be cloned all the way around.
// TODO: Implement aritmethics for &FieldElements to no need to clone all over the place
impl Add for Point<'_, WeierstrassCurve> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let curve = self.curve; // Ensure curve is accessible
        let curve_other = other.curve;
        // TODO: Handle this case gracefully
        assert!(
            !(curve != curve_other),
            "Cannot add points on different curves"
        );

        match (&self.coords, &other.coords) {
            // If either operand is the identity (point at infinity), return the other.
            (Coords::Infinity, _) => other,
            (_, Coords::Infinity) => self,

            (Coords::Point(x1, y1), Coords::Point(x2, y2)) => {
                if x1 == x2 {
                    if y1 == y2 {
                        // ---- Doubling case (P1 == P2) ----
                        if y1.is_zero() {
                            // Tangent line to the curve is vertical if y = 0, which results in infinity
                            Point::new_infinity(curve)
                        } else {
                            // slope = (3*x1^2 + A) / (2*y1)
                            let numerator = FieldElement::new(3.into(), curve.a().prime())
                                * x1.pow(&2.into())
                                + curve.a();
                            let denominator =
                                FieldElement::new(2.into(), curve.a().prime()) * y1.clone();
                            let slope = numerator / denominator;
                            // x3 = slope^2 - 2x1
                            // y3 = slope(x1 - x3) - y1
                            let x3 = slope.pow(&2.into())
                                - FieldElement::new(2.into(), curve.a().prime()) * x1.clone();
                            let y3 = slope * (x1.clone() - x3.clone()) - y1.clone();
                            Point::new_point(curve, &x3, &y3).unwrap()
                        }
                    } else {
                        // ---- P1 = -P2 => vertical line => infinity. ----
                        Point::new_infinity(curve)
                    }
                } else {
                    // ---- Addition case (x1 != x2) ----
                    let slope = (y2.clone() - y1.clone()) / (x2.clone() - x1.clone());
                    let x3 = slope.pow(&2.into()) - x1.clone() - x2.clone();
                    let y3 = slope * (x1.clone() - x3.clone()) - y1.clone();
                    Point::new_point(curve, &x3, &y3).unwrap()
                }
            }
        }
    }
}

impl<T> Mul<T> for Point<'_, WeierstrassCurve>
where
    T: Into<BigInt>,
{
    type Output = Self;

    fn mul(self, coefficient: T) -> Self {
        let mut coeff = coefficient.into();
        let mut current = self.clone();
        let mut result = Point::new_infinity(self.curve);

        while coeff != BigInt::zero() {
            if coeff.clone() & BigInt::from(1) != BigInt::zero() {
                result = result + current.clone();
            }
            coeff >>= 1;
            current = current.clone() + current;
        }
        result
    }
}

#[cfg(test)]
mod weierstrass_field_point_tests {
    use super::*;
    use num_bigint::BigInt;
    use num_bigint::ToBigInt;

    #[test]
    fn test_create_ec_field_valid_point() {
        let prime = 223.to_bigint().unwrap();
        let a = FieldElement::new(0.to_bigint().unwrap(), prime.clone());
        let b = FieldElement::new(7.to_bigint().unwrap(), prime.clone());
        let curve = WeierstrassCurve {
            a: a.clone(),
            b: b.clone(),
        };

        let valid_points: [(BigInt, BigInt); 3] = [
            (192.to_bigint().unwrap(), 105.to_bigint().unwrap()),
            (17.to_bigint().unwrap(), 56.to_bigint().unwrap()),
            (1.to_bigint().unwrap(), 193.to_bigint().unwrap()),
        ];
        let invalid_points: [(BigInt, BigInt); 2] = [
            (200.to_bigint().unwrap(), 119.to_bigint().unwrap()),
            (42.to_bigint().unwrap(), 99.to_bigint().unwrap()),
        ];

        for (x, y) in valid_points.iter() {
            let x = FieldElement::new(x.clone(), prime.clone());
            let y = FieldElement::new(y.clone(), prime.clone());
            assert!(Point::new_point(&curve, &x, &y).is_ok());
        }

        for (x, y) in invalid_points.iter() {
            let x = FieldElement::new(x.clone(), prime.clone());
            let y = FieldElement::new(y.clone(), prime.clone());
            assert_eq!(Point::new_point(&curve, &x, &y), Err(Errors::InvalidPoint));
        }
    }

    #[test]
    fn test_add_ec_field_points_different_x() {
        let prime = BigInt::from(223);
        let a = FieldElement::new(BigInt::from(0), prime.clone());
        let b = FieldElement::new(BigInt::from(7), prime.clone());
        let curve = WeierstrassCurve {
            a: a.clone(),
            b: b.clone(),
        };

        let x1 = FieldElement::new(BigInt::from(192), prime.clone());
        let y1 = FieldElement::new(BigInt::from(105), prime.clone());
        let x2 = FieldElement::new(BigInt::from(17), prime.clone());
        let y2 = FieldElement::new(BigInt::from(56), prime.clone());

        let p1 = Point::new_point(&curve, &x1, &y1).unwrap();
        let p2 = Point::new_point(&curve, &x2, &y2).unwrap();

        let xr = FieldElement::new(BigInt::from(170), prime.clone());
        let yr = FieldElement::new(BigInt::from(142), prime.clone());
        let r = Point::new_point(&curve, &xr, &yr).unwrap();

        assert_eq!(p1 + p2, r);

        // Additional test cases
        let x1 = FieldElement::new(BigInt::from(170), prime.clone());
        let y1 = FieldElement::new(BigInt::from(142), prime.clone());
        let x2 = FieldElement::new(BigInt::from(60), prime.clone());
        let y2 = FieldElement::new(BigInt::from(139), prime.clone());
        let p1 = Point::new_point(&curve, &x1, &y1).unwrap();
        let p2 = Point::new_point(&curve, &x2, &y2).unwrap();
        let xr = FieldElement::new(BigInt::from(220), prime.clone());
        let yr = FieldElement::new(BigInt::from(181), prime.clone());
        let r = Point::new_point(&curve, &xr, &yr).unwrap();
        assert_eq!(p1 + p2, r);

        let x1 = FieldElement::new(BigInt::from(47), prime.clone());
        let y1 = FieldElement::new(BigInt::from(71), prime.clone());
        let x2 = FieldElement::new(BigInt::from(17), prime.clone());
        let y2 = FieldElement::new(BigInt::from(56), prime.clone());
        let p1 = Point::new_point(&curve, &x1, &y1).unwrap();
        let p2 = Point::new_point(&curve, &x2, &y2).unwrap();
        let xr = FieldElement::new(BigInt::from(215), prime.clone());
        let yr = FieldElement::new(BigInt::from(68), prime.clone());
        let r = Point::new_point(&curve, &xr, &yr).unwrap();
        assert_eq!(p1 + p2, r);

        let x1 = FieldElement::new(BigInt::from(143), prime.clone());
        let y1 = FieldElement::new(BigInt::from(98), prime.clone());
        let x2 = FieldElement::new(BigInt::from(76), prime.clone());
        let y2 = FieldElement::new(BigInt::from(66), prime.clone());
        let p1 = Point::new_point(&curve, &x1, &y1).unwrap();
        let p2 = Point::new_point(&curve, &x2, &y2).unwrap();
        let xr = FieldElement::new(BigInt::from(47), prime.clone());
        let yr = FieldElement::new(BigInt::from(71), prime.clone());
        let r = Point::new_point(&curve, &xr, &yr).unwrap();
        assert_eq!(p1 + p2, r);
    }

    #[test]
    fn test_add_ec_field_points_inf() {
        let prime = BigInt::from(223);
        let a = FieldElement::new(BigInt::from(0), prime.clone());
        let b = FieldElement::new(BigInt::from(7), prime.clone());
        let curve = WeierstrassCurve {
            a: a.clone(),
            b: b.clone(),
        };

        let x1 = FieldElement::new(BigInt::from(192), prime.clone());
        let y1 = FieldElement::new(BigInt::from(105), prime.clone());

        let p1 = Point::new_point(&curve, &x1, &y1).unwrap();

        assert_eq!(p1.clone() + Point::new_infinity(&curve), p1);
    }

    #[test]
    fn test_add_vertical_line() {
        // This happen when points have same x and different y coordinates
        let prime = BigInt::from(223);
        let a = FieldElement::new(BigInt::from(5), prime.clone());
        let b = FieldElement::new(BigInt::from(7), prime.clone());
        let curve = WeierstrassCurve {
            a: a.clone(),
            b: b.clone(),
        };

        let one = FieldElement::new(BigInt::from(1), prime.clone());
        let one_minus = FieldElement::new(BigInt::from(-1), prime.clone());

        let p1 = Point::new_point(&curve, &one_minus, &one).unwrap();
        let p2 = Point::new_point(&curve, &one_minus, &one_minus).unwrap();

        assert_eq!(p1 + p2, Point::new_infinity(&curve));
    }

    #[test]
    fn test_add_same_point() {
        let prime = BigInt::from(223);
        let a = FieldElement::new(BigInt::from(0), prime.clone());
        let b = FieldElement::new(BigInt::from(7), prime.clone());
        let curve = WeierstrassCurve {
            a: a.clone(),
            b: b.clone(),
        };

        let x1 = FieldElement::new(BigInt::from(192), prime.clone());
        let y1 = FieldElement::new(BigInt::from(105), prime.clone());
        let p1 = Point::new_point(&curve, &x1, &y1).unwrap();

        let xr = FieldElement::new(BigInt::from(49), prime.clone());
        let yr = FieldElement::new(BigInt::from(71), prime.clone());
        let r = Point::new_point(&curve, &xr, &yr).unwrap();

        assert_eq!(p1.clone() + p1.clone(), r);

        let x1 = FieldElement::new(BigInt::from(143), prime.clone());
        let y1 = FieldElement::new(BigInt::from(98), prime.clone());
        let p1 = Point::new_point(&curve, &x1, &y1).unwrap();

        let xr = FieldElement::new(BigInt::from(64), prime.clone());
        let yr = FieldElement::new(BigInt::from(168), prime.clone());
        let r = Point::new_point(&curve, &xr, &yr).unwrap();

        assert_eq!(p1.clone() + p1.clone(), r);

        let x1 = FieldElement::new(BigInt::from(47), prime.clone());
        let y1 = FieldElement::new(BigInt::from(71), prime.clone());
        let p1 = Point::new_point(&curve, &x1, &y1).unwrap();

        let xr = FieldElement::new(BigInt::from(36), prime.clone());
        let yr = FieldElement::new(BigInt::from(111), prime.clone());
        let r = Point::new_point(&curve, &xr, &yr).unwrap();

        assert_eq!(p1.clone() + p1.clone(), r);

        let xr = FieldElement::new(BigInt::from(194), prime.clone());
        let yr = FieldElement::new(BigInt::from(51), prime.clone());
        let r = Point::new_point(&curve, &xr, &yr).unwrap();

        assert_eq!(p1.clone() + p1.clone() + p1.clone() + p1.clone(), r);

        let xr = FieldElement::new(BigInt::from(116), prime.clone());
        let yr = FieldElement::new(BigInt::from(55), prime.clone());
        let r = Point::new_point(&curve, &xr, &yr).unwrap();

        assert_eq!(
            p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone(),
            r
        );

        let r = Point::new_infinity(&curve);

        assert_eq!(
            p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone(),
            r
        );
        assert_eq!(
            p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone()
                + p1.clone(),
            p1
        );
    }

    #[test]
    fn test_scalar_mul() {
        let prime = BigInt::from(223);
        let a = FieldElement::new(BigInt::from(0), prime.clone());
        let b = FieldElement::new(BigInt::from(7), prime.clone());
        let curve = WeierstrassCurve {
            a: a.clone(),
            b: b.clone(),
        };

        let x = FieldElement::new(BigInt::from(15), prime.clone());
        let y = FieldElement::new(BigInt::from(86), prime.clone());

        let p = Point::new_point(&curve, &x, &y).unwrap();

        assert_eq!(p.clone(), p.clone() * 1u64);
        assert_eq!(p.clone() * 7u64, Point::new_infinity(&curve));
        assert_eq!(p.clone() * 3u64, p.clone() + p.clone() + p);
    }

    #[test]
    fn test_scalar_multiplication_sequence() {
        let prime = BigInt::from(223);
        let a = FieldElement::new(BigInt::from(0), prime.clone());
        let b = FieldElement::new(BigInt::from(7), prime.clone());
        let curve = WeierstrassCurve {
            a: a.clone(),
            b: b.clone(),
        };

        let x = FieldElement::new(BigInt::from(47), prime.clone());
        let y = FieldElement::new(BigInt::from(71), prime.clone());

        let p = Point::new_point(&curve, &x, &y).unwrap();

        let expected_results = [
            (47, 71),
            (36, 111),
            (15, 137),
            (194, 51),
            (126, 96),
            (139, 137),
            (92, 47),
            (116, 55),
            (69, 86),
            (154, 150),
            (154, 73),
            (69, 137),
            (116, 168),
            (92, 176),
            (139, 86),
            (126, 127),
            (194, 172),
            (15, 86),
            (36, 112),
            (47, 152),
        ];

        for (s, &(expected_x, expected_y)) in (1..=20).zip(expected_results.iter()) {
            let result = p.clone() * s;
            if let Coords::Point(rx, ry) = result.coords {
                assert_eq!(rx.num, BigInt::from(expected_x));
                assert_eq!(ry.num, BigInt::from(expected_y));
            } else {
                panic!("Result is not a point");
            }
        }

        // Test for multiplying by 21, expecting the point at infinity
        let result = p.clone() * 21;
        assert_eq!(result, Point::new_infinity(&curve));

        // Group "starts again"
        let result = p.clone() * 22;
        assert_eq!(result, p);
    }
}
