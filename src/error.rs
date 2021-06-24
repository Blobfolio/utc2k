/*!
# UTC2K - Errors.
*/

use std::{
	error::Error,
	fmt,
};

#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
/// # Errors.
pub enum Utc2kError {
	Invalid,
	Overflow,
	Underflow,
}

impl Error for Utc2kError {}

impl AsRef<str> for Utc2kError {
	#[inline]
	fn as_ref(&self) -> &str { self.as_str() }
}

impl fmt::Display for Utc2kError {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Utc2kError {
	#[must_use]
	/// # As Str.
	///
	/// Return the error as a string slice.
	pub const fn as_str(self) -> &'static str {
		match self {
			Self::Invalid => "Invalid date/time format.",
			Self::Overflow => "Date/time is post-2099.",
			Self::Underflow => "Date/time is pre-2000.",
		}
	}
}
