#![doc(issue_tracker_base_url = "https://github.com/gabrielfalcao/unique-pointer/issues/")]
//! # Gradient String
//!
//! gradient-string is a safe crate to iterate over a gradient of
//! permutations of string slices
//!
//! ## Example
//!
//! ```
//! use gradient_string::Gradient;
//! let result = Gradient::new(" abc ")
//!     .collect::<Vec<String>>();
//! assert_eq!(
//!     result,
//!     vec![
//!         " ", "a", "b", "c", " ", " a", "ab", "bc", "c ", " ab", "abc", "bc ", " abc",
//!         "abc ", " abc "
//!     ]
//! );
//! ```
use std::fmt::Display;
use gradient_slice::Gradient as Slicer;


/// ```
/// use gradient_string::Gradient;
/// let result = Gradient::new(" abc ")
///     .collect::<Vec<String>>();
/// assert_eq!(
///     result,
///     vec![
///         " ", "a", "b", "c", " ", " a", "ab", "bc", "c ", " ab", "abc", "bc ", " abc",
///         "abc ", " abc "
///     ]
/// );
/// ```
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Gradient<'a> {
    input: &'a str,
    slice: Slicer<'a, char>,
}
impl<'a> Iterator for Gradient<'a> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        self.slice.next().map(|slice| {
            slice
                .iter()
                .map(Clone::clone)
                .map(String::from)
                .collect::<String>()
        })
    }
}
impl<'a> Gradient<'a> {
    pub fn input(&self) -> &'a str {
        self.input
    }
}
impl<'a> Gradient<'a> {
    pub fn window(&self) -> String {
        self.slice.window().iter().map(Clone::clone).map(String::from).collect()
    }

    pub fn finished(&self) -> bool {
        self.slice.finished()
    }

    pub fn width(&self) -> usize {
        self.slice.width()
    }

    pub fn start(&self) -> usize {
        self.slice.start()
    }

    pub fn end(&self) -> usize {
        self.slice.end()
    }

    pub fn range(&self) -> core::ops::Range<usize> {
        self.slice.range()
    }

    pub fn len(&self) -> usize {
        self.slice.len()
    }

    pub fn new<T: Display>(s: T) -> Gradient<'a> {
        Gradient {
            input: s.to_string().leak(),
            slice: Slicer::new(s.to_string().chars().collect::<Vec<char>>()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gradient() {
        let result = Gradient::new(" abc ").collect::<Vec<String>>();
        assert_eq!(
            result,
            vec![
                " ", "a", "b", "c", " ", " a", "ab", "bc", "c ", " ab", "abc", "bc ", " abc",
                "abc ", " abc "
            ]
        );
    }
    #[test]
    fn empty() {
        assert_eq!(
            Gradient::new("").collect::<Vec<_>>().len(),
            0
        );
    }
}
