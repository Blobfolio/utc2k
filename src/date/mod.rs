/*!
# UTC2K
*/

pub(super) mod parse;

use crate::{
	Abacus,
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
		Deref,
		Sub,
		SubAssign,
	},
	str::FromStr,
};



/// # Double-Digit ASCII.
static DD: [[u8; 2]; 100] = [
	[48, 48], [48, 49], [48, 50], [48, 51], [48, 52], [48, 53], [48, 54], [48, 55], [48, 56], [48, 57],
	[49, 48], [49, 49], [49, 50], [49, 51], [49, 52], [49, 53], [49, 54], [49, 55], [49, 56], [49, 57],
	[50, 48], [50, 49], [50, 50], [50, 51], [50, 52], [50, 53], [50, 54], [50, 55], [50, 56], [50, 57],
	[51, 48], [51, 49], [51, 50], [51, 51], [51, 52], [51, 53], [51, 54], [51, 55], [51, 56], [51, 57],
	[52, 48], [52, 49], [52, 50], [52, 51], [52, 52], [52, 53], [52, 54], [52, 55], [52, 56], [52, 57],
	[53, 48], [53, 49], [53, 50], [53, 51], [53, 52], [53, 53], [53, 54], [53, 55], [53, 56], [53, 57],
	[54, 48], [54, 49], [54, 50], [54, 51], [54, 52], [54, 53], [54, 54], [54, 55], [54, 56], [54, 57],
	[55, 48], [55, 49], [55, 50], [55, 51], [55, 52], [55, 53], [55, 54], [55, 55], [55, 56], [55, 57],
	[56, 48], [56, 49], [56, 50], [56, 51], [56, 52], [56, 53], [56, 54], [56, 55], [56, 56], [56, 57],
	[57, 48], [57, 49], [57, 50], [57, 51], [57, 52], [57, 53], [57, 54], [57, 55], [57, 56], [57, 57]
];



/// # Helper: `TryFrom` Unixtime For Non-u32 Formats.
macro_rules! try_from_unixtime {
	($($ty:ty),+) => ($(
		impl TryFrom<$ty> for Utc2k {
			type Error = Utc2kError;
			fn try_from(src: $ty) -> Result<Self, Self::Error> {
				u32::try_from(src)
					.map(Self::from)
					.map_err(|_| Utc2kError::Invalid)
			}
		}

		impl TryFrom<$ty> for FmtUtc2k {
			type Error = Utc2kError;
			#[inline]
			fn try_from(src: $ty) -> Result<Self, Self::Error> {
				Utc2k::try_from(src).map(Self::from)
			}
		}
	)+);
}



#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
/// # Formatted UTC2K.
///
/// This is the formatted companion to [`Utc2k`]. You can use it to obtain a
/// string version of the date, print it, etc.
///
/// While this acts essentially as a glorified `String`, it is sized exactly
/// and therefore requires less memory to represent. It also implements `Copy`.
///
/// It follows the simple Unix date format of `YYYY-MM-DD HH:MM:SS`.
///
/// Speaking of, you can obtain an `&str` using `Deref`, `AsRef<str>`,
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
pub struct FmtUtc2k([u8; 19]);

impl AsRef<[u8]> for FmtUtc2k {
	#[inline]
	fn as_ref(&self) -> &[u8] { self.as_bytes() }
}

macros::as_ref_borrow_cast!(FmtUtc2k: as_str str);

impl Default for FmtUtc2k {
	#[inline]
	fn default() -> Self { Self::MIN }
}

impl Deref for FmtUtc2k {
	type Target = str;

	#[inline]
	fn deref(&self) -> &Self::Target { self.as_str() }
}

macros::display_str!(as_str FmtUtc2k);

impl From<u32> for FmtUtc2k {
	#[inline]
	fn from(src: u32) -> Self { Self::from(Utc2k::from_unixtime(src)) }
}

impl From<&Utc2k> for FmtUtc2k {
	#[inline]
	fn from(src: &Utc2k) -> Self { Self::from(*src) }
}

