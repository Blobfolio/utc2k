/*!
# UTC2K - Month
*/

#![expect(clippy::cast_possible_truncation, reason = "Macros made me do it.")]

use crate::{
	ASCII_LOWER,
	macros,
	Utc2k,
	Utc2kError,
};
use std::{
	borrow::Cow,
	cmp::Ordering,
	ops::{
		Add,
		AddAssign,
		Sub,
		SubAssign,
	},
	str::FromStr,
};



#[repr(u8)]
#[derive(Debug, Clone, Copy, Default, Eq, Hash, PartialEq)]
/// # Month.
///
/// This is a simple enum representing months of the year, useful, perhaps, for
/// printing month names or abbreviations.
pub enum Month {
	#[default]
	/// # January.
	January = 1_u8,

	/// # February.
	February = 2_u8,

	/// # March.
	March = 3_u8,

	/// # April.
	April = 4_u8,

	/// # May.
	May = 5_u8,

	/// # June.
	June = 6_u8,

	/// # July.
	July = 7_u8,

	/// # August.
	August = 8_u8,

	/// # September.
	September = 9_u8,

	/// # October.
	October = 10_u8,

	/// # November.
	November = 11_u8,

	/// # December.
	December = 12_u8,
}

impl Add<u8> for Month {
	type Output = Self;

	#[inline]
	fn add(self, other: u8) -> Self {
		Self::from_u8(self as u8 + other % 12)
	}
}

impl AddAssign<u8> for Month {
	#[inline]
	fn add_assign(&mut self, other: u8) { *self = *self + other; }
}

macros::as_ref_borrow_cast!(Month: as_str str);

macros::display_str!(as_str Month);

impl From<u8> for Month {
	#[inline]
	fn from(src: u8) -> Self { Self::from_u8(src) }
}

impl From<Month> for u8 {
	#[inline]
	/// # As `u8`
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Month;
	///
	/// // January is one.
	/// assert_eq!(
	///     u8::from(Month::January),
	///     1,
	/// );
	///
	/// // As casts work too.
	/// assert_eq!(
	///     u8::from(Month::January),
	///     Month::January as u8,
	/// );
	/// ```
	fn from(src: Month) -> Self { src as Self }
}

/// # Helper: Add/From/Sub Impls.
macro_rules! impl_int {
	($($ty:ty),+) => ($(
		impl Add<$ty> for Month {
			type Output = Self;
			#[inline]
			fn add(self, other: $ty) -> Self {
				Self::from(<$ty>::from(self) + other % 12)
			}
		}

		impl AddAssign<$ty> for Month {
			#[inline]
			fn add_assign(&mut self, other: $ty) { *self = *self + other; }
		}

		impl From<$ty> for Month {
			#[inline]
			fn from(src: $ty) -> Self { Self::from_u8((src % 12) as u8) }
		}

		impl From<Month> for $ty {
			#[inline]
			#[doc = concat!("# As `", stringify!($ty), "`")]
			///
			/// ## Examples
			///
			/// ```
			/// use utc2k::Month;
			///
			/// // January is one.
			/// assert_eq!(
			#[doc = concat!("    ", stringify!($ty), "::from(Month::January),")]
			///     1,
			/// );
			///
			/// // As casts work too.
			/// assert_eq!(
			#[doc = concat!("    ", stringify!($ty), "::from(Month::January),")]
			#[doc = concat!("    Month::January as ", stringify!($ty), ",")]
			/// );
			/// ```
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

		impl Sub<$ty> for Month {
			type Output = Self;

			#[inline]
			fn sub(self, other: $ty) -> Self {
				let mut lhs = <$ty>::from(self);
				let mut rhs = other % 12;

				while rhs > 0 {
					rhs -= 1;
					if lhs == 1 { lhs = 12; }
					else { lhs -= 1; }
				}

				Self::from(lhs)
			}
		}

		impl SubAssign<$ty> for Month {
			#[inline]
			fn sub_assign(&mut self, other: $ty) { *self = *self - other; }
		}
	)+);
}

impl_int!(u16, u32, u64, usize);

impl From<Utc2k> for Month {
	#[inline]
	fn from(src: Utc2k) -> Self { src.month() }
}

impl FromStr for Month {
	type Err = Utc2kError;

	#[inline]
	fn from_str(src: &str) -> Result<Self, Self::Err> { Self::try_from(src) }
}

impl IntoIterator for Month {
	type Item = Self;
	type IntoIter = RepeatingMonthIter;

	#[inline]
	/// # Repeating Iterator.
	///
	/// Return an iterator that will cycle endlessly through the years,
	/// starting from this `Month`.
	fn into_iter(self) -> Self::IntoIter { RepeatingMonthIter(self) }
}

impl Ord for Month {
	#[inline]
	fn cmp(&self, other: &Self) -> Ordering {
		let a = *self as u8;
		let b = *other as u8;
		a.cmp(&b)
	}
}

/// # Helper: Reciprocal `PartialEq`.
macro_rules! eq {
	($($ty:ty),+) => ($(
		impl PartialEq<$ty> for Month {
			#[inline]
			fn eq(&self, other: &$ty) -> bool { (*self as $ty) == *other }
		}
		impl PartialEq<Month> for $ty {
			#[inline]
			fn eq(&self, other: &Month) -> bool { <Month as PartialEq<$ty>>::eq(other, self) }
		}
	)+);
}
eq!(u8, u16, u32, u64, usize);

impl PartialOrd for Month {
	#[inline]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Sub<u8> for Month {
	type Output = Self;

	#[inline]
	fn sub(self, other: u8) -> Self {
		let mut lhs = self as u8;
		let mut rhs = other % 12;

		while rhs > 0 {
			rhs -= 1;
			if lhs == 1 { lhs = 12; }
			else { lhs -= 1; }
		}

		Self::from_u8(lhs)
	}
}

impl SubAssign<u8> for Month {
	#[inline]
	fn sub_assign(&mut self, other: u8) { *self = *self - other; }
}

impl TryFrom<&[u8]> for Month {
	type Error = Utc2kError;

	#[inline]
	/// # From Str.
	///
	/// Note: this is a lazy match, using only the first three characters.
	/// "Decimal", for example, will match `Month::December`.
	fn try_from(src: &[u8]) -> Result<Self, Self::Error> {
		if 2 < src.len() {
			Self::from_abbreviation(src[0], src[1], src[2]).ok_or(Utc2kError::Invalid)
		}
		else { Err(Utc2kError::Invalid) }
	}
}

/// # Helper: `TryFrom` Wrappers.
macro_rules! try_from {
	($($ty:ty)+) => ($(
		impl TryFrom<$ty> for Month {
			type Error = Utc2kError;
			#[inline]
			fn try_from(src: $ty) -> Result<Self, Self::Error> {
				Self::try_from(src.as_bytes())
			}
		}
	)+);
}

try_from! { &str &String String &Cow<'_, str> Cow<'_, str> &Box<str> Box<str> }

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
	pub fn now() -> Self { Self::from(Utc2k::now()) }
}

