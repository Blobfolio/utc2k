/*!
# UTC2K
*/

mod abacus;

#[cfg(feature = "local")]
#[cfg_attr(docsrs, doc(cfg(feature = "local")))]
pub(super) mod local;

use crate::{
	DateChar,
	DAY_IN_SECONDS,
	HOUR_IN_SECONDS,
	macros,
	MINUTE_IN_SECONDS,
	Month,
	unixtime,
	Utc2kError,
	Weekday,
};
use std::{
	borrow::Cow,
	cmp::Ordering,
	ffi::OsStr,
	fmt,
	ops::{
		Add,
		AddAssign,
		Sub,
		SubAssign,
	},
	str::FromStr,
};
use abacus::Abacus;



#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
/// # Formatted UTC2K.
///
/// This is the formatted companion to [`Utc2k`]. You can use it to obtain a
/// string version of the date, print it, etc.
///
/// While this acts essentially as a glorified `String`, it is sized exactly
/// and therefore requires less memory to represent. It also implements `Copy`.
///
/// It follows the simple Unix date format of `YYYY-MM-DD hh:mm:ss`.
///
/// Speaking of, you can obtain an `&str` using `AsRef<str>`,
/// `Borrow<str>`, or [`FmtUtc2k::as_str`].
///
/// If you only want the date or time half, call [`FmtUtc2k::date`] or
/// [`FmtUtc2k::time`] respectively.
///
/// ## Examples
///
/// Generally it makes more sense to initialize a [`Utc2k`] first, but you can
/// skip straight to a `FmtUtc2k` instead:
///
/// ```
/// use utc2k::{FmtUtc2k, Utc2k};
///
/// // Start with the current date/time.
/// let date = FmtUtc2k::now();
///
/// // Source from a specific timestamp.
/// let date = FmtUtc2k::from(946_684_800_u32);
/// assert_eq!(date.as_str(), "2000-01-01 00:00:00");
///
/// // Source from a `Utc2k`.
/// let utc_date = Utc2k::from(946_684_800_u32);
/// assert_eq!(FmtUtc2k::from(utc_date), utc_date.formatted());
/// ```
pub struct FmtUtc2k([DateChar; 19]);

impl AsRef<[u8]> for FmtUtc2k {
	#[inline]
	fn as_ref(&self) -> &[u8] { self.as_bytes() }
}

macros::as_ref_borrow_cast!(FmtUtc2k: as_str str);

impl Default for FmtUtc2k {
	#[inline]
	fn default() -> Self { Self::MIN }
}

macros::display_str!(as_str FmtUtc2k);

impl From<u32> for FmtUtc2k {
	#[inline]
	fn from(src: u32) -> Self { Self::from(Utc2k::from_unixtime(src)) }
}

impl From<&Utc2k> for FmtUtc2k {
	#[inline]
	fn from(src: &Utc2k) -> Self { Self::from_utc2k(*src) }
}

impl From<Utc2k> for FmtUtc2k {
	#[inline]
	fn from(src: Utc2k) -> Self { Self::from_utc2k(src) }
}

impl From<FmtUtc2k> for String {
	#[inline]
	fn from(src: FmtUtc2k) -> Self { src.as_str().to_owned() }
}

impl FromStr for FmtUtc2k {
	type Err = Utc2kError;

	#[inline]
	fn from_str(src: &str) -> Result<Self, Self::Err> { Self::try_from(src) }
}

impl Ord for FmtUtc2k {
	#[inline]
	fn cmp(&self, other: &Self) -> Ordering { self.0.cmp(&other.0) }
}

impl PartialEq<str> for FmtUtc2k {
	#[inline]
	fn eq(&self, other: &str) -> bool { self.as_str() == other }
}
impl PartialEq<FmtUtc2k> for str {
	#[inline]
	fn eq(&self, other: &FmtUtc2k) -> bool { <FmtUtc2k as PartialEq<Self>>::eq(other, self) }
}

/// # Helper: Reciprocal `PartialEq`.
macro_rules! fmt_eq {
	($($ty:ty)+) => ($(
		impl PartialEq<$ty> for FmtUtc2k {
			#[inline]
			fn eq(&self, other: &$ty) -> bool { <Self as PartialEq<str>>::eq(self, other) }
		}
		impl PartialEq<FmtUtc2k> for $ty {
			#[inline]
			fn eq(&self, other: &FmtUtc2k) -> bool { <FmtUtc2k as PartialEq<str>>::eq(other, self) }
		}
	)+);
}
fmt_eq! { &str &String String &Cow<'_, str> Cow<'_, str> &Box<str> Box<str> }

impl PartialOrd for FmtUtc2k {
	#[inline]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

/// # Helper: `TryFrom` Wrappers.
macro_rules! fmt_try_from {
	($($ty:ty)+) => ($(
		impl TryFrom<$ty> for FmtUtc2k {
			type Error = Utc2kError;
			#[inline]
			fn try_from(src: $ty) -> Result<Self, Self::Error> {
				Utc2k::try_from(src).map(Self::from)
			}
		}
	)+);
}

fmt_try_from! { &[u8] &OsStr &str u64 usize i32 i64 isize }

/// ## Min/Max.
impl FmtUtc2k {
	/// # Minimum Date/Time.
	///
	/// ```
	/// assert_eq!(
	///     utc2k::FmtUtc2k::MIN.as_str(),
	///     "2000-01-01 00:00:00",
	/// );
	/// ```
	pub const MIN: Self = Self([
		DateChar::Digit2, DateChar::Digit0, DateChar::Digit0, DateChar::Digit0,
		DateChar::Dash,
		DateChar::Digit0, DateChar::Digit1,
		DateChar::Dash,
		DateChar::Digit0, DateChar::Digit1,
		DateChar::Space,
		DateChar::Digit0, DateChar::Digit0,
		DateChar::Colon,
		DateChar::Digit0, DateChar::Digit0,
		DateChar::Colon,
		DateChar::Digit0, DateChar::Digit0,
	]);

	/// # Maximum Date/Time.
	///
	/// ```
	/// assert_eq!(
	///     utc2k::FmtUtc2k::MAX.as_str(),
	///     "2099-12-31 23:59:59",
	/// );
	/// ```
	pub const MAX: Self = Self([
		DateChar::Digit2, DateChar::Digit0, DateChar::Digit9, DateChar::Digit9,
		DateChar::Dash,
		DateChar::Digit1, DateChar::Digit2,
		DateChar::Dash,
		DateChar::Digit3, DateChar::Digit1,
		DateChar::Space,
		DateChar::Digit2, DateChar::Digit3,
		DateChar::Colon,
		DateChar::Digit5, DateChar::Digit9,
		DateChar::Colon,
		DateChar::Digit5, DateChar::Digit9,
	]);

	/// # Length.
	///
	/// The length of the formatted datetime in string/byte form.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::FmtUtc2k;
	///
	/// assert_eq!(
	///     FmtUtc2k::MIN.as_str().len(),
	///     FmtUtc2k::LEN,
	/// );
	/// ```
	pub const LEN: usize = 19;
}

/// ## Instantiation/Reuse.
impl FmtUtc2k {
	#[must_use]
	#[inline]
	/// # From ASCII Date/Time Slice.
	///
	/// Try to parse a date/time value from an ASCII slice, returning a
	/// [`FmtUtc2k`] instance if successful, `None` if not.
	///
	/// Note that this method will automatically clamp dates outside the
	/// supported `2000..=2099` range to [`FmtUtc2k::MIN`]/[`FmtUtc2k::MAX`].
	///
	/// See [`Utc2k::from_ascii`] for a rundown of supported formats, etc.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::FmtUtc2k;
	///
	/// // Separators are flexible.
	/// let dates: [&[u8]; 5] = [
	///     b"20250615",   // Squished.
	///     b"2025 06 15", // Spaced.
	///     b"2025/06/15", // Slashed.
	///     b"2025-06-15", // Dashed.
	///     b"2025#06#15", // Hashed? Haha.
	/// ];
	/// for raw in dates {
	///     assert_eq!(
	///         FmtUtc2k::from_ascii(raw).unwrap().as_str(),
	///         "2025-06-15 00:00:00",
	/// //                  ^  ^  ^ Time defaults to midnight.
	///     );
	/// }
	///
	/// // Same for datetimes.
	/// let datetimes: [&[u8]; 8] = [
	///     b"20250615123001",
	///     b"2025-06-15 12:30:01",
	///     b"2025-06-15T12:30:01Z",
	///     b"2025/06/15:12:30:01 GMT",
	///     b"2025/06/15:12:30:01 UT",
	///     b"2025/06/15:12:30:01 UTC",
	///     b"2025/06/15 12:30:01.000 +0000",
	///     b"2025/06/15 12:30:01+0000",
	/// ];
	/// for raw in datetimes {
	///     assert_eq!(
	///         FmtUtc2k::from_ascii(raw).unwrap().as_str(),
	///         "2025-06-15 12:30:01",
	///     );
	/// }
	/// ```
	pub const fn from_ascii(src: &[u8]) -> Option<Self> {
		if let Some(parts) = Utc2k::from_ascii(src) {
			Some(Self::from_utc2k(parts))
		}
		else { None }
	}