impl From<Utc2k> for FmtUtc2k {
	fn from(src: Utc2k) -> Self {
		let mut out = Self::default();
		out.set_datetime(src);
		out
	}
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
macro_rules! eq {
	($($ty:ty),+) => ($(
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
eq!(&str, &String, String, &Cow<'_, str>, Cow<'_, str>, &Box<str>, Box<str>);

impl PartialOrd for FmtUtc2k {
	#[inline]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl TryFrom<&OsStr> for FmtUtc2k {
	type Error = Utc2kError;

	#[inline]
	/// # From `OsStr`.
	///
	/// ```
	/// use std::ffi::OsStr;
	/// use utc2k::FmtUtc2k;
	///
	/// assert_eq!(
	///     FmtUtc2k::try_from(OsStr::new("2013-12-15 21:30:02")).unwrap().as_str(),
	///     "2013-12-15 21:30:02"
	/// );
	/// assert_eq!(
	///     FmtUtc2k::try_from(OsStr::new("2013-12-15")).unwrap().as_str(),
	///     "2013-12-15 00:00:00"
	/// );
	/// ```
	fn try_from(src: &OsStr) -> Result<Self, Self::Error> {
		Utc2k::try_from(src).map(Self::from)
	}
}

impl TryFrom<&[u8]> for FmtUtc2k {
	type Error = Utc2kError;

	#[inline]
	fn try_from(src: &[u8]) -> Result<Self, Self::Error> {
		Utc2k::try_from(src).map(Self::from)
	}
}

impl TryFrom<&str> for FmtUtc2k {
	type Error = Utc2kError;

	#[inline]
	fn try_from(src: &str) -> Result<Self, Self::Error> {
		Utc2k::try_from(src).map(Self::from)
	}
}

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
	pub const MIN: Self = Self(*b"2000-01-01 00:00:00");

	/// # Maximum Date/Time.
	///
	/// ```
	/// assert_eq!(
	///     utc2k::FmtUtc2k::MAX.as_str(),
	///     "2099-12-31 23:59:59",
	/// );
	/// ```
	pub const MAX: Self = Self(*b"2099-12-31 23:59:59");
}

/// ## Instantiation/Reuse.
impl FmtUtc2k {
	#[inline]
	#[must_use]
	/// # Now.
	///
	/// This returns an instance using the current unixtime as the seed.
	pub fn now() -> Self { Self::from(Utc2k::now()) }

	#[cfg(feature = "local")]
	#[cfg_attr(docsrs, doc(cfg(feature = "local")))]
	#[must_use]
	/// # Now (Local).
	///
	/// This returns an instance using the current, local time as the seed. If
	/// no local offset can be determined, this is equivalent to [`FmtUtc2k::now`].
	///
	/// Refer to [`LocalOffset`](crate::LocalOffset) for important caveats and
	/// limitations.
	pub fn now_local() -> Self { Self::from(crate::LocalOffset::now()) }

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
	pub fn set_datetime(&mut self, src: Utc2k) {
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
	pub fn set_parts(&mut self, y: u16, m: u8, d: u8, hh: u8, mm: u8, ss: u8) {
		let (y, m, d, hh, mm, ss) = Abacus::new(y, m, d, hh, mm, ss).parts();
		self.set_parts_unchecked(y, m, d, hh, mm, ss);
	}

	/// # Set Parts (Unchecked).
	///
	/// Carry-overs, saturating, and 4-to-2-digit year-chopping have already
	/// been applied by the time this method is called.
	///
	/// From here, it's just straight ASCII-writing.
	fn set_parts_unchecked(&mut self, y: u8, m: u8, d: u8, hh: u8, mm: u8, ss: u8) {
		for (chunk, v) in self.0[1..].chunks_exact_mut(3).zip([y, m, d, hh, mm, ss]) {
			chunk[1..].copy_from_slice(DD[usize::from(v)].as_slice());
		}

		// Additionally make sure the result is ASCII.
		debug_assert!(self.0.is_ascii(), "Bug: Datetime is not ASCII.");
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
	/// Return a byte string slice in `YYYY-MM-DD HH:MM:SS` format.
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
	pub const fn as_bytes(&self) -> &[u8] { &self.0 }

	#[expect(unsafe_code, reason = "Content is ASCII.")]
	#[inline]
	#[must_use]
	/// # As Str.
	///
	/// Return a string slice in `YYYY-MM-DD HH:MM:SS` format.
	///
	/// A string slice can also be obtained using [`FmtUtc2k::as_ref`] or
	/// through dereferencing.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::FmtUtc2k;
	///
	/// let fmt = FmtUtc2k::MAX;
	/// assert_eq!(fmt.as_str(), "2099-12-31 23:59:59");
	/// ```
	pub const fn as_str(&self) -> &str {
		// Safety: datetimes are valid ASCII.
		unsafe { std::str::from_utf8_unchecked(&self.0) }
	}

	#[expect(unsafe_code, reason = "Content is ASCII.")]
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
		if let Some(v) = self.0.first_chunk::<10>() {
			debug_assert!(
				v[0].is_ascii_digit() &&
				v[1].is_ascii_digit() &&
				v[2].is_ascii_digit() &&
				v[3].is_ascii_digit() &&
				v[4] == b'-' &&
				v[5].is_ascii_digit() &&
				v[6].is_ascii_digit() &&
				v[7] == b'-' &&
				v[8].is_ascii_digit() &&
				v[9].is_ascii_digit(),
				"Bug: Date is not ASCII.",
			);

			// Safety: datetimes are valid ASCII.
			unsafe { std::str::from_utf8_unchecked(v.as_slice()) }
		}
		// Unreachable.
		else { "2000-01-01" }
	}

	#[expect(unsafe_code, reason = "Content is ASCII.")]
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
		if let Some(v) = self.0.first_chunk::<4>() {
			debug_assert!(
				v[0].is_ascii_digit() &&
				v[1].is_ascii_digit() &&
				v[2].is_ascii_digit() &&
				v[3].is_ascii_digit(),
				"Bug: Year is not ASCII.",
			);

			// Safety: datetimes are valid ASCII.
			unsafe { std::str::from_utf8_unchecked(v.as_slice()) }
		}
		// Unreachable.
		else { "2000" }
	}

	#[expect(unsafe_code, reason = "Content is ASCII.")]
	#[inline]
	#[must_use]
	/// # Just the Time Bits.
	///
	/// This returns the time as a string slice in `HH:MM:SS` format.
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
		if let Some(v) = self.0.last_chunk::<8>() {
			debug_assert!(
				v[0].is_ascii_digit() &&
				v[1].is_ascii_digit() &&
				v[2] == b':' &&
				v[3].is_ascii_digit() &&
				v[4].is_ascii_digit() &&
				v[5] == b':' &&
				v[6].is_ascii_digit() &&
				v[7].is_ascii_digit(),
				"Bug: Time is not ASCII.",
			);

			// Safety: datetimes are valid ASCII.
			unsafe { std::str::from_utf8_unchecked(v.as_slice()) }
		}
		// Unreachable.
		else { "00:00:00" }
	}
}

/// ## Formatting.
impl FmtUtc2k {
	#[must_use]
	/// # To RFC3339.
	///
	/// Return a string formatted according to [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339).
	///
	/// Note: this method is allocating.
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
	/// ```
	pub fn to_rfc3339(&self) -> String {
		let mut out = String::with_capacity(20);
		out.push_str(self.date());
		out.push('T');
		out.push_str(self.time());
		out.push('Z');
		out
	}

	#[inline]
	/// # From RFC2822.
	///
	/// This method can be used to construct a `FmtUtc2k` from an RFC2822-formatted
	/// string. Variations with and without a leading weekday, and with and
	/// without a trailing offset, are supported. If an offset is included, the
	/// datetime will be adjusted accordingly to make it properly UTC.
	///
	/// Note: missing offsets are meant to imply "localized" time, but as this
	/// library has no timezone handling, strings without any "+HHMM" at the
	/// end will be parsed as if they were already in UTC.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::{FmtUtc2k, Utc2k};
	///
	/// assert_eq!(
	///     FmtUtc2k::from_rfc2822("Tue, 1 Jul 2003 10:52:37 +0000"),
	///     FmtUtc2k::try_from("2003-07-01 10:52:37").ok(),
	/// );
	///
	/// assert_eq!(
	///     FmtUtc2k::from_rfc2822("Tue, 01 Jul 2003 10:52:37 +0000"),
	///     FmtUtc2k::try_from("2003-07-01 10:52:37").ok(),
	/// );
	///
	/// assert_eq!(
	///     FmtUtc2k::from_rfc2822("1 Jul 2003 10:52:37"),
	///     FmtUtc2k::try_from("2003-07-01 10:52:37").ok(),
	/// );
	///
	/// assert_eq!(
	///     FmtUtc2k::from_rfc2822("01 Jul 2003 10:52:37"),
	///     FmtUtc2k::try_from("2003-07-01 10:52:37").ok(),
	/// );
	///
	/// assert_eq!(
	///     FmtUtc2k::from_rfc2822("Tue, 10 Jul 2003 10:52:37 -0700"),
	///     FmtUtc2k::try_from("2003-07-10 17:52:37").ok(),
	/// );
	///
	/// assert_eq!(
	///     FmtUtc2k::from_rfc2822("Tue, 1 Jul 2003 10:52:37 +0430"),
	///     FmtUtc2k::try_from("2003-07-01 06:22:37").ok(),
	/// );
	/// ```
	pub fn from_rfc2822<S>(src: S) -> Option<Self>
	where S: AsRef<str> {
		Utc2k::from_rfc2822(src).map(Self::from)
	}

	#[expect(unsafe_code, reason = "Content is ASCII.")]
	#[must_use]
	/// # To RFC2822.
	///
	/// Return a string formatted according to [RFC2822](https://datatracker.ietf.org/doc/html/rfc2822).
	///
	/// There are a couple things to consider:
	/// * This method is allocating;
	/// * The length of the resulting string will either be `30` or `31` depending on whether the day is double-digit;
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::{FmtUtc2k, Utc2k};
	///
	/// let date = FmtUtc2k::from(Utc2k::new(2003, 7, 1, 10, 52, 37));
	/// assert_eq!(date.to_rfc2822(), "Tue, 01 Jul 2003 10:52:37 +0000");
	/// assert_eq!(date.to_rfc2822(), Utc2k::new(2003, 7, 1, 10, 52, 37).to_rfc2822());
	///
	/// let date = FmtUtc2k::from(Utc2k::new(2020, 6, 13, 8, 8, 8));
	/// assert_eq!(date.to_rfc2822(), "Sat, 13 Jun 2020 08:08:08 +0000");
	/// ```
	pub fn to_rfc2822(&self) -> String {
		let utc = Utc2k::from(self);
		let weekday: [u8; 3] = utc.weekday().abbreviation_bytes();
		let month: [u8; 3] = utc.month_enum().abbreviation_bytes();

		// Working from bytes is ugly, but performs much better than any
		// string-based operations.
		let out: Vec<u8> = vec![
			weekday[0], weekday[1], weekday[2],
			b',', b' ',
			self.0[8], self.0[9],
			b' ',
			month[0], month[1], month[2],
			b' ',
			b'2', b'0', self.0[2], self.0[3],
			b' ',
			self.0[11], self.0[12], self.0[13], self.0[14], self.0[15], self.0[16], self.0[17], self.0[18],
			b' ', b'+', b'0', b'0', b'0', b'0'
		];

		debug_assert!(out.is_ascii(), "Bug: Datetime is not ASCII.");
		// Safety: datetimes are valid ASCII.
		unsafe { String::from_utf8_unchecked(out) }
	}
}



#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
/// # UTC2K.
///
/// This is a lightweight date/time object for UTC date ranges within the
/// current century (e.g. `2000-01-01 00:00:00` to `2099-12-31 23:59:59`).
///
/// Values outside this range are saturated to fit, unless using
/// [`Utc2k::checked_from_unixtime`].
///
/// To instantiate from a UTC unix timestamp, use `From<u32>`. To try to parse
/// from a `YYYY-MM-DD HH:MM:SS` string, use `TryFrom<&str>` or `FromStr`.
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
/// assert!(Utc2k::try_from("2099-12-31T23:59:59.0000").is_ok()); // Also fine.
/// assert!(Utc2k::try_from("2099-12-31").is_ok()); // Also fine, but midnight.
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
		Self::from_abacus(Abacus::from(self).plus_seconds(other))
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
		<FmtUtc2k as fmt::Display>::fmt(&FmtUtc2k::from(*self), f)
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

impl FromStr for Utc2k {
	type Err = Utc2kError;

	#[inline]
	fn from_str(src: &str) -> Result<Self, Self::Err> { Self::try_from(src) }
}

impl Ord for Utc2k {
	/// # Compare.
	///
	/// Compare two dates.
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

try_from_unixtime!(i32, u64, i64, usize, isize);

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

	#[expect(clippy::option_if_let_else, reason = "Too messy.")]
	/// # Parse Slice.
	///
	/// This will attempt to construct a [`Utc2k`] from a date/time or date
	/// slice. See [`Utc2k::from_datetime_str`] and [`Utc2k::from_date_str`] for more
	/// information.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// let date = Utc2k::try_from(&b"2021/06/25"[..]).unwrap();
	/// assert_eq!(date.to_string(), "2021-06-25 00:00:00");
	///
	/// let date = Utc2k::try_from(&b"2021-06-25 13:15:25.0000"[..]).unwrap();
	/// assert_eq!(date.to_string(), "2021-06-25 13:15:25");
	///
	/// assert!(Utc2k::try_from(&b"2021-06-applesauces"[..]).is_err());
	/// ```
	fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
		if let Some(b) = bytes.first_chunk::<19>() {
			parse::parts_from_datetime(b)
		}
		else if let Some(b) = bytes.first_chunk::<10>() {
			parse::parts_from_date(b)
		}
		else { Err(Utc2kError::Invalid) }
	}
}

impl TryFrom<&str> for Utc2k {
	type Error = Utc2kError;

