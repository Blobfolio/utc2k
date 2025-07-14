/*!
# UTC2K - Weekday
*/

#![expect(
	clippy::cast_possible_truncation,
	trivial_numeric_casts,
	reason = "False positive.",
)]

use crate::{
	macros,
	Month,
	Utc2k,
	Utc2kError,
	Year,
};



macros::weekmonth! {
	Weekday weekday
	RepeatingWeekdayIter
	Sunday    1 "Sun" (0   0),
	Monday    2 "Mon" (1 250),
	Tuesday   3 "Tue" (2 251),
	Wednesday 4 "Wed" (3 252),
	Thursday  5 "Thu" (4 253),
	Friday    6 "Fri" (5 254),
	Saturday  7 "Sat" (6 255),
}

impl TryFrom<&[u8]> for Weekday {
	type Error = Utc2kError;

	#[inline]
	/// # From Byte Slice.
	///
	/// Parse a `Weekday` from the first three bytes of a slice,
	/// case-insensitively.
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
	/// use utc2k::{Month, Weekday};
	///
	/// // The first Friday in November 2023 was on the 3rd.
	/// assert_eq!(
	///     Weekday::Friday.first_in_month(2023, Month::November),
	///     Some(3),
	/// );
	/// ```
	pub const fn first_in_month(self, y: u16, m: Month) -> Option<u8> {
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
	/// use utc2k::{Month, Weekday};
	///
	/// // The last Saturday in Februrary 2020 was the 29th. LEAP!
	/// assert_eq!(
	///     Weekday::Saturday.last_in_month(2020, Month::February),
	///     Some(29),
	/// );
	/// ```
	pub const fn last_in_month(self, y: u16, m: Month) -> Option<u8> {
		// Make sure the year is valid.
		if let Some(y) = Year::from_u16_checked(y) {
			// Load the first of the month.
			let first = Utc2k::from_ym(y, m);
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
		else { None }
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
	/// use utc2k::{Month, Weekday};
	///
	/// let day = Weekday::Monday;
	///
	/// // There are five Mondays in October 2023:
	/// assert_eq!(day.nth_in_month(2023, Month::October, 1), Some(2));
	/// assert_eq!(day.nth_in_month(2023, Month::October, 2), Some(9));
	/// assert_eq!(day.nth_in_month(2023, Month::October, 3), Some(16));
	/// assert_eq!(day.nth_in_month(2023, Month::October, 4), Some(23));
	/// assert_eq!(day.nth_in_month(2023, Month::October, 5), Some(30));
	///
	/// // But no more!
	/// assert_eq!(day.nth_in_month(2023, Month::October, 6), None);
	/// ```
	pub const fn nth_in_month(self, y: u16, m: Month, n: u8) -> Option<u8> {
		// Make sure the year is valid.
		if 0 < n && n < 6 && let Some(y) = Year::from_u16_checked(y) {
			// Load the first of the month.
			let first = Utc2k::from_ym(y, m);
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
		for next in Weekday::Sunday.into_iter().take(25) {
			assert_eq!(next, last + 1_u8);
			assert_eq!(next, last.next());
			last = next;
		}

		last = Weekday::Sunday;
		for next in Weekday::Saturday.into_iter().rev().take(25) {
			assert_eq!(next, last - 1_u8);
			assert_eq!(next, last.previous());
			last = next;
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
				let tmp = weekday.nth_in_month(2023, Month::October, k as u8 + 1);
				assert_eq!(
					tmp,
					Some(v),
					"Expected {} {weekday} to be {v}, not {tmp:?}.",
					k + 1,
				);

				// Test first for the first. This is an alias so shouldn't
				// ever fail, but just in case…
				if k == 0 { assert_eq!(weekday.first_in_month(2023, Month::October), tmp); }
				// And last for the last.
				else if k + 1 == dates.len() {
					assert_eq!(
						weekday.last_in_month(2023, Month::October),
						Some(v),
						"Expected {weekday} to end on {v}, not {tmp:?}.",
					);
				}
			}

			// And make sure one more is too many.
			assert_eq!(weekday.nth_in_month(2023, Month::October, dates.len() as u8 + 1), None);
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
