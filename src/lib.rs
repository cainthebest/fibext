#![no_std]

/// A trait representing unsigned integer types.
///
/// Provides methods and constants for working with unsigned integers.
pub trait UnsignedInteger: Copy {
    /// The zero value of this unsigned integer type.
    const ZERO: Self;

    /// The one value of this unsigned integer type.
    const ONE: Self;

    /// Returns the result of adding `self` and `rhs` if the addition does not overflow.
    fn checked_add(self, rhs: Self) -> Option<Self>;
}

macro_rules! impl_unsigned_integer {
    ($($t:ty)*) => ($(impl UnsignedInteger for $t {
        const ZERO: Self = 0;
        const ONE: Self = 1;

        fn checked_add(self, rhs: Self) -> Option<Self> {
            self.checked_add(rhs)
        }
    })*)
}

impl_unsigned_integer! { u32 u64 u128 }

/// A Fibonacci sequence generator.
///
/// Generates Fibonacci numbers using an iterator.
///
/// # Examples
///
/// ```
/// use fibext::Fibonacci;
///
/// let mut fib = Fibonacci::<u64>::new();
/// assert_eq!(fib.next(), Some(0));
/// assert_eq!(fib.next(), Some(1));
/// assert_eq!(fib.next(), Some(1));
/// assert_eq!(fib.next(), Some(2));
/// ```
pub struct Fibonacci<T: UnsignedInteger> {
    current: T,
    next: T,
}

impl<T: UnsignedInteger> Fibonacci<T> {
    /// Creates a new Fibonacci sequence generator.
    ///
    /// # Examples
    ///
    /// ```
    /// use fibext::Fibonacci;
    ///
    /// let fib = Fibonacci::<u32>::new();
    /// ```
    pub fn new() -> Fibonacci<T> {
        Fibonacci {
            current: T::ZERO,
            next: T::ONE,
        }
    }
}

impl<T: UnsignedInteger> Default for Fibonacci<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: UnsignedInteger> Iterator for Fibonacci<T> {
    type Item = T;

    /// Returns the next Fibonacci number in the sequence.
    /// Stops generating the sequence when the next number overflows the numeric type.
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        let next = self.next.checked_add(self.current)?;

        self.current = self.next;
        self.next = next;

        Some(current)
    }
}

/// Fills the provided buffer with a Fibonacci sequence.
///
/// # Examples
///
/// ```
/// use fibext::fill_fibonacci_sequence;
///
/// let mut buffer = [0_u64; 10];
/// fill_fibonacci_sequence(&mut buffer);
/// assert_eq!(buffer, [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
/// ```
pub fn fill_fibonacci_sequence<T: UnsignedInteger>(buffer: &mut [T]) {
    let mut fibonacci = Fibonacci::<T>::new();
    for element in buffer {
        *element = fibonacci.next().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::{fill_fibonacci_sequence, Fibonacci, UnsignedInteger};
    use core::ops::Add;

    fn test_fibonacci_sequence<T>()
    where
        T: UnsignedInteger + PartialEq + core::fmt::Debug + Add<Output = T>,
    {
        let expected_sequence = [
            T::ZERO,
            T::ONE,
            T::ONE,
            T::ONE + T::ONE,
            T::ONE + T::ONE + T::ONE,
            T::ONE + T::ONE + T::ONE + T::ONE + T::ONE,
            T::ONE + T::ONE + T::ONE + T::ONE + T::ONE + T::ONE + T::ONE + T::ONE,
        ];

        let mut generated_sequence = [T::ZERO; 7];
        fill_fibonacci_sequence(&mut generated_sequence);
        assert_eq!(generated_sequence, expected_sequence);

        let generated_sequence: [T; 7] = {
            let mut fib = Fibonacci::new();
            [
                fib.next().unwrap(),
                fib.next().unwrap(),
                fib.next().unwrap(),
                fib.next().unwrap(),
                fib.next().unwrap(),
                fib.next().unwrap(),
                fib.next().unwrap(),
            ]
        };
        assert_eq!(generated_sequence, expected_sequence);
    }

    #[test]
    fn test_fibonacci_u32() {
        test_fibonacci_sequence::<u32>();
    }

    #[test]
    fn test_fibonacci_u64() {
        test_fibonacci_sequence::<u64>();
    }

    #[test]
    fn test_fibonacci_u128() {
        test_fibonacci_sequence::<u128>();
    }
}
