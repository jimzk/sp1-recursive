//! A simple script to generate and verify the proof of a given program.

use std::time::Instant;

use sp1_core::{SP1Prover, SP1Stdin, SP1Verifier};

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    // Generate proof.
    let mut stdin = SP1Stdin::new();
    let n = 500u32;
    stdin.write(&n);
    let start = Instant::now();
    let mut proof = SP1Prover::prove(ELF, stdin).expect("proving failed");
    println!("Proof Time: {:?}", start.elapsed());

    // Read output.
    // let ret = proof.stdout.read::<bool>();
    // println!("ret: {}", ret);

    // Verify proof.
    let start = Instant::now();
    SP1Verifier::verify(ELF, &proof).expect("verification failed");
    println!("Proof Time: {:?}", start.elapsed());

    println!("succesfully generated and verified proof for the program!")
}