	#[inline]
	/// # Parse String.
	///
	/// This will attempt to construct a [`Utc2k`] from a date/time or date
	/// string. Parsing is naive; only the positions where numbers are
	/// expected will be looked at.
	///
	/// String length is used to determine whether or not the value should be
	/// parsed as a full date/time (19) or just a date (10).
	///
	/// See [`Utc2k::from_datetime_str`] and [`Utc2k::from_date_str`] for more
	/// information.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// let date = Utc2k::try_from("2021/06/25").unwrap();
	/// assert_eq!(date.to_string(), "2021-06-25 00:00:00");
	///
	/// let date = Utc2k::try_from("2021-06-25 13:15:25.0000").unwrap();
	/// assert_eq!(date.to_string(), "2021-06-25 13:15:25");
	///
	/// // The `FromStr` impl is an alias of `TryFrom<&str>` so can be used to
	/// // the same end:
	/// let date2 = "2021-06-25 13:15:25.0000".parse::<Utc2k>().unwrap();
	/// assert_eq!(date, date2);
	///
	/// // Really bad strings won't parse.
	/// assert!(Utc2k::try_from("2021-06-applesauces").is_err());
	/// ```
	fn try_from(src: &str) -> Result<Self, Self::Error> {
		Self::try_from(src.as_bytes())
	}
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
	/// # From Timestamp.
	///
	/// Initialize a new [`Utc2k`] from a unix timestamp, saturating to
	/// [`Utc2k::MIN_UNIXTIME`] or [`Utc2k::MAX_UNIXTIME`] if out of range.
	///
	/// This is identical to the `From<u32>`, but `const`.
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
			let (y, m, d) = parse::date_seconds(src.wrapping_div(DAY_IN_SECONDS));
			let (hh, mm, ss) = parse::time_seconds(src % DAY_IN_SECONDS);

