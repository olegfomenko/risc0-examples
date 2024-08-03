// These constants represent the RISC-V ELF and the image ID generated by risc0-build.
// The ELF is used for proving and the ID is used for verification.
use methods::{
    EXAMPLE_GUEST_ELF, EXAMPLE_GUEST_ID,
};
use risc0_zkvm::{default_prover, ExecutorEnv};
use k256::{AffinePoint, Scalar};
use k256::elliptic_curve::Group;
use k256::elliptic_curve::group::Curve;
use rand_core::OsRng;

fn main() {
    let mut rand = OsRng::default();

    // Private data
    let private_key = Scalar::generate_biased(&mut rand);
    let amount = 12345u64;

    println!("Private key: {}", serde_json::to_string_pretty(&private_key).unwrap());

    // Public data
    let g = k256::ProjectivePoint::random(&mut rand).to_affine();
    let h = k256::ProjectivePoint::random(&mut rand).to_affine();

    // Passing data to environment
    let env = ExecutorEnv::builder()
        .write(&(private_key, amount, g, h))
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Proof information by proving the specified ELF binary.
    // This struct contains the receipt along with statistics about execution of the guest
    let prove_info = prover
        .prove(env, EXAMPLE_GUEST_ELF)
        .unwrap();

    // extract the receipt.
    let receipt = prove_info.receipt;

    let (commitment, g, h): (AffinePoint, AffinePoint, AffinePoint) = receipt.journal.decode().unwrap();

    // The receipt was verified at the end of proving, but the below code is an
    // example of how someone else could verify this receipt.
    receipt
        .verify(EXAMPLE_GUEST_ID)
        .unwrap();

    println!("Prove verified!");

    //println!("Receipt: {}", serde_json::to_string_pretty(&receipt).unwrap());

    println!("Commitment: {}", serde_json::to_string_pretty(&commitment).unwrap());
    println!("G: {}", serde_json::to_string_pretty(&g).unwrap());
    println!("H: {}", serde_json::to_string_pretty(&h).unwrap());
}
