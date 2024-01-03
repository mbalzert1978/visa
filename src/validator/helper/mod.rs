/// A trait for types that can provide their length.
///
/// Types implementing this trait must provide a `len` method that returns the length of the object.
///
/// # Examples
///
/// Implementing `HasLength` for a custom type:
///
/// ```rust
/// struct MyContainer(Vec<i32>);
///
/// impl HasLength for MyContainer {
///     fn len(&self) -> usize {
///         self.0.len()
///     }
/// }
///
/// let my_object = MyContainer(vec![1, 2, 3]);
/// assert_eq!(my_object.len(), 3);
/// ```
///
/// # Requirements
///
/// Types implementing this trait are expected to provide a correct and efficient implementation
/// of the `len` method.
///
pub trait HasLength {
    /// Returns the length of the object.
    fn len(&self) -> usize;
}

impl HasLength for String {
    fn len(&self) -> usize {
        self.len()
    }
}

impl HasLength for Vec<&str> {
    fn len(&self) -> usize {
        self.len()
    }
}

/// Checks if the length of the provided value is equal to a specified count.
///
/// This function compares the length of the given value with the specified count and returns
/// `true` if they are equal, and `false` otherwise.
///
/// # Parameters
///
/// * `value`: A reference to the value that needs its length to be checked.
/// * `count`: The expected length that the value should have.
///
/// # Returns
///
/// Returns `true` if the length of the value is equal to the specified count, otherwise `false`.
///
/// # Examples
///
/// ```rust
/// assert!(is_valid_length("abc", 3));     // Returns true for a string with length 3.
/// assert!(!is_valid_length("xyz", 2));    // Returns false for a string with length 3.
/// assert!(is_valid_length(&[1, 2, 3], 3)); // Returns true for a vector with length 3.
/// ```
///
/// # Generic Requirement
///
/// The generic type `T` must implement the `HasLength` trait, providing a way to obtain its length.
///
pub fn is_valid_length<T: HasLength>(value: &T, count: usize) -> bool {
    value.len() == count
}

/// Checks if the given string contains only ASCII digits.
///
/// This function iterates over the characters in the provided string and checks if each character
/// is an ASCII digit. It returns `true` if all characters are ASCII digits, and `false` otherwise.
///
/// # Parameters
///
/// * `value`: A reference to the input string that needs to be checked.
///
/// # Returns
///
/// Returns `true` if the entire string contains only ASCII digits, otherwise `false`.
///
/// # Examples
///
/// ```rust
/// assert!(is_numeric("12345")); // Returns true for a string containing only ASCII digits.
/// assert!(!is_numeric("12A45")); // Returns false for a string containing non-digit characters.
/// assert!(is_numeric(""));       // Returns true for an empty string.
/// ```
///
/// # Note
///
/// This function treats non-ASCII digits (e.g., Eastern Arabic numerals) as non-digits.
///
pub fn is_numeric(value: &str) -> bool {
    value.chars().all(|c| c.is_ascii_digit())
}

/// Checks if the given string contains at least one numeric digit greater than zero.
///
/// This function iterates over the characters in the provided string and checks if any of them
/// represent a numeric digit greater than zero. It returns `true` if such a digit is found, and
/// `false` otherwise.
///
/// # Parameters
///
/// * `value`: A reference to the input string that needs to be checked.
///
/// # Returns
///
/// Returns `true` if at least one numeric digit greater than zero is found, otherwise `false`.
///
/// # Examples
///
/// ```rust
/// assert!(is_valid_numbers("abc123")); // Returns true because '1' is greater than zero.
/// assert!(!is_valid_numbers("0xyz"));  // Returns false as there are no digits greater than zero.
/// assert!(!is_valid_numbers(""));       // Returns false for an empty string.
/// ```
///
/// # Panics
///
/// This function does not panic.
///
/// # Safety
///
/// This function assumes that the input string represents a valid UTF-8 encoded string.
///
pub fn is_valid_numbers(value: &str) -> bool {
    value.chars().filter_map(|c| c.to_digit(10)).any(|d| d > 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_zero_should_be_false() {
        assert!(
            !is_valid_numbers("0000"),
            "Expected false for a string containing only zeros."
        );
    }
}