			Self { y, m, d, hh, mm, ss }
		}
	}

	#[inline]
	#[must_use]
	/// # Now.
	///
	/// Create a new instance representing the current UTC time.
	pub fn now() -> Self { Self::from_unixtime(unixtime()) }

	#[cfg(feature = "local")]
	#[cfg_attr(docsrs, doc(cfg(feature = "local")))]
	#[must_use]
	/// # Now (Local).
	///
	/// This returns an instance using the current, local time as the seed. If
	/// no local offset can be determined, this is equivalent to [`Utc2k::now`].
	///
	/// Refer to [`LocalOffset`](crate::LocalOffset) for important caveats and
	/// limitations.
	pub fn now_local() -> Self { Self::from(crate::LocalOffset::now()) }

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
	/// assert_eq!(Utc2k::tomorrow(), Utc2k::now() + 86_400_u32);
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
	/// assert_eq!(Utc2k::yesterday(), Utc2k::now() - 86_400_u32);
	/// ```
	pub fn yesterday() -> Self { Self::from_unixtime(unixtime() - DAY_IN_SECONDS) }
}

/// ## String Parsing.
impl Utc2k {
	/// # From Date/Time.
	///
	/// Parse a string containing a date/time in `YYYY-MM-DD HH:MM:SS` format.
	/// This operation is naive and only looks at the positions where numbers
	/// are expected.
	///
	/// In other words, `2020-01-01 00:00:00` will parse the same as
	/// `2020/01/01 00:00:00` or even `2020-01-01 00:00:00.0000 PDT`.
	///
	/// As with all the other methods, dates outside the `2000..=2099` range
	/// will be saturated (non-failing), and overflows will be carried over to
	/// the appropriate unit (e.g. 13 months will become +1 year and 1 month).
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// // This isn't long enough.
	/// assert!(Utc2k::from_datetime_str("2021/06/25").is_err());
	///
	/// // This is fine.
	/// let date = Utc2k::from_datetime_str("2021-06-25 13:15:25.0000").unwrap();
	/// assert_eq!(date.to_string(), "2021-06-25 13:15:25");
	///
	/// // This is all wrong.
	/// assert!(Utc2k::from_datetime_str("Applebutter").is_err());
	/// ```
	///
	/// ## Errors
	///
	/// If any of the digits fail to parse, or if the string is insufficiently
	/// sized, an error will be returned.
	pub fn from_datetime_str<B>(src: B) -> Result<Self, Utc2kError>
	where B: AsRef<[u8]> {
		src.as_ref().first_chunk::<19>()
			.ok_or(Utc2kError::Invalid)
			.and_then(parse::parts_from_datetime)
	}

