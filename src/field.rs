// Field (num)
pub struct Field(u64);

const MOD: u64 = 13;
          
impl Field {
    pub const ONE: Field(1);
    pub const ZERO: Field(0);

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
        let mut result 
    }
    
}
