#![cfg_attr(not(feature = "std"), no_std)]

// Import BigUint when the feature "large-numbers" is enabled.
#[cfg(feature = "large-numbers")]
use num_bigint::BigUint;

use core::fmt;
// Import the core version of Wrapping when the "std" feature is disabled.
#[cfg(all(not(feature = "checked-overflow"), not(feature = "std")))]
use core::num::Wrapping;
// Import the std version of Wrapping when the "std" feature is enabled.
#[cfg(all(not(feature = "checked-overflow"), feature = "std"))]
use std::num::Wrapping;

/// This enum represents the possible errors that could occur during
/// arithmetic operations in this library.
#[derive(Debug, Clone, PartialEq)]
pub enum ArithmeticError {
    Overflow,
}

/// Display implementation for ArithmeticError. This allows the error
/// to be printed in a user-friendly manner.
impl fmt::Display for ArithmeticError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ArithmeticError::Overflow => write!(f, "Arithmetic operation overflowed"),
        }
    }
}

/// This implementation allows ArithmeticError to be used with the ? operator in functions that
/// return a Result. This is only available when the "std" feature is enabled.
#[cfg(feature = "std")]
impl std::error::Error for ArithmeticError {}

/// The UnsignedInteger trait represents an unsigned integer.
///
/// This trait is used to abstract over different types of unsigned integers,
/// allowing functions in this library to work with any type of unsigned integer.
pub trait UnsignedInteger: Clone + core::ops::Add<Output = Self> {
    /// Returns the zero value for this type of unsigned integer.
    fn zero() -> Self;
    /// Returns the one value for this type of unsigned integer.
    fn one() -> Self;

    /// Adds the given unsigned integer to this one, returning an error if the result would overflow.
    /// This method is only available when the "checked-overflow" feature is enabled.
    #[cfg(feature = "checked-overflow")]
    fn safe_add(self, rhs: Self) -> Result<Self, ArithmeticError>;

    /// Adds the given unsigned integer to this one, wrapping around at the maximum value of this type.
    /// This method is only available when the "checked-overflow" feature is disabled.
    #[cfg(all(not(feature = "checked-overflow"), feature = "std"))]
    fn unchecked_add(self, rhs: Self) -> Self;

    /// Adds the given unsigned integer to this one, wrapping around at the maximum value of this type.
    /// This method is only available when the "checked-overflow" feature is disabled and the "std" feature is also disabled.
    #[cfg(all(not(feature = "checked-overflow"), not(feature = "std")))]
    fn unchecked_add(self, rhs: Self) -> Self;
}

// This macro implements the UnsignedInteger trait for the given types.
macro_rules! impl_unsigned_integer {
    ($($t:ty)*) => ($(impl UnsignedInteger for $t {
        fn zero() -> Self { 0 }
        fn one() -> Self { 1 }

        // Rust's inherent checked_add
        #[cfg(feature = "checked-overflow")]
        fn safe_add(self, rhs: Self) -> Result<Self, ArithmeticError> {
            self.checked_add(rhs).ok_or(ArithmeticError::Overflow)
        }

        // Adds the given unsigned integer to this one, wrapping around at the maximum value of this type.
        // This method is only available when the "checked-overflow" feature is disabled and the "std" feature is enabled.
        #[cfg(all(not(feature = "checked-overflow"), feature = "std"))]
        fn unchecked_add(self, rhs: Self) -> Self {
            Wrapping(self).0.wrapping_add(Wrapping(rhs).0)
        }
    })*)
}

// Implement UnsignedInteger for the unsigned primitive integer types.
impl_unsigned_integer! { u8 u16 u32 u64 u128 }

// Implement UnsignedInteger for BigUint when the "large-numbers" feature is enabled.
#[cfg(feature = "large-numbers")]
impl UnsignedInteger for BigUint {
    fn zero() -> Self {
        BigUint::from(0u32)
    }
    fn one() -> Self {
        BigUint::from(1u32)
    }

    #[cfg(feature = "checked-overflow")]
    fn safe_add(self, rhs: Self) -> Result<Self, ArithmeticError> {
        Ok(self + rhs) // BigUint never overflows
    }

    #[cfg(all(not(feature = "checked-overflow"), feature = "std"))]
    fn unchecked_add(self, rhs: Self) -> Self {
        self + rhs // BigUint never overflows
    }
}

/// The Fibonacci struct represents a Fibonacci sequence.
///
/// The sequence is represented by its current and next values.
pub struct Fibonacci<T: UnsignedInteger> {
    current: T,
    next: T,
}

impl<T: UnsignedInteger> Fibonacci<T> {
    /// Creates a new Fibonacci sequence starting at zero.
    pub fn new() -> Fibonacci<T> {
        Fibonacci {
            current: T::zero(),
            next: T::one(),
        }
    }
}

// Implement Default for Fibonacci, which just calls Fibonacci::new.
impl<T: UnsignedInteger> Default for Fibonacci<T> {
    fn default() -> Self {
        Self::new()
    }
}

