// Create struct for a finite field element.
use num_bigint::BigInt;
use num_bigint::ToBigInt;


#[derive(Debug, PartialEq, Eq)]
struct FieldElement {
    num: BigInt,
    prime: BigInt,
}

#[allow(dead_code)]
impl FieldElement {
    fn new(num: BigInt, prime: BigInt) -> FieldElement {
        FieldElement {
            num,
            prime,
        }
    }

    fn eq(&self, elem: Option<FieldElement>) -> bool {
        match elem {
            Some(field_elem) => self.num == field_elem.num && self.prime == field_elem.prime,
            None => false,
        }
    }

    fn add(&self, elem: &FieldElement) -> FieldElement {
        assert!(self.prime == elem.prime, "Cannot add two numbers in different fields");
        let num = (self.num.clone() + elem.num.clone()).rem_euclid(self.prime.clone());

        FieldElement::new(num, self.prime.clone())
    }

    fn sub(&self, elem: &FieldElement) -> FieldElement {
        assert!(self.prime == elem.prime, "Cannot subtract two numbers in different fields");
        let num = (self.num.clone() - elem.num.clone()).rem_euclid(self.prime.clone());

        FieldElement::new(num, self.prime.clone())
    }

    fn mul(&self, elem: &FieldElement) -> FieldElement {
        assert!(self.prime == elem.prime, "Cannot multiply two numbers in different fields");
        let num = (self.num.clone() * &elem.num).rem_euclid(self.prime.clone());

        FieldElement::new(num, self.prime.clone())
    }

    fn pow(&self, exp: &BigInt) -> FieldElement {
        let positive_exponent = exp.rem_euclid(self.prime.clone() - 1);
        let num = self.num.modpow(&positive_exponent, &self.prime);

        FieldElement::new(num, self.prime.clone())
    }

    fn truediv(&self, elem: &FieldElement) -> FieldElement {
        assert!(self.prime == elem.prime, "Cannot divide two numbers in different fields");
        let factor = elem.num.modpow(&(self.prime.clone() - 2_i32.to_bigint().unwrap()), &self.prime);
        let num = (self.num.clone() * factor) % self.prime.clone();

        FieldElement::new(num, self.prime.clone())
    }
}

trait RemEuclid {
    fn rem_euclid(&self, rhs: Self) -> Self;
}

impl RemEuclid for BigInt {
    fn rem_euclid(&self, rhs: Self) -> Self {
        self.modpow(&1_i32.to_bigint().unwrap(), &rhs)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_field_element() {
        let num = 4.to_bigint().unwrap();
        let prime = 7.to_bigint().unwrap();
        // Create a field element
        let field_element = FieldElement::new(num, prime);

        assert_eq!(field_element.num.clone(), 4.to_bigint().unwrap());
        assert_eq!(field_element.prime, 7.to_bigint().unwrap());
    }

    #[test]
    fn two_field_elements_are_equal() {
        let num1 = 3;
        let num2 = 4;
        let prime1 = 7;
        let prime2 = 11;

        let field_element1 = FieldElement::new(num1.to_bigint().unwrap(), prime1.to_bigint().unwrap());
        let field_element2 = FieldElement::new(num1.to_bigint().unwrap(), prime1.to_bigint().unwrap());
        let field_element3 = FieldElement::new(num2.to_bigint().unwrap(), prime1.to_bigint().unwrap());
        let field_element4 = FieldElement::new(num1.to_bigint().unwrap(), prime2.to_bigint().unwrap());

        assert_eq!(field_element1.eq(Some(field_element2)), true);
        assert_eq!(field_element1.eq(Some(field_element3)), false);
        assert_eq!(field_element1.eq(Some(field_element4)), false);
        assert_eq!(field_element1.eq(None), false);
    }

    #[test]
    fn add_field_elements() {
        let field_element1 = FieldElement::new(7.to_bigint().unwrap(), 13.to_bigint().unwrap());
        let field_element2 = FieldElement::new(12.to_bigint().unwrap(), 13.to_bigint().unwrap());
        let result = field_element1.add(&field_element2);

        assert_eq!(result.num, 6.to_bigint().unwrap());
        assert_eq!(result.prime, 13.to_bigint().unwrap());
    }

    #[test]
    fn sub_field_elements() {
        let field_element1 = FieldElement::new(7.to_bigint().unwrap(), 13.to_bigint().unwrap());
        let field_element2 = FieldElement::new(12.to_bigint().unwrap(), 13.to_bigint().unwrap());
        let result = field_element1.sub(&field_element2);

        assert_eq!(result.num, 8.to_bigint().unwrap());
        assert_eq!(result.prime, 13.to_bigint().unwrap());
    }

    #[test]
    fn mul_field_elements() {
        let field_element1 = FieldElement::new(3.to_bigint().unwrap(), 13.to_bigint().unwrap());
        let field_element2 = FieldElement::new(12.to_bigint().unwrap(), 13.to_bigint().unwrap());
        let result = field_element1.mul(&field_element2);

        assert_eq!(result.num, 10.to_bigint().unwrap());
        assert_eq!(result.prime, 13.to_bigint().unwrap());
    }

    #[test]
    fn pow_field_elements() {
        let field_element1 = FieldElement::new(17.to_bigint().unwrap(), 31.to_bigint().unwrap());
        let exp = 3.to_bigint().unwrap();
        let result = field_element1.pow(&exp);

        assert_eq!(result.num, 15.to_bigint().unwrap());
        assert_eq!(result.prime, field_element1.prime);
    }

    #[test]
    fn truediv_field_elements() {
        let field_element1 = FieldElement::new(3.to_bigint().unwrap(), 31.to_bigint().unwrap());
        let field_element2 = FieldElement::new(24.to_bigint().unwrap(), 31.to_bigint().unwrap());
        let result = field_element1.truediv(&field_element2);

        assert_eq!(result.num, 4.to_bigint().unwrap());
        assert_eq!(result.prime, field_element1.prime.clone());
    }
}