	/// # From Date/Time (Smooshed).
	///
	/// This is just like [`Utc2k::from_datetime_str`] for "smooshed" datetime
	/// strings, i.e. `YYYYMMDDHHMMSS` (no separators).
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// // This isn't long enough.
	/// assert!(Utc2k::from_smooshed_datetime_str("20210625").is_err());
	///
	/// // This is fine.
	/// let date = Utc2k::from_smooshed_datetime_str("20210625131525").unwrap();
	/// assert_eq!(date.to_string(), "2021-06-25 13:15:25");
	///
	/// // This *won't* work because there are separators in the way.
	/// assert!(Utc2k::from_smooshed_datetime_str("2021-06-25 13:15:25").is_err());
	///
	/// // This is all wrong.
	/// assert!(Utc2k::from_smooshed_datetime_str("Applebutterful").is_err());
	/// ```
	///
	/// ## Errors
	///
	/// If any of the digits fail to parse, or if the string is insufficiently
	/// sized, an error will be returned.
	pub fn from_smooshed_datetime_str<B>(src: B) -> Result<Self, Utc2kError>
	where B: AsRef<[u8]> {
		src.as_ref().first_chunk::<14>()
			.ok_or(Utc2kError::Invalid)
			.and_then(parse::parts_from_smooshed_datetime)
	}

