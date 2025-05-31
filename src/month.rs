/*!
# UTC2K - Month
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
	February,

	/// # March.
	March,

	/// # April.
	April,

	/// # May.
	May,

	/// # June.
	June,

	/// # July.
	July,

	/// # August.
	August,

	/// # September.
	September,

	/// # October.
	October,

	/// # November.
	November,

	/// # December.
	December,
}

impl Add<u8> for Month {
	type Output = Self;

	#[inline]
	fn add(self, other: u8) -> Self {
		Self::from(self as u8 + other % 12)
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
			#[expect(clippy::cast_possible_truncation, reason = "False positive.")]
			fn from(src: $ty) -> Self {
				if src <= 12 { Self::from_u8(src as u8) }
				else { Self::from_u8((src % 12) as u8) }
			}
		}

		impl From<Month> for $ty {
			#[inline]
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
	fn from(src: Utc2k) -> Self { Self::from(src.month()) }
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

	fn sub(self, other: u8) -> Self {
		let mut lhs = self as u8;
		let mut rhs = other % 12;

		while rhs > 0 {
			rhs -= 1;
			if lhs == 1 { lhs = 12; }
			else { lhs -= 1; }
		}

		Self::from(lhs)
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
		Self::from_abbreviation(src).ok_or(Utc2kError::Invalid)
	}
}

impl TryFrom<&str> for Month {
	type Error = Utc2kError;

	#[inline]
	/// # From Str.
	///
	/// Note: this is a lazy match, using only the first three characters.
	/// "Decimal", for example, will match `Month::December`.
	fn try_from(src: &str) -> Result<Self, Self::Error> {
		Self::from_abbreviation(src.as_bytes()).ok_or(Utc2kError::Invalid)
	}
}

impl TryFrom<String> for Month {
	type Error = Utc2kError;

	#[inline]
	/// # From Str.
	///
	/// Note: this is a lazy match, using only the first three characters.
	/// "Decimal", for example, will match `Month::December`.
	fn try_from(src: String) -> Result<Self, Self::Error> {
		Self::from_abbreviation(src.as_bytes()).ok_or(Utc2kError::Invalid)
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
	pub fn now() -> Self { Self::from(Utc2k::now()) }

	/// # From Abbreviation Bytes.
	///
	/// This matches the first three non-whitespace bytes, case-insensitively,
	/// against the `Month` abbreviations.
	pub(crate) const fn from_abbreviation(src: &[u8]) -> Option<Self> {
		if let [a, b, c, _rest @ ..] = src.trim_ascii_start() {
			match [a.to_ascii_lowercase(), b.to_ascii_lowercase(), c.to_ascii_lowercase()] {
				[b'j', b'a', b'n'] => Some(Self::January),
				[b'f', b'e', b'b'] => Some(Self::February),
				[b'm', b'a', b'r'] => Some(Self::March),
				[b'a', b'p', b'r'] => Some(Self::April),
				[b'm', b'a', b'y'] => Some(Self::May),
				[b'j', b'u', b'n'] => Some(Self::June),
				[b'j', b'u', b'l'] => Some(Self::July),
				[b'a', b'u', b'g'] => Some(Self::August),
				[b's', b'e', b'p'] => Some(Self::September),
				[b'o', b'c', b't'] => Some(Self::October),
				[b'n', b'o', b'v'] => Some(Self::November),
				[b'd', b'e', b'c'] => Some(Self::December),
				_ => None,
			}
		}
		else { None }
	}
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

	/// # Abbreviation (bytes).
	///
	/// This returns the abbreviation as a fixed-size byte array.
	pub(crate) const fn abbreviation_bytes(self) -> [u8; 3] {
		match self {
			Self::January => *b"Jan",
			Self::February => *b"Feb",
			Self::March => *b"Mar",
			Self::April => *b"Apr",
			Self::May => *b"May",
			Self::June => *b"Jun",
			Self::July => *b"Jul",
			Self::August => *b"Aug",
			Self::September => *b"Sep",
			Self::October => *b"Oct",
			Self::November => *b"Nov",
			Self::December => *b"Dec",
		}
	}

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

	#[inline]
	/// # From U8 Unchecked.
	///
	/// This is the same as From, but const.
	pub(crate) const fn from_u8(src: u8) -> Self {
		match src {
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
			11 => Self::November,
			0 | 12 => Self::December,
			_ => Self::from_u8(src % 12),
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
		for d in Month::ALL {
			assert_eq!(d.abbreviation(), &d.as_str()[..3]);
		}
	}

	#[test]
	/// # Test Fromness.
	fn t_from() {
		// There and back again.
		for i in 1..=12_u8 {
			let month = Month::from(i);
			assert_eq!(month as u8, i);
			assert_eq!(month.abbreviation().as_bytes(), month.abbreviation_bytes());
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
			assert_eq!(Ok(m), Month::try_from(m.as_str().to_ascii_uppercase()));
			assert_eq!(Ok(m), m.abbreviation().parse());
		}

		assert!(Month::try_from("Hello").is_err());
	}
}
