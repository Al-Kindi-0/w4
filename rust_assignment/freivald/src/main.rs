#![allow(non_snake_case)]
use std::ops::{Deref, Mul};

use ark_ff::{fields::Fp64, MontBackend, MontConfig, MontFp};
use ark_std::rand::Rng;

#[derive(MontConfig)]
#[modulus = "17"]
#[generator = "3"]
pub struct FpConfigMont;
pub type Fp = Fp64<MontBackend<FpConfigMont, 1>>;

type Vector = Vec<Fp>;
pub struct Matrix(Vec<Vector>);

impl Deref for Matrix {
    type Target = Vec<Vector>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Mul<&Vector> for &Matrix {
    type Output = Vector;

    fn mul(self, rhs: &Vector) -> Self::Output {
        let mut result = Vector::new();
        self.iter().for_each(|row| {
            let product = row
                .iter()
                .zip(rhs.iter())
                .fold(MontFp!(Fp, "0"), |acc, (a, b)| acc + *a * *b);
            result.push(product);
        });
        result
    }
}
#[test]
fn test_mul() {
    let a: Vector = vec![MontFp!(Fp, "3"), MontFp!(Fp, "4")];
    let b: Vector = vec![MontFp!(Fp, "8"), MontFp!(Fp, "2")];
    let c: Vector = vec![MontFp!(Fp, "1"), MontFp!(Fp, "5")];
    let A = Matrix(vec![b, c]);

    assert_eq!(vec![MontFp!(Fp, "15"), MontFp!(Fp, "6")], (&A * &a));
}

/// The following function takes as input 3 nxn-matrices A, B and C
/// and it checks, using the IP of Frievald that A . B = C.
///
/// The checking is done from the point of view of a verifier who
/// is checking the above claim made by an untrustworthy prover.
/// The verfier would like to compute C, but prefers to delegate this computation
/// to the prover. Thus the verifier would like to check the soudness of the above claim.
/// To do so, the verifier samples a random field element r and computes the vector
///  v := (1,r,r²,r³,...,r^(n-1))
/// it then computes and accepts the claim if
/// (A . (B . v) == C . v) == True
///
pub fn verify(A: &Matrix, B: &Matrix, C: &Matrix) -> bool {
    // Generate the random field element r
    let mut rng = ark_std::test_rng();
    let r: Fp = rng.gen();

    // Compute the vector v := (1, r, r^2,..., r^(n-1))
    let mut v: Vector = Vec::with_capacity(A.len());
    let mut cur: Fp = MontFp!(Fp, "1");
    for _ in 0..A.len() {
        v.push(cur);
        cur *= r;
    }

    return A * &(B * &v) == C * &v;
}

#[test]
fn test_frievald() {
    let b: Vector = vec![MontFp!(Fp, "8"), MontFp!(Fp, "2")];
    let c: Vector = vec![MontFp!(Fp, "1"), MontFp!(Fp, "5")];
    let a: Vector = vec![MontFp!(Fp, "3"), MontFp!(Fp, "4")];
    let d: Vector = vec![MontFp!(Fp, "9"), MontFp!(Fp, "2")];

    let A = Matrix(vec![b, c]);
    let B = Matrix(vec![a, d]);
    let C = Matrix(vec![
        vec![MontFp!(Fp, "8"), MontFp!(Fp, "2")],
        vec![MontFp!(Fp, "14"), MontFp!(Fp, "14")],
    ]);

    assert_eq!(verify(&A, &B, &C), true);
}

fn main() {
    println!("Hello, world!");
}