	#[must_use]
	#[inline]
	/// # From [RFC2822](https://datatracker.ietf.org/doc/html/rfc2822) Date/Time Slice.
	///
	/// Try to parse a date/time value from a [RFC2822](https://datatracker.ietf.org/doc/html/rfc2822)-formatted
	/// byte slice, returning a [`FmtUtc2k`] instance if successful, `None` if
	/// not.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::FmtUtc2k;
	///
	/// // This spec tolerates a lot of variation…
	/// let dates: [&[u8]; 7] = [
	///     b"Tue, 1 Jul 2003 10:52:37 +0000",  // Single-digit day.
	///     b"Tue,  1 Jul 2003 10:52:37 +0000", // Digit/space substitution.
	///     b"Tue, 01 Jul 2003 10:52:37 +0000", // Leading zero.
	///     b"1 Jul 2003 10:52:37",             // No weekday or offset.
	///     b"01 Jul 2003 10:52:37",            // Same, but w/ leading zero.
	///     b"Tue, 01 Jul 2003 03:52:37 -0700", // Negative UTC offset.
	///     b"Tue, 1 Jul 2003 15:22:37 +0430",  // Positive UTC offset.
	/// ];
	///
	/// for raw in dates {
	///     assert_eq!(
	///         FmtUtc2k::from_rfc2822(raw).unwrap().as_str(),
	///         "2003-07-01 10:52:37",
	///     );
	/// }
	///
	/// // The same variation exists for date-only representations too.
	/// let dates: [&[u8]; 5] = [
	///     b"Tue, 1 Jul 2003",  // Single-digit day.
	///     b"Tue,  1 Jul 2003", // Digit/space substitution.
	///     b"Tue, 01 Jul 2003", // Leading zero.
	///     b"1 Jul 2003",       // No weekday or offset.
	///     b"01 Jul 2003",      // Same, but w/ leading zero.
	/// ];
	///
	/// for raw in dates {
	///     assert_eq!(
	///         FmtUtc2k::from_rfc2822(raw).unwrap().as_str(),
	///         "2003-07-01 00:00:00",
	///     );
	/// }
	/// ```
	pub const fn from_rfc2822(src: &[u8]) -> Option<Self> {
		if let Some(parts) = Utc2k::from_rfc2822(src) {
			Some(Self::from_utc2k(parts))
		}
		else { None }
	}

	#[must_use]
	#[inline]
	/// # From Timestamp.
	///
	/// Initialize a new [`FmtUtc2k`] from a unix timestamp, saturating to
	/// [`FmtUtc2k::MIN`]/[`FmtUtc2k::MAX`] if out of range.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::FmtUtc2k;
	///
	/// assert_eq!(
	///     FmtUtc2k::from_unixtime(1_748_672_925).as_str(),
	///     "2025-05-31 06:28:45",
	/// );
	///
	/// // Same as the above, but using the `From<u32>` impl.
	/// assert_eq!(
	///     FmtUtc2k::from(1_748_672_925_u32).as_str(),
	///     "2025-05-31 06:28:45",
	/// );
	///
	/// // Out of range values will saturate to the boundaries of the
	/// // century.
	/// assert_eq!(
	///     FmtUtc2k::from_unixtime(0).as_str(),
	///     "2000-01-01 00:00:00",
	/// );
	/// assert_eq!(
	///     FmtUtc2k::from_unixtime(u32::MAX).as_str(),
	///     "2099-12-31 23:59:59",
	/// );
	/// ```
	pub const fn from_unixtime(src: u32) -> Self {
		Self::from_utc2k(Utc2k::from_unixtime(src))
	}

	#[must_use]
	#[inline]
	/// # Now.
	///
	/// This returns an instance using the current unixtime as the seed.
	pub fn now() -> Self { Self::from_utc2k(Utc2k::now()) }

	#[expect(clippy::cast_possible_truncation, reason = "False positive.")]
	/// # Set Date/Time.
	///
	/// This can be used to recycle an existing buffer.
	///
	/// As with all other part-based operations, overflows and underflows will
	/// be adjusted automatically, with minimum and maximum dates capped to
	/// [`FmtUtc2k::MIN`] and [`FmtUtc2k::MAX`] respectively.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::{FmtUtc2k, Utc2k};
	///
	/// let mut fmt = FmtUtc2k::default();
	/// assert_eq!(fmt.as_str(), "2000-01-01 00:00:00");
	///
	/// fmt.set_datetime(Utc2k::from(Utc2k::MAX_UNIXTIME));
	/// assert_eq!(fmt.as_str(), "2099-12-31 23:59:59");
	/// ```
	pub const fn set_datetime(&mut self, src: Utc2k) {
		let (y, m, d, hh, mm, ss) = src.parts();
		self.set_parts_unchecked((y - 2000) as u8, m, d, hh, mm, ss);
	}

	/// # Set Parts.
	///
	/// This can be used to recycle an existing buffer.
	///
	/// As with all other part-based operations, overflows and underflows will
	/// be adjusted automatically, with minimum and maximum dates capped to
	/// [`FmtUtc2k::MIN`] and [`FmtUtc2k::MAX`] respectively.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::{FmtUtc2k, Utc2k};
	///
	/// let mut fmt = FmtUtc2k::default();
	/// assert_eq!(fmt.as_str(), "2000-01-01 00:00:00");
	///
	/// fmt.set_parts(2010, 10, 31, 12, 33, 59);
	/// assert_eq!(fmt.as_str(), "2010-10-31 12:33:59");
	///
	/// // And if you do something weird with the dates...
	/// fmt.set_parts(2010, 10, 32, 12, 33, 59);
	/// assert_eq!(fmt.as_str(), "2010-11-01 12:33:59");
	/// ```
	pub const fn set_parts(&mut self, y: u16, m: u8, d: u8, hh: u8, mm: u8, ss: u8) {
		let (y, m, d, hh, mm, ss) = Abacus::new(y, m, d, hh, mm, ss).parts();
		self.set_parts_unchecked(y, m, d, hh, mm, ss);
	}

	#[inline]
	/// # Set Unixtime.
	///
	/// This can be used to recycle an existing buffer.
	///
	/// As with all other part-based operations, overflows and underflows will
	/// be adjusted automatically, with minimum and maximum dates capped to
	/// [`Utc2k::MIN_UNIXTIME`] and [`Utc2k::MAX_UNIXTIME`] respectively.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::{FmtUtc2k, Utc2k};
	///
	/// let mut fmt = FmtUtc2k::from(Utc2k::MIN_UNIXTIME);
	/// assert_eq!(fmt.as_str(), "2000-01-01 00:00:00");
	///
	/// fmt.set_unixtime(Utc2k::MAX_UNIXTIME);
	/// assert_eq!(fmt.as_str(), "2099-12-31 23:59:59");
	/// ```
	pub fn set_unixtime(&mut self, src: u32) { self.set_datetime(Utc2k::from(src)); }
}

/// ## Getters.
impl FmtUtc2k {
	#[inline]
	#[must_use]
	/// # As Bytes.
	///
	/// Return a byte string slice in `YYYY-MM-DD hh:mm:ss` format.
	///
	/// A byte slice can also be obtained using [`FmtUtc2k::as_ref`].
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::FmtUtc2k;
	///
	/// let fmt = FmtUtc2k::MAX;
	/// assert_eq!(fmt.as_bytes(), b"2099-12-31 23:59:59");
	/// ```
	pub const fn as_bytes(&self) -> &[u8] { DateChar::as_bytes(self.0.as_slice()) }

	#[inline]
	#[must_use]
	/// # As Str.
	///
	/// Return a string slice in `YYYY-MM-DD hh:mm:ss` format.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::FmtUtc2k;
	///
	/// let fmt = FmtUtc2k::MAX;
	/// assert_eq!(fmt.as_str(), "2099-12-31 23:59:59");
	/// ```
	pub const fn as_str(&self) -> &str { DateChar::as_str(self.0.as_slice()) }

	#[inline]
	#[must_use]
	/// # Just the Date Bits.
	///
	/// This returns the date as a string slice in `YYYY-MM-DD` format.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::{FmtUtc2k, Utc2k};
	///
	/// let fmt = FmtUtc2k::from(Utc2k::MAX_UNIXTIME);
	/// assert_eq!(fmt.as_str(), "2099-12-31 23:59:59");
	/// assert_eq!(fmt.date(), "2099-12-31");
	/// ```
	pub const fn date(&self) -> &str {
		let (out, _) = self.0.split_at(10);
		DateChar::as_str(out)
	}

	#[inline]
	#[must_use]
	/// # Just the Year Bit.
	///
	/// This returns the year as a string slice.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::{FmtUtc2k, Utc2k};
	///
	/// let fmt = FmtUtc2k::from(Utc2k::MAX_UNIXTIME);
	/// assert_eq!(fmt.as_str(), "2099-12-31 23:59:59");
	/// assert_eq!(fmt.year(), "2099");
	/// ```
	pub const fn year(&self) -> &str {
		let (out, _) = self.0.split_at(4);
		DateChar::as_str(out)
	}

	#[inline]
	#[must_use]
	/// # Just the Time Bits.
	///
	/// This returns the time as a string slice in `hh:mm:ss` format.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::{FmtUtc2k, Utc2k};
	///
	/// let fmt = FmtUtc2k::from(Utc2k::MAX_UNIXTIME);
	/// assert_eq!(fmt.as_str(), "2099-12-31 23:59:59");
	/// assert_eq!(fmt.time(), "23:59:59");
	/// ```
	pub const fn time(&self) -> &str {
		let (_, out) = self.0.split_at(11);
		DateChar::as_str(out)
	}
}

/// ## Formatting.
impl FmtUtc2k {
	#[must_use]
	/// # To RFC2822.
	///
	/// Return a string formatted according to [RFC2822](https://datatracker.ietf.org/doc/html/rfc2822).
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::{FmtUtc2k, Utc2k};
	///
	/// let date = FmtUtc2k::from(Utc2k::new(2003, 7, 1, 10, 52, 37));
	/// assert_eq!(
	///     date.to_rfc2822(),
	///     "Tue, 01 Jul 2003 10:52:37 +0000",
	/// //        ^ This implementation zero-pads short day
	/// //          numbers rather than truncating them…
	/// );
	///
	/// let date = FmtUtc2k::from(Utc2k::new(2036, 12, 15, 16, 30, 55));
	/// assert_eq!(
	///     date.to_rfc2822(),
	///     "Mon, 15 Dec 2036 16:30:55 +0000",
	/// //   ^-----------------------------^ …to keep the output
	/// //                                   length consistent.
	/// );
	/// ```
	pub fn to_rfc2822(&self) -> String {
		let utc = Utc2k::from_fmtutc2k(*self);

		let mut out = String::with_capacity(31);
		out.push_str(utc.weekday().abbreviation());
		out.push_str(", ");
		out.push(self.0[8].as_char());
		out.push(self.0[9].as_char());
		out.push(' ');
		out.push_str(utc.month_enum().abbreviation());
		out.push(' ');
		out.push_str(self.year());
		out.push(' ');
		out.push_str(self.time());
		out.push_str(" +0000");

		out
	}

