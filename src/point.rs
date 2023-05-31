use bitcoin::types::errors::Errors;
use num_bigint::BigInt;

#[derive(Clone, Debug, PartialEq)]
pub enum Point<const A: i64, const B: i64> {
    Point(BigInt, BigInt),
    Infinity
}

impl<const A: i64, const B:i64> Point<A, B> {
    fn new_point(x: BigInt, y: BigInt) -> Result<Self, Errors> {
        // Checks if point is included in the curve y2 = x3 + ax + b
        if y.pow(2) != x.pow(3) + A * &x + B {
            return Err(Errors::InvalidPoint);
        }

        return Ok(Point::<A, B>::Point(x, y));
        }

    fn new_infinity() -> Self {
        Point::<A,B>::Infinity
    }

    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Point::Point(x1, y1), Point::Point(x2, y2)) => x1 == x2 && y1 == y2,
            (Point::Infinity, Point::Infinity) => true,
            _ => false,
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
}