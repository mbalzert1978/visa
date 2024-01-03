//! # Credit Card Validator
//!
//! A module for validating credit card numbers based on various criteria such as format,
//! checksums, and numeric properties.
//!
//! ## Usage
//!
//! To use the validator module, you can create a `CreditCard` instance and perform validation
//! checks on it.
//!
//! ```rust
//! use validator::CreditCard;
//!
//! let valid_card = CreditCard::new("1234 5678 9012 3450");
//! assert!(valid_card.is_valid()); // Returns true for a valid credit card.
//! ```
//!
//! ## Validations
//!
//! The `CreditCard` struct provides the following validation methods:
//!
//! - `check_sum10`: Checks if the sum of the numeric digits is divisible by 10.
//! - `check_sum2`: Checks if the sum of the numeric digits (ignoring spaces) is even.
//! - `check_format`: Checks if the credit card number adheres to a specific format.
//!
//! ## Tests
//!
//! The module includes tests to ensure the correctness of the validation methods.
//!
//! ```rust
//! use validator::CreditCard;
//!
//! #[test]
//! fn test_valid_card_should_be_true() {
//!     let card = CreditCard::new("1234 5678 9012 3450");
//!     assert!(card.is_valid());
//! }
//!
//! #[test]
//! fn test_check_sum10_should_be_true() {
//!     let card = CreditCard::new("37");
//!     assert!(card.check_sum10());
//! }
//!
//! #[test]
//! fn test_check_sum2_should_be_true() {
//!     let card = CreditCard::new("4 8");
//!     assert!(card.check_sum2());
//! }
//! ```
//!
//! # Safety
//!
//! The safety of the validation methods relies on the safety of the underlying functions
//! such as `is_valid_length`, `is_numeric`, and `is_valid_numbers`.
//!
mod helper;
use std::fmt;

use helper::{is_numeric, is_valid_length, is_valid_numbers};

/// Represents a Credit Card with validation capabilities.
pub struct CreditCard {
    number: String,
}

impl fmt::Display for CreditCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Credit Card Number: {}", self.number)
    }
}

impl CreditCard {
    /// Creates a new CreditCard instance with the given number.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use validator::CreditCard;
    ///
    /// let card = CreditCard::new("1234 5678 9012 3450");
    /// ```
    pub fn new(number: &str) -> CreditCard {
        CreditCard {
            number: number.to_string(),
        }
    }

    /// Checks if the credit card is valid based on various criteria such as format and checksums.
    ///
    /// # Returns
    ///
    /// Returns `true` if the credit card is valid, otherwise returns `false`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use validator::CreditCard;
    ///
    /// let card = CreditCard::new("1234 5678 9012 3450");
    /// assert!(card.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        self.check_format() && self.check_sum10() && self.check_sum2()
    }

    /// Checks if the sum of the numeric digits of the credit card number is divisible by 10.
    ///
    /// # Returns
    ///
    /// Returns `true` if the sum is divisible by 10, otherwise returns `false`.
    pub fn check_sum10(&self) -> bool {
        let sum: u32 = self.number.chars().filter_map(|c| c.to_digit(10)).sum();
        sum % 10 == 0
    }

    /// Checks if the sum of the numeric digits (ignoring spaces) of the credit card number is even.
    ///
    /// # Returns
    ///
    /// Returns `true` if the sum is even, otherwise returns `false`.
    fn check_sum2(&self) -> bool {
        let sum: u32 = self
            .number
            .split_whitespace()
            .flat_map(|s| s.chars().filter_map(|c| c.to_digit(10)))
            .sum();
        sum % 2 == 0
    }

    /// Checks if the credit card number adheres to a specific format.
    ///
    /// The format includes:
    ///
    /// - Dividing the number into four blocks separated by spaces.
    /// - Having exactly four blocks in total.
    /// - Having a total length of 16 characters (excluding spaces).
    /// - Containing only ASCII digits.
    /// - Containing at least one numeric digit greater than zero.
    ///
    /// # Returns
    ///
    /// Returns `true` if the credit card number meets the specified format conditions,
    /// otherwise returns `false`.
    fn check_format(&self) -> bool {
        let blocks_count = self.number.split(' ').count();
        let numbers = self.number.replace(' ', "");

        blocks_count == 4
            && is_valid_length(&numbers, 16)
            && is_numeric(&numbers)
            && is_valid_numbers(&numbers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_card_should_be_true() {
        let card = CreditCard::new("1234 5678 9012 3450");
        assert!(card.is_valid());
    }

    #[test]
    fn test_check_sum10_should_be_true() {
        let card = CreditCard::new("37");
        assert!(card.check_sum10());
    }

    #[test]
    fn test_check_sum2_should_be_true() {
        let card = CreditCard::new("4 8");
        assert!(card.check_sum2());
    }
}