// Implement Iterator for Fibonacci, allowing the Fibonacci sequence to be generated lazily.
#[cfg(feature = "iterator")]
impl<T: UnsignedInteger> Iterator for Fibonacci<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // With the "checked-overflow" feature, return None on overflow.
        #[cfg(feature = "checked-overflow")]
        {
            let next = match self.current.clone().safe_add(self.next.clone()) {
                Ok(value) => value,
                Err(ArithmeticError::Overflow) => return None,
            };

            let current = self.current.clone();
            self.current = self.next.clone();
            self.next = next;

            return Some(current);
        }

        // Without "checked-overflow" and with "std", use wrapping addition.
        #[cfg(all(not(feature = "checked-overflow"), feature = "std"))]
        {
            let current = self.current.clone();
            self.current = self.next.clone();
            self.next = self.current.clone().unchecked_add(self.next.clone());

            return Some(current);
        }

        // Without "checked-overflow" and without "std", use regular addition.
        #[cfg(all(not(feature = "checked-overflow"), not(feature = "std")))]
        {
            let current = self.current.clone();
            self.current = self.next.clone();
            self.next = self.current + self.next;

            return Some(current);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // This function tests the Fibonacci sequence for a given unsigned integer type.
    // It is only available when the "std" feature is enabled.
    #[cfg(feature = "std")]
    fn test_fibonacci<T: UnsignedInteger + std::cmp::PartialEq + std::fmt::Debug>() {
        let mut fib = Fibonacci::<T>::new();
        // Test initial values
        assert_eq!(fib.current, T::zero());
        assert_eq!(fib.next, T::one());

        // Test the first 6 values of the Fibonacci sequence
        assert_eq!(fib.next(), Some(T::zero())); // 0
        assert_eq!(fib.next(), Some(T::one())); // 1
        assert_eq!(fib.next(), Some(T::one())); // 1
        assert_eq!(fib.next(), Some(T::one() + T::one())); // 2
        assert_eq!(fib.next(), Some(T::one() + T::one() + T::one())); // 3
        assert_eq!(
            fib.next(),
            Some(T::one() + T::one() + T::one() + T::one() + T::one())
        );
        // 5
    }

    // Test the Fibonacci sequence for different unsigned integer types.
    #[cfg(feature = "std")]
    #[test]
    fn test_fibonacci_u8() {
        test_fibonacci::<u8>();
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_fibonacci_u16() {
        test_fibonacci::<u16>();
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_fibonacci_u32() {
        test_fibonacci::<u32>();
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_fibonacci_u64() {
        test_fibonacci::<u64>();
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_fibonacci_u128() {
        test_fibonacci::<u128>();
    }

    #[cfg(all(feature = "std", feature = "large-numbers"))]
    #[test]
    fn test_fibonacci_big_uint() {
        test_fibonacci::<BigUint>();
    }

    // Test the checked_add and wrapping_add functions of the UnsignedInteger trait.
    #[cfg(feature = "checked-overflow")]
    #[test]
    fn test_unsigned_integer() {
        let a: u8 = 1;
        let b: u8 = 1;
        let c: u8 = 255;

        assert_eq!(a.safe_add(b), Ok(2u8)); // Checked addition within bounds should succeed.
        assert_eq!(c.safe_add(b), Err(ArithmeticError::Overflow)); // Checked addition out of bounds should fail.
    }

    // Test the new function of the Fibonacci struct.
    #[test]
    fn test_fibonacci_new() {
        let fib: Fibonacci<u8> = Fibonacci::new();
        // Test that new correctly initializes the current and next fields.
        assert_eq!(fib.current, 0);
        assert_eq!(fib.next, 1);
    }

    // Test the default function of the Fibonacci struct.
    #[test]
    fn test_fibonacci_default() {
        let fib: Fibonacci<u8> = Default::default();
        // Test that default correctly initializes the current and next fields.
        assert_eq!(fib.current, 0);
        assert_eq!(fib.next, 1);
    }

    #[cfg(feature = "iterator")]
    // Test the next function of the Fibonacci struct.
    #[test]
    fn test_fibonacci_iterator() {
        let mut fib = Fibonacci::<u8>::new();
        // Test the first 6 values of the Fibonacci sequence.
        assert_eq!(fib.next(), Some(0));

        assert_eq!(fib.next(), Some(1));
        assert_eq!(fib.next(), Some(1));
        assert_eq!(fib.next(), Some(2));
        assert_eq!(fib.next(), Some(3));
        assert_eq!(fib.next(), Some(5));
    }

    // Test the handling of overflow in the Fibonacci sequence.
    #[test]
    fn test_fibonacci_overflow() {
        let mut fib = Fibonacci::<u8>::new();
        for _ in 0..255 {
            fib.next(); // Advance to the end of the sequence.
        }
        // After overflowing the u8 type, the sequence should return None.
        assert_eq!(fib.next(), None);
    }
}
