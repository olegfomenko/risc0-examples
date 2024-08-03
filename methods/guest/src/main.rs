use risc0_zkvm::guest::env;

use std::ops::{Add, Mul};
use k256::{AffinePoint, Scalar, ProjectivePoint};
use k256::elliptic_curve::{Group};


fn main() {
    // read the input
    let (private_key, amount, g, h): (Scalar, u64, AffinePoint, AffinePoint) = env::read();
    let commitment =
        ProjectivePoint::from(g)
            .mul(private_key)
            .add(
                ProjectivePoint::from(h).mul(Scalar::from(amount))
            );

    // write public output to the journal
    env::commit(&commitment.to_affine());
    env::commit(&g);
    env::commit(&h);
}
