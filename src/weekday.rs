/*!
# UTC2K - Weekday
*/

use crate::{
	macros,
	Utc2k,
};
use std::{
	cmp::Ordering,
	ops::{
		Add,
		Deref,
	},
};



#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
/// # Weekday.
///
/// This is a simple enum representing days of the week.
///
/// While not particularly useful on its own, you can use it for wrapping
/// addition operations.
///
/// Otherwise this is only really used by [`Utc2k::weekday`].
pub enum Weekday {
	Sunday,
	Monday,
	Tuesday,
	Wednesday,
	Thursday,
	Friday,
	Saturday,
}

macro_rules! impl_bigint {
	($($ty:ty),+) => ($(
		impl Add<$ty> for Weekday {
			type Output = Self;
			#[allow(clippy::cast_possible_truncation)] // It fits.
			#[inline]
			fn add(self, other: $ty) -> Self { Self::from(self.as_u8() + (other % 7) as u8) }
		}

		impl From<$ty> for Weekday {
			#[allow(clippy::cast_possible_truncation)] // It fits.
			fn from(src: $ty) -> Self {
				match src {
					1 => Self::Sunday,
					2 => Self::Monday,
					3 => Self::Tuesday,
					4 => Self::Wednesday,
					5 => Self::Thursday,
					6 => Self::Friday,
					0 | 7 => Self::Saturday,
					_ => Self::from((src % 7) as u8),
				}
			}
		}
	)+);
}

macros::as_ref_borrow_cast!(Weekday: as_str str);

impl Default for Weekday {
	#[inline]
	fn default() -> Self { Self::Sunday }
}

impl Deref for Weekday {
	type Target = str;
	#[inline]
	fn deref(&self) -> &Self::Target { self.as_str() }
}

macros::display_str!(as_str Weekday);

impl From<u8> for Weekday {
	fn from(src: u8) -> Self {
		match src {
			1 => Self::Sunday,
			2 => Self::Monday,
			3 => Self::Tuesday,
			4 => Self::Wednesday,
			5 => Self::Thursday,
			6 => Self::Friday,
			0 | 7 => Self::Saturday,
			_ => Self::from(src % 7),
		}
	}
}

impl From<Utc2k> for Weekday {
	#[inline]
	fn from(src: Utc2k) -> Self { src.weekday() }
}

impl Ord for Weekday {
	#[inline]
	fn cmp(&self, other: &Self) -> Ordering { self.as_u8().cmp(&other.as_u8()) }
}

impl PartialOrd for Weekday {
	#[inline]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl_bigint!(u16, u32, u64, usize);

impl Weekday {
	#[must_use]
	/// # As Str (Abbreviated).
	///
	/// Return a string slice representing the day's abbreviated name.
	///
	/// ## Examples.
	///
	/// ```
	/// use utc2k::Weekday;
	///
	/// assert_eq!(Weekday::Sunday.abbreviation(), "Sun");
	/// ```
	pub const fn abbreviation(self) -> &'static str {
		match self {
			Self::Sunday => "Sun",
			Self::Monday => "Mon",
			Self::Tuesday => "Tue",
			Self::Wednesday => "Wed",
			Self::Thursday => "Thu",
			Self::Friday => "Fri",
			Self::Saturday => "Sat",
		}
	}

	#[must_use]
	/// # As Str.
	///
	/// Return the day as a string slice.
	///
	/// ## Examples.
	///
	/// ```
	/// use utc2k::Weekday;
	///
	/// assert_eq!(Weekday::Sunday.as_str(), "Sunday");
	/// ```
	pub const fn as_str(self) -> &'static str {
		match self {
			Self::Sunday => "Sunday",
			Self::Monday => "Monday",
			Self::Tuesday => "Tuesday",
			Self::Wednesday => "Wednesday",
			Self::Thursday => "Thursday",
			Self::Friday => "Friday",
			Self::Saturday => "Saturday",
		}
	}

	#[must_use]
	/// # As U8.
	///
	/// Return the weekday as an integer, starting with Sunday as `1_u8`,
	/// ending with Saturday as `7_u8`.
	///
	/// ## Examples.
	///
	/// ```
	/// use utc2k::Weekday;
	///
	/// assert_eq!(Weekday::Sunday.as_u8(), 1);
	/// ```
	pub const fn as_u8(self) -> u8 {
		match self {
			Self::Sunday => 1,
			Self::Monday => 2,
			Self::Tuesday => 3,
			Self::Wednesday => 4,
			Self::Thursday => 5,
			Self::Friday => 6,
			Self::Saturday => 7,
		}
	}
}

impl Weekday {
	#[must_use]
	/// # Start of Year.
	///
	/// Return the first day of the given year.
	///
	/// Note: this only matches the years `2000..=2099`. Anything outside that
	/// range is counted as a Friday.
	pub(crate) const fn year_begins_on(y: u8) -> Self {
		match y {
			0 | 5 | 11 | 22 | 28 | 33 | 39 | 50 | 56 | 61 | 67 | 78 | 84 | 89 | 95 => Self::Saturday,
			1 | 7 | 18 | 24 | 29 | 35 | 46 | 52 | 57 | 63 | 74 | 80 | 85 | 91 => Self::Monday,
			2 | 8 | 13 | 19 | 30 | 36 | 41 | 47 | 58 | 64 | 69 | 75 | 86 | 92 | 97 => Self::Tuesday,
			3 | 14 | 20 | 25 | 31 | 42 | 48 | 53 | 59 | 70 | 76 | 81 | 87 | 98 => Self::Wednesday,
			4 | 9 | 15 | 26 | 32 | 37 | 43 | 54 | 60 | 65 | 71 | 82 | 88 | 93 | 99 => Self::Thursday,
			6 | 12 | 17 | 23 | 34 | 40 | 45 | 51 | 62 | 68 | 73 | 79 | 90 | 96 => Self::Sunday,
			_ => Self::Friday,
		}
	}
}

impl From<Weekday> for u8 {
	#[inline]
	fn from(src: Weekday) -> Self { src.as_u8() }
}



#[cfg(test)]
mod tests {
	use super::*;
	use time::{
		Date,
		Month,
	};

	#[test]
	/// # Test First of Year.
	fn t_year_start() {
		for y in 2000..=2099 {
			let c = Date::from_calendar_date(y, Month::January, 1)
				.expect("Unable to create time::Date.");
			assert_eq!(
				Weekday::year_begins_on((y - 2000) as u8).as_ref(),
				c.weekday().to_string(),
				"Failed with year {}", y
			);
		}
	}
}