	#[must_use]
	/// # To RFC3339.
	///
	/// Return a string formatted according to [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339).
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::{FmtUtc2k, Utc2k};
	///
	/// let mut fmt = FmtUtc2k::from(Utc2k::MIN_UNIXTIME);
	/// assert_eq!(fmt.to_rfc3339(), "2000-01-01T00:00:00Z");
	///
	/// fmt.set_unixtime(Utc2k::MAX_UNIXTIME);
	/// assert_eq!(fmt.to_rfc3339(), "2099-12-31T23:59:59Z");
	///
	/// // The reverse operation — parsing an RFC3339 datetime string into
	/// // a FmtUtc2k — can be done using `FmtUtc2k::from_ascii`.
	/// assert_eq!(
	///     FmtUtc2k::from_ascii(fmt.to_rfc3339().as_bytes()),
	///     Some(fmt),
	/// );
	/// ```
	pub fn to_rfc3339(&self) -> String {
		let mut out = String::with_capacity(20);
		out.push_str(self.date());
		out.push('T');
		out.push_str(self.time());
		out.push('Z');
		out
	}
}

/// ## Internal Helpers.
impl FmtUtc2k {
	#[must_use]
	/// # From `Utc2k`.
	const fn from_utc2k(src: Utc2k) -> Self {
		let mut out = Self::MIN;
		out.set_datetime(src);
		out
	}

	/// # Set Parts (Unchecked).
	///
	/// Carry-overs, saturating, and 4-to-2-digit year-chopping have already
	/// been applied by the time this method is called.
	///
	/// From here, it's just straight ASCII-writing.
	const fn set_parts_unchecked(&mut self, y: u8, m: u8, d: u8, hh: u8, mm: u8, ss: u8) {
		[self.0[2],  self.0[3]] =  DateChar::dd(y);
		[self.0[5],  self.0[6]] =  DateChar::dd(m);
		[self.0[8],  self.0[9]] =  DateChar::dd(d);
		[self.0[11], self.0[12]] = DateChar::dd(hh);
		[self.0[14], self.0[15]] = DateChar::dd(mm);
		[self.0[17], self.0[18]] = DateChar::dd(ss);
	}
}



#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
/// # UTC2K.
///
/// This is a lightweight date/time object for UTC date ranges within the
/// current century (i.e. `2000-01-01 00:00:00..=2099-12-31 23:59:59`).
///
/// Values outside this range are saturated to fit, unless using methods like
/// [`Utc2k::checked_from_ascii`] or [`Utc2k::checked_from_unixtime`].
///
/// To manually construct from individual parts, you can just call [`Utc2k::new`].
///
/// A `Utc2k` object can be turned back into its constituent parts via
/// [`Utc2k::parts`], or the individual methods like [`Utc2k::year`], [`Utc2k::month`],
/// etc.
///
/// It can be converted into a unix timestamp with [`Utc2k::unixtime`].
///
/// ## Examples
///
/// ```
/// use utc2k::Utc2k;
///
/// let date = Utc2k::default(); // 2000-01-01 00:00:00
/// let date = Utc2k::now(); // The current time.
/// let date = Utc2k::from(4_102_444_799_u32); // 2099-12-31 23:59:59
///
/// // String parsing is fallible, but flexible. So long as the numbers we
/// // need are in the right place, it will be fine.
/// assert!(Utc2k::try_from("2099-12-31 23:59:59").is_ok()); // Fine.
/// assert!(Utc2k::try_from("2099-12-31").is_ok()); // Also fine, but midnight.
/// assert!(Utc2k::try_from("2099-12-31T23:59:59.1234").is_ok()); // Also fine, but floored.
/// assert!(Utc2k::try_from("January 1, 2010").is_err()); // Nope!
/// ```
pub struct Utc2k {
	/// # Year.
	y: u8,

	/// # Month.
	m: u8,

	/// # Day.
	d: u8,

	/// # Hour.
	hh: u8,

	/// # Minute.
	mm: u8,

	/// # Second.
	ss: u8,
}

impl Add<u32> for Utc2k {
	type Output = Self;

	#[inline]
	fn add(self, other: u32) -> Self {
		Self::from_abacus(Abacus::from_utc2k(self).plus_seconds(other))
	}
}

impl AddAssign<u32> for Utc2k {
	#[inline]
	fn add_assign(&mut self, other: u32) { *self = *self + other; }
}

impl Default for Utc2k {
	#[inline]
	fn default() -> Self { Self::MIN }
}

impl fmt::Display for Utc2k {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		<FmtUtc2k as fmt::Display>::fmt(&FmtUtc2k::from_utc2k(*self), f)
	}
}

impl From<u32> for Utc2k {
	#[inline]
	/// # From Timestamp.
	///
	/// Note, this will saturate to [`Utc2k::MIN_UNIXTIME`] and
	/// [`Utc2k::MAX_UNIXTIME`] if the timestamp is out of range.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// assert_eq!(Utc2k::from(0).to_string(), "2000-01-01 00:00:00");
	/// assert_eq!(Utc2k::from(u32::MAX).to_string(), "2099-12-31 23:59:59");
	/// ```
	fn from(src: u32) -> Self { Self::from_unixtime(src) }
}

impl From<&FmtUtc2k> for Utc2k {
	#[inline]
	fn from(src: &FmtUtc2k) -> Self { Self::from(*src) }
}

impl From<FmtUtc2k> for Utc2k {
	#[inline]
	fn from(src: FmtUtc2k) -> Self { Self::from_fmtutc2k(src) }
}

impl From<Utc2k> for String {
	#[inline]
	fn from(src: Utc2k) -> Self { Self::from(FmtUtc2k::from_utc2k(src)) }
}

impl FromStr for Utc2k {
	type Err = Utc2kError;

	#[inline]
	fn from_str(src: &str) -> Result<Self, Self::Err> { Self::try_from(src) }
}

impl Ord for Utc2k {
	/// # Compare.
	///
	/// Compare two date/times.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// let date1 = Utc2k::new(2020, 10, 15, 20, 25, 30);
	/// let date2 = Utc2k::new(2020, 10, 15, 0, 0, 0);
	/// let date3 = Utc2k::new(2022, 10, 15, 0, 0, 0);
	///
	/// assert!(date1 > date2);
	/// assert!(date1 < date3);
	/// ```
	fn cmp(&self, other: &Self) -> Ordering {
		let other = *other;
		match self.cmp_date(other) {
			Ordering::Equal => self.cmp_time(other),
			cmp => cmp,
		}
	}
}

impl PartialOrd for Utc2k {
	#[inline]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Sub<u32> for Utc2k {
	type Output = Self;

	#[inline]
	/// # Subtraction.
	///
	/// This method returns a new `Utc2k` object reduced by a given number of
	/// seconds.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// assert_eq!(
	///     Utc2k::new(2020, 1, 5, 0, 0, 0) - 86_400_u32,
	///     Utc2k::new(2020, 1, 4, 0, 0, 0),
	/// );
	///
	/// assert_eq!(
	///     Utc2k::new(2020, 1, 5, 0, 0, 0) - 86_399_u32,
	///     Utc2k::new(2020, 1, 4, 0, 0, 1),
	/// );
	///
	/// assert_eq!(
	///     Utc2k::new(2020, 1, 5, 3, 10, 20) - 14_400_u32,
	///     Utc2k::new(2020, 1, 4, 23, 10, 20),
	/// );
	///
	/// assert_eq!(
	///     Utc2k::new(2020, 1, 1, 3, 10, 20) - 14_400_u32,
	///     Utc2k::new(2019, 12, 31, 23, 10, 20),
	/// );
	/// ```
	fn sub(self, other: u32) -> Self { self.minus_seconds(other) }
}

impl SubAssign<u32> for Utc2k {
	#[inline]
	fn sub_assign(&mut self, other: u32) { *self = *self - other; }
}

/// # Helper: `TryFrom` Unixtime For Non-u32 Formats.
macro_rules! try_from_unixtime {
	($($ty:ty)+) => ($(
		impl TryFrom<$ty> for Utc2k {
			type Error = Utc2kError;
			#[inline]
			fn try_from(src: $ty) -> Result<Self, Self::Error> {
				u32::try_from(src)
					.map(Self::from)
					.map_err(|_| Utc2kError::Invalid)
			}
		}
	)+);
}

try_from_unixtime! { u64 usize i32 i64 isize }

impl TryFrom<&OsStr> for Utc2k {
	type Error = Utc2kError;

	#[inline]
	/// # From `OsStr`.
	///
	/// ```
	/// use std::ffi::OsStr;
	/// use utc2k::Utc2k;
	///
	/// assert_eq!(
	///     Utc2k::try_from(OsStr::new("2013-12-15 21:30:02")).unwrap().to_string(),
	///     "2013-12-15 21:30:02"
	/// );
	/// assert_eq!(
	///     Utc2k::try_from(OsStr::new("2013-12-15")).unwrap().to_string(),
	///     "2013-12-15 00:00:00"
	/// );
	/// ```
	fn try_from(src: &OsStr) -> Result<Self, Self::Error> {
		let src: &str = src.to_str().ok_or(Utc2kError::Invalid)?;
		Self::try_from(src)
	}
}

impl TryFrom<&[u8]> for Utc2k {
	type Error = Utc2kError;

	#[inline]
	fn try_from(src: &[u8]) -> Result<Self, Self::Error> {
		Abacus::from_ascii(src)
			.map(Self::from_abacus)
			.ok_or(Utc2kError::Invalid)
	}
}

impl TryFrom<&str> for Utc2k {
	type Error = Utc2kError;

	#[inline]
	fn try_from(src: &str) -> Result<Self, Self::Error> {
		Self::try_from(src.as_bytes())
	}
}

impl From<Utc2k> for u32 {
	#[inline]
	fn from(src: Utc2k) -> Self { src.unixtime() }
}

/// ## Min/Max.
impl Utc2k {
	/// # Minimum Date/Time.
	///
	/// ```
	/// assert_eq!(
	///     utc2k::Utc2k::MIN.to_string(),
	///     "2000-01-01 00:00:00",
	/// );
	/// ```
	pub const MIN: Self = Self { y: 0, m: 1, d: 1, hh: 0, mm: 0, ss: 0 };

	/// # Maximum Date/Time.
	///
	/// ```
	/// assert_eq!(
	///     utc2k::Utc2k::MAX.to_string(),
	///     "2099-12-31 23:59:59",
	/// );
	/// ```
	pub const MAX: Self = Self { y: 99, m: 12, d: 31, hh: 23, mm: 59, ss: 59 };

