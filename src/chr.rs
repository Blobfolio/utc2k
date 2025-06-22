/*!
# UTC2K: Printable Characters.
*/


/// # Helper: `DateChar` Definition.
macro_rules! date_chars {
	($($k:ident $v:literal),+ $(,)*) => (
		#[cfg_attr(not(feature = "local"), expect(dead_code, reason = "Macro made me do it."))]
		#[repr(u8)]
		#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
		/// # Date Characters.
		///
		/// This enum holds the small subset of ASCII characters comprising
		/// datetime strings. (It's an alternative to unqualified `u8`.)
		///
		/// This adds some complication to the data population side of things,
		/// but reduces the "unsafe" footprint to just two methods, both
		/// located here and easy to verify.
		///
		/// TODO: replace this with `AsciiChar` once stable.
		pub(crate) enum DateChar {
			$($k = $v,)+
		}

		impl DateChar {
			#[inline(always)]
			#[must_use]
			/// # As Char.
			///
			/// Return as a single char.
			pub(crate) const fn as_char(self) -> char { (self as u8) as char }

			#[inline(always)]
			#[must_use]
			/// # As Digit.
			///
			/// Convert the ASCII back to a real number.
			pub(crate) const fn as_digit(self) -> u8 {
				debug_assert!((self as u8 ^ b'0') < 10, "BUG: trying to digit a non-digit!");
				self as u8 & 0b0000_1111_u8
			}

			#[expect(unsafe_code, reason = "For transmute.")]
			#[inline(always)]
			#[must_use]
			/// # As Bytes.
			///
			/// Transmute a slice of `DateChar` into a slice of bytes.
			pub(crate) const fn as_bytes(src: &[Self]) -> &[u8] {
				// This check is overly-paranoid, but the compiler should
				// optimize it out.
				const {
					assert!(
						align_of::<&[Self]>() == align_of::<&[u8]>() &&
						size_of::<&[Self]>() == size_of::<&[u8]>(),
						"BUG: DateChar and u8 slices have different layouts?!",
					);
				}

				// Safety: `DateChar` is represented by `u8` so shares the
				// same size and alignment.
				unsafe { std::mem::transmute::<&[Self], &[u8]>(src) }
			}

			#[expect(unsafe_code, reason = "For transmute.")]
			#[inline(always)]
			#[must_use]
			/// # As Str.
			///
			/// Transmute a slice of `DateChar` into a string slice.
			pub(crate) const fn as_str(src: &[Self]) -> &str {
				// Safety: all `DateChar` variants are valid ASCII, so no
				// matter how they're sliced up, will always yield valid UTF-8
				// sequences.
				unsafe { std::str::from_utf8_unchecked(Self::as_bytes(src)) }
			}

			#[inline(always)]
			/// # One Digit.
			pub(crate) const fn from_digit(src: u8) -> Self {
				match (src % 10) | b'0' {
					b'0' => Self::Digit0,
					b'1' => Self::Digit1,
					b'2' => Self::Digit2,
					b'3' => Self::Digit3,
					b'4' => Self::Digit4,
					b'5' => Self::Digit5,
					b'6' => Self::Digit6,
					b'7' => Self::Digit7,
					b'8' => Self::Digit8,
					b'9' => Self::Digit9,
					_ => unreachable!(),
				}
			}
		}
	);
}

date_chars!(
	Space      b' ',
	Plus       b'+',
	Dash       b'-',
	Digit0     b'0',
	Digit1     b'1',
	Digit2     b'2',
	Digit3     b'3',
	Digit4     b'4',
	Digit5     b'5',
	Digit6     b'6',
	Digit7     b'7',
	Digit8     b'8',
	Digit9     b'9',
	Colon      b':',
);
