mod field;
mod poly;

use field::Field;
use poly::Poly;

fn main() {
    let x = 2 + 2;
    let y = f(3).inv();
    println!("Hello, world! {}", x);
    println!("Field Addition {}", y);

    let b = Poly(vec![f(3),f(4),f(5)]);
    println!("This is {}", b);
}

fn f(v: u64) -> Field {
    Field::new(v)
}

// To implement Groth 16
//
// 1. We decide a prime order (p)
// 2. We need 2 groups G1 and G2 with pairing friendly algorithm (e)
// 3. R1CS definition
// 3. R1CS -> QAP
// 4. Random Number Generator
// 5. CRS construction
// 6. Proving Algorithm + Verification Algorithm
