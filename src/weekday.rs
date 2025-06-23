/*!
# UTC2K - Weekday
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
/// # Weekday.
///
/// This is a simple enum representing days of the week, useful, perhaps, for
/// printing weekday names or abbreviations.
pub enum Weekday {
	#[default]
	/// # Sunday.
	Sunday = 1_u8,

	/// # Monday.
	Monday = 2_u8,

	/// # Tuesday.
	Tuesday = 3_u8,

	/// # Wednesday.
	Wednesday = 4_u8,

	/// # Thursday.
	Thursday = 5_u8,

	/// # Friday.
	Friday = 6_u8,

	/// # Saturday.
	Saturday = 7_u8,
}

macros::as_ref_borrow_cast!(Weekday: as_str str);
macros::display_str!(as_str Weekday);

/// # Helper: Add/From/Sub Impls.
macro_rules! impl_int {
	($($ty:ty),+) => ($(
		impl Add<$ty> for Weekday {
			type Output = Self;
			#[inline]
			#[doc = concat!("# (Wrapping) Add `", stringify!($ty), "`")]
			///
			/// Weekdays range from `1..=7`.
			///
			/// ## Examples
			///
			/// ```
			/// use utc2k::Weekday;
			///
			/// let start = Weekday::Sunday;
			#[doc = concat!("assert_eq!(start + 0_", stringify!($ty), ",  Weekday::Sunday);    // Noop.")]
			#[doc = concat!("assert_eq!(start + 1_", stringify!($ty), ",  Weekday::Monday);")]
			#[doc = concat!("assert_eq!(start + 2_", stringify!($ty), ",  Weekday::Tuesday);")]
			#[doc = concat!("assert_eq!(start + 3_", stringify!($ty), ",  Weekday::Wednesday);")]
			#[doc = concat!("assert_eq!(start + 4_", stringify!($ty), ",  Weekday::Thursday);")]
			#[doc = concat!("assert_eq!(start + 5_", stringify!($ty), ",  Weekday::Friday);")]
			#[doc = concat!("assert_eq!(start + 6_", stringify!($ty), ",  Weekday::Saturday);")]
			#[doc = concat!("assert_eq!(start + 7_", stringify!($ty), ",  Weekday::Sunday);    // Wrap.")]
			#[doc = concat!("assert_eq!(start + 8_", stringify!($ty), ",  Weekday::Monday);    // Wrap.")]
			#[doc = concat!("assert_eq!(start + 9_", stringify!($ty), ",  Weekday::Tuesday);   // Wrap.")]
			#[doc = concat!("assert_eq!(start + 10_", stringify!($ty), ", Weekday::Wednesday); // Wrap.")]
			/// // …
			/// ```
			fn add(self, other: $ty) -> Self {
				Self::from(<$ty>::from(self) + other % 7)
			}
		}

		impl AddAssign<$ty> for Weekday {
			#[inline]
			fn add_assign(&mut self, other: $ty) { *self = *self + other; }
		}

		impl From<$ty> for Weekday {
			#[inline]
			#[doc = concat!("# From `", stringify!($ty), "`")]
			///
			/// Weekdays range from `1..=7`.
			///
			/// ## Examples
			///
			/// ```
			/// use utc2k::Weekday;
			///
			#[doc = concat!("assert_eq!(Weekday::from(0_", stringify!($ty), "),  Weekday::Saturday); // Wrap.")]
			#[doc = concat!("assert_eq!(Weekday::from(1_", stringify!($ty), "),  Weekday::Sunday);")]
			#[doc = concat!("assert_eq!(Weekday::from(2_", stringify!($ty), "),  Weekday::Monday);")]
			#[doc = concat!("assert_eq!(Weekday::from(3_", stringify!($ty), "),  Weekday::Tuesday);")]
			#[doc = concat!("assert_eq!(Weekday::from(4_", stringify!($ty), "),  Weekday::Wednesday);")]
			#[doc = concat!("assert_eq!(Weekday::from(5_", stringify!($ty), "),  Weekday::Thursday);")]
			#[doc = concat!("assert_eq!(Weekday::from(6_", stringify!($ty), "),  Weekday::Friday);")]
			#[doc = concat!("assert_eq!(Weekday::from(7_", stringify!($ty), "),  Weekday::Saturday);")]
			#[doc = concat!("assert_eq!(Weekday::from(8_", stringify!($ty), "),  Weekday::Sunday);   // Wrap.")]
			#[doc = concat!("assert_eq!(Weekday::from(9_", stringify!($ty), "),  Weekday::Monday);   // Wrap.")]
			#[doc = concat!("assert_eq!(Weekday::from(10_", stringify!($ty), "), Weekday::Tuesday);  // Wrap.")]
			/// // …
			/// ```
			fn from(src: $ty) -> Self {
				match src % 7 {
					1  => Self::Sunday,
					2  => Self::Monday,
					3  => Self::Tuesday,
					4  => Self::Wednesday,
					5  => Self::Thursday,
					6  => Self::Friday,
					_ => Self::Saturday,
				}
			}
		}

		impl From<Weekday> for $ty {
			#[inline]
			#[doc = concat!("# As `", stringify!($ty), "`")]
			///
			/// ## Examples
			///
			/// ```
			/// use utc2k::Weekday;
			///
			/// // Sunday is one, Saturday is seven.
			#[doc = concat!("assert_eq!(", stringify!($ty), "::from(Weekday::Sunday), 1);")]
			#[doc = concat!("assert_eq!(", stringify!($ty), "::from(Weekday::Saturday), 7);")]
			///
			/// // As casts work too.
			/// for w in Weekday::ALL {
			#[doc = concat!("    assert_eq!(", stringify!($ty), "::from(w), w as ", stringify!($ty),");")]
			/// }
			/// ```
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

		impl PartialEq<$ty> for Weekday {
			#[inline]
			#[doc = concat!("# Equality w/ `", stringify!($ty), "`")]
			///
			/// ```
			/// use utc2k::Weekday;
			///
			#[doc = concat!("assert_eq!(Weekday::Sunday, 1_", stringify!($ty), ");")]
			#[doc = concat!("assert_eq!(Weekday::Saturday, 7_", stringify!($ty), ");")]
			/// ```
			fn eq(&self, other: &$ty) -> bool { (*self as $ty) == *other }
		}
		impl PartialEq<Weekday> for $ty {
			#[inline]
			#[doc = concat!("# Equality w/ `", stringify!($ty), "`")]
			///
			/// ```
			/// use utc2k::Weekday;
			///
			#[doc = concat!("assert_eq!(1_", stringify!($ty), ", Weekday::Sunday);")]
			#[doc = concat!("assert_eq!(7_", stringify!($ty), ", Weekday::Saturday);")]
			/// ```
			fn eq(&self, other: &Weekday) -> bool { <Weekday as PartialEq<$ty>>::eq(other, self) }
		}

		impl Sub<$ty> for Weekday {
			type Output = Self;

			#[inline]
			#[doc = concat!("# (Wrapping) Sub `", stringify!($ty), "`")]
			///
			/// Weekdays range from `1..=7`.
			///
			/// ## Examples
			///
			/// ```
			/// use utc2k::Weekday;
			///
			/// let start = Weekday::Sunday;
			#[doc = concat!("assert_eq!(start - 0_", stringify!($ty), ",  Weekday::Sunday);  // Noop.")]
			#[doc = concat!("assert_eq!(start - 1_", stringify!($ty), ",  Weekday::Saturday);")]
			#[doc = concat!("assert_eq!(start - 2_", stringify!($ty), ",  Weekday::Friday);")]
			#[doc = concat!("assert_eq!(start - 3_", stringify!($ty), ",  Weekday::Thursday);")]
			#[doc = concat!("assert_eq!(start - 4_", stringify!($ty), ",  Weekday::Wednesday);")]
			#[doc = concat!("assert_eq!(start - 5_", stringify!($ty), ",  Weekday::Tuesday);")]
			#[doc = concat!("assert_eq!(start - 6_", stringify!($ty), ",  Weekday::Monday);")]
			#[doc = concat!("assert_eq!(start - 7_", stringify!($ty), ",  Weekday::Sunday);   // Full circle!")]
			#[doc = concat!("assert_eq!(start - 8_", stringify!($ty), ",  Weekday::Saturday); // Wrap #2.")]
			#[doc = concat!("assert_eq!(start - 9_", stringify!($ty), ",  Weekday::Friday);   // Wrap #2.")]
			#[doc = concat!("assert_eq!(start - 10_", stringify!($ty), ", Weekday::Thursday); // Wrap #2.")]
			/// // …
			/// ```
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

impl_int!(u8, u16, u32, u64, usize);

impl From<Utc2k> for Weekday {
	#[inline]
	/// # From [`Utc2k`].
	///
	/// This is equivalent to calling [`Utc2k::weekday`].
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::{Weekday, Utc2k};
	///
	/// let utc = Utc2k::new(2025, 6, 22, 0, 0, 0);
	/// assert_eq!(utc.weekday(),      Weekday::Sunday);
	/// assert_eq!(Weekday::from(utc), Weekday::Sunday);
	/// ```
	fn from(src: Utc2k) -> Self { src.weekday() }
}

impl FromStr for Weekday {
	type Err = Utc2kError;

	#[inline]
	fn from_str(src: &str) -> Result<Self, Self::Err> { Self::try_from(src) }
}

impl IntoIterator for Weekday {
	type Item = Self;
	type IntoIter = RepeatingWeekdayIter;

	#[inline]
	/// # Repeating Iterator.
	///
	/// Return an iterator that will cycle endlessly through the years,
	/// starting from this `Weekday`.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Weekday;
	///
	/// let mut iter = Weekday::Wednesday.into_iter();
	/// assert_eq!(iter.next(), Some(Weekday::Wednesday));
	/// assert_eq!(iter.next(), Some(Weekday::Thursday));
	/// assert_eq!(iter.next(), Some(Weekday::Friday));
	/// assert_eq!(iter.next(), Some(Weekday::Saturday));
	/// assert_eq!(iter.next(), Some(Weekday::Sunday));
	/// assert_eq!(iter.next(), Some(Weekday::Monday));
	/// assert_eq!(iter.next(), Some(Weekday::Tuesday));
	/// assert_eq!(iter.next(), Some(Weekday::Wednesday)); // Full circle!
	/// assert_eq!(iter.next(), Some(Weekday::Thursday));
	/// // …
	///
	/// // You can also go backwards.
	/// let mut iter = Weekday::Tuesday.into_iter().rev();
	/// assert_eq!(iter.next(), Some(Weekday::Tuesday));
	/// assert_eq!(iter.next(), Some(Weekday::Monday));
	/// assert_eq!(iter.next(), Some(Weekday::Sunday));
	/// assert_eq!(iter.next(), Some(Weekday::Saturday)); // Wrap!
	/// // …
	/// ```
	fn into_iter(self) -> Self::IntoIter { RepeatingWeekdayIter(self) }
}

impl Ord for Weekday {
	#[inline]
	/// # Ordering.
	///
	/// ```
	/// use utc2k::Weekday;
	///
	/// for pair in Weekday::ALL.windows(2) {
	///     assert!(pair[0] < pair[1]);
	/// }
	/// ```
	fn cmp(&self, other: &Self) -> Ordering {
		let a = *self as u8;
		let b = *other as u8;
		a.cmp(&b)
	}
}

impl PartialOrd for Weekday {
	#[inline]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl TryFrom<&[u8]> for Weekday {
	type Error = Utc2kError;

	#[inline]
	/// # From Str.
	///
	/// Note: this is a lazy match, using only the first three characters.
	/// "Saturnalia", for example, will match `Weekday::Saturday`.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Weekday;
	///
	/// // Case doesn't matter.
	/// assert_eq!(
	///     Weekday::try_from(b"monday".as_slice()),
	///     Ok(Weekday::Monday),
	/// );
	/// assert_eq!(
	///     Weekday::try_from(b"Monday".as_slice()),
	///     Ok(Weekday::Monday),
	/// );
	/// assert_eq!(
	///     Weekday::try_from(b"MONDAY".as_slice()),
	///     Ok(Weekday::Monday),
	/// );
	///
	/// // Only the first three bytes are actually inspected.
	/// assert_eq!(
	///     Weekday::try_from(b"Mon".as_slice()),
	///     Ok(Weekday::Monday),
	/// );
	/// assert_eq!(
	///     Weekday::try_from(b"money".as_slice()), // Close enough!
	///     Ok(Weekday::Monday),
	/// );
	///
	/// // Wrong is wrong.
	/// assert!(Weekday::try_from(b"moonday".as_slice()).is_err());
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
		impl TryFrom<$ty> for Weekday {
			type Error = Utc2kError;
			#[inline]
			fn try_from(src: $ty) -> Result<Self, Self::Error> {
				Self::try_from(src.as_bytes())
			}
		}
	)+);
}

try_from! { &str &String String &Cow<'_, str> Cow<'_, str> &Box<str> Box<str> }

impl Weekday {
	/// # All Weekdays.
	///
	/// An array containing all possible weekdays, in order.
	pub const ALL: [Self; 7] = [
		Self::Sunday,
		Self::Monday,
		Self::Tuesday,
		Self::Wednesday,
		Self::Thursday,
		Self::Friday,
		Self::Saturday,
	];

	#[inline]
	#[must_use]
	/// # As Str (Abbreviated).
	///
	/// Return a string slice representing the day's abbreviated name, i.e.
	/// the first three letters.
	///
	/// ## Examples.
	///
	/// ```
	/// use utc2k::Weekday;
	///
	/// for w in Weekday::ALL {
	///     assert_eq!(
	///         &w.as_str()[..3],
	///         w.abbreviation(),
	///     );
	/// }
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

	#[inline]
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
}

impl Weekday {
	#[inline]
	#[must_use]
	/// # Date of First Weekday.
	///
	/// Return the day corresponding to the first occurrence of this weekday in
	/// a given year/month.
	///
	/// This will only return `None` if you pass a bad year and/or month.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Weekday;
	///
	/// // The first Friday in November 2023 was on the 3rd.
	/// assert_eq!(
	///     Weekday::Friday.first_in_month(2023, 11),
	///     Some(3),
	/// );
	/// ```
	pub const fn first_in_month(self, y: u16, m: u8) -> Option<u8> {
		self.nth_in_month(y, m, 1)
	}

	#[inline]
	#[must_use]
	/// # Date of Last Weekday.
	///
	/// Return the day corresponding to the last occurrence of this weekday in
	/// a given year/month.
	///
	/// This will only return `None` if you pass a bad year and/or month.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Weekday;
	///
	/// // The last Saturday in Februrary 2020 was the 29th. LEAP!
	/// assert_eq!(
	///     Weekday::Saturday.last_in_month(2020, 02),
	///     Some(29),
	/// );
	/// ```
	pub const fn last_in_month(self, y: u16, m: u8) -> Option<u8> {
		// Load the first date of the month, and make sure it is sane.
		let first = Utc2k::new(y, m, 1, 0, 0, 0);
		let check = first.ymd();
		if check.0 != y || check.1 != m || check.2 != 1 { return None; }

		// Pull that first day's weekday.
		let weekday = first.weekday();

		// Find the first day.
		let w_num = weekday as u8;
		let s_num = self as u8;
		let d =
			if w_num == s_num { 1 }
			else if w_num < s_num { 1 + s_num - w_num }
			else { 8 - (w_num - s_num) };

		// Now find out how many weeks we can add to that without going over.
		let n = (first.month_size() - d).wrapping_div(7);

		// Add them and we have our answer!
		Some(d + n * 7)
	}

	#[must_use]
	/// # Date of Nth Weekday.
	///
	/// Return the day corresponding to the nth occurrence of this weekday in a
	/// given year/month, if any. (`None` is returned if it rolls over.)
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Weekday;
	///
	/// let day = Weekday::Monday;
	///
	/// // There are five Mondays in October 2023:
	/// assert_eq!(day.nth_in_month(2023, 10, 1), Some(2));
	/// assert_eq!(day.nth_in_month(2023, 10, 2), Some(9));
	/// assert_eq!(day.nth_in_month(2023, 10, 3), Some(16));
	/// assert_eq!(day.nth_in_month(2023, 10, 4), Some(23));
	/// assert_eq!(day.nth_in_month(2023, 10, 5), Some(30));
	///
	/// // But no more!
	/// assert_eq!(day.nth_in_month(2023, 10, 6), None);
	/// ```
	pub const fn nth_in_month(self, y: u16, m: u8, n: u8) -> Option<u8> {
		// Zero is meaningless, and there will never be more than five.
		if n == 0 || 6 <= n { return None; }

		// Load the first date of the month, and make sure it is sane.
		let first = Utc2k::new(y, m, 1, 0, 0, 0);
		let check = first.ymd();
		if check.0 != y || check.1 != m || check.2 != 1 { return None; }

		// Pull that first day's weekday.
		let weekday = first.weekday();

		// Calculate the day!
		let w_num = weekday as u8;
		let s_num = self as u8;
		let d =
			if w_num == s_num { 1 }
			else if w_num < s_num { 1 + s_num - w_num }
			else { 8 - (w_num - s_num) }
			// Scale to the nth.
			+ (n - 1) * 7;

		// Return it, unless we've passed into a different month.
		if d <= first.month_size() { Some(d) }
		else { None }
	}
}

impl Weekday {
	#[must_use]
	/// # From Abbreviation Bytes.
	///
	/// This matches the first three non-whitespace bytes, case-insensitively,
	/// against the `Weekday` abbreviations.
	pub(crate) const fn from_abbreviation(a: u8, b: u8, c: u8) -> Option<Self> {
		match crate::needle3(a, b, c) {
			1_684_371_200 => Some(Self::Wednesday),
			1_702_196_224 => Some(Self::Tuesday),
			1_769_104_896 => Some(Self::Friday),
			1_852_796_160 => Some(Self::Monday),
			1_853_190_912 => Some(Self::Sunday),
			1_952_543_488 => Some(Self::Saturday),
			1_969_779_712 => Some(Self::Thursday),
			_ => None,
		}
	}

	#[inline]
	#[must_use]
	/// # From `u8`.
	pub(crate) const fn from_u8(src: u8) -> Self {
		match src % 7 {
			1 => Self::Sunday,
			2 => Self::Monday,
			3 => Self::Tuesday,
			4 => Self::Wednesday,
			5 => Self::Thursday,
			6 => Self::Friday,
			_ => Self::Saturday,
		}
	}
}



#[derive(Debug)]
/// # Endless Weekdays!
///
/// This iterator yields an infinite number of `Weekday`s, in order, starting
/// from any arbitrary day.
///
/// See [`Weekday::into_iter`] for more details.
pub struct RepeatingWeekdayIter(Weekday);

impl Iterator for RepeatingWeekdayIter {
	type Item = Weekday;

	/// # Next Weekday.
	fn next(&mut self) -> Option<Self::Item> {
		let next = self.0;
		self.0 = Weekday::from_u8(self.0 as u8 + 1);
		Some(next)
	}

	/// # Infinity.
	///
	/// This iterator never stops!
	fn size_hint(&self) -> (usize, Option<usize>) { (usize::MAX, None) }
}

impl DoubleEndedIterator for RepeatingWeekdayIter {
	/// # Previous Weekday.
	fn next_back(&mut self) -> Option<Self::Item> {
		let next = self.0;
		self.0 = Weekday::from_u8(self.0 as u8 - 1);
		Some(next)
	}
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	/// # Test Fromness.
	fn t_abbr() {
		for d in Weekday::ALL {
			assert_eq!(d.abbreviation(), &d.as_str()[..3]);
		}
	}

	#[test]
	fn t_into_iter() {
		let mut last = Weekday::Saturday;
		for next in Weekday::Sunday.into_iter().take(15) {
			assert_eq!(next, last + 1_u8);
			last = next;
		}
	}

	#[test]
	/// # Test Fromness.
	fn t_from() {
		// There and back again.
		for i in 1..=7_u8 {
			let weekday = Weekday::from(i);
			assert_eq!(weekday as u8, i);
		}
		for i in 1..=7_u64 {
			assert_eq!(u64::from(Weekday::from(i)), i);
		}

		assert_eq!(Weekday::from(0_u64), Weekday::Saturday);

		let many: Vec<Weekday> = (1..=35_u32)
			.map(Weekday::from)
			.collect();

		let mut when = 0;
		for days in many.as_slice().chunks_exact(7) {
			when += 1;
			assert_eq!(days, Weekday::ALL, "Round #{when}");
		}
	}

	#[test]
	/// # Test Some Math!
	fn t_math() {
		let days: Vec<Weekday> = std::iter::repeat(Weekday::ALL)
			.take(6)
			.flatten()
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
	/// # Nth Day.
	fn t_nth_in_month() {
		// One full month should cover our bases.
		for (weekday, dates) in [
			(Weekday::Sunday,    vec![1, 8,  15, 22, 29]),
			(Weekday::Monday,    vec![2, 9,  16, 23, 30]),
			(Weekday::Tuesday,   vec![3, 10, 17, 24, 31]),
			(Weekday::Wednesday, vec![4, 11, 18, 25]),
			(Weekday::Thursday,  vec![5, 12, 19, 26]),
			(Weekday::Friday,    vec![6, 13, 20, 27]),
			(Weekday::Saturday,  vec![7, 14, 21, 28]),
		] {
			for (k, v) in dates.iter().copied().enumerate() {
				let tmp = weekday.nth_in_month(2023, 10, k as u8 + 1);
				assert_eq!(
					tmp,
					Some(v),
					"Expected {} {weekday} to be {v}, not {tmp:?}.",
					k + 1,
				);

				// Test first for the first. This is an alias so shouldn't
				// ever fail, but just in case…
				if k == 0 { assert_eq!(weekday.first_in_month(2023, 10), tmp); }
				// And last for the last.
				else if k + 1 == dates.len() {
					assert_eq!(
						weekday.last_in_month(2023, 10),
						Some(v),
						"Expected {weekday} to end on {v}, not {tmp:?}.",
					);
				}
			}

			// And make sure one more is too many.
			assert_eq!(weekday.nth_in_month(2023, 10, dates.len() as u8 + 1), None);
		}
	}

	#[test]
	/// # String Tests.
	fn t_str() {
		for d in Weekday::ALL {
			assert_eq!(Ok(d), Weekday::try_from(d.abbreviation()));
			assert_eq!(Ok(d), Weekday::try_from(d.as_str()));
			assert_eq!(Ok(d), Weekday::try_from(d.as_str().to_ascii_lowercase()));
			assert_eq!(Ok(d), Weekday::try_from(d.as_str().to_ascii_uppercase()));
			assert_eq!(Ok(d), d.abbreviation().parse());
		}

		assert!(Weekday::try_from("Hello").is_err());
	}
}
