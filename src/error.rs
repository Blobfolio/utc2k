/*!
# UTC2K - Errors.
*/

use crate::macros;
use std::error::Error;



/// # Helper: Errors.
macro_rules! err {
	( $( $k:ident $v:literal ),+ $(,)? ) => (
		#[derive(Debug, Copy, Clone, Eq, PartialEq)]
		/// # Errors.
		pub enum Utc2kError {
			$(
				#[doc = concat!("# ", $v)]
				$k,
			)+
		}

		impl Utc2kError {
			#[must_use]
			/// # As Str.
			///
			/// Return the error as a string slice.
			pub const fn as_str(self) -> &'static str {
				match self {
					$( Self::$k => $v, )+
				}
			}
		}
	);
}

err! {
	Invalid   "Invalid date/time format.",
	Overflow  "Date/time is post-2099.",
	Underflow "Date/time is pre-2000.",
}

impl Error for Utc2kError {}

macros::as_ref_borrow_cast!(Utc2kError: as_str str);
macros::display_str!(as_str Utc2kError);



#[derive(Debug, Clone, Copy, Eq, PartialEq)]
/// # Formatting Error.
pub enum Utc2kFormatError {
	/// # Unexpected End.
	Eof,

	/// # Invalid Component.
	InvalidComponent,

	/// # Invalid Year Modifier.
	InvalidModifier(&'static str),

	/// # Format String Not ASCII.
	NotAscii,
}

impl Error for Utc2kFormatError {}

impl Utc2kFormatError {
	#[must_use]
	/// # As String Slice.
	///
	/// Same as `Display`, but `const`.
	pub const fn as_str(self) -> &'static str {
		match self {
			Self::Eof => "Expected ']'.",
			Self::InvalidComponent => "Expected [year | month | day | hour | minute | second | ordinal | period | unixtime].",
			Self::InvalidModifier(s) => s,
			Self::NotAscii => "Date format strings must be ASCII.",
		}
	}
}

macros::as_ref_borrow_cast!(Utc2kFormatError: as_str str);
macros::display_str!(as_str Utc2kFormatError);
