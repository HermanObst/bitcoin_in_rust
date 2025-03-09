use crate::elliptic_curve::traits::{Coords, EllipticCurve, Point};
use crate::types::errors::Errors;
use core::ops::Add;
use num_bigint::BigInt;
use num_traits::Zero;

// This module implements the `RealWeierstrassCurve` and associated `Point` operations
// for elliptic curves defined over the real numbers using the Weierstrass form.
//
// The Weierstrass form of an elliptic curve is given by the equation:
//
//     y² = x³ + ax + b
//
// This module provides functionality to create points on the curve, including the
// point at infinity, and to perform point addition and doubling operations.
//
// The `RealWeierstrassCurve` struct represents the curve itself, defined by the
// coefficients `a` and `b`.
//
// The `Point` struct represents a point on the curve, which can be either a
// coordinate pair (x, y) or the point at infinity. The module ensures that points
// satisfy the curve's defining equation.
//
// The module also includes tests to verify the correctness of point creation and
// arithmetic operations.

#[derive(Debug, PartialEq)]
struct RealWeierstrassCurve {
    a: BigInt,
    b: BigInt,
}

impl EllipticCurve for RealWeierstrassCurve {
    type Field = BigInt;

    fn a(&self) -> Self::Field {
        self.a.clone()
    }

    fn b(&self) -> Self::Field {
        self.b.clone()
    }

    fn defining_equation(&self, x: &Self::Field, y: &Self::Field) -> Self::Field {
        y.pow(2) - x.pow(3) - self.a() * x - self.b()
    }
}

#[allow(dead_code)]
impl<'a> Point<'a, RealWeierstrassCurve> {
    fn new_point(curve: &'a RealWeierstrassCurve, x: &BigInt, y: &BigInt) -> Result<Self, Errors> {
        if curve.defining_equation(x, y) != BigInt::from(0) {
            return Err(Errors::InvalidPoint);
        }

        Ok(Point {
            coords: Coords::Point(x.clone(), y.clone()),
            curve,
        })
    }

    fn new_infinity(curve: &'a RealWeierstrassCurve) -> Self {
        Point {
            coords: Coords::Infinity,
            curve,
        }
    }
}

impl PartialEq for Point<'_, RealWeierstrassCurve> {
    fn eq(&self, other: &Self) -> bool {
        if self.curve != other.curve {
            // TODO: Handle this case gracefully
            panic!("Cannot compare points on different curves");
        }

        match (&self.coords, &other.coords) {
            (Coords::Point(x1, y1), Coords::Point(x2, y2)) => x1 == x2 && y1 == y2,
            (Coords::Infinity, Coords::Infinity) => true,
            _ => false,
        }
    }
}

impl Add for Point<'_, RealWeierstrassCurve> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let curve = self.curve; // Ensure curve is accessible
        let curve_other = other.curve;
        if curve != curve_other {
            // TODO: Handle this case gracefully
            panic!("Cannot add points on different curves");
        }

        match (&self.coords, &other.coords) {
            // If either operand is the identity (point at infinity), return the other.
            (Coords::Infinity, _) => other,
            (_, Coords::Infinity) => self,

            // Both are actual points on the curve.
            (Coords::Point(x1, y1), Coords::Point(x2, y2)) => {
                if x1 == x2 {
                    if y1 == y2 {
                        // ---- Doubling case (P1 == P2) ----
                        if y1.is_zero() {
                            // Tangent line to the curve is vertical if y = 0, which results in infinity
                            Point::new_infinity(curve)
                        } else {
                            // slope = (3*x1^2 + A) / (2*y1)
                            let numerator = BigInt::from(3) * x1.pow(2_u32) + curve.a();
                            let denominator = BigInt::from(2) * y1;
                            let slope = numerator / denominator;

                            let x3 = slope.pow(2_u32) - (BigInt::from(2) * x1);
                            let y3 = &slope * (x1 - &x3) - y1;
                            Point::new_point(curve, &x3, &y3).unwrap()
                        }
                    } else {
                        // ---- P1 = -P2 => vertical line => infinity. ----
                        Point::new_infinity(curve)
                    }
                } else {
                    // ---- Addition case (x1 != x2) ----
                    let slope = (y2 - y1) / (x2 - x1);
                    let x3 = slope.pow(2_u32) - x1 - x2;
                    let y3 = &slope * (x1 - &x3) - y1;
                    Point::new_point(curve, &x3, &y3).unwrap()
                }
            }
        }
    }
}

#[cfg(test)]
mod elliptic_curve_tests {
    use super::*;
    use num_bigint::ToBigInt;

    #[test]
    fn test_create_valid_point() {
        let curve = RealWeierstrassCurve {
            a: 5.to_bigint().unwrap(),
            b: 7.to_bigint().unwrap(),
        };
        assert!(
            Point::new_point(&curve, &-1.to_bigint().unwrap(), &-1.to_bigint().unwrap()).is_ok()
        );
    }

    #[test]
    fn test_create_valid_point_and_check_result() {
        let curve = RealWeierstrassCurve {
            a: 5.to_bigint().unwrap(),
            b: 7.to_bigint().unwrap(),
        };
        let result = Point::new_point(&curve, &-1.to_bigint().unwrap(), &-1.to_bigint().unwrap());
        assert!(result.is_ok());

        let point = result.unwrap();
        assert_eq!(
            point,
            Point {
                coords: Coords::Point(-1.to_bigint().unwrap(), -1.to_bigint().unwrap()),
                curve: &curve
            }
        );
    }

