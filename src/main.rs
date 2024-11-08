mod circuit;
mod r1cs;
mod merkle;
mod qap;
mod field;
mod proof;

use num_bigint::{ToBigInt};
use circuit::Circuit;
use crate::field::FieldElement;

/// A simple addition proof using the Circuit
fn addition_proof() {
    let mut circuit = Circuit::new();

    let input1 = circuit.add_input(FieldElement::new(10.to_bigint().unwrap()));
    let input2 = circuit.add_input(FieldElement::new(20.to_bigint().unwrap()));

    // Directly compute the expected sum as a FieldElement
    let expected_sum = circuit.get_input(input1).expect("Invalid input index") +
        circuit.get_input(input2).expect("Invalid input index");

    // Store the result of add_input in a temporary variable
    let output_index = circuit.add_input(expected_sum.clone());
    circuit.add_gate(circuit::Gate::Add(input1, input2, output_index)); // Add gate to the circuit
    circuit.set_output(expected_sum); // Set the output to the expected sum

    // Generate and verify the addition proof
    println!("Generating Addition Proof...");
    circuit.generate_proof("addition_proof.bin");
    let is_valid = circuit.verify_proof("addition_proof.bin");
    println!("Addition Proof is valid: {}", is_valid);
}

/// A Merkle Tree proof demonstrating the use of a Merkle path in a zk-circuit
fn merkle_tree_proof() {
    let transactions = vec![
        10.to_bigint().unwrap(),
        20.to_bigint().unwrap(),
        50.to_bigint().unwrap(),
        80.to_bigint().unwrap(),
    ];

    // Create the MerkleTree
    let merkle_tree = merkle::MerkleTree::new(transactions.clone());
    let leaf_index = 2;
    let leaf_value = transactions[leaf_index].clone();
    let merkle_path = merkle_tree.merkle_path(leaf_index);

    let mut circuit = Circuit::new();  // Use modulus for Merkle proofs

    let leaf_index_var = circuit.add_input(FieldElement::new(leaf_value));
    let mut current_hash_index = leaf_index_var;

    for (sibling_hash, is_left) in merkle_path {
        let sibling_index_var = circuit.add_input(FieldElement::new(sibling_hash.clone()));

        // Compute the new hash based on the sibling relationship
        let new_hash_value = if is_left {
            merkle_tree.apply_hash(
                circuit.get_input(sibling_index_var).expect("Invalid input index"),
                circuit.get_input(current_hash_index).expect("Invalid input index"),
            )
        } else {
            merkle_tree.apply_hash(
                circuit.get_input(current_hash_index).expect("Invalid input index"),
                circuit.get_input(sibling_index_var).expect("Invalid input index"),
            )
        };

        let new_hash_index = circuit.add_input(new_hash_value.clone());
        circuit.set_output(new_hash_value.clone());

        // Add a hash gate with correct sibling ordering for Merkle path
        circuit.add_gate(if is_left {
            circuit::Gate::Add(sibling_index_var, current_hash_index, new_hash_index)
        } else {
            circuit::Gate::Add(current_hash_index, sibling_index_var, new_hash_index)
        });

        current_hash_index = new_hash_index;
    }

    // Set the final computed root in the circuit
    circuit.set_output(FieldElement::new(merkle_tree.root.clone()));

    println!("Expected Merkle root: {}", merkle_tree.root);
    circuit.generate_proof("merkle_proof.bin");
    let is_valid = circuit.verify_proof("merkle_proof.bin");
    println!("Merkle Tree Proof is valid: {}", is_valid);
}

/// A function to demonstrate a multiplication proof using the Circuit and R1CS components
fn multiplication_proof() {
    let mut circuit = Circuit::new();  // Using modulus for demonstration

    let input1 = circuit.add_input(FieldElement::new(3.to_bigint().unwrap())); // `a`
    let input2 = circuit.add_input(FieldElement::new(4.to_bigint().unwrap())); // `b`

    // Compute expected product
    let expected_product = circuit.get_input(input1).unwrap().get_value() * circuit.get_input(input2).unwrap().get_value();

    // Add the multiplication gate and set the output
    let output_index = circuit.add_input(FieldElement::new(expected_product.clone())); // `output`
    circuit.add_gate(circuit::Gate::Mul(input1, input2, output_index));
    circuit.set_output(FieldElement::new(expected_product));

    // Generate and verify the multiplication proof
    println!("Generating Multiplication Proof...");
    circuit.generate_proof("multiplication_proof.bin");
    let is_valid = circuit.verify_proof("multiplication_proof.bin");
    println!("Multiplication Proof is valid: {}", is_valid);
}

fn main() {
    // Run each proof demonstration
    addition_proof();
    multiplication_proof();
    merkle_tree_proof(); // Include the merkle_tree_proof function
}