	/// # From Date.
	///
	/// Parse a string containing a date/time in `YYYY-MM-DD` format. This
	/// operation is naive and only looks at the positions where numbers are
	/// expected.
	///
	/// In other words, `2020-01-01` will parse the same as `2020/01/01` or
	/// even `2020-01-01 13:03:33.5900 PDT`.
	///
	/// As with all the other methods, dates outside the `2000..=2099` range
	/// will be saturated (non-failing), and overflows will be carried over to
	/// the appropriate unit (e.g. 13 months will become +1 year and 1 month).
	///
	/// The time will always be set to midnight when using this method.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// // This is fine.
	/// let date = Utc2k::from_date_str("2021/06/25").unwrap();
	/// assert_eq!(date.to_string(), "2021-06-25 00:00:00");
	///
	/// // This is fine, but the time will be ignored.
	/// let date = Utc2k::from_date_str("2021-06-25 13:15:25.0000").unwrap();
	/// assert_eq!(date.to_string(), "2021-06-25 00:00:00");
	///
	/// // This is all wrong.
	/// assert!(Utc2k::from_date_str("Applebutter").is_err());
	/// ```
	///
	/// ## Errors
	///
	/// If any of the digits fail to parse, or if the string is insufficiently
	/// sized, an error will be returned.
	pub fn from_date_str<B>(src: B) -> Result<Self, Utc2kError>
	where B: AsRef<[u8]> {
		src.as_ref().first_chunk::<10>()
			.ok_or(Utc2kError::Invalid)
			.and_then(parse::parts_from_date)
	}

	/// # From Date (Smooshed).
	///
	/// This is just like [`Utc2k::from_date_str`] for "smooshed" date strings,
	/// i.e. `YYYYMMDD` (no separators).
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// // This is fine.
	/// let date = Utc2k::from_smooshed_date_str("20210625").unwrap();
	/// assert_eq!(date.to_string(), "2021-06-25 00:00:00");
	///
	/// // This is fine, but the time will be ignored.
	/// let date = Utc2k::from_smooshed_date_str("20210625131525").unwrap();
	/// assert_eq!(date.to_string(), "2021-06-25 00:00:00");
	///
	/// // This *won't* work because it has dashes in the way.
	/// assert!(Utc2k::from_smooshed_date_str("2021-06-25").is_err());
	///
	/// // This is all wrong.
	/// assert!(Utc2k::from_smooshed_date_str("Applebutter").is_err());
	/// ```
	///
	/// ## Errors
	///
	/// If any of the digits fail to parse, or if the string is insufficiently
	/// sized, an error will be returned.
	pub fn from_smooshed_date_str<B>(src: B) -> Result<Self, Utc2kError>
	where B: AsRef<[u8]> {
		src.as_ref().first_chunk::<8>()
			.copied()
			.ok_or(Utc2kError::Invalid)
			.and_then(parse::parts_from_smooshed_date)
	}

	/// # Parse Time.
	///
	/// This method attempts to parse a time string in the `HH:MM:SS` format,
	/// returning the hours, minutes, and seconds as integers.
	///
	/// As with other methods in this library, only positions where numbers are
	/// expected will be looked at. `01:02:03` will parse the same way as
	/// `01-02-03`.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// assert_eq!(
	///     Utc2k::parse_time_str("15:35:47"),
	///     Ok((15, 35, 47))
	/// );
	///
	/// // The hours are out of range.
	/// assert!(Utc2k::parse_time_str("30:35:47").is_err());
	/// ```
	///
	/// ## Errors
	///
	/// This method will return an error if any of the numeric bits are invalid
	/// or out of range (hours must be < 24, minutes and seconds < 60).
	pub fn parse_time_str<B>(src: B) -> Result<(u8, u8, u8), Utc2kError>
	where B: AsRef<[u8]> {
		if let Some(b) = src.as_ref().first_chunk::<8>() {
			let (hh, mm, ss) = parse::hms(b.as_slice())?;
			if hh < 24 && mm < 60 && ss < 60 {
				return Ok((hh, mm, ss));
			}
		}

		Err(Utc2kError::Invalid)
	}
}

/// ## Get Parts.
impl Utc2k {
	#[inline]
	#[must_use]
	/// # Parts.
	///
	/// Return the year, month, etc., parts.
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
	/// This returns the total number of days this month could hold, or put
	/// another way, the last day of this month.
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
	/// Return the number of seconds since midnight. In other words, this adds
	/// up all of the time bits.
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
	pub fn formatted(self) -> FmtUtc2k { FmtUtc2k::from(self) }

	#[inline]
	#[must_use]
	/// # To RFC3339.
	///
	/// Return a string formatted according to [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339).
	///
	/// Note: this method is allocating.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// let date = Utc2k::new(2021, 12, 13, 11, 56, 1);
	/// assert_eq!(date.to_rfc3339(), "2021-12-13T11:56:01Z");
	/// ```
	pub fn to_rfc3339(&self) -> String { FmtUtc2k::from(*self).to_rfc3339() }

