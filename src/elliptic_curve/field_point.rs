use std::ops::Add;

use crate::types::errors::Errors;
use crate::elliptic_curve::{traits::{EllipticCurve, Point, Coords}, finite_field::FieldElement};

#[derive(Debug)]
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
    fn new_point(curve: &'a WeierstrassCurve, x: &FieldElement, y: &FieldElement) -> Result<Self, Errors> {
        if curve.defining_equation(x, y) != FieldElement::zero(x.prime()) {
            return Err(Errors::InvalidPoint);
        }

        Ok(Point { coords: Coords::Point(x.clone(), y.clone()), curve })
    }

    fn new_infinity(curve: &'a WeierstrassCurve) -> Self {
        Point { coords: Coords::Infinity, curve }
    }
}   

impl<'a> PartialEq for Point<'a, WeierstrassCurve> {
    fn eq(&self, other: &Self) -> bool {
        match (&self.coords, &other.coords) {
            (Coords::Point(x1, y1), Coords::Point(x2, y2)) => x1 == x2 && y1 == y2,
            (Coords::Infinity, Coords::Infinity) => true,
            _ => false,
        }
    }
}   

impl<'a> Add for Point<'a, WeierstrassCurve> {
    type Output = Self;

    fn add(self, _other: Self) -> Self {
        todo!()
    }
}

