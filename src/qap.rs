use num_bigint::BigInt;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use num_traits::Zero;
use crate::field::FieldElement;

#[derive(Clone, Serialize, Deserialize)]
pub struct Variable {
    pub index: usize,
    pub value: BigInt,
}

#[derive(Serialize, Deserialize)]
pub struct Polynomial {
    coefficients: HashMap<usize, FieldElement>, // Coefficients keyed by variable index
}

#[derive(Serialize, Deserialize)]
pub struct QAP {
    pub left: Polynomial,
    pub right: Polynomial,
    pub output: Polynomial,
}

impl QAP {
    pub fn new() -> Self {
        QAP {
            left: Polynomial::new(),
            right: Polynomial::new(),
            output: Polynomial::new(),
        }
    }

    pub fn add_constraint(&mut self, left_coeffs: &[(usize, FieldElement)], right_coeffs: &[(usize, FieldElement)], output_coeffs: &[(usize, FieldElement)], _modulus: &BigInt) {
        for (index, coeff) in left_coeffs {
            *self.left.coefficients.entry(*index).or_insert(FieldElement::new(BigInt::zero())) += coeff.clone();
        }
        for (index, coeff) in right_coeffs {
            *self.right.coefficients.entry(*index).or_insert(FieldElement::new(BigInt::zero())) += coeff.clone();
        }
        for (index, coeff) in output_coeffs {
            *self.output.coefficients.entry(*index).or_insert(FieldElement::new(BigInt::zero())) += coeff.clone();
        }
    }

    pub fn evaluate(&self, assignment: &Vec<FieldElement>) -> FieldElement {
        let left_eval = self.left.evaluate(assignment);
        let right_eval = self.right.evaluate(assignment);
        let output_eval = self.output.evaluate(assignment);

        // Return the evaluation result: left * right - output
        left_eval.mul(&right_eval).sub(&output_eval)
    }
}

impl Polynomial {
    pub fn new() -> Self {
        Polynomial { coefficients: HashMap::new() }
    }

    pub fn add_term(&mut self, index: usize, coefficient: FieldElement) {
        self.coefficients.insert(index, coefficient);
    }

    pub fn evaluate(&self, assignment: &Vec<FieldElement>) -> FieldElement {
        let mut result = FieldElement::new(BigInt::zero()); // Use the same modulus
        for (index, coefficient) in &self.coefficients {
            result = result.add(&coefficient.mul(&assignment[*index]));
        }
        result
    }
}
