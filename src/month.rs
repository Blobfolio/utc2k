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
		Deref,
		Sub,
		SubAssign,
	},
};



#[allow(missing_docs)]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
/// # Month.
///
/// This is a simple enum representing months of the year, useful, perhaps, for
/// printing month names or abbreviations.
pub enum Month {
	January = 1_u8,
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

impl From<u8> for Month {
	fn from(src: u8) -> Self {
		if src > 12 { Self::from(src % 12) }
		else if src == 0 { Self::December }
		else {
			unsafe { std::mem::transmute(src) }
		}
	}
}

impl From<Month> for u8 {
	#[inline]
	fn from(src: Month) -> Self { src as Self }
}

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

		impl Sub<$ty> for Month {
			type Output = Self;

			#[allow(clippy::semicolon_if_nothing_returned)] // We are returning?
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

impl Ord for Month {
	#[inline]
	fn cmp(&self, other: &Self) -> Ordering {
		let a = *self as u8;
		let b = *other as u8;
		a.cmp(&b)
	}
}

impl PartialEq<u8> for Month {
	#[inline]
	fn eq(&self, other: &u8) -> bool { (*self as u8).eq(other) }
}

impl PartialEq<Month> for u8 {
	#[inline]
	fn eq(&self, other: &Month) -> bool { (*other as Self).eq(self) }
}

macros::partial_eq_from!(Month: u16, u32, u64, usize);

impl PartialOrd for Month {
	#[inline]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Sub<u8> for Month {
	type Output = Self;

	#[allow(clippy::semicolon_if_nothing_returned)] // We are returning?
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

impl TryFrom<&str> for Month {
	type Error = Utc2kError;

	/// # From Str.
	///
	/// Note: this is a lazy match, using only the first three characters.
	/// "Decimal", for example, will match `Month::December`.
	fn try_from(src: &str) -> Result<Self, Self::Error> {
		Self::from_abbreviation(src.trim().as_bytes())
			.ok_or(Utc2kError::Invalid)
	}
}

impl TryFrom<String> for Month {
	type Error = Utc2kError;

	/// # From Str.
	///
	/// Note: this is a lazy match, using only the first three characters.
	/// "Decimal", for example, will match `Month::December`.
	fn try_from(src: String) -> Result<Self, Self::Error> {
		Self::from_abbreviation(src.trim().as_bytes())
			.ok_or(Utc2kError::Invalid)
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
	/// This matches the first three bytes, case-insensitively, against the
	/// `Month` abbreviations.
	pub(crate) fn from_abbreviation(src: &[u8]) -> Option<Self> {
		let src = src.get(..3)?;
		match &[src[0].to_ascii_lowercase(), src[1].to_ascii_lowercase(), src[2].to_ascii_lowercase()] {
			b"jan" => Some(Self::January),
			b"feb" => Some(Self::February),
			b"mar" => Some(Self::March),
			b"apr" => Some(Self::April),
			b"may" => Some(Self::May),
			b"jun" => Some(Self::June),
			b"jul" => Some(Self::July),
			b"aug" => Some(Self::August),
			b"sep" => Some(Self::September),
			b"oct" => Some(Self::October),
			b"nov" => Some(Self::November),
			b"dec" => Some(Self::December),
			_ => None,
		}
	}
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

	#[doc(hidden)]
	#[inline]
	/// # From U8 Unchecked.
	///
	/// ## Safety
	///
	/// The value must be between 1-12 or undefined things will happen!
	pub(crate) const unsafe fn from_u8_unchecked(src: u8) -> Self {
		std::mem::transmute(src)
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
			let month = Month::from(i);
			assert_eq!(month as u8, i);
			assert_eq!(month.abbreviation().as_bytes(), month.abbreviation_bytes());
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

	#[test]
	/// # Test Some Math!
	fn t_math() {
		let months: Vec<Month> = std::iter::repeat(ALL_MONTHS)
			.take(4)
			.flatten()
			.copied()
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
		for &m in ALL_MONTHS {
			assert_eq!(Ok(m), Month::try_from(m.abbreviation()));
			assert_eq!(Ok(m), Month::try_from(m.as_str()));
			assert_eq!(Ok(m), Month::try_from(m.as_str().to_ascii_uppercase()));
		}

		assert!(Month::try_from("Hello").is_err());
	}
}
