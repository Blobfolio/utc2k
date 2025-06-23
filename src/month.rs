/*!
# UTC2K - Month
*/

use crate::{
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

macros::as_ref_borrow_cast!(Month: as_str str);

macros::display_str!(as_str Month);

/// # Helper: Add/From/Sub Impls.
macro_rules! impl_int {
	($($ty:ty),+) => ($(
		impl Add<$ty> for Month {
			type Output = Self;
			#[inline]
			#[doc = concat!("# (Wrapping) Add `", stringify!($ty), "`")]
			///
			/// Months range from `1..=12`.
			///
			/// ## Examples
			///
			/// ```
			/// use utc2k::Month;
			///
			/// let start = Month::January;
			#[doc = concat!("assert_eq!(start + 0_", stringify!($ty), ",  Month::January);  // Noop.")]
			#[doc = concat!("assert_eq!(start + 1_", stringify!($ty), ",  Month::February);")]
			#[doc = concat!("assert_eq!(start + 2_", stringify!($ty), ",  Month::March);")]
			#[doc = concat!("assert_eq!(start + 3_", stringify!($ty), ",  Month::April);")]
			#[doc = concat!("assert_eq!(start + 4_", stringify!($ty), ",  Month::May);")]
			#[doc = concat!("assert_eq!(start + 5_", stringify!($ty), ",  Month::June);")]
			#[doc = concat!("assert_eq!(start + 6_", stringify!($ty), ",  Month::July);")]
			#[doc = concat!("assert_eq!(start + 7_", stringify!($ty), ",  Month::August);")]
			#[doc = concat!("assert_eq!(start + 8_", stringify!($ty), ",  Month::September);")]
			#[doc = concat!("assert_eq!(start + 9_", stringify!($ty), ",  Month::October);")]
			#[doc = concat!("assert_eq!(start + 10_", stringify!($ty), ", Month::November);")]
			#[doc = concat!("assert_eq!(start + 11_", stringify!($ty), ", Month::December);")]
			#[doc = concat!("assert_eq!(start + 12_", stringify!($ty), ", Month::January);  // Wrap.")]
			#[doc = concat!("assert_eq!(start + 13_", stringify!($ty), ", Month::February); // Wrap.")]
			#[doc = concat!("assert_eq!(start + 14_", stringify!($ty), ", Month::March);    // Wrap.")]
			#[doc = concat!("assert_eq!(start + 15_", stringify!($ty), ", Month::April);    // Wrap.")]
			/// // …
			/// ```
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
			#[doc = concat!("# From `", stringify!($ty), "`")]
			///
			/// Months range from `1..=12`.
			///
			/// ## Examples
			///
			/// ```
			/// use utc2k::Month;
			///
			#[doc = concat!("assert_eq!(Month::from(0_", stringify!($ty), "),  Month::December);  // Wrap.")]
			#[doc = concat!("assert_eq!(Month::from(1_", stringify!($ty), "),  Month::January);")]
			#[doc = concat!("assert_eq!(Month::from(2_", stringify!($ty), "),  Month::February);")]
			#[doc = concat!("assert_eq!(Month::from(3_", stringify!($ty), "),  Month::March);")]
			#[doc = concat!("assert_eq!(Month::from(4_", stringify!($ty), "),  Month::April);")]
			#[doc = concat!("assert_eq!(Month::from(5_", stringify!($ty), "),  Month::May);")]
			#[doc = concat!("assert_eq!(Month::from(6_", stringify!($ty), "),  Month::June);")]
			#[doc = concat!("assert_eq!(Month::from(7_", stringify!($ty), "),  Month::July);")]
			#[doc = concat!("assert_eq!(Month::from(8_", stringify!($ty), "),  Month::August);")]
			#[doc = concat!("assert_eq!(Month::from(9_", stringify!($ty), "),  Month::September);")]
			#[doc = concat!("assert_eq!(Month::from(10_", stringify!($ty), "), Month::October);")]
			#[doc = concat!("assert_eq!(Month::from(11_", stringify!($ty), "), Month::November);")]
			#[doc = concat!("assert_eq!(Month::from(12_", stringify!($ty), "), Month::December);")]
			#[doc = concat!("assert_eq!(Month::from(13_", stringify!($ty), "), Month::January);  // Wrap.")]
			#[doc = concat!("assert_eq!(Month::from(14_", stringify!($ty), "), Month::February); // Wrap.")]
			#[doc = concat!("assert_eq!(Month::from(15_", stringify!($ty), "), Month::March);    // Wrap.")]
			/// // …
			/// ```
			fn from(src: $ty) -> Self {
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
			/// // January is one, December is twelve.
			#[doc = concat!("assert_eq!(", stringify!($ty), "::from(Month::January), 1);")]
			#[doc = concat!("assert_eq!(", stringify!($ty), "::from(Month::December), 12);")]
			///
			/// // As casts work too.
			/// for m in Month::ALL {
			#[doc = concat!("    assert_eq!(", stringify!($ty), "::from(m), m as ", stringify!($ty),");")]
			/// }
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

		impl PartialEq<$ty> for Month {
			#[inline]
			#[doc = concat!("# Equality w/ `", stringify!($ty), "`")]
			///
			/// ```
			/// use utc2k::Month;
			///
			#[doc = concat!("assert_eq!(Month::January, 1_", stringify!($ty), ");")]
			#[doc = concat!("assert_eq!(Month::December, 12_", stringify!($ty), ");")]
			/// ```
			fn eq(&self, other: &$ty) -> bool { (*self as $ty) == *other }
		}
		impl PartialEq<Month> for $ty {
			#[inline]
			#[doc = concat!("# Equality w/ `", stringify!($ty), "`")]
			///
			/// ```
			/// use utc2k::Month;
			///
			#[doc = concat!("assert_eq!(1_", stringify!($ty), ", Month::January);")]
			#[doc = concat!("assert_eq!(12_", stringify!($ty), ", Month::December);")]
			/// ```
			fn eq(&self, other: &Month) -> bool { <Month as PartialEq<$ty>>::eq(other, self) }
		}

		impl Sub<$ty> for Month {
			type Output = Self;

			#[inline]
			#[doc = concat!("# (Wrapping) Sub `", stringify!($ty), "`")]
			///
			/// Months range from `1..=12`.
			///
			/// ## Examples
			///
			/// ```
			/// use utc2k::Month;
			///
			/// let start = Month::January;
			#[doc = concat!("assert_eq!(start - 0_", stringify!($ty), ",  Month::January);  // Noop.")]
			#[doc = concat!("assert_eq!(start - 1_", stringify!($ty), ",  Month::December);")]
			#[doc = concat!("assert_eq!(start - 2_", stringify!($ty), ",  Month::November);")]
			#[doc = concat!("assert_eq!(start - 3_", stringify!($ty), ",  Month::October);")]
			#[doc = concat!("assert_eq!(start - 4_", stringify!($ty), ",  Month::September);")]
			#[doc = concat!("assert_eq!(start - 5_", stringify!($ty), ",  Month::August);")]
			#[doc = concat!("assert_eq!(start - 6_", stringify!($ty), ",  Month::July);")]
			#[doc = concat!("assert_eq!(start - 7_", stringify!($ty), ",  Month::June);")]
			#[doc = concat!("assert_eq!(start - 8_", stringify!($ty), ",  Month::May);")]
			#[doc = concat!("assert_eq!(start - 9_", stringify!($ty), ",  Month::April);")]
			#[doc = concat!("assert_eq!(start - 10_", stringify!($ty), ", Month::March);")]
			#[doc = concat!("assert_eq!(start - 11_", stringify!($ty), ", Month::February);")]
			#[doc = concat!("assert_eq!(start - 12_", stringify!($ty), ", Month::January);  // Full circle!")]
			#[doc = concat!("assert_eq!(start - 13_", stringify!($ty), ", Month::December); // Wrap #2.")]
			#[doc = concat!("assert_eq!(start - 14_", stringify!($ty), ", Month::November); // Wrap #2.")]
			#[doc = concat!("assert_eq!(start - 15_", stringify!($ty), ", Month::October);  // Wrap #2.")]
			/// // …
			/// ```
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

impl_int!(u8, u16, u32, u64, usize);

impl From<Utc2k> for Month {
	#[inline]
	/// # From [`Utc2k`].
	///
	/// This is equivalent to calling [`Utc2k::month`].
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::{Month, Utc2k};
	///
	/// let utc = Utc2k::new(2030, 3, 17, 0, 0, 0);
	/// assert_eq!(utc.month(),      Month::March);
	/// assert_eq!(Month::from(utc), Month::March);
	/// ```
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
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Month;
	///
	/// let mut iter = Month::March.into_iter();
	/// assert_eq!(iter.next(), Some(Month::March));
	/// assert_eq!(iter.next(), Some(Month::April));
	/// assert_eq!(iter.next(), Some(Month::May));
	/// assert_eq!(iter.next(), Some(Month::June));
	/// assert_eq!(iter.next(), Some(Month::July));
	/// assert_eq!(iter.next(), Some(Month::August));
	/// assert_eq!(iter.next(), Some(Month::September));
	/// assert_eq!(iter.next(), Some(Month::October));
	/// assert_eq!(iter.next(), Some(Month::November));
	/// assert_eq!(iter.next(), Some(Month::December));
	/// assert_eq!(iter.next(), Some(Month::January));
	/// assert_eq!(iter.next(), Some(Month::February));
	/// assert_eq!(iter.next(), Some(Month::March)); // Back around again!
	/// // …
	///
	/// // You can also go backwards.
	/// let mut iter = Month::March.into_iter().rev();
	/// assert_eq!(iter.next(), Some(Month::March));
	/// assert_eq!(iter.next(), Some(Month::February));
	/// assert_eq!(iter.next(), Some(Month::January));
	/// assert_eq!(iter.next(), Some(Month::December)); // Wrap!
	/// // …
	/// ```
	fn into_iter(self) -> Self::IntoIter { RepeatingMonthIter(self) }
}

impl Ord for Month {
	#[inline]
	/// # Ordering.
	///
	/// ```
	/// use utc2k::Month;
	///
	/// for pair in Month::ALL.windows(2) {
	///     assert!(pair[0] < pair[1]);
	/// }
	/// ```
	fn cmp(&self, other: &Self) -> Ordering {
		let a = *self as u8;
		let b = *other as u8;
		a.cmp(&b)
	}
}

impl PartialOrd for Month {
	#[inline]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl TryFrom<&[u8]> for Month {
	type Error = Utc2kError;

	#[inline]
	/// # From Str.
	///
	/// Note: this is a lazy match, using only the first three characters.
	/// "Decimal", for example, will match `Month::December`.
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
	pub fn now() -> Self { Utc2k::now().month() }
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
	/// Return a string slice representing the month's abbreviated name, i.e.
	/// the first three letters.
	///
	/// ## Examples.
	///
	/// ```
	/// use utc2k::Month;
	///
	/// for m in Month::ALL {
	///     assert_eq!(
	///         &m.as_str()[..3],
	///         m.abbreviation(),
	///     );
	/// }
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
///
/// See [`Month::into_iter`] for more details.
pub struct RepeatingMonthIter(Month);

impl Iterator for RepeatingMonthIter {
	type Item = Month;

	/// # Next Month.
	fn next(&mut self) -> Option<Self::Item> {
		let next = self.0;
		self.0 = Month::from_u8(self.0 as u8 + 1);
		Some(next)
	}

	/// # Infinity.
	///
	/// This iterator never stops!
	fn size_hint(&self) -> (usize, Option<usize>) { (usize::MAX, None) }
}

impl DoubleEndedIterator for RepeatingMonthIter {
	/// # Previous Month.
	fn next_back(&mut self) -> Option<Self::Item> {
		let next = self.0;
		self.0 = Month::from_u8(self.0 as u8 - 1);
		Some(next)
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
