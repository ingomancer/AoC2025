use std::ops::{Add, Div, Mul, Sub};

use num::Num;

// [a.0,a.1] + [b.0,b.1] = [a.0 + b.0, a.1 + b.1]
pub fn add_tuple<T: Add<Output = T>>(a: (T, T), b: (T, T)) -> (T, T) {
    (a.0 + b.0, a.1 + b.1)
}

// [a.0,a.1] * [b.0,b.1] = [a.0 * b.0 - a.1 * b.1, a.0 * b.1 + a.1 * b.0]
pub fn mul_tuple<T: Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Copy>(
    a: (T, T),
    b: (T, T),
) -> (T, T) {
    (a.0 * b.0 - a.1 * b.1, a.0 * b.1 + a.1 * b.0)
}

// [a.0,a.1] / [b.0,b.1] = [a.0 / b.0, a.1 / b.1]
pub fn div_tupl<T: Div<Output = T>>(a: (T, T), b: (T, T)) -> (T, T) {
    (a.0 / b.0, a.1 / b.1)
}

pub struct Clamped<T: Num + PartialOrd + Copy> {
    inner: T,
    lower: T,
    upper: T,
}

impl<T: Num + PartialOrd + Copy> Clamped<T> {
    pub fn new(inner: T, lower: T, upper: T) -> Clamped<T> {
        Clamped {
            inner,
            lower,
            upper,
        }
    }

    pub fn get(self) -> T {
        self.inner
    }
}

impl<T: Num + PartialOrd + Copy> Add<T> for Clamped<T> {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let mut new = self.inner + rhs;
        if new > self.upper {
            new = self.upper
        }
        if new < self.lower {
            new = self.lower
        }
        Clamped { inner: new, ..self }
    }
}

impl<T: Num + PartialOrd + Copy> Add for Clamped<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self + rhs.inner
    }
}

impl<T: Num + PartialOrd + Copy> Sub<T> for Clamped<T> {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        let mut new = self.inner - rhs;
        if new > self.upper {
            new = self.upper
        }
        if new < self.lower {
            new = self.lower
        }
        Clamped { inner: new, ..self }
    }
}

impl<T: Num + PartialOrd + Copy> Sub for Clamped<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self - rhs.inner
    }
}
