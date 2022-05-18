// Create struct for a finite field element.

struct FieldElement {
	num: i32,
	prime: u32,
}

impl FieldElement {
	fn new(num: i32, prime: u32) -> FieldElement {
		FieldElement {
			num: num,
			prime: prime,
		}
	}

	fn eq(&self, elem: Option<FieldElement>) -> bool {
		match elem {
			Some(field_elem) => self.num == field_elem.num && self.prime == field_elem.prime,
			None => false,
		}
	}

	fn add(&self, elem: FieldElement) -> FieldElement {
		if self.prime != elem.prime {
			panic!("Cannot add two numbers in different fields");
		}
		let num = (self.num + elem.num).rem_euclid(self.prime as i32);

		FieldElement::new(num, self.prime)
	}

	fn sub(&self, elem: FieldElement) -> FieldElement {
		if self.prime != elem.prime {
			panic!("Cannot subtract two numbers in different fields");
		}
		let num = (self.num - elem.num).rem_euclid(self.prime as i32);

		FieldElement::new(num, self.prime)
	}

	fn mul(&self, elem: FieldElement) -> FieldElement {
		if self.prime != elem.prime {
			panic!("Cannot subtract two numbers in different fields");
		}
		let num = (self.num * elem.num).rem_euclid(self.prime as i32);

		FieldElement::new(num, self.prime)
	}

	fn pow(&self, exp: i32) -> FieldElement {
		let n = exp.rem_euclid(self.prime as i32 - 1);
		let num = i32::pow(self.num, n as u32);

		FieldElement::new(num.rem_euclid(self.prime as i32), self.prime)
	}

	fn truediv(&self, elem: FieldElement) -> FieldElement {
		if self.prime != elem.prime {
			panic!("Cannot subtract two numbers in different fields");
		}
		let num = self.num * i32::pow(elem.num, self.prime - 2);

		FieldElement::new(num.rem_euclid(self.prime as i32), self.prime)
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn create_field_element() {
		let num = 4;
		let prime = 7;
		// Create a field element
		let field_element = FieldElement::new(num, prime);

		assert_eq!(field_element.num, num);
		assert_eq!(field_element.prime, prime);
	}

	#[test]
	fn two_field_elements_are_equal() {
		let num1 = 3;
		let num2 = 4;
		let prime1 = 7;
		let prime2 = 11;

		let field_element1 = FieldElement::new(num1, prime1);
		let field_element2 = FieldElement::new(num1, prime1);
		let field_element3 = FieldElement::new(num2, prime1);
		let field_element4 = FieldElement::new(num1, prime2);

		assert_eq!(field_element1.eq(Some(field_element2)), true);
		assert_eq!(field_element1.eq(Some(field_element3)), false);
		assert_eq!(field_element1.eq(Some(field_element4)), false);
		assert_eq!(field_element1.eq(None), false);
	}

	#[test]
	fn add_field_elements() {
		let field_element1 = FieldElement::new(7, 13);
		let field_element2 = FieldElement::new(12, 13);
		let result = field_element1.add(field_element2);

		assert_eq!(result.num, 6);
		assert_eq!(result.prime, 13);
	}

	#[test]
	fn sub_field_elements() {
		let field_element1 = FieldElement::new(7, 13);
		let field_element2 = FieldElement::new(12, 13);
		let result = field_element1.sub(field_element2);

		assert_eq!(result.num, 8);
		assert_eq!(result.prime, 13);
	}

	#[test]
	fn mul_field_elements() {
		let field_element1 = FieldElement::new(3, 13);
		let field_element2 = FieldElement::new(12, 13);
		let result = field_element1.mul(field_element2);

		assert_eq!(result.num, 10);
		assert_eq!(result.prime, 13);
	}

	#[test]
	fn pow_field_elements() {
		let field_element1 = FieldElement::new(17, 31);
		let exp = 3;
		let result = field_element1.pow(exp);

		assert_eq!(result.num, 15);
		assert_eq!(result.prime, field_element1.prime);
	}

	#[test]
	fn truediv_field_elements() {
		let field_element1 = FieldElement::new(3, 31);
		let field_element2 = FieldElement::new(24, 31);
		let result = field_element1.truediv(field_element2);

		assert_eq!(result.num, 4);
		assert_eq!(result.prime, field_element1.prime);
	}
}
