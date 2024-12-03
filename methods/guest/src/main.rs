use risc0_zkvm::guest::env;

use std::ops::{Add, Mul};
use k256::Scalar;
use k256::elliptic_curve::{Group};


fn main() {
    // read the input
    let private_key = Scalar::from(12345678909876554321u128);
    let amount = Scalar::from(12345u64);

    let g = k256::ProjectivePoint::GENERATOR;
    // Only for educational reasons :)
    let h = g.mul(k256::Scalar::from(12345u32));

    let commitment = g.mul(private_key).add(h.mul(amount));

    // write public output to the journal
    env::commit(&commitment.to_affine());
    env::commit(&g.to_affine());
    env::commit(&h.to_affine());
}
