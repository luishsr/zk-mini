use std::ops::AddAssign;
use num_bigint::BigInt;
use num_traits::{One, Zero};
use std::ops::{Add, Mul};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct FieldElement {
    value: BigInt,
    modulus: BigInt,
}

impl FieldElement {
    // Constructor
    pub fn new(value: BigInt) -> Self {
        let default_modulus = BigInt::from(1_000_000_007);
        let normalized_value = value.clone() % &default_modulus;
        FieldElement { value: normalized_value, modulus: default_modulus }
    }

    // Method to get the underlying value
    pub fn get_value(&self) -> BigInt {
        self.value.clone()
    }

    pub fn get_modulus(&self) -> &BigInt { // Getter for modulus
        &self.modulus
    }

    pub fn add(&self, other: &FieldElement) -> FieldElement {
        assert_eq!(self.modulus, other.modulus);
        FieldElement::new(self.value.clone() + other.value.clone())
    }

    pub fn sub(&self, other: &FieldElement) -> FieldElement {
        assert_eq!(self.modulus, other.modulus);
        FieldElement::new(self.value.clone() - other.value.clone())
    }

    pub fn mul(&self, other: &FieldElement) -> FieldElement {
        assert_eq!(self.modulus, other.modulus);
        FieldElement::new(&self.value * &other.value % &self.modulus)
    }

    pub fn inv(&self) -> FieldElement {
        let (gcd, x, _) = self.extended_gcd(&self.value, &self.modulus);
        if gcd != BigInt::one() {
            panic!("Inverse does not exist");
        }
        // Normalize the inverse to be positive
        let normalized_inverse = x % &self.modulus;
        FieldElement::new(normalized_inverse)
    }

    pub fn negate(&self) -> FieldElement {
        FieldElement::new(&self.modulus - &self.value)
    }

    fn extended_gcd(&self, a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
        if *b == BigInt::zero() {
            return (a.clone(), BigInt::one(), BigInt::zero());
        }
        let (gcd, x1, y1) = self.extended_gcd(b, &(a % b));
        let x = y1.clone();
        let y = x1 - (a / b) * &y1;
        (gcd, x, y)
    }
}

// Implement AddAssign for FieldElement
impl AddAssign for FieldElement {
    fn add_assign(&mut self, other: FieldElement) {
        assert_eq!(self.modulus, other.modulus, "Moduli must match for addition");
        self.value = (self.value.clone() + other.value) % &self.modulus; // Perform addition and normalize
    }
}

impl Add for FieldElement {
    type Output = FieldElement;

    fn add(self, other: FieldElement) -> FieldElement {
        let mut result = self.clone(); // Start with a copy of self
        result += other; // Use the AddAssign implementation
        result
    }
}

// Implementing Add for references
impl Add for &FieldElement {
    type Output = FieldElement;

    fn add(self, other: &FieldElement) -> FieldElement {
        assert_eq!(self.modulus, other.modulus, "Moduli must match for addition");
        FieldElement::new((self.value.clone() + other.value.clone()) % &self.modulus)
    }
}

// Implementing Mul trait for FieldElement
impl Mul<&BigInt> for FieldElement {
    type Output = FieldElement;

    fn mul(self, rhs: &BigInt) -> FieldElement {
        FieldElement::new((self.value.clone() * rhs) % &self.modulus) // Perform multiplication and normalize
    }
}

impl Mul<BigInt> for FieldElement {
    type Output = FieldElement;

    fn mul(self, rhs: BigInt) -> FieldElement {
        self * &rhs // Delegate to the implementation that takes a reference
    }
}