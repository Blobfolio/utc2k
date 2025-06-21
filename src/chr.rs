/*!
# UTC2K: Printable Characters.
*/

/// # Helper: `DateChar` Definition.
macro_rules! date_chars {
	($($k:ident $v:literal),+ $(,)*) => (
		#[repr(u8)]
		#[cfg_attr(not(feature = "local"), expect(dead_code, reason = "Macro made me do it."))]
		#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
		/// # Date Characters.
		///
		/// This enum holds the small subset of ASCII characters comprising
		/// datetime strings. (It's an alternative to unqualified `u8`.)
		///
		/// This adds some complication to the data population side of things,
		/// but reduces the "unsafe" footprint to just two methods, both
		/// located here.
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
			pub(crate) const fn as_char(self) -> char {
				(self as u8) as char
			}

			#[inline(always)]
			#[must_use]
			/// # As Digit.
			///
			/// Convert the ASCII back to a real number.
			pub(crate) const fn as_digit(self) -> u8 {
				debug_assert!((self as u8 ^ b'0') < 10, "BUG: trying to digit a non-digit!");
				self as u8 ^ b'0'
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
						"BUG: DateChar and u8 have different layouts?!",
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
			#[must_use]
			/// # Double Digit.
			///
			/// Return the number as two digits.
			///
			/// Note: this is used by non-year date/time pieces, so maxes out
			/// at fifty-nine.
			pub(crate) const fn dd(src: u8) -> [Self; 2] {
				match src {
					 0 => [DateChar::Digit0, DateChar::Digit0],
					 1 => [DateChar::Digit0, DateChar::Digit1],
					 2 => [DateChar::Digit0, DateChar::Digit2],
					 3 => [DateChar::Digit0, DateChar::Digit3],
					 4 => [DateChar::Digit0, DateChar::Digit4],
					 5 => [DateChar::Digit0, DateChar::Digit5],
					 6 => [DateChar::Digit0, DateChar::Digit6],
					 7 => [DateChar::Digit0, DateChar::Digit7],
					 8 => [DateChar::Digit0, DateChar::Digit8],
					 9 => [DateChar::Digit0, DateChar::Digit9],
					10 => [DateChar::Digit1, DateChar::Digit0],
					11 => [DateChar::Digit1, DateChar::Digit1],
					12 => [DateChar::Digit1, DateChar::Digit2],
					13 => [DateChar::Digit1, DateChar::Digit3],
					14 => [DateChar::Digit1, DateChar::Digit4],
					15 => [DateChar::Digit1, DateChar::Digit5],
					16 => [DateChar::Digit1, DateChar::Digit6],
					17 => [DateChar::Digit1, DateChar::Digit7],
					18 => [DateChar::Digit1, DateChar::Digit8],
					19 => [DateChar::Digit1, DateChar::Digit9],
					20 => [DateChar::Digit2, DateChar::Digit0],
					21 => [DateChar::Digit2, DateChar::Digit1],
					22 => [DateChar::Digit2, DateChar::Digit2],
					23 => [DateChar::Digit2, DateChar::Digit3],
					24 => [DateChar::Digit2, DateChar::Digit4],
					25 => [DateChar::Digit2, DateChar::Digit5],
					26 => [DateChar::Digit2, DateChar::Digit6],
					27 => [DateChar::Digit2, DateChar::Digit7],
					28 => [DateChar::Digit2, DateChar::Digit8],
					29 => [DateChar::Digit2, DateChar::Digit9],
					30 => [DateChar::Digit3, DateChar::Digit0],
					31 => [DateChar::Digit3, DateChar::Digit1],
					32 => [DateChar::Digit3, DateChar::Digit2],
					33 => [DateChar::Digit3, DateChar::Digit3],
					34 => [DateChar::Digit3, DateChar::Digit4],
					35 => [DateChar::Digit3, DateChar::Digit5],
					36 => [DateChar::Digit3, DateChar::Digit6],
					37 => [DateChar::Digit3, DateChar::Digit7],
					38 => [DateChar::Digit3, DateChar::Digit8],
					39 => [DateChar::Digit3, DateChar::Digit9],
					40 => [DateChar::Digit4, DateChar::Digit0],
					41 => [DateChar::Digit4, DateChar::Digit1],
					42 => [DateChar::Digit4, DateChar::Digit2],
					43 => [DateChar::Digit4, DateChar::Digit3],
					44 => [DateChar::Digit4, DateChar::Digit4],
					45 => [DateChar::Digit4, DateChar::Digit5],
					46 => [DateChar::Digit4, DateChar::Digit6],
					47 => [DateChar::Digit4, DateChar::Digit7],
					48 => [DateChar::Digit4, DateChar::Digit8],
					49 => [DateChar::Digit4, DateChar::Digit9],
					50 => [DateChar::Digit5, DateChar::Digit0],
					51 => [DateChar::Digit5, DateChar::Digit1],
					52 => [DateChar::Digit5, DateChar::Digit2],
					53 => [DateChar::Digit5, DateChar::Digit3],
					54 => [DateChar::Digit5, DateChar::Digit4],
					55 => [DateChar::Digit5, DateChar::Digit5],
					56 => [DateChar::Digit5, DateChar::Digit6],
					57 => [DateChar::Digit5, DateChar::Digit7],
					58 => [DateChar::Digit5, DateChar::Digit8],
					_ => [DateChar::Digit5, DateChar::Digit9],
				}
			}

			#[inline(always)]
			#[must_use]
			/// # Double Digit (String).
			///
			/// Return the nubmer as a two-digit string.
			///
			/// Note: this is used by non-year date/time pieces, so maxes out
			/// at fifty-nine.
			pub(crate) const fn dd_str(src: u8) -> &'static str {
				match src {
					 0 => "00",
					 1 => "01",
					 2 => "02",
					 3 => "03",
					 4 => "04",
					 5 => "05",
					 6 => "06",
					 7 => "07",
					 8 => "08",
					 9 => "09",
					10 => "10",
					11 => "11",
					12 => "12",
					13 => "13",
					14 => "14",
					15 => "15",
					16 => "16",
					17 => "17",
					18 => "18",
					19 => "19",
					20 => "20",
					21 => "21",
					22 => "22",
					23 => "23",
					24 => "24",
					25 => "25",
					26 => "26",
					27 => "27",
					28 => "28",
					29 => "29",
					30 => "30",
					31 => "31",
					32 => "32",
					33 => "33",
					34 => "34",
					35 => "35",
					36 => "36",
					37 => "37",
					38 => "38",
					39 => "39",
					40 => "40",
					41 => "41",
					42 => "42",
					43 => "43",
					44 => "44",
					45 => "45",
					46 => "46",
					47 => "47",
					48 => "48",
					49 => "49",
					50 => "50",
					51 => "51",
					52 => "52",
					53 => "53",
					54 => "54",
					55 => "55",
					56 => "56",
					57 => "57",
					58 => "58",
					_ => "59",
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
