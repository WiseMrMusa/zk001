use crate::field::Field;
use crate::poly::Poly;
use crate::r1cs::QAP;

#[derive(Debug, Clone)]
pub struct CRS {
    pub alpha: Field, // encoding A
    pub beta: Field, // encoding B
    pub gamma: Field, // encoding public
    pub delta: Field, // encoding private
    pub tau: Field, // secret evaluation point
    
    pub tau_powers: Vec<Field>,

    pub public_terms: Vec<Field>,
    pub private_terms: Vec<Field>,

    pub h_powers: Vec<Field>,
}

#[derive(Debug, Clone)]
pub struct Proof {
    pub a: Field,
    pub b: Field,
    pub c: Field,
}

pub fn setup(qap: &QAP, rng: &mut impl FnMut() -> Field) -> CRS {
    let alpha = rng();
    let beta = rng();
    let gamma = rng();
    let delta = rng();
    let tau = rng();

    let n = qap.t.degree();

    // [ 1, s, s^2, s^3, ... , s^n-1 ]
    let mut tau_powers = Vec::with_capacity(n);
    let mut pow = Field::ONE;
    for _ in 0..n {
        tau_powers.push(pow);
        pow = pow * tau;
    }

    let t_at_tau = qap.t.eval(tau);

    let mut public_terms = Vec::with_capacity(qap.num_pub);
    for i in 0..qap.num_pub {
        let ui = qap.u[i].eval(tau);
        let vi = qap.v[i].eval(tau);
        let wi = qap.w[i].eval(tau);
        let combined = (beta * ui) + (alpha * vi) + wi;
        public_terms.push(combined/gamma);
    }

    let mut private_terms = Vec::with_capacity(qap.num_vars - qap.num_pub);
    for i in qap.num_pub..qap.num_vars {
        let ui = qap.u[i].eval(tau);
        let vi = qap.v[i].eval(tau);
        let wi = qap.w[i].eval(tau);
        let combined = (beta * ui) + (alpha * vi) + wi;
        private_terms.push(combined/delta);
    }

    let mut h_powers = Vec::with_capacity(n-1);
    let mut pow_tau = Field::ONE;
    for _ in 0..n-1 {
        h_powers.push(
            pow_tau * t_at_tau / delta);
        pow_tau = pow_tau * tau;
    }

    CRS {
        alpha,
        beta,
        gamma,
        delta,
        tau_powers,
        public_terms,
        private_terms,
        h_powers,
        tau,
    }
}

pub fn prove(crs: &CRS, qap: &QAP, witness: &[Field], r: Field, s: Field) -> Proof {
    let num_pub = qap.num_pub;
    let num_vars = qap.num_vars;

    let mut u_sum = Field::ZERO;
    let mut v_sum = Field::ZERO;
    let mut c_sum = Field::ZERO;

    for i in 0..num_vars {
        let ui = qap.u[i].eval(crs.tau);
        let vi = qap.v[i].eval(crs.tau);
        u_sum = u_sum + ui * witness[i];
        v_sum = v_sum + vi * witness[i];

        if i >= num_pub {
            c_sum = c_sum + witness[i] * crs.private_terms[i - num_pub];
        }
    }

    let h_poly = qap.compute_h(witness);
    let h_at_tau = h_poly.eval(crs.tau);
    let h_term = h_at_tau * crs.h_powers[0] / crs.tau_powers[0];



    let a = crs.alpha + u_sum + r * crs.delta;
    let b = crs.beta + v_sum + s * crs.delta;
    let c = c_sum + h_term + a * s + r * b - r * s * crs.delta;

    Proof { a, b, c } 
}

pub fn verify (crs: &CRS, qap: &QAP, public_inputs: &[Field], proof: &Proof) -> bool {
    assert_eq!(public_inputs.len(), qap.num_pub);

    let mut public_sum = Field::ZERO;
    for i in 0..qap.num_pub {
        public_sum = public_sum + public_inputs[i] * crs.public_terms[i];
    }

    let lhs = proof.a * proof.b;
    let rhs = crs.alpha * crs.beta 
        + public_sum * crs.gamma
        + proof.c * crs.delta;

    lhs == rhs
}
