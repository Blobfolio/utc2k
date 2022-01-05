/*!
# UTC2K - Month
*/

use crate::{
	macros,
	Utc2k,
};
use std::{
	cmp::Ordering,
	ops::Deref,
};



#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
/// # Month.
///
/// This is a simple enum representing months of the year, useful, perhaps, for
/// printing month names or abbreviations.
pub enum Month {
	January,
	February,
	March,
	April,
	May,
	June,
	July,
	August,
	September,
	October,
	November,
	December,
}

macros::as_ref_borrow_cast!(Month: as_str str);

impl Default for Month {
	#[inline]
	fn default() -> Self { Self::January }
}

impl Deref for Month {
	type Target = str;
	#[inline]
	fn deref(&self) -> &Self::Target { self.as_str() }
}

macros::display_str!(as_str Month);

macro_rules! from_int {
	($($ty:ty),+) => ($(
		impl From<$ty> for Month {
			fn from(src: $ty) -> Self {
				match src {
					1 => Self::January,
					2 => Self::February,
					3 => Self::March,
					4 => Self::April,
					5 => Self::May,
					6 => Self::June,
					7 => Self::July,
					8 => Self::August,
					9 => Self::September,
					10 => Self::October,
					11 => Self::November,
					0 | 12 => Self::December,
					_ => Self::from(src % 12),
				}
			}
		}

		impl From<Month> for $ty {
			fn from(src: Month) -> Self {
				match src {
					Month::January => 1,
					Month::February => 2,
					Month::March => 3,
					Month::April => 4,
					Month::May => 5,
					Month::June => 6,
					Month::July => 7,
					Month::August => 8,
					Month::September => 9,
					Month::October => 10,
					Month::November => 11,
					Month::December => 12,
				}
			}
		}
	)+);
}

from_int!(u8, u16, u32, u64, usize);

impl From<Utc2k> for Month {
	#[inline]
	fn from(src: Utc2k) -> Self { Self::from(src.month()) }
}

impl Ord for Month {
	#[inline]
	fn cmp(&self, other: &Self) -> Ordering { self.as_u8().cmp(&other.as_u8()) }
}

macros::partial_eq_from!(Month: u8, u16, u32, u64, usize);

impl PartialOrd for Month {
	#[inline]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
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
	/// assert_eq!(u8::from(Month::now()), Utc2k::now().month());
	/// ```
	pub fn now() -> Self { Self::from(Utc2k::now()) }
}

impl Month {
	#[must_use]
	/// # As Str (Abbreviated).
	///
	/// Return a string slice representing the month's abbreviated name.
	///
	/// ## Examples.
	///
	/// ```
	/// use utc2k::Month;
	///
	/// assert_eq!(Month::January.abbreviation(), "Jan");
	/// ```
	pub const fn abbreviation(self) -> &'static str {
		match self {
			Self::January => "Jan",
			Self::February => "Feb",
			Self::March => "Mar",
			Self::April => "Apr",
			Self::May => "May",
			Self::June => "Jun",
			Self::July => "Jul",
			Self::August => "Aug",
			Self::September => "Sep",
			Self::October => "Oct",
			Self::November => "Nov",
			Self::December => "Dec",
		}
	}

	#[must_use]
	/// # As Str.
	///
	/// Return the month as a string slice.
	///
	/// ## Examples.
	///
	/// ```
	/// use utc2k::Month;
	///
	/// assert_eq!(Month::January.as_str(), "January");
	/// ```
	pub const fn as_str(self) -> &'static str {
		match self {
			Self::January => "January",
			Self::February => "February",
			Self::March => "March",
			Self::April => "April",
			Self::May => "May",
			Self::June => "June",
			Self::July => "July",
			Self::August => "August",
			Self::September => "September",
			Self::October => "October",
			Self::November => "November",
			Self::December => "December",
		}
	}

	#[must_use]
	/// # As U8.
	///
	/// Return the month as an integer, starting with January as `1_u8`,
	/// ending with December as `12_u8`.
	///
	/// ## Examples.
	///
	/// ```
	/// use utc2k::Month;
	///
	/// assert_eq!(Month::January.as_u8(), 1);
	/// ```
	pub const fn as_u8(self) -> u8 {
		match self {
			Self::January => 1,
			Self::February => 2,
			Self::March => 3,
			Self::April => 4,
			Self::May => 5,
			Self::June => 6,
			Self::July => 7,
			Self::August => 8,
			Self::September => 9,
			Self::October => 10,
			Self::November => 11,
			Self::December => 12,
		}
	}

	#[doc(hidden)]
	/// # From U8.
	///
	/// This exists solely for the `const`, which helps us maintain backward
	/// compatibility with some dependent functions.
	pub(crate) const fn from_u8(src: u8) -> Self {
		match src {
			1 => Self::January,
			2 => Self::February,
			3 => Self::March,
			4 => Self::April,
			5 => Self::May,
			6 => Self::June,
			7 => Self::July,
			8 => Self::August,
			9 => Self::September,
			10 => Self::October,
			11 => Self::November,
			0 | 12 => Self::December,
			_ => Self::from_u8(src % 12),
		}
	}
}



#[cfg(test)]
mod tests {
	use super::*;

	const ALL_MONTHS: &[Month] = &[
		Month::January,
		Month::February,
		Month::March,
		Month::April,
		Month::May,
		Month::June,
		Month::July,
		Month::August,
		Month::September,
		Month::October,
		Month::November,
		Month::December,
	];

	#[test]
	/// # Test Fromness.
	fn t_abbr() {
		for d in ALL_MONTHS {
			assert_eq!(d.abbreviation(), &d.as_str()[..3]);
		}
	}

	#[test]
	/// # Test Fromness.
	fn t_from() {
		// There and back again.
		for i in 1..=12_u8 {
			assert_eq!(Month::from(i).as_u8(), i);
		}
		for i in 1..=12_u64 {
			assert_eq!(u64::from(Month::from(i)), i);
		}

		assert_eq!(Month::from(0_u64), Month::December);

		let many: Vec<Month> = (1..=60_u32).into_iter()
			.map(Month::from)
			.collect();

		let mut when = 0;
		for months in many.as_slice().chunks_exact(12) {
			when += 1;
			assert_eq!(months, ALL_MONTHS, "Round #{}", when);
		}
	}
}