    #[test]
    fn test_create_valid_point_at_infinity() {
        let curve = RealWeierstrassCurve {
            a: 5.to_bigint().unwrap(),
            b: 7.to_bigint().unwrap(),
        };
        assert_eq!(
            Point::new_infinity(&curve),
            Point {
                coords: Coords::Infinity,
                curve: &curve
            }
        );
    }

    #[test]
    fn test_eq() {
        let curve = RealWeierstrassCurve {
            a: 5.to_bigint().unwrap(),
            b: 7.to_bigint().unwrap(),
        };
        assert!(
            Point::new_infinity(&curve)
                == Point {
                    coords: Coords::Infinity,
                    curve: &curve
                }
        );
        assert!(
            Point {
                coords: Coords::Point(-1.to_bigint().unwrap(), -1.to_bigint().unwrap()),
                curve: &curve
            } == Point {
                coords: Coords::Point(-1.to_bigint().unwrap(), -1.to_bigint().unwrap()),
                curve: &curve
            }
        );
        assert!(
            Point {
                coords: Coords::Point(-1.to_bigint().unwrap(), -1.to_bigint().unwrap()),
                curve: &curve
            } != Point {
                coords: Coords::Point(-1.to_bigint().unwrap(), 1.to_bigint().unwrap()),
                curve: &curve
            }
        );
        assert!(
            Point {
                coords: Coords::Infinity,
                curve: &curve
            } != Point {
                coords: Coords::Point(-1.to_bigint().unwrap(), 1.to_bigint().unwrap()),
                curve: &curve
            }
        );
    }

    #[test]
    fn test_add_infinity_to_point() {
        let curve = RealWeierstrassCurve {
            a: 5.to_bigint().unwrap(),
            b: 7.to_bigint().unwrap(),
        };
        let infinity = Point::new_infinity(&curve);
        let point =
            Point::new_point(&curve, &-1.to_bigint().unwrap(), &-1.to_bigint().unwrap()).unwrap();

        assert_eq!(
            infinity + point,
            Point::new_point(&curve, &-1.to_bigint().unwrap(), &-1.to_bigint().unwrap()).unwrap()
        );
    }

    #[test]
    fn test_add_infinity_to_point_reverse() {
        let curve = RealWeierstrassCurve {
            a: 5.to_bigint().unwrap(),
            b: 7.to_bigint().unwrap(),
        };
        let infinity = Point::new_infinity(&curve);
        let point =
            Point::new_point(&curve, &-1.to_bigint().unwrap(), &-1.to_bigint().unwrap()).unwrap();

        assert_eq!(
            point + infinity,
            Point::new_point(&curve, &-1.to_bigint().unwrap(), &-1.to_bigint().unwrap()).unwrap()
        );
    }

    #[test]
    fn test_add_vertical_line() {
        // This happens when points have the same x and different y coordinates
        let curve = RealWeierstrassCurve {
            a: 5.to_bigint().unwrap(),
            b: 7.to_bigint().unwrap(),
        };
        let point1 =
            Point::new_point(&curve, &-1.to_bigint().unwrap(), &1.to_bigint().unwrap()).unwrap();
        let point2 =
            Point::new_point(&curve, &-1.to_bigint().unwrap(), &-1.to_bigint().unwrap()).unwrap();

        assert_eq!(point1 + point2, Point::new_infinity(&curve));
    }

    #[test]
    fn test_add_same_point_with_vertical_slope() {
        // This happens when points are the same and have y == 0
        let curve = RealWeierstrassCurve {
            a: 0.to_bigint().unwrap(),
            b: 0.to_bigint().unwrap(),
        };
        let point1 =
            Point::new_point(&curve, &0.to_bigint().unwrap(), &0.to_bigint().unwrap()).unwrap();
        let point2 =
            Point::new_point(&curve, &0.to_bigint().unwrap(), &0.to_bigint().unwrap()).unwrap();

        assert!(point1 == point2);
        assert_eq!(point1 + point2, Point::new_infinity(&curve));
    }

    #[test]
    fn test_add_same_point() {
        // p(-1,-1) + p(-1,-1) = p(18,77)
        let curve = RealWeierstrassCurve {
            a: 5.to_bigint().unwrap(),
            b: 7.to_bigint().unwrap(),
        };
        let point1 =
            Point::new_point(&curve, &-1.to_bigint().unwrap(), &-1.to_bigint().unwrap()).unwrap();
        let point2 =
            Point::new_point(&curve, &-1.to_bigint().unwrap(), &-1.to_bigint().unwrap()).unwrap();

        assert!(point1 == point2);
        assert_eq!(
            point1 + point2,
            Point::new_point(&curve, &18.to_bigint().unwrap(), &77.to_bigint().unwrap()).unwrap()
        );
    }

    #[test]
    fn test_add_points_with_different_x() {
        // p(2,5) + p(-1,-1) = p(3,-7)
        let curve = RealWeierstrassCurve {
            a: 5.to_bigint().unwrap(),
            b: 7.to_bigint().unwrap(),
        };
        let point1 =
            Point::new_point(&curve, &2.to_bigint().unwrap(), &5.to_bigint().unwrap()).unwrap();
        let point2 =
            Point::new_point(&curve, &-1.to_bigint().unwrap(), &-1.to_bigint().unwrap()).unwrap();

        assert!(point1 != point2);
        assert_eq!(
            point1 + point2,
            Point::new_point(&curve, &3.to_bigint().unwrap(), &-7.to_bigint().unwrap()).unwrap()
        );
    }
}
