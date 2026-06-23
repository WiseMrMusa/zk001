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
pub struct Poly(Vec<Field>);

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
}