	/// # Minimum Unix Timestamp.
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// assert_eq!(Utc2k::MIN_UNIXTIME, 946_684_800);
	/// assert_eq!(
	///     Utc2k::MIN.unixtime(),
	///     Utc2k::MIN_UNIXTIME,
	/// );
	/// ```
	pub const MIN_UNIXTIME: u32 = 946_684_800;

	/// # Maximum Unix Timestamp.
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// assert_eq!(Utc2k::MAX_UNIXTIME, 4_102_444_799);
	/// assert_eq!(
	///     Utc2k::MAX.unixtime(),
	///     Utc2k::MAX_UNIXTIME,
	/// );
	/// ```
	pub const MAX_UNIXTIME: u32 = 4_102_444_799;
}

/// ## Instantiation.
impl Utc2k {
	#[inline]
	#[must_use]
	/// # New (From Parts).
	///
	/// This will create a new instance from individual year, month, etc.,
	/// parts.
	///
	/// Overflowing units will be carried over where appropriate, so for
	/// example, 13 months becomes 1 year and 1 month.
	///
	/// Dates prior to 2000 or after 2099 will be saturated to fit.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// let date = Utc2k::new(2010, 5, 5, 16, 30, 1);
	/// assert_eq!(date.to_string(), "2010-05-05 16:30:01");
	/// ```
	pub const fn new(y: u16, m: u8, d: u8, hh: u8, mm: u8, ss: u8) -> Self {
		Self::from_abacus(Abacus::new(y, m, d, hh, mm, ss))
	}

	#[must_use]
	/// # From ASCII Date/Time Slice.
	///
	/// Try to parse a date/time value from an ASCII slice, returning a
	/// [`Utc2k`] instance if successful, `None` if not.
	///
	/// Note that this method will automatically clamp dates outside the
	/// supported `2000..=2099` range to [`Utc2k::MIN`]/[`Utc2k::MAX`].
	///
	/// If you'd rather out-of-range values "fail" instead, use
	/// [`Utc2k::checked_from_ascii`].
	///
	/// ## Supported Formats.
	///
	/// This method can be used to parse dates and datetimes — but not times
	/// by themselves — from formats that A) order the components biggest to
	/// smallest; and B) use four digits to express the year, and two digits
	/// for everything else.
	///
	/// Digits can either be squished together like `YYYYMMDD` or
	/// `YYYYMMDDhhmmss`, or separated by single non-digit bytes, like
	/// `YYYY/MM/DD` or `YYYY-MM-DD hh:mm:ss`.
	///
	/// (Times can technically end `…ss.ffff`, but [`Utc2k`] doesn't support
	/// fractional seconds; they're ignored if present.)
	///
	/// Complete datetimes can optionally end with "Z", " UT", " UTC", or
	/// " GMT" — all of which are ignored — or a fixed UTC offset of the
	/// `±hhmm` variety which, if present, will be parsed and factored into
	/// the result.
	///
	/// Parsing will fail for sources containing any _other_ random trailing
	/// data, including things like "CST"-style time zone abbreviations.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// // Separators are flexible.
	/// let dates: [&[u8]; 5] = [
	///     b"20250615",   // Squished.
	///     b"2025 06 15", // Spaced.
	///     b"2025/06/15", // Slashed.
	///     b"2025-06-15", // Dashed.
	///     b"2025#06#15", // Hashed? Haha.
	/// ];
	/// for raw in dates {
	///     assert_eq!(
	///         Utc2k::from_ascii(raw).unwrap().parts(),
	///         (2025, 6, 15, 0, 0, 0),
	/// //                    ^  ^  ^ Time defaults to midnight.
	///     );
	/// }
	///
	/// // Same for datetimes.
	/// let datetimes: [&[u8]; 10] = [
	///     b"20250615123001",
	///     b"2025-06-15 12:30:01",
	///     b"2025-06-15T12:30:01Z",
	///     b"2025/06/15:12:30:01 GMT",
	///     b"2025/06/15:12:30:01GMT", // Space-insensitive.
	///     b"2025/06/15:12:30:01 UT",
	///     b"2025/06/15:12:30:01 UTC",
	///     b"2025/06/15:12:30:01 utc", // Case-insensitive.
	///     b"2025/06/15 12:30:01.000 +0000",
	///     b"2025/06/15 12:30:01+0000",
	/// ];
	/// for raw in datetimes {
	///     assert_eq!(
	///         Utc2k::from_ascii(raw).unwrap().parts(),
	///         (2025, 6, 15, 12, 30, 1),
	///     );
	/// }
	///
	/// // Not everything will work out…
	/// let bad: [&[u8]; 9] = [
	///     b"2025-06-15 123001", // Formats cannot mix-and-match
	///     b"2025-06-15123001",  // squished/separated.
	///     b"20250615 12:30:01",
	///     b"2025061512:30:01",
	///     b"2025-01-01Z",       // Date-only strings cannot contain
	///     b"2025-01-01 +0000",  // tz/offset details.
	///     b"2025-01-01 UTC",
	///     b"2025-01-01 00:00:00 PDT", // Only UTC-related identifiers are
	///     b"2025-01-01 00:00:00 EST", // supported.
	/// ];
	/// for raw in bad {
	///     assert!(Utc2k::from_ascii(raw).is_none());
	/// }
	///
	/// // UTC offsets will get factored in accordingly.
	/// assert_eq!(
	///     Utc2k::from_ascii(b"2025-06-15 12:30:01 +0330")
	///         .unwrap()
	///         .parts(),
	///     (2025, 6, 15, 9, 0, 1),
	/// );
	/// assert_eq!(
	///     Utc2k::from_ascii(b"2025-06-15T12:30:01.54321-0700")
	///         .unwrap() //                       ^----^ Ignored.
	///         .parts(),
	///     (2025, 6, 15, 19, 30, 1),
	/// );
	///
	/// // The input doesn't have to follow calendar/clock grouping
	/// // conventions, but the output always will.
	/// assert_eq!(
	///     Utc2k::from_ascii(b"2000-13-10 24:60:61")
	///         .unwrap() //         ^     ^  ^  ^ Logical "overflow".
	///         .parts(),
	///     (2001, 1, 11, 1, 1, 1),
	/// //   ^     ^  ^   ^  ^  ^ Realigned.
	/// );
	/// assert_eq!(
	///     Utc2k::from_ascii(b"2050-02-00")
	///         .unwrap() //            ^ Logical "underflow".
	///         .parts(),
	///     (2050, 1, 31, 0, 0, 0),
	/// //         ^  ^ Realigned.
	/// );
	///
	/// // Returned values are clamped to the `2000..=2099` range.
	/// assert_eq!(
	///     Utc2k::from_ascii(b"1994 04 08"),
	///     Some(Utc2k::MIN), // 2000-01-01 00:00:00
	/// );
	/// assert_eq!(
	///     Utc2k::from_ascii(b"3000/01/01"),
	///     Some(Utc2k::MAX), // 2099-12-31 23:59:59
	/// );
	/// ```
	pub const fn from_ascii(src: &[u8]) -> Option<Self> {
		if let Some(parts) = Abacus::from_ascii(src) {
			Some(Self::from_abacus(parts))
		}
		else { None }
	}

	#[must_use]
	/// # From [RFC2822](https://datatracker.ietf.org/doc/html/rfc2822) Date/Time Slice.
	///
	/// Try to parse a date/time value from a [RFC2822](https://datatracker.ietf.org/doc/html/rfc2822)-formatted
	/// byte slice, returning a [`Utc2k`] instance if successful, `None` if
	/// not.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// // This spec tolerates a lot of variation…
	/// let dates: [&[u8]; 7] = [
	///     b"Tue, 1 Jul 2003 10:52:37 +0000",  // Single-digit day.
	///     b"Tue,  1 Jul 2003 10:52:37 +0000", // Digit/space substitution.
	///     b"Tue, 01 Jul 2003 10:52:37 +0000", // Leading zero.
	///     b"1 Jul 2003 10:52:37",             // No weekday or offset.
	///     b"01 Jul 2003 10:52:37",            // Same, but w/ leading zero.
	///     b"Tue, 01 Jul 2003 03:52:37 -0700", // Negative UTC offset.
	///     b"Tue, 1 Jul 2003 15:22:37 +0430",  // Positive UTC offset.
	/// ];
	///
	/// for raw in dates {
	///     assert_eq!(
	///         Utc2k::from_rfc2822(raw).unwrap().parts(),
	///         (2003, 7, 1, 10, 52, 37),
	///     );
	/// }
	/// #
	/// # // For the offset sign, and _only_ the offset sign, this method
	/// # // will accept a Unicode "Minus Sign" as being the same as an
	/// # // regular ASCII `'-'`.
	/// # assert_eq!(
	/// #     Utc2k::from_rfc2822(
	/// #         "Tue, 01 Jul 2003 03:52:37\u{2212}0700".as_bytes(),
	/// #     ).unwrap().parts(),
	/// #         (2003, 7, 1, 10, 52, 37),
	/// # );
	///
	/// // The same variation exists for date-only representations too.
	/// let dates: [&[u8]; 5] = [
	///     b"Tue, 1 Jul 2003",  // Single-digit day.
	///     b"Tue,  1 Jul 2003", // Digit/space substitution.
	///     b"Tue, 01 Jul 2003", // Leading zero.
	///     b"1 Jul 2003",       // No weekday or offset.
	///     b"01 Jul 2003",      // Same, but w/ leading zero.
	/// ];
	///
	/// for raw in dates {
	///     assert_eq!(
	///         Utc2k::from_rfc2822(raw).unwrap().parts(),
	///         (2003, 7, 1, 0, 0, 0), // Default time is midnight.
	///     );
	/// }
	/// ```
	pub const fn from_rfc2822(src: &[u8]) -> Option<Self> {
		if let Some(parts) = Abacus::from_rfc2822(src) {
			Some(Self::from_abacus(parts))
		}
		else { None }
	}