	#[expect(unsafe_code, reason = "Content is ASCII.")]
	#[must_use]
	/// # To RFC2822.
	///
	/// Return a string formatted according to [RFC2822](https://datatracker.ietf.org/doc/html/rfc2822).
	///
	/// There are a couple things to consider:
	/// * This method is allocating;
	/// * The length of the resulting string will either be `30` or `31` depending on whether the day is double-digit;
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// let date = Utc2k::new(2003, 7, 1, 10, 52, 37);
	/// assert_eq!(date.to_rfc2822(), "Tue, 01 Jul 2003 10:52:37 +0000");
	///
	/// let date = Utc2k::new(2036, 12, 15, 16, 30, 55);
	/// assert_eq!(date.to_rfc2822(), "Mon, 15 Dec 2036 16:30:55 +0000");
	/// ```
	pub fn to_rfc2822(&self) -> String {
		let weekday: [u8; 3] = self.weekday().abbreviation_bytes();
		let month: [u8; 3] = self.month_enum().abbreviation_bytes();

		let day = DD[usize::from(self.d)];
		let year = DD[usize::from(self.y)];
		let hh = DD[usize::from(self.hh)];
		let mm = DD[usize::from(self.mm)];
		let ss = DD[usize::from(self.ss)];

		// Working from bytes is ugly, but performs much better than any
		// string-based operations.
		let out: Vec<u8> = vec![
			weekday[0], weekday[1], weekday[2],
			b',', b' ',
			day[0], day[1],
			b' ',
			month[0], month[1], month[2],
			b' ',
			b'2', b'0', year[0], year[1],
			b' ',
			hh[0], hh[1], b':', mm[0], mm[1], b':', ss[0], ss[1],
			b' ', b'+', b'0', b'0', b'0', b'0'
		];

		debug_assert!(out.is_ascii(), "Bug: Datetime is not ASCII.");
		// Safety: datetimes are valid ASCII.
		unsafe { String::from_utf8_unchecked(out) }
	}

	/// # From RFC2822.
	///
	/// This method can be used to construct a `Utc2k` from an RFC2822-formatted
	/// string. Variations with and without a leading weekday, and with and
	/// without a trailing offset, are supported. If an offset is included, the
	/// datetime will be adjusted accordingly to make it properly UTC.
	///
	/// Note: missing offsets are meant to imply "localized" time, but as this
	/// library has no timezone handling, strings without any "+HHMM" at the
	/// end will be parsed as if they were already in UTC.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// assert_eq!(
	///     Utc2k::from_rfc2822("Tue, 1 Jul 2003 10:52:37 +0000"),
	///     Some(Utc2k::new(2003, 7, 1, 10, 52, 37)),
	/// );
	///
	/// assert_eq!(
	///     Utc2k::from_rfc2822("Tue, 01 Jul 2003 10:52:37 +0000"),
	///     Some(Utc2k::new(2003, 7, 1, 10, 52, 37)),
	/// );
	///
	/// assert_eq!(
	///     Utc2k::from_rfc2822("1 Jul 2003 10:52:37"),
	///     Some(Utc2k::new(2003, 7, 1, 10, 52, 37)),
	/// );
	///
	/// assert_eq!(
	///     Utc2k::from_rfc2822("01 Jul 2003 10:52:37"),
	///     Some(Utc2k::new(2003, 7, 1, 10, 52, 37)),
	/// );
	///
	/// assert_eq!(
	///     Utc2k::from_rfc2822("Tue, 10 Jul 2003 10:52:37 -0700"),
	///     Some(Utc2k::new(2003, 7, 10, 17, 52, 37)),
	/// );
	///
	/// assert_eq!(
	///     Utc2k::from_rfc2822("Tue, 1 Jul 2003 10:52:37 +0430"),
	///     Some(Utc2k::new(2003, 7, 1, 6, 22, 37)),
	/// );
	/// ```
	pub fn from_rfc2822<S>(src: S) -> Option<Self>
	where S: AsRef<str> {
		let src: &[u8] = src.as_ref().as_bytes().trim_ascii();
		if 19 <= src.len() {
			// Strip off the optional weekday, if any, so we can parse the day
			// from a predictable starting place.
			if src[0].is_ascii_alphabetic() { parse::rfc2822_day(&src[5..]) }
			else { parse::rfc2822_day(src) }
		}
		else { None }
	}

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
	/// # Change Time.
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
	/// use utc2k::Utc2k;
	///
	/// // Too old.
	/// assert!(Utc2k::checked_from_unixtime(0).is_err());
	///
	/// // Too new.
	/// assert!(Utc2k::checked_from_unixtime(u32::MAX).is_err());
	///
	/// // This fits.
	/// assert!(Utc2k::checked_from_unixtime(Utc2k::MIN_UNIXTIME).is_ok());
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
	pub(crate) const fn from_abacus(src: Abacus) -> Self {
		let (y, m, d, hh, mm, ss) = src.parts();
		Self { y, m, d, hh, mm, ss }
	}

