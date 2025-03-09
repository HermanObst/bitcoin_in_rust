// This module defines the `EllipticCurve` trait, which serves as a generic interface
// for elliptic curves over various mathematical fields.
//
// An elliptic curve is a mathematical structure described by the equation:
//
//     y² = x³ + ax + b
//
// Different implementations of this trait will define the curve over specific fields,
// such as real numbers (`ℝ`), finite fields (`𝔽_p`), or binary fields (`𝔽_2^m`).
//
// Implementing this trait for a field allows the use of elliptic curve operations
// such as point addition, doubling, and scalar multiplication.

pub(crate) trait EllipticCurve where Self::Field: std::fmt::Debug {
    type Field;

    fn a(&self) -> Self::Field;
    fn b(&self) -> Self::Field;

    fn defining_equation(&self, x: &Self::Field, y: &Self::Field) -> Self::Field;
}

#[derive(Debug)]
pub(crate) struct Point<'a, E: EllipticCurve> {
    pub(crate) coords: Coords<E>,
    pub(crate) curve: &'a E,
}

#[derive(Debug)]
pub(crate) enum Coords<E: EllipticCurve> {
    Point(E::Field, E::Field),
    Infinity,
}