	#[must_use]
	/// # From Timestamp.
	///
	/// Initialize a new [`Utc2k`] from a unix timestamp, saturating to
	/// [`Utc2k::MIN_UNIXTIME`] or [`Utc2k::MAX_UNIXTIME`] if out of range.
	///
	/// For a non-saturating alternative, see [`Utc2k::checked_from_unixtime`].
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// assert_eq!(
	///     Utc2k::from_unixtime(1_748_672_925).to_string(),
	///     "2025-05-31 06:28:45",
	/// );
	///
	/// // Same as the above, but using the `From<u32>` impl.
	/// assert_eq!(
	///     Utc2k::from(1_748_672_925_u32).to_string(),
	///     "2025-05-31 06:28:45",
	/// );
	///
	/// // Out of range values will saturate to the boundaries of the
	/// // century.
	/// assert_eq!(
	///     Utc2k::from_unixtime(0).to_string(),
	///     "2000-01-01 00:00:00",
	/// );
	/// assert_eq!(
	///     Utc2k::from_unixtime(u32::MAX).to_string(),
	///     "2099-12-31 23:59:59",
	/// );
	/// ```
	pub const fn from_unixtime(src: u32) -> Self {
		if src <= Self::MIN_UNIXTIME { Self::MIN }
		else if src >= Self::MAX_UNIXTIME { Self::MAX }
		else {
			// Tease out the date parts with a lot of terrible math.
			let (y, m, d) = crate::date_seconds(src.wrapping_div(DAY_IN_SECONDS));
			let (hh, mm, ss) = crate::time_seconds(src % DAY_IN_SECONDS);

			Self { y, m, d, hh, mm, ss }
		}
	}

	#[inline]
	#[must_use]
	/// # Now.
	///
	/// Create a new instance representing the current UTC time.
	pub fn now() -> Self { Self::from_unixtime(unixtime()) }

	#[inline]
	#[must_use]
	/// # Tomorrow.
	///
	/// Create a new instance representing one day from now (present time).
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// assert_eq!(
	///     Utc2k::tomorrow(),
	///     Utc2k::now() + 86_400_u32,
	/// );
	/// ```
	pub fn tomorrow() -> Self { Self::from_unixtime(unixtime() + DAY_IN_SECONDS) }

	#[inline]
	#[must_use]
	/// # Yesterday.
	///
	/// Create a new instance representing one day ago (present time).
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// assert_eq!(
	///     Utc2k::yesterday(),
	///     Utc2k::now() - 86_400_u32,
	/// );
	/// ```
	pub fn yesterday() -> Self { Self::from_unixtime(unixtime() - DAY_IN_SECONDS) }
}

/// ## Get Parts.
impl Utc2k {
	#[inline]
	#[must_use]
	/// # Parts.
	///
	/// Return the individual numerical components of the datetime, from years
	/// down to seconds.
	///
	/// Alternatively, if you only want the date bits, use [`Utc2k::ymd`], or
	/// if you only want the time bits, use [`Utc2k::hms`].
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// let date = Utc2k::new(2010, 5, 4, 16, 30, 1);
	/// assert_eq!(date.parts(), (2010, 5, 4, 16, 30, 1));
	/// ```
	pub const fn parts(self) -> (u16, u8, u8, u8, u8, u8) {
		(
			self.year(),
			self.m,
			self.d,
			self.hh,
			self.mm,
			self.ss,
		)
	}

	#[inline]
	#[must_use]
	/// # Date Parts.
	///
	/// Return the year, month, and day.
	///
	/// If you want the time too, call [`Utc2k::parts`] instead.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// let date = Utc2k::new(2010, 5, 5, 16, 30, 1);
	/// assert_eq!(date.ymd(), (2010, 5, 5));
	/// ```
	pub const fn ymd(self) -> (u16, u8, u8) { (self.year(), self.m, self.d) }

	#[inline]
	#[must_use]
	/// # Time Parts.
	///
	/// Return the hours, minutes, and seconds.
	///
	/// If you want the date too, call [`Utc2k::parts`] instead.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// let date = Utc2k::new(2010, 5, 5, 16, 30, 1);
	/// assert_eq!(date.hms(), (16, 30, 1));
	/// ```
	pub const fn hms(self) -> (u8, u8, u8) { (self.hh, self.mm, self.ss) }

	#[inline]
	#[must_use]
	/// # Year.
	///
	/// This returns the year value.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// let date = Utc2k::new(2010, 5, 15, 16, 30, 1);
	/// assert_eq!(date.year(), 2010);
	/// ```
	pub const fn year(self) -> u16 { self.y as u16 + 2000 }

	#[inline]
	#[must_use]
	/// # Month.
	///
	/// This returns the month value.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// let date = Utc2k::new(2010, 5, 15, 16, 30, 1);
	/// assert_eq!(date.month(), 5);
	/// ```
	pub const fn month(self) -> u8 { self.m }

	#[inline]
	#[must_use]
	/// # Month (enum).
	///
	/// This returns the month value as a [`Month`].
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::{Month, Utc2k};
	///
	/// let date = Utc2k::new(2010, 5, 15, 16, 30, 1);
	/// assert_eq!(date.month_enum(), Month::May);
	/// ```
	pub const fn month_enum(self) -> Month { Month::from_u8(self.m) }

	#[inline]
	#[must_use]
	/// # Day.
	///
	/// This returns the day value.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// let date = Utc2k::new(2010, 5, 15, 16, 30, 1);
	/// assert_eq!(date.day(), 15);
	/// ```
	pub const fn day(self) -> u8 { self.d }

	#[inline]
	#[must_use]
	/// # Hour.
	///
	/// This returns the hour value.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// let date = Utc2k::new(2010, 5, 15, 16, 30, 1);
	/// assert_eq!(date.hour(), 16);
	/// ```
	pub const fn hour(self) -> u8 { self.hh }

	#[inline]
	#[must_use]
	/// # Minute.
	///
	/// This returns the minute value.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// let date = Utc2k::new(2010, 5, 15, 16, 30, 1);
	/// assert_eq!(date.minute(), 30);
	/// ```
	pub const fn minute(self) -> u8 { self.mm }

	#[inline]
	#[must_use]
	/// # Second.
	///
	/// This returns the second value.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// let date = Utc2k::new(2010, 5, 15, 16, 30, 1);
	/// assert_eq!(date.second(), 1);
	/// ```
	pub const fn second(self) -> u8 { self.ss }
}

/// ## Other Getters.
impl Utc2k {
	#[must_use]
	/// # Is Leap Year?
	///
	/// This returns `true` if this date is/was in a leap year.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// let date = Utc2k::try_from("2020-05-10 00:00:00").unwrap();
	/// assert!(date.leap_year());
	///
	/// let date = Utc2k::try_from("2021-03-15 00:00:00").unwrap();
	/// assert!(! date.leap_year());
	/// ```
	pub const fn leap_year(self) -> bool {
		/// # This Century's Leap Years.
		const LEAP_YEARS: [bool; 100] = [true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false];
		LEAP_YEARS[self.y as usize]
	}

	#[inline]
	#[must_use]
	/// # Abbreviated Month Name.
	///
	/// Return the abbreviated name of the month, nice and pretty.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// let date = Utc2k::try_from("2020-06-24 20:19:30").unwrap();
	/// assert_eq!(date.month_abbreviation(), "Jun");
	/// ```
	pub const fn month_abbreviation(self) -> &'static str {
		self.month_enum().abbreviation()
	}

	#[inline]
	#[must_use]
	/// # Month Name.
	///
	/// Return the name of the month, nice and pretty.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// let date = Utc2k::try_from("2020-06-24 20:19:30").unwrap();
	/// assert_eq!(date.month_name(), "June");
	/// ```
	pub const fn month_name(self) -> &'static str {
		self.month_enum().as_str()
	}

	#[must_use]
	/// # Month Size (Days).
	///
	/// This method returns the "size" of the datetime's month, or its last
	/// day, whichever way you prefer to think of it.
	///
	/// The value will always be between `28..=31`, with leap Februaries
	/// returning `29`.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// let date = Utc2k::try_from("2021-07-08 13:22:01").unwrap();
	/// assert_eq!(date.month_size(), 31);
	///
	/// let date = Utc2k::try_from("2020-02-01").unwrap();
	/// assert_eq!(date.month_size(), 29); // Leap!
	/// ```
	pub const fn month_size(self) -> u8 {
		if self.m == 2 && self.leap_year() { 29 }
		else { self.month_enum().days() }
	}

	#[must_use]
	/// # Ordinal.
	///
	/// Return the day-of-year value. This will be between `1..=365` (or `1..=366`
	/// for leap years).
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// let date = Utc2k::try_from("2020-05-10 00:00:00").unwrap();
	/// assert_eq!(date.ordinal(), 131);
	///
	/// let date = Utc2k::try_from("2021-01-15 00:00:00").unwrap();
	/// assert_eq!(date.ordinal(), 15);
	/// ```
	pub const fn ordinal(self) -> u16 {
		let days = self.d as u16 +
			match self.m {
				2 => 31,
				3 => 59,
				4 => 90,
				5 => 120,
				6 => 151,
				7 => 181,
				8 => 212,
				9 => 243,
				10 => 273,
				11 => 304,
				12 => 334,
				_ => 0,
			};

		if 2 < self.m && self.leap_year() { days + 1 }
		else { days }
	}

	#[inline]
	#[must_use]
	/// # Seconds From Midnight.
	///
	/// Return the number of seconds since (the current day's) midnight. In
	/// other words, this adds up all of the time bits.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// let date = Utc2k::new(2010, 11, 30, 0, 0, 0);
	/// assert_eq!(date.seconds_from_midnight(), 0);
	///
	/// let date = Utc2k::new(2010, 11, 30, 0, 0, 30);
	/// assert_eq!(date.seconds_from_midnight(), 30);
	///
	/// let date = Utc2k::new(2010, 11, 30, 0, 1, 30);
	/// assert_eq!(date.seconds_from_midnight(), 90);
	///
	/// let date = Utc2k::new(2010, 11, 30, 12, 30, 10);
	/// assert_eq!(date.seconds_from_midnight(), 45_010);
	/// ```
	pub const fn seconds_from_midnight(self) -> u32 {
		self.ss as u32 +
		self.mm as u32 * MINUTE_IN_SECONDS +
		self.hh as u32 * HOUR_IN_SECONDS
	}

	#[must_use]
	/// # Weekday.
	///
	/// Return the [`Weekday`] corresponding to the given date.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::{Utc2k, Weekday};
	///
	/// let date = Utc2k::try_from("2021-07-08 13:22:01").unwrap();
	/// assert_eq!(date.weekday(), Weekday::Thursday);
	/// assert_eq!(date.weekday().as_ref(), "Thursday");
	/// ```
	pub const fn weekday(self) -> Weekday {
		Weekday::from_u8(
			Weekday::year_begins_on(self.y) as u8 +
			((self.ordinal() - 1) % 7) as u8
		)
	}
}

