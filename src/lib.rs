use std::convert::From;
use std::cmp::PartialEq;
use std::cmp::PartialOrd;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Rem;
use std::ops::Sub;

use num::Zero;

use crate::FieldElementError::{NegativeOrderError, NumberGreaterThanOrderError};

#[derive(Debug)]
pub enum FieldElementError {
    NegativeOrderError,
    NumberGreaterThanOrderError,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FieldElement<T> {
    number: T,
    order: T,
}

impl<T> FieldElement<T>
    where T: Mul + Sub + Rem + Zero + Copy + From<i32> + From<<T as std::ops::Rem>::Output> + From<<T as std::ops::Sub>::Output> + From<<T as std::ops::Mul>::Output> + PartialEq + PartialOrd {
    pub fn new(number: T, order: T) -> Result<Self, FieldElementError> {
        if order < T::zero() {
            return Err(NegativeOrderError)
        } else if number > order {
            return Err(NumberGreaterThanOrderError)
        }

        Ok(FieldElement {
            number: (number % order).into(),
            order,
        })
    }

    pub fn pow(self, mut exp: T) -> FieldElement<T> {
        if exp < T::zero() {
            exp = (T::from(self.order - T::from(1)) + exp).into();
        }

        let mut r = T::from(1);

        while exp > T::zero() {
            r = (T::from(r * self.number) % self.order).into();
            exp = (exp - T::from(1)).into();
        }

        FieldElement {
            number: (r % self.order).into(),
            order: self.order,
        }
    }
}

impl<T> Add for FieldElement<T>
    where T: Add + Rem + Copy + From<<T as std::ops::Add>::Output> + From<<T as std::ops::Rem>::Output> + PartialEq + PartialOrd {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.order != rhs.order {
            panic!("The orders do not match!");
        } else {
            FieldElement {
                number: (T::from(self.number + rhs.number) % self.order).into(),
                order: self.order,
            }
        }
    }
}

impl<T> Sub for FieldElement<T>
    where T: Add + Sub + Rem + Copy + From<<T as std::ops::Add>::Output> + From<<T as std::ops::Sub>::Output> + From<<T as std::ops::Rem>::Output> + PartialEq + PartialOrd {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.order != rhs.order {
            panic!("The orders do not match!");
        } else {
            FieldElement {
                number: (T::from(T::from(self.number - rhs.number) + self.order) % self.order).into(),
                order: self.order,
            }
        }
    }
}

impl<T> Mul for FieldElement<T>
    where T: Mul + Rem + Copy + From<<T as std::ops::Mul>::Output> + From<<T as std::ops::Rem>::Output> + PartialEq + PartialOrd {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.order != rhs.order {
            panic!("The orders do not match!");
        } else {
            FieldElement {
                number: (T::from(self.number * rhs.number) % self.order).into(),
                order: self.order,
            }
        }
    }
}


impl<T> Div for FieldElement<T>
    where T: Mul + Sub + Rem + Zero + Copy + From<i32> + From<<T as std::ops::Rem>::Output> + From<<T as std::ops::Sub>::Output> + From<<T as std::ops::Mul>::Output> + PartialEq + PartialOrd {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        if self.order != rhs.order {
            panic!("The orders do not match!");
        } else {
            let mut inverse = T::from(-1);
            let divisor = rhs.pow(inverse);
            self * divisor
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::FieldElement;

    #[test]
    fn add_works() {
        let f1 = FieldElement::<i64>::new(6, 17).unwrap();
        let f2 = FieldElement::<i64>::new(13, 17).unwrap();
        let f3 = FieldElement::<i64>::new(2, 17).unwrap();

        assert_eq!(f1 + f2, f3);
    }

    #[test]
    fn subtract_works() {
        let f1 = FieldElement::<i64>::new(6, 17).unwrap();
        let f2 = FieldElement::<i64>::new(13, 17).unwrap();
        let f3 = FieldElement::<i64>::new(10, 17).unwrap();

        assert_eq!(f1 - f2, f3);
    }

    #[test]
    fn multiply_works() {
        let f1 = FieldElement::<i64>::new(6, 17).unwrap();
        let f2 = FieldElement::<i64>::new(13, 17).unwrap();
        let f3 = FieldElement::<i64>::new(10, 17).unwrap();

        assert_eq!(f1 * f2, f3);
    }

    #[test]
    fn pow_works() {
        let f1 = FieldElement::<i64>::new(6, 17).unwrap();
        let f3 = FieldElement::<i64>::new(2, 17).unwrap();

        let f2 = f1.pow(2);

        assert_eq!(f2, f3);
    }

    #[test]
    fn div_works() {
        let f1 = FieldElement::<i64>::new(10, 17).unwrap();
        let f2 = FieldElement::<i64>::new(13, 17).unwrap();
        let f3 = FieldElement::<i64>::new(6, 17).unwrap();

        assert_eq!(f1 / f2, f3);
    }
}
