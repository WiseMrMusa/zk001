use std::fmt;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign, Neg}; 

// Field (num)
#[derive(Debug, Clone, Copy)]
pub struct Field(u64);

const MOD: u64 = 13;
          
impl Field {
    pub const ONE: Field = Field(1);
    pub const ZERO: Field = Field(0);

    pub fn new(value: u64) -> Self {
        Field(value % MOD) 
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    // To do the inverse, be sure the number is not zero
    // The inverse of a is a^(MOD -2)
    // FLT: a^(p-1) = 1 (mod p)
    // a^(p-2) = 1/a (mod p)
    pub fn inv(&self) -> Self {
        assert!(!self.is_zero(), "Undefined: Zero Inverse");
        self.pow(MOD - 2)
    } 

    // We use the binary exponentiation to find the power
    pub fn pow(&self, mut exp: u64) -> Self {
        let mut base = *self;
        let mut result = Field::ONE;
        while exp > 0 {
            if exp & 1 == 1 {
                result = result * base;
            }
            base = base * base;
            exp >>= 1;
        }
        result
    }

    pub fn neg(&self) -> Self {
        if self.is_zero() { Field::ZERO } else { Field(MOD - self.0) } 
    }
    
}


impl Add for Field {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let s = self.0 + rhs.0;
        Field (if s >= MOD { s - MOD } else { s } )
    }
}

impl AddAssign for Field {
    fn add_assign(&mut self, rhs: Self) { *self = *self + rhs ;}
}

impl Sub for Field {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        if self.0 >= rhs.0 {
            Field(self.0 - rhs.0)
        } else {
            Field((self.0 - rhs.0) + MOD)
        }
    }
}

impl SubAssign for Field {
    fn sub_assign(&mut self, rhs: Self) { *self = *self - rhs; }
}


impl Mul for Field {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Field ((self.0 as u64 * rhs.0 as u64) % MOD)
    }
}

impl MulAssign for Field {
    fn mul_assign(&mut self, rhs: Self) { *self = *self * rhs;}
}

impl Neg for Field {
    type Output = Self;
    fn neg(self) -> Self {
        Field::neg(&self)
    }
}

impl Div for Field {
    type Output = Self;
    fn div(self, rhs: Self) -> Self { self * rhs.inv() }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