/// ## Conversion.
impl Utc2k {
	#[inline]
	#[must_use]
	/// # Formatted.
	///
	/// This returns a [`FmtUtc2k`] and is equivalent to calling
	/// `FmtUtc2k::from(self)`.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::{FmtUtc2k, Utc2k};
	///
	/// let date = Utc2k::new(2010, 5, 15, 16, 30, 1);
	/// assert_eq!(date.formatted(), FmtUtc2k::from(date));
	/// ```
	pub const fn formatted(self) -> FmtUtc2k { FmtUtc2k::from_utc2k(self) }

	#[must_use]
	/// # To Midnight.
	///
	/// Return a new instance with zeroed-out time pieces, i.e. truncated to
	/// the date's midnight.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// let date1 = Utc2k::new(2022, 7, 22, 20, 52, 41);
	/// assert_eq!(date1.to_midnight(), date1.with_time(0, 0, 0));
	/// ```
	pub const fn to_midnight(self) -> Self {
		Self {
			y: self.y,
			m: self.m,
			d: self.d,
			hh: 0,
			mm: 0,
			ss: 0,
		}
	}

	#[must_use]
	/// # To RFC2822.
	///
	/// Return a string formatted according to [RFC2822](https://datatracker.ietf.org/doc/html/rfc2822).
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// let date = Utc2k::new(2003, 7, 1, 10, 52, 37);
	/// assert_eq!(
	///     date.to_rfc2822(),
	///     "Tue, 01 Jul 2003 10:52:37 +0000",
	/// //        ^ This implementation zero-pads short day
	/// //          numbers rather than truncating them…
	/// );
	///
	/// let date = Utc2k::new(2036, 12, 15, 16, 30, 55);
	/// assert_eq!(
	///     date.to_rfc2822(),
	///     "Mon, 15 Dec 2036 16:30:55 +0000",
	/// //   ^-----------------------------^ …to keep the output
	/// //                                   length consistent.
	/// );
	/// ```
	pub fn to_rfc2822(&self) -> String {
		let mut out = String::with_capacity(31);

		out.push_str(self.weekday().abbreviation());
		out.push_str(", ");
		out.push_str(DateChar::as_str(DateChar::dd(self.d).as_slice()));
		out.push(' ');
		out.push_str(self.month_enum().abbreviation());
		out.push_str(" 20");
		out.push_str(DateChar::as_str(DateChar::dd(self.y).as_slice()));
		out.push(' ');
		out.push_str(DateChar::as_str(DateChar::dd(self.hh).as_slice()));
		out.push(':');
		out.push_str(DateChar::as_str(DateChar::dd(self.mm).as_slice()));
		out.push(':');
		out.push_str(DateChar::as_str(DateChar::dd(self.ss).as_slice()));
		out.push_str(" +0000");

		out
	}

	#[inline]
	#[must_use]
	/// # To RFC3339.
	///
	/// Return a string formatted according to [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339).
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// let date = Utc2k::new(2021, 12, 13, 11, 56, 1);
	/// assert_eq!(date.to_rfc3339(), "2021-12-13T11:56:01Z");
	///
	/// // The reverse operation — parsing an RFC3339 datetime string into
	/// // a Utc2k — can be done using `Utc2k::from_ascii`.
	/// assert_eq!(
	///     Utc2k::from_ascii(date.to_rfc3339().as_bytes()).unwrap(),
	///     date,
	/// );
	/// ```
	pub fn to_rfc3339(&self) -> String { FmtUtc2k::from_utc2k(*self).to_rfc3339() }

	#[must_use]
	/// # Unix Timestamp.
	///
	/// Return the unix timestamp for this object.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// let date = Utc2k::default(); // 2000-01-01 00:00:00
	/// assert_eq!(date.unixtime(), Utc2k::MIN_UNIXTIME);
	/// ```
	pub const fn unixtime(self) -> u32 {
		/// # Seconds from the new year up to the start of the month.
		const MONTH_SECONDS: [u32; 12] = [0, 2_678_400, 5_097_600, 7_776_000, 10_368_000, 13_046_400, 15_638_400, 18_316_800, 20_995_200, 23_587_200, 26_265_600, 28_857_600];

		/// # Seconds *before* the new year.
		const YEAR_SECONDS: [u32; 100] = [946_684_800, 978_307_200, 1_009_843_200, 1_041_379_200, 1_072_915_200, 1_104_537_600, 1_136_073_600, 1_167_609_600, 1_199_145_600, 1_230_768_000, 1_262_304_000, 1_293_840_000, 1_325_376_000, 1_356_998_400, 1_388_534_400, 1_420_070_400, 1_451_606_400, 1_483_228_800, 1_514_764_800, 1_546_300_800, 1_577_836_800, 1_609_459_200, 1_640_995_200, 1_672_531_200, 1_704_067_200, 1_735_689_600, 1_767_225_600, 1_798_761_600, 1_830_297_600, 1_861_920_000, 1_893_456_000, 1_924_992_000, 1_956_528_000, 1_988_150_400, 2_019_686_400, 2_051_222_400, 2_082_758_400, 2_114_380_800, 2_145_916_800, 2_177_452_800, 2_208_988_800, 2_240_611_200, 2_272_147_200, 2_303_683_200, 2_335_219_200, 2_366_841_600, 2_398_377_600, 2_429_913_600, 2_461_449_600, 2_493_072_000, 2_524_608_000, 2_556_144_000, 2_587_680_000, 2_619_302_400, 2_650_838_400, 2_682_374_400, 2_713_910_400, 2_745_532_800, 2_777_068_800, 2_808_604_800, 2_840_140_800, 2_871_763_200, 2_903_299_200, 2_934_835_200, 2_966_371_200, 2_997_993_600, 3_029_529_600, 3_061_065_600, 3_092_601_600, 3_124_224_000, 3_155_760_000, 3_187_296_000, 3_218_832_000, 3_250_454_400, 3_281_990_400, 3_313_526_400, 3_345_062_400, 3_376_684_800, 3_408_220_800, 3_439_756_800, 3_471_292_800, 3_502_915_200, 3_534_451_200, 3_565_987_200, 3_597_523_200, 3_629_145_600, 3_660_681_600, 3_692_217_600, 3_723_753_600, 3_755_376_000, 3_786_912_000, 3_818_448_000, 3_849_984_000, 3_881_606_400, 3_913_142_400, 3_944_678_400, 3_976_214_400, 4_007_836_800, 4_039_372_800, 4_070_908_800];

		// Add up everything as it would be in a non-leap year.
		let time = YEAR_SECONDS[self.y as usize] +
			MONTH_SECONDS[self.m as usize - 1] +
			self.seconds_from_midnight() +
			DAY_IN_SECONDS * (self.d as u32 - 1);

		// Add a day's worth of seconds if we need to.
		if 2 < self.m && self.leap_year() { time + DAY_IN_SECONDS }
		else { time }
	}

	#[must_use]
	/// # With a New Time.
	///
	/// Return a new [`Utc2k`] instance with the original date — unless there
	/// is carry-over needed — and a new time.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// let date = Utc2k::default();
	/// assert_eq!(date.to_string(), "2000-01-01 00:00:00");
	///
	/// // Change the time bits.
	/// assert_eq!(date.with_time(13, 14, 15).to_string(), "2000-01-01 13:14:15");
	/// ```
	pub const fn with_time(self, hh: u8, mm: u8, ss: u8) -> Self {
		Self::from_abacus(Abacus::new(self.year(), self.month(), self.day(), hh, mm, ss))
	}
}

/// ## Checked Operations.
impl Utc2k {
	#[must_use]
	/// # Checked Add.
	///
	/// Return a new [`Utc2k`] instance set _n_ seconds into the future from
	/// this one, returning `none` (rather than saturating) on overflow.
	///
	/// If you'd rather saturate addition, you can just use [`std::ops::Add`].
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// let date = Utc2k::MAX;
	/// assert!(date.checked_add(1).is_none());
	///
	/// let date = Utc2k::new(2010, 1, 1, 0, 0, 0);
	/// let added = date.checked_add(86_413).unwrap();
	/// assert_eq!(added.to_string(), "2010-01-02 00:00:13");
	/// ```
	pub const fn checked_add(self, secs: u32) -> Option<Self> {
		if let Some(s) = self.unixtime().checked_add(secs) {
			if s <= Self::MAX_UNIXTIME {
				return Some(Self::from_unixtime(s));
			}
		}

		None
	}

	/// # From ASCII Date/Time Slice (Checked).
	///
	/// Same as [`Utc2k::from_ascii`], but will return an error if the
	/// resulting date is too old or new to be represented faithfully
	/// (rather than clamping it to [`Utc2k::MIN`]/[`Utc2k::MAX`]).
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::{Utc2k, Utc2kError};
	///
	/// assert_eq!(
	///     Utc2k::checked_from_ascii(b"1990-01-15 10:20:00"),
	///     Err(Utc2kError::Underflow), // Too old.
	/// );
	///
	/// assert_eq!(
	///     Utc2k::checked_from_ascii(b"3000-12-01 13:00:00"),
	///     Err(Utc2kError::Overflow), // Too new.
	/// );
	///
	/// assert_eq!(
	///     Utc2k::checked_from_ascii(b"2025-06-17 00:00:00")
	///         .map(Utc2k::parts),
	///     Ok((2025, 6, 17, 0, 0, 0)), // Just right!
	/// );
	/// ```
	///
	/// ## Errors
	///
	/// This method will return an error if the slice cannot be parsed, or the
	/// parsed value is too big or small to fit within our century.
	pub const fn checked_from_ascii(src: &[u8]) -> Result<Self, Utc2kError> {
		if let Some(parts) = Abacus::from_ascii(src) {
			match parts.parts_checked() {
				Ok((y, m, d, hh, mm, ss)) => Ok(Self { y, m, d, hh, mm, ss }),
				Err(e) => Err(e),
			}
		}
		else { Err(Utc2kError::Invalid) }
	}

