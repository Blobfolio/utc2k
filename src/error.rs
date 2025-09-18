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
