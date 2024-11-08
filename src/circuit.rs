use num_bigint::BigInt;
use crate::r1cs::{R1CS};
use crate::field::FieldElement;
use crate::proof::Proof;

pub enum Gate {
    Add(usize, usize, usize), // Add: input1, input2, output
    Mul(usize, usize, usize), // Multiply: input1, input2, output
}

pub struct Circuit {
    inputs: Vec<FieldElement>, // Use FieldElement instead of BigInt
    gates: Vec<Gate>,
    outputs: Vec<FieldElement>, // Use FieldElement instead of BigInt
    modulus: BigInt, // Store modulus for FieldElements
}

impl Circuit {
    pub fn new() -> Self {
        let default_modulus = BigInt::from(1_000_000_007); // Default modulus
        Circuit {
            inputs: Vec::new(),
            gates: Vec::new(),
            outputs: Vec::new(),
            modulus: default_modulus,
        }
    }

    pub fn add_input(&mut self, value: FieldElement) -> usize { // Accept FieldElement directly
        let index = self.inputs.len();
        self.inputs.push(value);
        index
    }

    pub fn add_gate(&mut self, gate: Gate) {
        self.gates.push(gate);
    }

    pub fn set_output(&mut self, value: FieldElement) { // Accept FieldElement directly
        self.outputs.push(value);
    }

    /// Retrieves an input value by index, if it exists
    pub fn get_input(&self, index: usize) -> Option<&FieldElement> {
        self.inputs.get(index)
    }

    /// Generates the proof and checks constraint satisfaction, then saves it to a binary file
    pub fn generate_proof(&self, proof_file: &str) {
        // Ensure inputs are added before generating proof
        if self.inputs.is_empty() {
            panic!("No inputs available to generate proof.");
        }

        let mut r1cs = R1CS::new();

        // Add variables to R1CS
        for input in &self.inputs {
            r1cs.add_variable(input.clone()); // input is of type FieldElement
        }

        // Process each gate and add constraints to R1CS
        for gate in &self.gates {
            match gate {
                Gate::Add(a, b, output) => {
                    r1cs.add_constraint(
                        &[
                            (r1cs.variables[*a].index, FieldElement::new(BigInt::from(1))), // Extract index
                        ],
                        &[
                            (r1cs.variables[*b].index, FieldElement::new(BigInt::from(1))), // Extract index
                        ],
                        &[
                            (r1cs.variables[*output].index, FieldElement::new(BigInt::from(1))), // Extract index
                        ],
                        &self.modulus, // Pass modulus dynamically
                    );
                },
                Gate::Mul(a, b, output) => {
                    r1cs.add_constraint(
                        &[
                            (r1cs.variables[*a].index, FieldElement::new(BigInt::from(1))), // Extract index
                        ],
                        &[
                            (r1cs.variables[*b].index, FieldElement::new(BigInt::from(1))), // Extract index
                        ],
                        &[
                            (r1cs.variables[*output].index, FieldElement::new(BigInt::from(1))), // Extract index
                        ],
                        &self.modulus, // Pass modulus dynamically
                    );
                },
            }
        }

        // Save the R1CS to a binary file
        r1cs.save_to_binary("r1cs_file.bin");

        // Generate the witness and proof
        let witness = r1cs.generate_witness();
        let proof = r1cs.generate_proof(&witness);

        // Save the proof to a specified file
        proof.save_to_binary(proof_file).expect("failed to save the proof");
    }

    /// Verifies the proof by reading from a binary file
    pub fn verify_proof(&self, proof_file: &str) -> bool {
        let proof_data = std::fs::read(proof_file).expect("Could not read proof file");

        let proof = bincode::deserialize::<Proof>(&proof_data).expect("Failed to deserialize proof");

        // Ensure that witness is Vec<FieldElement> and not Vec<BigInt>
        let witness: Vec<FieldElement> = proof.witness.iter()
            .map(|value| FieldElement::new(value.clone()))
            .collect();

        let r1cs = R1CS::load_from_binary("r1cs_file.bin");

        let is_valid = r1cs.verify_witness(&witness);

        println!("Proof verification result: {}", is_valid);
        is_valid
    }
}
