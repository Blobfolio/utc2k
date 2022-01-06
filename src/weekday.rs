/*!
# UTC2K - Weekday
*/

use crate::{
	macros,
	Utc2k,
	Utc2kError,
};
use std::{
	cmp::Ordering,
	ops::{
		Add,
		AddAssign,
		Deref,
		Sub,
		SubAssign,
	},
};



#[allow(missing_docs)]
#[repr(u8)]
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
	Sunday = 1_u8,
	Monday,
	Tuesday,
	Wednesday,
	Thursday,
	Friday,
	Saturday,
}

impl Add<u8> for Weekday {
	type Output = Self;
	#[inline]
	fn add(self, other: u8) -> Self {
		Self::from(self as u8 + other % 7)
	}
}

impl AddAssign<u8> for Weekday {
	#[inline]
	fn add_assign(&mut self, other: u8) { *self = *self + other; }
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
		if src > 7 { Self::from(src % 7) }
		else if src == 0 { Self::Saturday }
		else {
			unsafe { std::mem::transmute(src) }
		}
	}
}

impl From<Weekday> for u8 {
	#[inline]
	fn from(src: Weekday) -> Self { src as Self }
}

macro_rules! impl_int {
	($($ty:ty),+) => ($(
		impl Add<$ty> for Weekday {
			type Output = Self;
			#[inline]
			fn add(self, other: $ty) -> Self {
				Self::from(<$ty>::from(self) + other % 7)
			}
		}

		impl AddAssign<$ty> for Weekday {
			#[inline]
			fn add_assign(&mut self, other: $ty) { *self = *self + other; }
		}

		impl From<$ty> for Weekday {
			fn from(src: $ty) -> Self {
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

		impl From<Weekday> for $ty {
			fn from(src: Weekday) -> Self {
				match src {
					Weekday::Sunday => 1,
					Weekday::Monday => 2,
					Weekday::Tuesday => 3,
					Weekday::Wednesday => 4,
					Weekday::Thursday => 5,
					Weekday::Friday => 6,
					Weekday::Saturday => 7,
				}
			}
		}

		impl Sub<$ty> for Weekday {
			type Output = Self;

			#[allow(clippy::semicolon_if_nothing_returned)] // We are returning?
			fn sub(self, other: $ty) -> Self {
				let mut lhs = <$ty>::from(self);
				let mut rhs = other % 7;

				while rhs > 0 {
					rhs -= 1;
					if lhs == 1 { lhs = 7; }
					else { lhs -= 1; }
				}

				Self::from(lhs)
			}
		}

		impl SubAssign<$ty> for Weekday {
			#[inline]
			fn sub_assign(&mut self, other: $ty) { *self = *self - other; }
		}
	)+);
}

impl_int!(u16, u32, u64, usize);

impl From<Utc2k> for Weekday {
	#[inline]
	fn from(src: Utc2k) -> Self { src.weekday() }
}

impl Ord for Weekday {
	#[inline]
	fn cmp(&self, other: &Self) -> Ordering {
		let a = *self as u8;
		let b = *other as u8;
		a.cmp(&b)
	}
}

impl PartialEq<u8> for Weekday {
	#[inline]
	fn eq(&self, other: &u8) -> bool { (*self as u8).eq(other) }
}

impl PartialEq<Weekday> for u8 {
	#[inline]
	fn eq(&self, other: &Weekday) -> bool { (*other as Self).eq(self) }
}

macros::partial_eq_from!(Weekday: u16, u32, u64, usize);

impl PartialOrd for Weekday {
	#[inline]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Sub<u8> for Weekday {
	type Output = Self;

	#[allow(clippy::semicolon_if_nothing_returned)] // We are returning?
	fn sub(self, other: u8) -> Self {
		let mut lhs = self as u8;
		let mut rhs = other % 7;

		while rhs > 0 {
			rhs -= 1;
			if lhs == 1 { lhs = 7; }
			else { lhs -= 1; }
		}

		Self::from(lhs)
	}
}

impl SubAssign<u8> for Weekday {
	#[inline]
	fn sub_assign(&mut self, other: u8) { *self = *self - other; }
}

impl TryFrom<&str> for Weekday {
	type Error = Utc2kError;

	/// # From Str.
	///
	/// Note: this is a lazy match, using only the first three characters.
	/// "Saturnalia", for example, will match `Weekday::Saturday`.
	fn try_from(src: &str) -> Result<Self, Self::Error> {
		Self::from_abbreviation(src.trim().as_bytes())
			.ok_or(Utc2kError::Invalid)
	}
}

impl TryFrom<String> for Weekday {
	type Error = Utc2kError;

	/// # From Str.
	///
	/// Note: this is a lazy match, using only the first three characters.
	/// "Saturnalia", for example, will match `Weekday::Saturday`.
	fn try_from(src: String) -> Result<Self, Self::Error> {
		Self::from_abbreviation(src.trim().as_bytes())
			.ok_or(Utc2kError::Invalid)
	}
}

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

	#[deprecated(since = "0.3.3", note = "Use `Weekday::XYZ as u8` instead.")]
	#[inline]
	#[must_use]
	/// # As U8.
	///
	/// Return the weekday as an integer, starting with Sunday as `1_u8`,
	/// ending with Saturday as `7_u8`.
	pub const fn as_u8(self) -> u8 { unsafe { std::mem::transmute(self) } }
}

impl Weekday {
	#[must_use]
	/// # Current Day.
	///
	/// Return the current day of the week (i.e. today).
	///
	/// ## Examples.
	///
	/// ```
	/// use utc2k::{Weekday, Utc2k};
	///
	/// assert_eq!(Weekday::now(), Utc2k::now().weekday());
	/// ```
	pub fn now() -> Self { Utc2k::now().weekday() }

	#[inline]
	#[must_use]
	/// # Tomorrow.
	///
	/// Create a new instance representing one day from now (present time).
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::{Weekday, Utc2k};
	///
	/// assert_eq!(Weekday::tomorrow(), Utc2k::tomorrow().weekday());
	/// ```
	pub fn tomorrow() -> Self { Utc2k::tomorrow().weekday() }

	#[inline]
	#[must_use]
	/// # Yesterday.
	///
	/// Create a new instance representing one day ago (present time).
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::{Weekday, Utc2k};
	///
	/// assert_eq!(Weekday::yesterday(), Utc2k::yesterday().weekday());
	/// ```
	pub fn yesterday() -> Self { Utc2k::yesterday().weekday() }

	/// # From Abbreviation Bytes.
	///
	/// This matches the first three bytes, case-insensitively, against the
	/// `Month` abbreviations.
	pub(crate) fn from_abbreviation(src: &[u8]) -> Option<Self> {
		let src = src.get(..3)?;
		match &[src[0].to_ascii_lowercase(), src[1].to_ascii_lowercase(), src[2].to_ascii_lowercase()] {
			b"sun" => Some(Self::Sunday),
			b"mon" => Some(Self::Monday),
			b"tue" => Some(Self::Tuesday),
			b"wed" => Some(Self::Wednesday),
			b"thu" => Some(Self::Thursday),
			b"fri" => Some(Self::Friday),
			b"sat" => Some(Self::Saturday),
			_ => None,
		}
	}

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



#[cfg(test)]
mod tests {
	use super::*;
	use time::{
		Date,
		Month,
	};

	const ALL_DAYS: &[Weekday] = &[
		Weekday::Sunday,
		Weekday::Monday,
		Weekday::Tuesday,
		Weekday::Wednesday,
		Weekday::Thursday,
		Weekday::Friday,
		Weekday::Saturday,
	];

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

	#[test]
	/// # Test Fromness.
	fn t_abbr() {
		for d in ALL_DAYS {
			assert_eq!(d.abbreviation(), &d.as_str()[..3]);
		}
	}

	#[test]
	/// # Test Fromness.
	fn t_from() {
		// There and back again.
		for i in 1..=7_u8 {
			assert_eq!(Weekday::from(i) as u8, i);
		}
		for i in 1..=7_u64 {
			assert_eq!(u64::from(Weekday::from(i)), i);
		}

		assert_eq!(Weekday::from(0_u64), Weekday::Saturday);

		let many: Vec<Weekday> = (1..=35_u32).into_iter()
			.map(Weekday::from)
			.collect();

		let mut when = 0;
		for days in many.as_slice().chunks_exact(7) {
			when += 1;
			assert_eq!(days, ALL_DAYS, "Round #{}", when);
		}
	}

	#[test]
	/// # Test Some Math!
	fn t_math() {
		let days: Vec<Weekday> = std::iter::repeat(ALL_DAYS)
			.take(6)
			.flatten()
			.copied()
			.collect();

		// Test additions and subtractions.
		for idx in 0..7 {
			for a in 0..36 {
				// Add and sub.
				let b = days[idx] + a;
				assert_eq!(b, days[idx + a]);
				assert_eq!(b - a, days[idx]);

				// Assigning add and sub.
				let mut c = days[idx];
				c += a;
				assert_eq!(c, b);
				c -= a;
				assert_eq!(c, days[idx]);
			}
		}
	}

	#[test]
	/// # String Tests.
	fn t_str() {
		for &d in ALL_DAYS {
			assert_eq!(Ok(d), Weekday::try_from(d.abbreviation()));
			assert_eq!(Ok(d), Weekday::try_from(d.as_str()));
			assert_eq!(Ok(d), Weekday::try_from(d.as_str().to_ascii_uppercase()));
		}

		assert!(Weekday::try_from("Hello").is_err());
	}
}
