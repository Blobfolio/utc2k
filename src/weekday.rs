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
	str::FromStr,
};



#[repr(u8)]
#[derive(Debug, Clone, Copy, Default, Eq, Hash, PartialEq)]
/// # Weekday.
///
/// This is a simple enum representing days of the week.
///
/// While not particularly useful on its own, you can use it for wrapping
/// addition operations.
///
/// Otherwise this is only really used by [`Utc2k::weekday`].
pub enum Weekday {
	#[default]
	/// # Sunday.
	Sunday = 1_u8,

	/// # Monday.
	Monday,

	/// # Tuesday.
	Tuesday,

	/// # Wednesday.
	Wednesday,

	/// # Thursday.
	Thursday,

	/// # Friday.
	Friday,

	/// # Saturday.
	Saturday,
}

impl Add<u8> for Weekday {
	type Output = Self;

	#[inline]
	fn add(self, other: u8) -> Self { Self::from_u8(self as u8 + other % 7) }
}

impl AddAssign<u8> for Weekday {
	#[inline]
	fn add_assign(&mut self, other: u8) { *self = *self + other; }
}

macros::as_ref_borrow_cast!(Weekday: as_str str);

impl Deref for Weekday {
	type Target = str;

	#[inline]
	fn deref(&self) -> &Self::Target { self.as_str() }
}

macros::display_str!(as_str Weekday);

impl From<u8> for Weekday {
	#[inline]
	fn from(src: u8) -> Self { Self::from_u8(src) }
}

impl From<Weekday> for u8 {
	#[inline]
	fn from(src: Weekday) -> Self { src as Self }
}

/// # Helper: Add/From/Sub Impls.
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
			#[expect(clippy::cast_possible_truncation, reason = "False positive.")]
			fn from(src: $ty) -> Self {
				if src <= 7 { Self::from(src as u8) }
				else { Self::from((src % 7) as u8) }
			}
		}

		impl From<Weekday> for $ty {
			#[inline]
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
	/// Return an iterator that will cycle endlessly through the weeks,
	/// starting from this `Weekday`.
	fn into_iter(self) -> Self::IntoIter { RepeatingWeekdayIter(self) }
}

impl Ord for Weekday {
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
		impl PartialEq<$ty> for Weekday {
			#[inline]
			fn eq(&self, other: &$ty) -> bool { (*self as $ty) == *other }
		}
		impl PartialEq<Weekday> for $ty {
			#[inline]
			fn eq(&self, other: &Weekday) -> bool { <Weekday as PartialEq<$ty>>::eq(other, self) }
		}
	)+);
}
eq!(u8, u16, u32, u64, usize);

impl PartialOrd for Weekday {
	#[inline]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Sub<u8> for Weekday {
	type Output = Self;

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

impl TryFrom<&[u8]> for Weekday {
	type Error = Utc2kError;

	#[inline]
	/// # From Str.
	///
	/// Note: this is a lazy match, using only the first three characters.
	/// "Saturnalia", for example, will match `Weekday::Saturday`.
	fn try_from(src: &[u8]) -> Result<Self, Self::Error> {
		Self::from_abbreviation(src).ok_or(Utc2kError::Invalid)
	}
}

impl TryFrom<&str> for Weekday {
	type Error = Utc2kError;

	#[inline]
	/// # From Str.
	///
	/// Note: this is a lazy match, using only the first three characters.
	/// "Saturnalia", for example, will match `Weekday::Saturday`.
	fn try_from(src: &str) -> Result<Self, Self::Error> {
		Self::from_abbreviation(src.as_bytes()).ok_or(Utc2kError::Invalid)
	}
}

impl TryFrom<String> for Weekday {
	type Error = Utc2kError;

	#[inline]
	/// # From Str.
	///
	/// Note: this is a lazy match, using only the first three characters.
	/// "Saturnalia", for example, will match `Weekday::Saturday`.
	fn try_from(src: String) -> Result<Self, Self::Error> {
		Self::from_abbreviation(src.as_bytes()).ok_or(Utc2kError::Invalid)
	}
}

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

	/// # Abbreviation (bytes).
	///
	/// This returns the abbreviation as a fixed-size byte array.
	pub(crate) const fn abbreviation_bytes(self) -> [u8; 3] {
		match self {
			Self::Sunday => *b"Sun",
			Self::Monday => *b"Mon",
			Self::Tuesday => *b"Tue",
			Self::Wednesday => *b"Wed",
			Self::Thursday => *b"Thu",
			Self::Friday => *b"Fri",
			Self::Saturday => *b"Sat",
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
	pub(crate) const fn from_abbreviation(src: &[u8]) -> Option<Self> {
		if let [a, b, c, _rest @ ..] = src.trim_ascii_start() {
			match [a.to_ascii_lowercase(), b.to_ascii_lowercase(), c.to_ascii_lowercase()] {
				[b's', b'u', b'n'] => Some(Self::Sunday),
				[b'm', b'o', b'n'] => Some(Self::Monday),
				[b't', b'u', b'e'] => Some(Self::Tuesday),
				[b'w', b'e', b'd'] => Some(Self::Wednesday),
				[b't', b'h', b'u'] => Some(Self::Thursday),
				[b'f', b'r', b'i'] => Some(Self::Friday),
				[b's', b'a', b't'] => Some(Self::Saturday),
				_ => None,
			}
		}
		else { None }
	}

	#[must_use]
	/// # From `u8`.
	pub(crate) const fn from_u8(src: u8) -> Self {
		match src {
			1 => Self::Sunday,
			2 => Self::Monday,
			3 => Self::Tuesday,
			4 => Self::Wednesday,
			5 => Self::Thursday,
			6 => Self::Friday,
			0 | 7 => Self::Saturday,
			_ => Self::from_u8(src % 7),
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



#[derive(Debug)]
/// # Endless Weekdays!
///
/// This iterator yields an infinite number of `Weekday`s, in order, starting
/// from any arbitrary day.
pub struct RepeatingWeekdayIter(Weekday);

impl Iterator for RepeatingWeekdayIter {
	type Item = Weekday;

	/// # Next Weekday.
	fn next(&mut self) -> Option<Self::Item> {
		let next = self.0;
		self.0 = match next {
			Weekday::Sunday => Weekday::Monday,
			Weekday::Monday => Weekday::Tuesday,
			Weekday::Tuesday => Weekday::Wednesday,
			Weekday::Wednesday => Weekday::Thursday,
			Weekday::Thursday => Weekday::Friday,
			Weekday::Friday => Weekday::Saturday,
			Weekday::Saturday => Weekday::Sunday,
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
	/// # Test First of Year.
	fn t_year_start() {
		for y in 2000..=2099 {
			let c = time::Date::from_calendar_date(y, time::Month::January, 1)
				.expect("Unable to create time::Date.");
			assert_eq!(
				Weekday::year_begins_on((y - 2000) as u8).as_ref(),
				c.weekday().to_string(),
				"Failed with year {y}"
			);
		}
	}

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
			assert_eq!(weekday.abbreviation().as_bytes(), weekday.abbreviation_bytes());
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
			assert_eq!(Ok(d), Weekday::try_from(d.as_str().to_ascii_uppercase()));
			assert_eq!(Ok(d), d.abbreviation().parse());
		}

		assert!(Weekday::try_from("Hello").is_err());
	}
}