impl Month {
	/// # All Months.
	///
	/// Return an array containing all possible months, in order.
	pub const ALL: [Self; 12] = [
		Self::January,
		Self::February,
		Self::March,
		Self::April,
		Self::May,
		Self::June,
		Self::July,
		Self::August,
		Self::September,
		Self::October,
		Self::November,
		Self::December,
	];

	#[inline]
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

	#[inline]
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
}

impl Month {
	#[must_use]
	/// # From Abbreviation Bytes.
	///
	/// This matches the first three non-whitespace bytes, case-insensitively,
	/// against the `Month` abbreviations.
	pub(crate) const fn from_abbreviation(a: u8, b: u8, c: u8) -> Option<Self> {
		let src = u32::from_le_bytes([0, a, b, c]) | ASCII_LOWER;
		match src {
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
	/// # From U8 Unchecked.
	///
	/// This is the same as From, but const.
	pub(crate) const fn from_u8(src: u8) -> Self {
		match src % 12 {
			1  => Self::January,
			2  => Self::February,
			3  => Self::March,
			4  => Self::April,
			5  => Self::May,
			6  => Self::June,
			7  => Self::July,
			8  => Self::August,
			9  => Self::September,
			10 => Self::October,
			11 =>  Self::November,
			_ => Self::December,
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



#[derive(Debug)]
/// # Endless Months!
///
/// This iterator yields an infinite number of `Month`s, in order, starting
/// from any arbitrary month.
pub struct RepeatingMonthIter(Month);

impl Iterator for RepeatingMonthIter {
	type Item = Month;

	/// # Next Month.
	fn next(&mut self) -> Option<Self::Item> {
		let next = self.0;
		self.0 = match next {
			Month::January => Month::February,
			Month::February => Month::March,
			Month::March => Month::April,
			Month::April => Month::May,
			Month::May => Month::June,
			Month::June => Month::July,
			Month::July => Month::August,
			Month::August => Month::September,
			Month::September => Month::October,
			Month::October => Month::November,
			Month::November => Month::December,
			Month::December => Month::January,
		};
		Some(next)
	}

	/// # Infinity.
	///
	/// This iterator never stops!
	fn size_hint(&self) -> (usize, Option<usize>) { (usize::MAX, None) }
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
	/// # Test Fromness.
	fn t_from() {
		// There and back again.
		for i in 1..=12_u8 {
			let month = Month::from(i);
			assert_eq!(month as u8, i);
		}
		for i in 1..=12_u64 {
			assert_eq!(u64::from(Month::from(i)), i);
		}

		assert_eq!(Month::from(0_u64), Month::December);

		let many: Vec<Month> = (1..=60_u32)
			.map(Month::from)
			.collect();

		let mut when = 0;
		for months in many.as_slice().chunks_exact(12) {
			when += 1;
			assert_eq!(months, Month::ALL, "Round #{when}");
		}
	}

	#[test]
	fn t_into_iter() {
		let mut last = Month::December;
		for next in Month::January.into_iter().take(25) {
			assert_eq!(next, last + 1_u8);
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
