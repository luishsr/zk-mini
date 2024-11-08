use std::fs::File;
use std::io;
use std::io::Write;
use num_bigint::BigInt;
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use crate::field::FieldElement;
use crate::r1cs::{Operation, R1CS};

#[derive(Serialize, Deserialize)]
pub struct Proof {
    pub witness: Vec<BigInt>,  // The witness values used for proof generation
    pub commitment: BigInt,     // Commitment to the witness, for verification
}

impl Proof {
    // Generate a proof from R1CS and witness
    pub fn generate_proof(_r1cs: &R1CS, witness: &Vec<FieldElement>) -> Proof {
        // Create a commitment based on the witness
        let mut commitment_input = BigInt::zero();
        let witness_bigint: Vec<BigInt> = witness.iter().map(|w| w.get_value()).collect(); // Convert to Vec<BigInt>

        for w in &witness_bigint {
            commitment_input += w; // Use the BigInt value directly
        }

        let commitment = Self::hash(&commitment_input, &BigInt::zero()); // Simplified hash, use appropriate values

        Proof {
            witness: witness_bigint, // Assign the converted Vec<BigInt>
            commitment,
        }
    }

    // Implement a method to save the proof to a binary file
    pub fn save_to_binary(&self, filename: &str) -> io::Result<()> {
        let mut file = File::create(filename)?;
        let encoded: Vec<u8> = bincode::serialize(self).expect("Failed to serialize proof");
        file.write_all(&encoded)?;
        Ok(())
    }

    // Verify a proof against the R1CS constraints
    pub fn verify_proof(proof: &Proof, r1cs: &R1CS) -> bool {
        // Check if the commitment matches the expected hash
        let mut commitment_input = BigInt::zero();
        for w in &proof.witness {
            commitment_input += w; // Combine witness values
        }
        let expected_commitment = Self::hash(&commitment_input, &BigInt::zero()); // Use the same hash function

        if proof.commitment != expected_commitment {
            return false; // Commitment mismatch
        }

        // Check if the proof's witness satisfies the R1CS constraints
        for constraint in &r1cs.constraints {
            let left_eval = constraint.left.iter().map(|(var, coeff)| {
                var.value.clone() * coeff // This produces FieldElement
            }).map(|fe| fe.get_value()).sum::<BigInt>(); // Convert to BigInt and sum

            let right_eval = constraint.right.iter().map(|(var, coeff)| {
                var.value.clone() * coeff
            }).map(|fe| fe.get_value()).sum::<BigInt>();

            let output_eval = constraint.output.iter().map(|(var, coeff)| {
                var.value.clone() * coeff
            }).map(|fe| fe.get_value()).sum::<BigInt>();

            // Verify the specific operation
            match constraint.operation {
                Operation::Add => {
                    if left_eval + right_eval != output_eval {
                        return false; // Constraint not satisfied
                    }
                },
                Operation::Mul => {
                    if left_eval * right_eval != output_eval {
                        return false; // Constraint not satisfied
                    }
                },
                Operation::Hash => todo!(),
            }
        }

        true // If all checks pass, the proof is valid
    }

    /// A simple hashing mechanism combining two BigInt values.
    fn hash(left: &BigInt, right: &BigInt) -> BigInt {
        // For simplicity, we will just combine the two values
        // and take the modulo of a large prime number.
        let combined = left + right; // Simple addition as the "hashing" operation
        combined % BigInt::from(1_000_000_007) // A large prime for modulo
    }
}
