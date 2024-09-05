/*!
# UTC2K - Errors.
*/

use crate::macros;
use std::error::Error;



#[allow(missing_docs)] // Self-explanatory.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
/// # Errors.
pub enum Utc2kError {
	Invalid,
	Overflow,
	Underflow,
}

impl Error for Utc2kError {}

macros::as_ref_borrow_cast!(Utc2kError: as_str str);
macros::display_str!(as_str Utc2kError);

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