	/// # From Unixtime (Checked).
	///
	/// This can be used instead of the usual [`Utc2k::from_unixtime`] or
	/// `From<u32>` if you'd like to trigger an error when the timestamp is out
	/// of range (rather than just saturating it).
	///
	/// ## Errors
	///
	/// An error will be returned if the timestamp is less than [`Utc2k::MIN_UNIXTIME`]
	/// or greater than [`Utc2k::MAX_UNIXTIME`].
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::{Utc2k, Utc2kError};
	///
	/// assert_eq!(
	///     Utc2k::checked_from_unixtime(u32::MIN),
	///     Err(Utc2kError::Underflow), // Too old.
	/// );
	///
	/// assert_eq!(
	///     Utc2k::checked_from_unixtime(u32::MAX),
	///     Err(Utc2kError::Overflow), // Too new.
	/// );
	///
	/// assert_eq!(
	///     Utc2k::checked_from_unixtime(1_750_187_543)
	///         .map(Utc2k::parts),
	///     Ok((2025, 6, 17, 19, 12, 23)), // Just right!
	/// );
	/// ```
	pub const fn checked_from_unixtime(src: u32) -> Result<Self, Utc2kError> {
		if src < Self::MIN_UNIXTIME { Err(Utc2kError::Underflow) }
		else if src > Self::MAX_UNIXTIME { Err(Utc2kError::Overflow) }
		else { Ok(Self::from_unixtime(src)) }
	}

	#[must_use]
	/// # Checked Sub.
	///
	/// Return a new [`Utc2k`] instance set _n_ seconds before this one,
	/// returning `none` (rather than saturating) on overflow.
	///
	/// If you'd rather saturate subtraction, you can just use [`std::ops::Sub`].
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// let date = Utc2k::MIN;
	/// assert!(date.checked_sub(1).is_none());
	///
	/// let date = Utc2k::new(2010, 1, 1, 0, 0, 0);
	/// let subbed = date.checked_sub(86_413).unwrap();
	/// assert_eq!(subbed.to_string(), "2009-12-30 23:59:47");
	/// ```
	pub const fn checked_sub(self, secs: u32) -> Option<Self> {
		if let Some(s) = self.unixtime().checked_sub(secs) {
			if Self::MIN_UNIXTIME <= s {
				return Some(Self::from_unixtime(s));
			}
		}

		None
	}
}

/// # Comparison.
impl Utc2k {
	#[must_use]
	/// # Absolute Difference.
	///
	/// This returns the (absolute) number of seconds between two datetimes.
	///
	/// ## Examples.
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// let date1 = Utc2k::new(2022, 10, 15, 11, 30, 0);
	/// let date2 = Utc2k::new(2022, 10, 15, 11, 31, 0);
	///
	/// // ABS means the ordering does not matter.
	/// assert_eq!(date1.abs_diff(date2), 60);
	/// assert_eq!(date2.abs_diff(date1), 60);
	///
	/// // If the dates are equal, the difference is zero.
	/// assert_eq!(date1.abs_diff(date1), 0);
	///
	/// // Because we're only dealing with a single century, there is an
	/// // upper limit to the possible return values…
	/// assert_eq!(Utc2k::MIN.abs_diff(Utc2k::MAX), 3_155_759_999);
	/// ```
	pub const fn abs_diff(self, other: Self) -> u32 {
		self.unixtime().abs_diff(other.unixtime())
	}

	#[must_use]
	/// # Compare (Only) Dates.
	///
	/// Compare `self` to another `Utc2k` instance, ignoring the time
	/// components of each.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	/// use std::cmp::Ordering;
	///
	/// // The times are different, but the dates match.
	/// let date1 = Utc2k::new(2020, 3, 15, 0, 0, 0);
	/// let date2 = Utc2k::new(2020, 3, 15, 16, 30, 20);
	/// assert_eq!(date1.cmp_date(date2), Ordering::Equal);
	///
	/// // If the dates don't match, it's what you'd expect.
	/// let date3 = Utc2k::new(2022, 10, 31, 0, 0, 0);
	/// assert_eq!(date1.cmp_date(date3), Ordering::Less);
	/// ```
	pub const fn cmp_date(self, other: Self) -> Ordering {
		if self.y == other.y {
			if self.m == other.m {
				if self.d == other.d { Ordering::Equal }
				else if self.d < other.d { Ordering::Less }
				else { Ordering::Greater }
			}
			else if self.m < other.m { Ordering::Less }
			else { Ordering::Greater }
		}
		else if self.y < other.y { Ordering::Less }
		else { Ordering::Greater }
	}

	#[must_use]
	/// # Compare (Only) Times.
	///
	/// Compare `self` to another `Utc2k` instance, ignoring the date
	/// components of each.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	/// use std::cmp::Ordering;
	///
	/// // The dates match, but the times are different.
	/// let date1 = Utc2k::new(2020, 3, 15, 0, 0, 0);
	/// let date2 = Utc2k::new(2020, 3, 15, 16, 30, 20);
	/// assert_eq!(date1.cmp_time(date2), Ordering::Less);
	///
	/// // If the times match, it's what you'd expect.
	/// let date3 = Utc2k::new(2022, 10, 31, 0, 0, 0);
	/// assert_eq!(date1.cmp_time(date3), Ordering::Equal);
	/// ```
	pub const fn cmp_time(self, other: Self) -> Ordering {
		if self.hh == other.hh {
			if self.mm == other.mm {
				if self.ss == other.ss { Ordering::Equal }
				else if self.ss < other.ss { Ordering::Less }
				else { Ordering::Greater }
			}
			else if self.mm < other.mm { Ordering::Less }
			else { Ordering::Greater }
		}
		else if self.hh < other.hh { Ordering::Less }
		else { Ordering::Greater }
	}
}

/// # Internal Helpers.
impl Utc2k {
	#[must_use]
	/// # From `Abacus`.
	const fn from_abacus(src: Abacus) -> Self {
		let (y, m, d, hh, mm, ss) = src.parts();
		Self { y, m, d, hh, mm, ss }
	}

	#[must_use]
	/// # From `FmtUtc2k`.
	const fn from_fmtutc2k(src: FmtUtc2k) -> Self {
		Self::new(
			2000 + (src.0[2].as_digit() * 10 + src.0[3].as_digit()) as u16,
			src.0[5].as_digit() * 10 + src.0[6].as_digit(),
			src.0[8].as_digit() * 10 + src.0[9].as_digit(),
			src.0[11].as_digit() * 10 + src.0[12].as_digit(),
			src.0[14].as_digit() * 10 + src.0[15].as_digit(),
			src.0[17].as_digit() * 10 + src.0[18].as_digit(),
		)
	}

	#[expect(clippy::cast_possible_truncation, reason = "False positive.")]
	#[must_use]
	/// # Subtract Seconds.
	const fn minus_seconds(self, offset: u32) -> Self {
		// Count up the "easy" seconds. If we're subtracting less than this
		// amount, we can handle the subtraction without any month boundary or
		// leap year shenanigans.
		let mut easy: u32 =
			(self.d - 1) as u32 * DAY_IN_SECONDS +
			(self.hh) as u32 * HOUR_IN_SECONDS +
			(self.mm) as u32 * MINUTE_IN_SECONDS +
			(self.ss) as u32;

		if offset <= easy {
			easy -= offset;
			let d: u8 =
				if easy >= DAY_IN_SECONDS {
					let d = easy.wrapping_div(DAY_IN_SECONDS);
					easy -= d * DAY_IN_SECONDS;
					d as u8 + 1
				}
				else { 1 };

			let (hh, mm, ss) = crate::time_seconds(easy);
			Self {
				y: self.y,
				m: self.m,
				d,
				hh,
				mm,
				ss,
			}
		}
		// Otherwise it is best to convert to unixtime, perform the
		// subtraction, and convert it back.
		else {
			Self::from_unixtime(self.unixtime().saturating_sub(offset))
		}
	}
}



#[cfg(test)]
mod tests {
	use super::*;
	use time::OffsetDateTime;

	#[cfg(not(miri))]
	const SAMPLE_SIZE: usize = 1_000_000;

	#[cfg(miri)]
	const SAMPLE_SIZE: usize = 1000; // Miri runs way too slow for a million tests.

	/// # Helper: Test a Timestamp Many Ways.
	macro_rules! range_test {
		($i:ident, $buf:ident, $format:ident) => (
			let u = Utc2k::from($i);
			let f = FmtUtc2k::from(u);
			let c = OffsetDateTime::from_unix_timestamp($i as i64)
				.expect("Unable to create time::OffsetDateTime.");

			// Make sure the timestamp comes back the same.
			assert_eq!($i, u.unixtime(), "Timestamp out does not match timestamp in!");

			// Make sure back-and-forth conversions work as expected.
			assert_eq!(
				Utc2k::from(f),
				u,
				"Fmt/Utc back-and-forth failed for {}", $i,
			);

			assert_eq!(
				Some(u),
				Utc2k::from_rfc2822(u.to_rfc2822().as_bytes()),
				"RFC2822 back-and-forth failed for {}", $i,
			);
			assert_eq!(
				Some(u),
				Utc2k::from_ascii(u.to_rfc3339().as_bytes()),
				"RFC3339 back-and-forth failed for {}", $i,
			);

			assert_eq!(
				Some(f),
				FmtUtc2k::from_rfc2822(f.to_rfc2822().as_bytes()),
				"Fmt RFC2822 back-and-forth failed for {}", $i,
			);
			assert_eq!(
				Some(f),
				FmtUtc2k::from_ascii(f.to_rfc3339().as_bytes()),
				"Fmt RFC3339 back-and-forth failed for {}", $i,
			);

			assert_eq!(u.year(), c.year() as u16, "Year mismatch for unixtime {}", $i);
			assert_eq!(u.month(), u8::from(c.month()), "Month mismatch for unixtime {}", $i);
			assert_eq!(u.day(), c.day(), "Day mismatch for unixtime {}", $i);
			assert_eq!(u.hour(), c.hour(), "Hour mismatch for unixtime {}", $i);
			assert_eq!(u.minute(), c.minute(), "Minute mismatch for unixtime {}", $i);
			assert_eq!(u.second(), c.second(), "Second mismatch for unixtime {}", $i);
			assert_eq!(u.ordinal(), c.ordinal(), "Ordinal mismatch for unixtime {}", $i);

			// Make sure the weekdays match.
			assert_eq!(u.weekday().as_ref(), c.weekday().to_string());

			// We already checked the parts so they should match as a whole
			// too, but it shouldn't hurt (too much) to be exhaustive.
			$buf.truncate(0);
			c.format_into(&mut $buf, &$format).expect("Unable to format datetime.");
			assert_eq!(
				Ok(f.as_str()),
				std::str::from_utf8($buf.as_slice()),
				"Date mismatch for unixtime {}", $i,
			);
		);
	}

