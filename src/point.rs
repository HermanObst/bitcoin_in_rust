use bitcoin::types::errors::Errors;
use num_bigint::BigInt;

#[derive(Clone, Debug, PartialEq)]
pub enum Point<const A: i64, const B: i64> {
    Point(BigInt, BigInt),
    Infinity
}

impl<const A: i64, const B:i64> Point<A, B> {
    fn new_point(x: Option<BigInt>, y: Option<BigInt>) -> Result<Self, Errors> {
        match (x, y) {
            (Some(x_num), Some(y_num)) => {
                let y_squared = y_num.pow(2);
                let x_cubed = x_num.pow(3);
                let right_side = x_cubed + A * &x_num + B;

                if y_squared != right_side {
                    return Err(Errors::PointNotInCurve);
            }
            return Ok(Point::<A, B>::Point(x_num, y_num));
            }
            (None, Some(_y_num)) => {
                return Err(Errors::PointNotInCurve); 
            }
            (Some(_x_num), None) => {
                return Err(Errors::PointNotInCurve); 
            }
            (None, None) => {
                return Ok(Point::<A,B>::Infinity)
            }
        }
    }

    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Point::Point(x1, y1),
                Point::Point(x2, y2),
            ) => x1 == x2 && y1 == y2 && A == other.get_a() && B == other.get_b(),
            (Point::Infinity, Point::Infinity) => A == other.get_a() && B == other.get_b(),
            _ => false,
        }
    }

    // Helper methods to get the values of A and B
    fn get_a(&self) -> i64 {
        A
    }

    fn get_b(&self) -> i64 {
        B
    }
}


#[cfg(test)]
mod point_tests {
    use num_bigint::ToBigInt;

    use super::*;

    #[test]
    fn test00_create_valid_point() {
        assert!(Point::<5, 7>::new_point(Some(-1.to_bigint().unwrap()),Some(-1.to_bigint().unwrap())).is_ok());
    }

    #[test]
    fn test01_create_valid_point_and_check_result() {
        let result = Point::<5, 7>::new_point(Some(-1.to_bigint().unwrap()), Some(-1.to_bigint().unwrap()));
        assert!(result.is_ok());
    
        let point = result.unwrap();
        assert_eq!(point, Point::<5, 7>::Point(-1.to_bigint().unwrap(), -1.to_bigint().unwrap()));
    }
}