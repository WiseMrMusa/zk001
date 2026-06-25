use crate::field::Field;
use crate::poly::Poly;

#[derive(Debug, Clone)]
pub struct R1CS {
    pub num_vars: usize,
    pub num_pub: usize,
    pub num_constraints: usize,
    pub a: Vec<Vec<Field>>,
    pub b: Vec<Vec<Field>>,
    pub c: Vec<Vec<Field>>,
}

#[serive(Debug, Clone)]
pub struct QAP {
    pub num_vars: usize,
    pub num_pub: usize,
    pub u: Vec<Poly>,
    pub v: Vec<Poly>,
    pub w: Vec<Poly>,
    pub t: Poly,
}

impl R1CS {
    pub fn new(num_vars: usize, num_pub: usize) -> Self {
        R1CS {
            num_vars,
            num_pub,
            num_constraints: 0,
            a: Vec::new(),
            b: Vec::new(),
            c: Vec::new(),
        }
    }

    pub fn add_constraint(
        &mut self,
        a: Vec<Field>,
        b: Vec<Field>,
        c: Vec<Field>,
        ) {
        assert_eq!(a.len(), self.num_vars);
        assert_eq!(b.len(), self.num_vars);
        assert_eq!(c.len(), self.num_vars);
        self.a.push(a);
        self.b.push(b);
        self.c.push(c);
        self.num_constraints += 1;
    }

    pub fn to_qap(&self) -> QAP {
        let n = self.num_constraints;
        let m = self.num_vars;

        let mut u = Vec::with_capacity(m);
        let mut v = Vec::with_capacity(m);
        let mut w = Vec::with_capacity(m);

        for var_idx in 0..m {
            let a_pts : Vec<(Field, Field)> = (0..n)
                .map(|i| (Field::new((i+1) as u64), self.a[i][var_idx]))
                .collect();
            let b_pts : Vec<(Field, Field)> = (0..n)
                .map(|i| (Field::new((i+1) as u64), self.b[i][var_idx]))
                .collect();
            let c_pts : Vec<(Field, Field)> = (0..n)
                .map(|i| (Field::new((i+1) as u64), self.c[i][var_idx]))
                .collect();

            u.push(Poly::lagrange_interpolate(&a_pts));
            v.push(Poly::lagrange_interpolate(&b_pts));
            w.push(Poly::lagrange_interpolate(&c_pts));
       }

        let mut t = Poly::one();
        for i in 0..n {
            let xi = Field::new((i + 1) as u64);
            t = t * Poly(vec![-xi, Field::ONE]);
        }

        QAP {
            num_vars: m,
            num_pub: self.num_pub,
            u,
            v,
            w,
            t,
        }
    }
}

impl QAP {
    pub fn compute_h(&self, witness: &[Field]) -> Poly {
        let mut u_sum = Poly::zero();
        let mut v_sum = Poly::zero();
        let mut w_sum = Poly::zero();

        for i in 0..self.num_vars {
            u_sum = u_sum + self.u[i].clone() * Poly::constant(witness[i]);
            v_sum = v_sum + self.v[i].clone() * Poly::constant(witness[i]);
            w_sum = w_sum + self.w[i].clone() * Poly::constant(witness[i]);
        }

        let lhs = u_sum * v_sum;
        let numerator = lhs - w_sum;
        let (h, remainder) = numerator.poly_div(&self.t);

        assert!(remainder.is_zero(), "QAP divisibility check failed: remainder = {}", remainder);
        h
    }
}