	#[test]
	/// # Limited Range Unixtime Test.
	///
	/// This performs the same tests as `full_unixtime`, but only covers a
	/// random subset of the possible values — five million of them — to keep
	/// the runtime within the realm of reason.
	///
	/// (Testing every single second takes _forever_, so is disabled by
	/// default.)
	fn limited_unixtime() {
		let format = time::format_description::parse(
			"[year]-[month]-[day] [hour]:[minute]:[second]",
		).expect("Unable to parse datetime format.");

		let mut rng = fastrand::Rng::new();
		let mut buf = Vec::new();
		for i in std::iter::repeat_with(|| rng.u32(Utc2k::MIN_UNIXTIME..=Utc2k::MAX_UNIXTIME)).take(SAMPLE_SIZE).chain(std::iter::once(1_583_037_365)) {
			range_test!(i, buf, format);
		}
	}

	#[test]
	/// # Leap Years.
	fn t_leap_years() {
		for y in 2000..2100 {
			let date = Utc2k::new(y, 1, 1, 0, 0, 0);
			assert_eq!(date.year(), y);
			assert_eq!(
				date.leap_year(),
				y.trailing_zeros() >= 2 && (! y.is_multiple_of(100) || y.is_multiple_of(400))
			);
		}
	}

	#[test]
	/// # Test Min/Max Explicitly.
	fn t_min_max() {
		// Self and Timestamps.
		assert_eq!(Utc2k::MIN, Utc2k::from(Utc2k::MIN_UNIXTIME));
		assert_eq!(Utc2k::MAX, Utc2k::from(Utc2k::MAX_UNIXTIME));
		assert_eq!(Utc2k::MIN.unixtime(), Utc2k::MIN_UNIXTIME);
		assert_eq!(Utc2k::MAX.unixtime(), Utc2k::MAX_UNIXTIME);

		// Utc2k and FmtUtc2k.
		assert_eq!(Utc2k::MIN, Utc2k::from(FmtUtc2k::MIN));
		assert_eq!(Utc2k::MAX, Utc2k::from(FmtUtc2k::MAX));
		assert_eq!(FmtUtc2k::MIN, FmtUtc2k::from(Utc2k::MIN));
		assert_eq!(FmtUtc2k::MAX, FmtUtc2k::from(Utc2k::MAX));
	}

	#[test]
	/// # Test Ordering.
	fn t_ordering() {
		let expected = vec![
			Utc2k::try_from("2000-01-01 00:00:00").unwrap(),
			Utc2k::try_from("2010-05-31 01:02:03").unwrap(),
			Utc2k::try_from("2010-05-31 02:02:03").unwrap(),
			Utc2k::try_from("2020-10-10 10:10:10").unwrap(),
			Utc2k::try_from("2020-10-10 10:11:10").unwrap(),
			Utc2k::try_from("2020-10-10 10:11:11").unwrap(),
		];

		let mut shuffled = vec![
			Utc2k::try_from("2010-05-31 01:02:03").unwrap(),
			Utc2k::try_from("2020-10-10 10:11:11").unwrap(),
			Utc2k::try_from("2010-05-31 02:02:03").unwrap(),
			Utc2k::try_from("2000-01-01 00:00:00").unwrap(),
			Utc2k::try_from("2020-10-10 10:11:10").unwrap(),
			Utc2k::try_from("2020-10-10 10:10:10").unwrap(),
		];

		let f_expected: Vec<FmtUtc2k> = expected.iter().copied().map(FmtUtc2k::from).collect();
		let mut f_shuffled: Vec<FmtUtc2k> = shuffled.iter().copied().map(FmtUtc2k::from).collect();

		// Both sets should be not equal to start.
		assert_ne!(expected, shuffled);
		assert_ne!(f_expected, f_shuffled);

		// Sort 'em.
		shuffled.sort();
		f_shuffled.sort();

		// Now they should match.
		assert_eq!(expected, shuffled);
		assert_eq!(f_expected, f_shuffled);
	}

	#[test]
	/// # Test Manual `cmp_date`.
	fn t_cmp_date() {
		let set = vec![
			Utc2k::new(2024, 1, 1, 0, 0, 0),
			Utc2k::new(2024, 1, 2, 0, 0, 0),
			Utc2k::new(2024, 2, 1, 0, 0, 0),
			Utc2k::new(2024, 2, 2, 0, 0, 0),
			Utc2k::new(2025, 1, 1, 0, 0, 0),
			Utc2k::new(2025, 1, 2, 0, 0, 0),
			Utc2k::new(2025, 2, 1, 0, 0, 0),
			Utc2k::new(2025, 2, 2, 0, 0, 0),
		];

		let mut sorted = set.clone();
		sorted.sort();
		sorted.dedup();
		assert_eq!(set, sorted); // Double-check our manual sorting.

		for pair in set.windows(2) {
			let &[a, b] = pair else { panic!("Windows is broken?!"); };

			// Each should be equal to itself.
			assert!(a.cmp_date(a).is_eq());
			assert!(b.cmp_date(b).is_eq());

			// And times shouldn't matter.
			assert!(a.cmp_date(a.with_time(1, 2, 3)).is_eq());
			assert!(b.cmp_date(b.with_time(3, 2, 1)).is_eq());
			assert!(a.with_time(1, 2, 3).cmp_date(a).is_eq());
			assert!(b.with_time(3, 2, 1).cmp_date(b).is_eq());

			// A < B, B > A.
			assert!(a.cmp_date(b).is_lt());
			assert!(b.cmp_date(a).is_gt());

			// Again, times shouldn't matter.
			assert!(a.cmp_date(b.with_time(5, 6, 7)).is_lt());
			assert!(b.cmp_date(a.with_time(8, 9, 3)).is_gt());
			assert!(a.with_time(5, 6, 7).cmp_date(b).is_lt());
			assert!(b.with_time(8, 9, 3).cmp_date(a).is_gt());
		}
	}

	#[test]
	/// # Test Manual `cmp_time`.
	fn t_cmp_time() {
		let set = vec![
			Utc2k::new(2027, 6, 5, 0, 0, 0),
			Utc2k::new(2027, 6, 5, 0, 0, 1),
			Utc2k::new(2027, 6, 5, 0, 1, 0),
			Utc2k::new(2027, 6, 5, 0, 1, 1),
			Utc2k::new(2027, 6, 5, 1, 0, 0),
			Utc2k::new(2027, 6, 5, 1, 0, 1),
			Utc2k::new(2027, 6, 5, 1, 1, 0),
			Utc2k::new(2027, 6, 5, 1, 1, 1),
		];

		let mut sorted = set.clone();
		sorted.sort();
		sorted.dedup();
		assert_eq!(set, sorted); // Double-check our manual sorting.

		for pair in set.windows(2) {
			let &[a, b] = pair else { panic!("Windows is broken?!"); };

			// Each should be equal to itself.
			assert!(a.cmp_time(a).is_eq());
			assert!(b.cmp_time(b).is_eq());

			// The date shouldn't matter.
			let c = a + crate::YEAR_IN_SECONDS;
			assert!(a.cmp_time(c).is_eq());
			assert!(c.cmp_time(a).is_eq());
			let d = b + crate::YEAR_IN_SECONDS;
			assert!(b.cmp_time(d).is_eq());
			assert!(d.cmp_time(b).is_eq());

			// A < B, B > A.
			assert!(a.cmp_time(b).is_lt());
			assert!(b.cmp_time(a).is_gt());

			// Again, the date shouldn't matter.
			assert!(a.cmp_time(d).is_lt());
			assert!(b.cmp_time(c).is_gt());
			assert!(c.cmp_time(b).is_lt());
			assert!(d.cmp_time(a).is_gt());
		}
	}

	#[cfg(not(debug_assertions))]
	/// # Generate Century Tests.
	///
	/// There are a lot of seconds to test. Spreading them out across multiple
	/// functions can at least open up the possibility of parallelization.
	macro_rules! century_test {
		// A neat counting trick adapted from The Little Book of Rust Macros, used
		// here to figure out the step size.
		(@count $odd:tt) => ( 1 );
	    (@count $odd:tt $($a:tt $b:tt)+) => ( (century_test!(@count $($a)+) * 2) + 1 );
	    (@count $($a:tt $b:tt)+) =>         (  century_test!(@count $($a)+) * 2      );

	    // Generate the tests!
	    (@build $step:expr; $($fn:ident $offset:literal),+) => ($(
			#[test]
			#[ignore = "testing every second takes a long time"]
			fn $fn() {
				let format = time::format_description::parse(
					"[year]-[month]-[day] [hour]:[minute]:[second]",
				).expect("Unable to parse datetime format.");
				let mut buf = Vec::new();

				for i in (Utc2k::MIN_UNIXTIME + $offset..=Utc2k::MAX_UNIXTIME).step_by($step) {
					range_test!(i, buf, format);
				}
			}
		)+);

	    // Entrypoint.
		($($fn:ident $offset:literal),+ $(,)?) => (
			century_test! { @build century_test!(@count $($offset)+); $($fn $offset),+ }
		);
	}

	#[cfg(not(debug_assertions))]
	century_test! {
		full_unixtime_0 0,
		full_unixtime_1 1,
		full_unixtime_2 2,
		full_unixtime_3 3,
		full_unixtime_4 4,
		full_unixtime_5 5,
		full_unixtime_6 6,
		full_unixtime_7 7,
		full_unixtime_8 8,
		full_unixtime_9 9,
	}
}
