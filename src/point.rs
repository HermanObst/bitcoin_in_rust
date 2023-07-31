use bitcoin::types::errors::Errors;
use num_bigint::{BigInt, ToBigInt};
use core::ops::Add;

#[allow(dead_code)]
#[derive(Clone, Debug)]
enum Point<const A: i64, const B: i64> {
    Point(BigInt, BigInt),
    Infinity
}

#[allow(dead_code)]
impl<const A: i64, const B:i64> Point<A, B> {
    fn new_point(x: BigInt, y: BigInt) -> Result<Self, Errors> {
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

#[allow(dead_code)]
impl<const A: i64, const B: i64> PartialEq for Point<A, B> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Point::Point(x1, y1), Point::Point(x2, y2)) => x1 == x2 && y1 == y2,
            (Point::Infinity, Point::Infinity) => true,
            _ => false,
        }
    }
}

impl<const A: i64, const B: i64> Add<Point<A, B>> for Point<A, B> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self.clone(), other.clone()) {
            (Point::Point(x1,y1), Point::Point(x2,y2)) => {
                if x1 == x2 {
                    if y1 == y2 {
                        // Case when P1 == P2
                        let slope: BigInt = (3*&x1.pow(2) + A)/2*&y1;

                        // When tanget line is vertical
                        if slope == 0.to_bigint().unwrap() {
                            return Point::new_infinity()
                        }
                        let x3 = slope.pow(2) - 2*&x1;
                        let y3 = slope*(&x1 - &x3) - y1;
                        // This unwrap cannot fail as this functions already recives two valid points.
                        Point::new_point(x3, y3).unwrap()
                    } else {
                        // Vertical line (same x but different y coordinates)
                        Point::new_infinity()
                    }
                } else {
                    // Case were x coordinates are differents
                    let slope = (&y1 - &y2)/(&x2 - &x1);
                    let x3 = slope.pow(2) - &x1 - &x2;
                    let y3 = slope*(&x1 - &x3) - &y1;
                    // This unwrap cannot fail as this functions already recives two valid points.
                    Point::new_point(x3, y3).unwrap()
                }
            },
            // Handle identity (Infinity point). In case both are Infinity, returns Infinity (self).
            (_, Point::Infinity) => self,
            (Point::Infinity, _) => other
        }
    }
}

#[cfg(test)]
mod point_tests {
    use num_bigint::ToBigInt;

    use super::*;

    #[test]
    fn test_create_valid_point() {
        assert!(Point::<5, 7>::new_point(-1.to_bigint().unwrap(),-1.to_bigint().unwrap()).is_ok());
    }

    #[test]
    fn test_create_valid_point_and_check_result() {
        let result = Point::<5, 7>::new_point(-1.to_bigint().unwrap(), -1.to_bigint().unwrap());
        assert!(result.is_ok());
    
        let point = result.unwrap();
        assert_eq!(point, Point::<5, 7>::Point(-1.to_bigint().unwrap(), -1.to_bigint().unwrap()));
    }

    #[test]
    fn test_create_valid_point_at_infinity() {
        assert_eq!(Point::<5,7>::new_infinity(), Point::<5,7>::Infinity);
    }

    #[test]
    fn test_eq() {
        assert!(Point::<5,7>::new_infinity() == Point::<5,7>::Infinity);
        assert!(Point::<5, 7>::Point(-1.to_bigint().unwrap(), -1.to_bigint().unwrap()) == Point::<5, 7>::Point(-1.to_bigint().unwrap(), -1.to_bigint().unwrap()));
        assert!(Point::<5, 7>::Point(-1.to_bigint().unwrap(), -1.to_bigint().unwrap()) != Point::<5, 7>::Point(-1.to_bigint().unwrap(), 1.to_bigint().unwrap())); 
        assert!(Point::<5, 7>::Infinity != Point::<5, 7>::Point(-1.to_bigint().unwrap(), 1.to_bigint().unwrap()));  
    } 

    #[test]
    fn test_add_infinity_to_point() {
        let infinity = Point::<5,7>::new_infinity();
        let point = Point::<5,7>::new_point(-1.to_bigint().unwrap(),-1.to_bigint().unwrap()).unwrap();

        assert_eq!(infinity + point, Point::<5,7>::new_point(-1.to_bigint().unwrap(),-1.to_bigint().unwrap()).unwrap());
    }

    #[test]
    fn test_add_infinity_to_point_reverse() {
        let infinity = Point::<5,7>::new_infinity();
        let point = Point::<5,7>::new_point(-1.to_bigint().unwrap(),-1.to_bigint().unwrap()).unwrap();

        assert_eq!(point + infinity, Point::<5,7>::new_point(-1.to_bigint().unwrap(),-1.to_bigint().unwrap()).unwrap());
    }
}