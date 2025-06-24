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
pub struct Gradient {
    input: String,
    start: usize,
    end: usize,
    width: usize,
    wide: bool,
    max_width: Option<usize>,
}
impl Iterator for Gradient {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        if self.finished() {
            return None;
        }
        self.end += 1;
        if !self.wide {
            self.wide = true;
            self.width += 1;
            self.start = 0;
            self.end = self.width;
        }

        self.start = self.end - self.width;
        if self.end == self.len() {
            self.wide = false;
        }
        if let Some(max_width) = self.max_width {
            if self.width > max_width {
                return None;
            }
        }
        Some(self.window())
    }
}
impl<'a> Gradient {
    pub fn input(&self) -> &'a str {
        unsafe { core::mem::transmute::<&str, &'a str>(&self.input[self.range()]) }
    }
}
impl Gradient {
    pub fn window(&self) -> String {
        self.input[self.range()].to_string()
    }

    pub fn finished(&self) -> bool {
        if self.len() == 0 {
            return true;
        }
        if self.end == self.len() {
            if self.width == self.len() {
                return true;
            }
        }
        false
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn range(&self) -> core::ops::Range<usize> {
        self.start()..self.end()
    }

    pub fn len(&self) -> usize {
        self.input.len()
    }

    pub fn new<T: Display>(s: T) -> Gradient {
        Gradient::with_max_width(s, None)
    }

    /// Creates a [Gradient](Self) that optionally spans to a maximum
    /// string width.
    ///
    /// ```
    /// use gradient_string::Gradient;
    ///
    /// let result = Gradient::with_max_width(" abc ", Some(2))
    ///     .collect::<Vec<String>>();
    /// assert_eq!(
    ///     result,
    ///     vec![" ", "a", "b", "c", " ", " a", "ab", "bc", "c "]
    /// );
    /// ```

    pub fn with_max_width<T: Display>(s: T, max_width: Option<usize>) -> Gradient {
        Gradient {
            input: s.to_string(),
            start: 0,
            end: 0,
            width: 1,
            wide: true,
            max_width,
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
        assert_eq!(Gradient::new("").collect::<Vec<_>>().len(), 0);
    }

    #[test]
    fn max_width() {
        let result = Gradient::with_max_width(" abc ", Some(2)).collect::<Vec<String>>();
        assert_eq!(
            result,
            vec![" ", "a", "b", "c", " ", " a", "ab", "bc", "c "]
        );
    }
}
