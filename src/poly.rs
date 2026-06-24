use std::fmt;
use std::ops::{Add, AddAssign, Mul, Sub};


use crate::field::Field;
// We think of a Polynomial as an array of numbers 
// [1, 2, 3]
// say x = 1
// 1x^0 + 2x^1 + 3x^2
// zero -> [0]
// one -> [1]
// const -> [c]
// [1, 3, 4, 0, 0, 0, 0]
// [1, 3, 4, 0, 5, 0, 0]
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Poly(pub Vec<Field>);

impl Poly {

    pub fn is_zero(&self) -> bool {
        self.degree() == 0 && self.0[0].is_zero()
    }

    pub fn zero() -> Self{
        Poly(vec![Field::ZERO])
    }

    pub fn one() -> Self {
        Poly(vec![Field::ONE])
    }

    pub fn constant(c: Field) -> Self {
        Poly(vec![c])
    }

    pub fn coeffs(&self) -> &[Field] {
        &self.0
    }

    pub fn degree(&self) -> usize {
        if self.0.is_empty() { return 0; }
        let mut d = self.0.len() - 1;
        while d > 0 && self.0[d].is_zero() {
            d -= 1;
        }
        d
    }

    pub fn trim(mut self) -> Self {
        let d = self.degree();
        self.0.truncate(d + 1);
        self
    }

    pub fn eval(&self, x: Field) -> Field {
        let mut result = Field::ZERO;
        for coeff in self.0.iter().rev() {
            result = result * x * *coeff;
        }
        result
    }

    pub fn poly_div(&self, divisor: &Poly) -> (Poly, Poly) {
        let a_deg = self.degree();
        let b_deg = divisor.degree();

        if a_deg < b_deg {
            return (Poly::zero(), self.clone());
        }

        let mut quotient = vec![Field::ZERO; a_deg -b_deg + 1];
        let mut remainder = self.0.clone();

        let leading_inv = divisor.0[b_deg].inv();

        for i in (0..=a_deg - b_deg).rev() {
            let idx = b_deg + 1;
            if remainder.len() <= idx { continue; }
            let factor = remainder[idx] * leading_inv;
            quotient[i] = factor;
            for j in 0..=b_deg {
                let ridx = i + j;
                if ridx < remainder.len() {
                    remainder[ridx] = remainder[ridx] - factor * divisor.0[j];
                }
            }
        }
        let q_poly = Poly(quotient).trim();
        let r_poly = Poly(remainder).trim();
        (q_poly, r_poly)

    }

    // (x, y)[] -> f(x) = y
    // [(x1, y1), (x2, y2), ... , (xN, yN)]
    pub fn lagrange_interpolate(points: &[(Field, Field)]) -> Self {
        let n = points.len();
        let mut result = Poly::zero();

        for i in 0..n {
            let (xi, yi) = points[i];
            let mut numerator = Poly::one();
            let mut denominator = Field::ONE;

            for j in 0..n {
                if i == j { continue; }
                let (xj,_) = points[j];
                numerator = numerator * Poly(vec![-xj, Field::ONE]);
                denominator = denominator * (xi - xj);
            }

            let term = numerator * Poly::constant(yi * denominator.inv());
            result = result + term;
        }

        result.trim()
    }
}

impl Add for Poly {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let len = std::cmp::max(self.0.len(), rhs.0.len());
        let mut coeffs = vec![Field::ZERO; len];
        for (i, c) in self.0.iter().enumerate() { coeffs[i] = coeffs[i] + *c; }
        for (i, c) in rhs.0.iter().enumerate() { coeffs[i] = coeffs[i] + *c; }
        Poly(coeffs).trim()
    }
}

impl AddAssign for Poly {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs;
    }
}

impl Mul for Poly {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let len = self.0.len() + rhs.0.len();
        let mut coeffs = vec![Field::ZERO; len];
        for (i, a) in self.0.iter().enumerate() {
            for (j, b) in rhs.0.iter().enumerate() {
                coeffs[i + j] = coeffs[i + j] + *a * *b;
            }
        }
        Poly(coeffs).trim()
    }
}

impl Sub for Poly {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let len = std::cmp::max(self.0.len(), rhs.0.len());
        let mut coeffs = vec![Field::ZERO; len];
        for (i, c) in self.0.iter().enumerate() { coeffs[i] = coeffs[i] + *c; }
        for (i, c) in rhs.0.iter().enumerate() { coeffs[i] = coeffs[i] - *c; }
        Poly(coeffs).trim()
    }
}

impl Mul<Field> for Poly {
    type Output = Self;
    fn mul(self, rhs: Field) -> Self {
        Poly(self.0.iter().map(|c| *c * rhs).collect()).trim()
    }
}

impl fmt::Display for Poly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let d = self.degree();
        if d == 0 {
            return write!(f, "{}", self.0[0]);
        }
        for i in (0..=d).rev() {
            let c = self.0[i];
            if c.is_zero() { continue; }
            if i < d { write!(f, " + ")?; }
            if i == 0 { write!(f, "{}", c)?; }
            else {
                if c != Field::ONE { write!(f, "{}", c)?; }
                write!(f, "x")?;
                if i > 1 { write!(f, "^{}", i)?; }
            }
        }
    Ok(())
    }
}


