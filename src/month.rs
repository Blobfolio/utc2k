/*!
# UTC2K - Month
*/

#![expect(
	clippy::cast_possible_truncation,
	trivial_numeric_casts,
	reason = "False positive.",
)]

use crate::{
	macros,
	Utc2k,
	Utc2kError,
};



macros::weekmonth! {
	Month month
	RepeatingMonthIter
	January    1 "Jan" ( 0   0),
	February   2 "Feb" ( 1 245),
	March      3 "Mar" ( 2 246),
	April      4 "Apr" ( 3 247),
	May        5 "May" ( 4 248),
	June       6 "Jun" ( 5 249),
	July       7 "Jul" ( 6 250),
	August     8 "Aug" ( 7 251),
	September  9 "Sep" ( 8 252),
	October   10 "Oct" ( 9 253),
	November  11 "Nov" (10 254),
	December  12 "Dec" (11 255),
}

impl TryFrom<&[u8]> for Month {
	type Error = Utc2kError;

	#[inline]
	/// # From Byte Slice.
	///
	/// Parse a `Month` from the first three bytes of a slice,
	/// case-insensitively.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Month;
	///
	/// // Case doesn't matter.
	/// assert_eq!(
	///     Month::try_from(b"january".as_slice()),
	///     Ok(Month::January),
	/// );
	/// assert_eq!(
	///     Month::try_from(b"January".as_slice()),
	///     Ok(Month::January),
	/// );
	/// assert_eq!(
	///     Month::try_from(b"JANUARY".as_slice()),
	///     Ok(Month::January),
	/// );
	///
	/// // Only the first three bytes are actually inspected.
	/// assert_eq!(
	///     Month::try_from(b"Jan".as_slice()),
	///     Ok(Month::January),
	/// );
	/// assert_eq!(
	///     Month::try_from(b"janissary".as_slice()), // Close enough!
	///     Ok(Month::January),
	/// );
	///
	/// // Wrong is wrong.
	/// assert!(Month::try_from(b"jebruary".as_slice()).is_err());
	/// ```
	fn try_from(src: &[u8]) -> Result<Self, Self::Error> {
		if 2 < src.len() {
			Self::from_abbreviation(src[0], src[1], src[2]).ok_or(Utc2kError::Invalid)
		}
		else { Err(Utc2kError::Invalid) }
	}
}

impl Month {
	#[must_use]
	/// # Current Month.
	///
	/// Return the current month.
	///
	/// ## Examples.
	///
	/// ```
	/// use utc2k::{Month, Utc2k};
	///
	/// assert_eq!(Month::now(), Utc2k::now().month());
	/// ```
	pub fn now() -> Self { Utc2k::now().month() }
}

impl Month {
	#[inline]
	#[must_use]
	/// # Month Size (Days).
	///
	/// This returns the total number of days this month could hold, or put
	/// another way, the last day of this month.
	///
	/// Note: this method is not leap-aware. If the month is February and it is
	/// in a leap year, be sure to add `1` to reach `29`!
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Month;
	///
	/// assert_eq!(Month::January.days(), 31);
	/// assert_eq!(Month::February.days(), 28); // Not leap-aware.
	/// assert_eq!(Month::March.days(), 31);
	/// assert_eq!(Month::April.days(), 30);
	/// // …
	/// ```
	pub const fn days(self) -> u8 {
		match self {
			Self::January
				| Self::March
				| Self::May
				| Self::July
				| Self::August
				| Self::October
				| Self::December => 31,
			Self::April
				| Self::June
				| Self::September
				| Self::November => 30,
			Self::February => 28,
		}
	}
}

impl Month {
	#[must_use]
	/// # From Abbreviation Bytes.
	///
	/// This matches the first three non-whitespace bytes, case-insensitively,
	/// against the `Month` abbreviations.
	pub(crate) const fn from_abbreviation(a: u8, b: u8, c: u8) -> Option<Self> {
		match crate::needle3(a, b, c) {
			1_650_812_416 => Some(Self::February),
			1_667_589_120 => Some(Self::December),
			1_735_745_792 => Some(Self::August),
			1_819_634_176 => Some(Self::July),
			1_851_877_888 => Some(Self::January),
			1_853_188_608 => Some(Self::June),
			1_885_696_768 => Some(Self::September),
			1_918_987_520 => Some(Self::March),
			1_919_967_488 => Some(Self::April),
			1_952_673_536 => Some(Self::October),
			1_987_014_144 => Some(Self::November),
			2_036_428_032 => Some(Self::May),
			_ => None,
		}
	}

	#[inline]
	#[must_use]
	/// # Ordinal (Naive).
	///
	/// Return the total number of days from previous months.
	///
	/// Note this is _not_ leap aware.
	pub(crate) const fn ordinal(self) -> u16 {
		match self {
			Self::January => 0,
			Self::February => 31,
			Self::March => 59,
			Self::April => 90,
			Self::May => 120,
			Self::June => 151,
			Self::July => 181,
			Self::August => 212,
			Self::September => 243,
			Self::October => 273,
			Self::November => 304,
			Self::December => 334,
		}
	}

	#[inline]
	#[must_use]
	/// # Ordinal Seconds (Naive).
	///
	/// Return the total number of seconds from previous months.
	///
	/// Note this is _not_ leap aware.
	pub(crate) const fn ordinal_seconds(self) -> u32 {
		match self {
			Self::January => 0,
			Self::February => 2_678_400,
			Self::March => 5_097_600,
			Self::April => 7_776_000,
			Self::May => 10_368_000,
			Self::June => 13_046_400,
			Self::July => 15_638_400,
			Self::August => 18_316_800,
			Self::September => 20_995_200,
			Self::October => 23_587_200,
			Self::November => 26_265_600,
			Self::December => 28_857_600,
		}
	}
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	/// # Test Fromness.
	fn t_abbr() {
		for m in Month::ALL {
			assert_eq!(m.abbreviation(), &m.as_str()[..3]);
		}
	}

	#[test]
	fn t_into_iter() {
		let mut last = Month::December;
		for next in Month::January.into_iter().take(25) {
			assert_eq!(next, last + 1_u8);
			assert_eq!(next, last.next());
			last = next;
		}

		last = Month::January;
		for next in Month::December.into_iter().rev().take(25) {
			assert_eq!(next, last - 1_u8);
			assert_eq!(next, last.previous());
			last = next;
		}
	}

	#[test]
	/// # Test Some Math!
	fn t_math() {
		let months: Vec<Month> = std::iter::repeat(Month::ALL)
			.take(4)
			.flatten()
			.collect();

		// Test additions and subtractions.
		for idx in 0..12 {
			for a in 0..36 {
				// Add and sub.
				let b = months[idx] + a;
				assert_eq!(b, months[idx + a]);
				assert_eq!(b - a, months[idx]);

				// Assigning add and sub.
				let mut c = months[idx];
				c += a;
				assert_eq!(c, b);
				c -= a;
				assert_eq!(c, months[idx]);
			}
		}
	}

	#[test]
	/// # String Tests.
	fn t_str() {
		for m in Month::ALL {
			assert_eq!(Ok(m), Month::try_from(m.abbreviation()));
			assert_eq!(Ok(m), Month::try_from(m.as_str()));
			assert_eq!(Ok(m), Month::try_from(m.as_str().to_ascii_lowercase()));
			assert_eq!(Ok(m), Month::try_from(m.as_str().to_ascii_uppercase()));
			assert_eq!(Ok(m), m.abbreviation().parse());
		}

		assert!(Month::try_from("Hello").is_err());
	}
}