	#[must_use]
	/// # From `FmtUtc2k`.
	pub(crate) const fn from_fmtutc2k(src: FmtUtc2k) -> Self {
		Self::new(
			2000 + ((src.0[2] & 0x0f) * 10 + (src.0[3] & 0x0f)) as u16,
			(src.0[5] & 0x0f) * 10 + (src.0[6] & 0x0f),
			(src.0[8] & 0x0f) * 10 + (src.0[9] & 0x0f),
			(src.0[11] & 0x0f) * 10 + (src.0[12] & 0x0f),
			(src.0[14] & 0x0f) * 10 + (src.0[15] & 0x0f),
			(src.0[17] & 0x0f) * 10 + (src.0[18] & 0x0f),
		)
	}

	#[expect(clippy::cast_possible_truncation, reason = "False positive.")]
	#[must_use]
	/// # Subtract Seconds.
	pub(crate) const fn minus_seconds(self, offset: u32) -> Self {
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

			let (hh, mm, ss) = parse::time_seconds(easy);
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



impl From<Utc2k> for u32 {
	#[inline]
	fn from(src: Utc2k) -> Self { src.unixtime() }
}



#[cfg(test)]
mod tests {
	use super::*;
	use brunch as _;
	use time::OffsetDateTime;



	#[cfg(not(miri))]
	const SAMPLE_SIZE: usize = 1_000_000;

	#[cfg(miri)]
	const SAMPLE_SIZE: usize = 1000; // Miri runs way too slow for a million tests.



	macro_rules! range_test {
		($buf:ident, $i:ident, $format:ident) => (
			let u = Utc2k::from($i);
			let f = FmtUtc2k::from(u);
			let c = OffsetDateTime::from_unix_timestamp($i as i64)
				.expect("Unable to create time::OffsetDateTime.");
			$buf.set_datetime(u);

			// Make sure the timestamp comes back the same.
			assert_eq!($i, u.unixtime(), "Timestamp out does not match timestamp in!");

			// Make sure back-and-forth froms work as expected.
			assert_eq!(Utc2k::from(f), u);

			// Test RFC2822 back and forth.
			assert_eq!(Some(u), Utc2k::from_rfc2822(u.to_rfc2822()));

			assert_eq!(u.year(), c.year() as u16, "Year mismatch for unixtime {}", $i);
			assert_eq!(u.month(), u8::from(c.month()), "Month mismatch for unixtime {}", $i);
			assert_eq!(u.day(), c.day(), "Day mismatch for unixtime {}", $i);
			assert_eq!(u.hour(), c.hour(), "Hour mismatch for unixtime {}", $i);
			assert_eq!(u.minute(), c.minute(), "Minute mismatch for unixtime {}", $i);
			assert_eq!(u.second(), c.second(), "Second mismatch for unixtime {}", $i);
			assert_eq!(u.ordinal(), c.ordinal(), "Ordinal mismatch for unixtime {}", $i);

			// Make sure the weekdays match.
			assert_eq!(u.weekday().as_ref(), c.weekday().to_string());

			// Test string conversion.
			assert_eq!(
				$buf.as_str(),
				&c.format(&$format).expect("Unable to format datetime."),
				"Date mismatch for unixtime {}",
				$i
			);

		);
	}

	#[cfg(not(debug_assertions))]
	/// # Generate Century Tests.
	///
	/// There are a lot of seconds to test. Multiple functions allows
	/// parallelization where supported.
	macro_rules! century_test {
		($($fn:ident, $rem:literal),+ $(,)?) => ($(
			#[test]
			#[ignore = "testing a decade's worth of seconds takes a very long time"]
			/// # 1/10 Full Range Unixtime Test.
			fn $fn() {
				let mut buf = FmtUtc2k::default();
				let format = time::format_description::parse(
					"[year]-[month]-[day] [hour]:[minute]:[second]",
				).expect("Unable to parse datetime format.");
				for i in Utc2k::MIN_UNIXTIME..=Utc2k::MAX_UNIXTIME {
					if $rem == i % 10 { range_test!(buf, i, format); }
				}
			}
		)+);
	}

	#[cfg(not(debug_assertions))]
	century_test!(
		full_unixtime_0, 0,
		full_unixtime_1, 1,
		full_unixtime_2, 2,
		full_unixtime_3, 3,
		full_unixtime_4, 4,
		full_unixtime_5, 5,
		full_unixtime_6, 6,
		full_unixtime_7, 7,
		full_unixtime_8, 8,
		full_unixtime_9, 9,
	);

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
		let mut buf = FmtUtc2k::default();
		let format = time::format_description::parse(
			"[year]-[month]-[day] [hour]:[minute]:[second]",
		).expect("Unable to parse datetime format.");

		let mut rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.u32(Utc2k::MIN_UNIXTIME..=Utc2k::MAX_UNIXTIME)).take(SAMPLE_SIZE) {
			range_test!(buf, i, format);
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
}
