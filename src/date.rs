/*!
# UTC2K
*/

#![allow(clippy::shadow_unrelated)]

use crate::{
	Abacus,
	DAY_IN_SECONDS,
	HOUR_IN_SECONDS,
	JULIAN_EPOCH,
	MINUTE_IN_SECONDS,
	unixtime,
	Utc2kError,
};
use std::{
	borrow::Borrow,
	cmp::Ordering,
	convert::TryFrom,
	fmt,
	ops::{
		Add,
		AddAssign,
		Deref,
		Sub,
		SubAssign,
	},
};



/// # Double-Digit ASCII.
const DD: &[u8; 200] = b"\
	0001020304050607080910111213141516171819\
	2021222324252627282930313233343536373839\
	4041424344454647484950515253545556575859\
	6061626364656667686970717273747576777879\
	8081828384858687888990919293949596979899";



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

impl AsRef<str> for FmtUtc2k {
	#[inline]
	fn as_ref(&self) -> &str { self.as_str() }
}

impl Borrow<str> for FmtUtc2k {
	#[inline]
	fn borrow(&self) -> &str { self.as_str() }
}

impl Default for FmtUtc2k {
	#[inline]
	fn default() -> Self { Self::min() }
}

impl Deref for FmtUtc2k {
	type Target = str;
	#[inline]
	fn deref(&self) -> &Self::Target { self.as_str() }
}

impl fmt::Display for FmtUtc2k {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl From<u32> for FmtUtc2k {
	#[inline]
	fn from(src: u32) -> Self { Self::from(Utc2k::from(src)) }
}

impl From<Utc2k> for FmtUtc2k {
	fn from(src: Utc2k) -> Self {
		let mut out = Self::default();
		out.set_datetime(src);
		out
	}
}

impl Ord for FmtUtc2k {
	#[inline]
	fn cmp(&self, other: &Self) -> Ordering { self.0.cmp(&other.0) }
}

impl PartialOrd for FmtUtc2k {
	#[inline]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
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
	pub(crate) const MIN: [u8; 19] = *b"2000-01-01 00:00:00";

	/// # Maximum Date/Time.
	pub(crate) const MAX: [u8; 19] = *b"2099-12-31 23:59:59";

	#[inline]
	#[must_use]
	/// # Minimum Value.
	///
	/// This is equivalent to `2000-01-01 00:00:00`.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::FmtUtc2k;
	///
	/// let date = FmtUtc2k::min();
	/// assert_eq!(date.as_str(), "2000-01-01 00:00:00");
	/// ```
	pub const fn min() -> Self { Self(Self::MIN) }

	#[inline]
	#[must_use]
	/// # Maximum Value.
	///
	/// This is equivalent to `2099-12-31 23:59:59`.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::FmtUtc2k;
	///
	/// let date = FmtUtc2k::max();
	/// assert_eq!(date.as_str(), "2099-12-31 23:59:59");
	/// ```
	pub const fn max() -> Self { Self(Self::MAX) }
}

/// ## Instantiation/Reuse.
impl FmtUtc2k {
	#[inline]
	#[must_use]
	/// # Now.
	///
	/// This returns an instance using the current unixtime as the seed.
	pub fn now() -> Self { Self::from(Utc2k::now()) }

	#[allow(clippy::cast_possible_truncation)] // It fits.
	/// # Set Date/Time.
	///
	/// This can be used to recycle an existing buffer.
	///
	/// As with all other part-based operations, overflows and underflows will
	/// be adjusted automatically, with minimum and maximum dates capped to
	/// [`FmtUtc2k::min`] and [`FmtUtc2k::max`] respectively.
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
	/// [`FmtUtc2k::min`] and [`FmtUtc2k::max`] respectively.
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
		use std::ptr::copy_nonoverlapping;

		let src = DD.as_ptr();
		let dst = self.0.as_mut_ptr();

		unsafe {
			copy_nonoverlapping(src.add((y << 1) as usize), dst.add(2), 2);
			copy_nonoverlapping(src.add((m << 1) as usize), dst.add(5), 2);
			copy_nonoverlapping(src.add((d << 1) as usize), dst.add(8), 2);
			copy_nonoverlapping(src.add((hh << 1) as usize), dst.add(11), 2);
			copy_nonoverlapping(src.add((mm << 1) as usize), dst.add(14), 2);
			copy_nonoverlapping(src.add((ss << 1) as usize), dst.add(17), 2);
		}
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
	/// # As Str.
	///
	/// Return a string slice in `YYYY-MM-DD HH:MM:SS` format.
	///
	/// A string slice can also be obtained using [`FmtUtc2k::as_ref`] or
	/// through dereferencing.
	pub fn as_str(&self) -> &str {
		unsafe { std::str::from_utf8_unchecked(&self.0) }
	}

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
	pub fn date(&self) -> &str {
		unsafe { std::str::from_utf8_unchecked(&self.0[..10]) }
	}

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
	pub fn time(&self) -> &str {
		unsafe { std::str::from_utf8_unchecked(&self.0[11..]) }
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
/// from a `YYYY-MM-DD HH:MM:SS` string, use `TryFrom<&str>`.
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
/// use std::convert::TryFrom;
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
	y: u8,
	m: u8,
	d: u8,
	hh: u8,
	mm: u8,
	ss: u8,
}

impl Add<u32> for Utc2k {
	type Output = Self;
	fn add(self, other: u32) -> Self {
		let tmp = Abacus::from(self) + other;
		Self::from(tmp)
	}
}

impl AddAssign<u32> for Utc2k {
	#[inline]
	fn add_assign(&mut self, other: u32) { *self = *self + other; }
}

impl Default for Utc2k {
	#[inline]
	fn default() -> Self { Self::min() }
}

impl fmt::Display for Utc2k {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let buf = FmtUtc2k::from(*self);
		f.write_str(buf.as_str())
	}
}

impl From<u32> for Utc2k {
	#[allow(clippy::integer_division)]
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
	fn from(src: u32) -> Self {
		if src <= Self::MIN_UNIXTIME { Self::min() }
		else if src >= Self::MAX_UNIXTIME { Self::max() }
		else {
			// Tease out the date parts with a lot of terrible math.
			let (y, m, d) = parse_date_seconds(src / DAY_IN_SECONDS);
			let (hh, mm, ss) = parse_time_seconds(src % DAY_IN_SECONDS);

			Self { y, m, d, hh, mm, ss }
		}
	}
}

impl From<Abacus> for Utc2k {
	fn from(src: Abacus) -> Self {
		let (y, m, d, hh, mm, ss) = src.parts();
		Self { y, m, d, hh, mm, ss }
	}
}

impl Ord for Utc2k {
	fn cmp(&self, other: &Self) -> Ordering {
		// Work our way down until there's a difference!
		match self.y.cmp(&other.y) {
			Ordering::Equal => match self.m.cmp(&other.m) {
				Ordering::Equal => match self.d.cmp(&other.d) {
					Ordering::Equal => match self.hh.cmp(&other.hh) {
						Ordering::Equal => match self.mm.cmp(&other.mm) {
							Ordering::Equal => self.ss.cmp(&other.ss),
							x => x,
						},
						x => x,
					},
					x => x,
				},
				x => x,
			},
			x => x,
		}
	}
}

impl PartialOrd for Utc2k {
	#[inline]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Sub<u32> for Utc2k {
	type Output = Self;
	fn sub(self, other: u32) -> Self {
		Self::from(self.unixtime().saturating_sub(other))
	}
}

impl SubAssign<u32> for Utc2k {
	#[inline]
	fn sub_assign(&mut self, other: u32) { *self = *self - other; }
}

try_from_unixtime!(i32, u64, i64, usize, isize);

impl TryFrom<&str> for Utc2k {
	type Error = Utc2kError;

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
	/// use std::convert::TryFrom;
	///
	/// let date = Utc2k::try_from("2021/06/25").unwrap();
	/// assert_eq!(date.to_string(), "2021-06-25 00:00:00");
	///
	/// let date = Utc2k::try_from("2021-06-25 13:15:25.0000").unwrap();
	/// assert_eq!(date.to_string(), "2021-06-25 13:15:25");
	///
	/// assert!(Utc2k::try_from("Applebutter").is_err());
	/// ```
	fn try_from(src: &str) -> Result<Self, Self::Error> {
		// Work from bytes.
		let bytes = src.as_bytes();
		if bytes.len() >= 19 {
			parse_parts_from_datetime(unsafe {
				&*(bytes[..19].as_ptr().cast::<[u8; 19]>())
			})
		}
		else if bytes.len() >= 10 {
			parse_parts_from_date(unsafe {
				&*(bytes[..10].as_ptr().cast::<[u8; 10]>())
			})
		}
		else { Err(Utc2kError::Invalid) }
	}
}

/// ## Min/Max.
impl Utc2k {
	/// # Minimum Timestamp.
	pub const MIN_UNIXTIME: u32 = 946_684_800;

	/// # Maximum Timestamp.
	pub const MAX_UNIXTIME: u32 = 4_102_444_799;

	#[inline]
	#[must_use]
	/// # Minimum Value.
	///
	/// This is equivalent to `2000-01-01 00:00:00`.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// let date = Utc2k::min();
	/// assert_eq!(date.to_string(), "2000-01-01 00:00:00");
	/// ```
	pub const fn min() -> Self { Self { y: 0, m: 1, d: 1, hh: 0, mm: 0, ss: 0 } }

	#[inline]
	#[must_use]
	/// # Maximum Value.
	///
	/// This is equivalent to `2099-12-31 23:59:59`.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	///
	/// let date = Utc2k::max();
	/// assert_eq!(date.to_string(), "2099-12-31 23:59:59");
	/// ```
	pub const fn max() -> Self { Self { y: 99, m: 12, d: 31, hh: 23, mm: 59, ss: 59 } }
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
	pub fn new(y: u16, m: u8, d: u8, hh: u8, mm: u8, ss: u8) -> Self {
		Self::from(Abacus::new(y, m, d, hh, mm, ss))
	}

	#[inline]
	#[must_use]
	/// # Now.
	///
	/// Create a new instance representing the current UTC time.
	pub fn now() -> Self { Self::from(unixtime()) }
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
	pub fn from_datetime_str(src: &str) -> Result<Self, Utc2kError> {
		let bytes: &[u8] = src.as_bytes();
		if bytes.len() >= 19 {
			parse_parts_from_datetime(unsafe {
				&*(bytes[..19].as_ptr().cast::<[u8; 19]>())
			})
		}
		else { Err(Utc2kError::Invalid) }
	}

	/// # From Date/Time.
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
	pub fn from_date_str(src: &str) -> Result<Self, Utc2kError> {
		let bytes: &[u8] = src.as_bytes();
		if bytes.len() >= 10 {
			parse_parts_from_date(unsafe {
				&*(bytes[..10].as_ptr().cast::<[u8; 10]>())
			})
		}
		else { Err(Utc2kError::Invalid) }
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
	/// use std::convert::TryFrom;
	///
	/// let date = Utc2k::try_from("2020-05-10 00:00:00").unwrap();
	/// assert!(date.leap_year());
	///
	/// let date = Utc2k::try_from("2021-03-15 00:00:00").unwrap();
	/// assert!(! date.leap_year());
	/// ```
	pub const fn leap_year(self) -> bool {
		// Leap years this century.
		const LEAP_YEARS: [bool; 100] = [true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false];

		LEAP_YEARS[self.y as usize]
	}

	#[must_use]
	/// # Month Name.
	///
	/// Return the name of the month, nice and pretty.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Utc2k;
	/// use std::convert::TryFrom;
	///
	/// let date = Utc2k::try_from("2020-06-24 20:19:30").unwrap();
	/// assert_eq!(date.month_name(), "June");
	/// ```
	pub const fn month_name(self) -> &'static str {
		match self.m {
			1 => "January",
			2 => "February",
			3 => "March",
			4 => "April",
			5 => "May",
			6 => "June",
			7 => "July",
			8 => "August",
			9 => "September",
			10 => "October",
			11 => "November",
			_ => "December",
		}
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
	/// use std::convert::TryFrom;
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
	pub fn unixtime(self) -> u32 {
		// Seconds from the new year up to the start of the month.
		static MONTH_SECONDS: [u32; 12] = [0, 2_678_400, 5_097_600, 7_776_000, 10_368_000, 13_046_400, 15_638_400, 18_316_800, 20_995_200, 23_587_200, 26_265_600, 28_857_600];

		// Seconds *before* the new year.
		static YEAR_SECONDS: [u32; 100] = [946_684_800, 978_307_200, 1_009_843_200, 1_041_379_200, 1_072_915_200, 1_104_537_600, 1_136_073_600, 1_167_609_600, 1_199_145_600, 1_230_768_000, 1_262_304_000, 1_293_840_000, 1_325_376_000, 1_356_998_400, 1_388_534_400, 1_420_070_400, 1_451_606_400, 1_483_228_800, 1_514_764_800, 1_546_300_800, 1_577_836_800, 1_609_459_200, 1_640_995_200, 1_672_531_200, 1_704_067_200, 1_735_689_600, 1_767_225_600, 1_798_761_600, 1_830_297_600, 1_861_920_000, 1_893_456_000, 1_924_992_000, 1_956_528_000, 1_988_150_400, 2_019_686_400, 2_051_222_400, 2_082_758_400, 2_114_380_800, 2_145_916_800, 2_177_452_800, 2_208_988_800, 2_240_611_200, 2_272_147_200, 2_303_683_200, 2_335_219_200, 2_366_841_600, 2_398_377_600, 2_429_913_600, 2_461_449_600, 2_493_072_000, 2_524_608_000, 2_556_144_000, 2_587_680_000, 2_619_302_400, 2_650_838_400, 2_682_374_400, 2_713_910_400, 2_745_532_800, 2_777_068_800, 2_808_604_800, 2_840_140_800, 2_871_763_200, 2_903_299_200, 2_934_835_200, 2_966_371_200, 2_997_993_600, 3_029_529_600, 3_061_065_600, 3_092_601_600, 3_124_224_000, 3_155_760_000, 3_187_296_000, 3_218_832_000, 3_250_454_400, 3_281_990_400, 3_313_526_400, 3_345_062_400, 3_376_684_800, 3_408_220_800, 3_439_756_800, 3_471_292_800, 3_502_915_200, 3_534_451_200, 3_565_987_200, 3_597_523_200, 3_629_145_600, 3_660_681_600, 3_692_217_600, 3_723_753_600, 3_755_376_000, 3_786_912_000, 3_818_448_000, 3_849_984_000, 3_881_606_400, 3_913_142_400, 3_944_678_400, 3_976_214_400, 4_007_836_800, 4_039_372_800, 4_070_908_800];

		// Add up everything as it would be in a non-leap year.
		let time = YEAR_SECONDS[usize::from(self.y)] +
			MONTH_SECONDS[usize::from(self.m - 1)] +
			self.seconds_from_midnight() +
			DAY_IN_SECONDS * u32::from(self.d - 1);

		// Add a day's worth of seconds if we need to.
		if 2 < self.m && self.leap_year() { time + DAY_IN_SECONDS }
		else { time }
	}
}

/// ## Checked Operations.
impl Utc2k {
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
	/// let date = Utc2k::max();
	/// assert!(date.checked_add(1).is_none());
	///
	/// let date = Utc2k::new(2010, 1, 1, 0, 0, 0);
	/// let added = date.checked_add(86_413).unwrap();
	/// assert_eq!(added.to_string(), "2010-01-02 00:00:13");
	/// ```
	pub fn checked_add(self, secs: u32) -> Option<Self> {
		self.unixtime().checked_add(secs)
			.filter(|s| s <= &Self::MAX_UNIXTIME)
			.map(Self::from)
	}

	/// # From Unixtime (Checked).
	///
	/// This can be used instead of the usual `From<u32>` if you'd like to
	/// trigger an error when the timestamp is out of range (rather than just
	/// saturating it).
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
	pub fn checked_from_unixtime(src: u32) -> Result<Self, Utc2kError> {
		if src < Self::MIN_UNIXTIME { Err(Utc2kError::Underflow) }
		else if src > Self::MAX_UNIXTIME { Err(Utc2kError::Overflow) }
		else { Ok(Self::from(src)) }
	}

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
	/// let date = Utc2k::min();
	/// assert!(date.checked_sub(1).is_none());
	///
	/// let date = Utc2k::new(2010, 1, 1, 0, 0, 0);
	/// let subbed = date.checked_sub(86_413).unwrap();
	/// assert_eq!(subbed.to_string(), "2009-12-30 23:59:47");
	/// ```
	pub fn checked_sub(self, secs: u32) -> Option<Self> {
		self.unixtime().checked_sub(secs)
			.filter(|s| s >= &Self::MIN_UNIXTIME)
			.map(Self::from)
	}
}

impl From<Utc2k> for u32 {
	#[inline]
	fn from(src: Utc2k) -> Self { src.unixtime() }
}

#[allow(clippy::cast_possible_truncation)] // It fits.
#[allow(clippy::integer_division)]
/// # Parse Date From Seconds.
///
/// This parses the date portion of a date/time timestamp using the same
/// approach as [`time`](https://crates.io/crates/time), which is based on
/// algorithms by [Peter Baum](https://www.researchgate.net/publication/316558298_Date_Algorithms).
///
/// (Our version is a little simpler as we aren't worried about old times.)
const fn parse_date_seconds(mut z: u32) -> (u8, u8, u8) {
	z += JULIAN_EPOCH - 1_721_119;
	let h = 100 * z - 25;
	let mut a = h / 3_652_425;
	a -= a / 4;
	let year = (100 * a + h) / 36_525;
	a += z - 365 * year - year / 4;
	let month = (5 * a + 456) / 153;
	let day = (a - (153 * month - 457) / 5) as u8;

	if month > 12 {
		((year - 1999) as u8, month as u8 - 12, day)
	}
	else {
		((year - 2000) as u8, month as u8, day)
	}
}

/// # Parse Parts From Date.
///
/// This attempts to extract the year, month, and day from a `YYYY-MM-DD` byte
/// slice. Only the numeric ranges are parsed — separators can be whatever.
fn parse_parts_from_date(src: &[u8; 10]) -> Result<Utc2k, Utc2kError> {
	let tmp = Abacus::new(
		src.iter()
			.take(4)
			.try_fold(0, |a, &c|
				if c.is_ascii_digit() { Ok(a * 10 + u16::from(c & 0x0f)) }
				else { Err(Utc2kError::Invalid) }
			)?,
		parse_u8_str(src[5], src[6])?,
		parse_u8_str(src[8], src[9])?,
		0, 0, 0
	);
	Ok(Utc2k::from(tmp))
}

/// # Parse Parts From Date/Time.
///
/// This attempts to extract the year, month, day, hour, minute and second from
/// a `YYYY-MM-DD HH:MM:SS` byte slice. Only the numeric ranges are parsed —
/// separators can be whatever.
fn parse_parts_from_datetime(src: &[u8; 19]) -> Result<Utc2k, Utc2kError> {
	let tmp = Abacus::new(
		src.iter()
			.take(4)
			.try_fold(0, |a, &c|
				if c.is_ascii_digit() { Ok(a * 10 + u16::from(c & 0x0f)) }
				else { Err(Utc2kError::Invalid) }
			)?,
		parse_u8_str(src[5], src[6])?,
		parse_u8_str(src[8], src[9])?,
		parse_u8_str(src[11], src[12])?,
		parse_u8_str(src[14], src[15])?,
		parse_u8_str(src[17], src[18])?,
	);
	Ok(Utc2k::from(tmp))
}

#[allow(clippy::cast_possible_truncation)] // It fits.
/// # Parse Time From Seconds.
///
/// This parses the time portion of a date/time timestamp. It works the same
/// way a naive div/mod approach would, except it uses multiplication and bit
/// shifts to avoid actually having to div/mod.
///
/// (This only works because time values stop at 23 or 59; rounding errors
/// would creep in if the full u8 range was used.)
const fn parse_time_seconds(mut src: u32) -> (u8, u8, u8) {
	let hh =
		if src >= HOUR_IN_SECONDS {
			let hh = ((src * 0x91A3) >> 27) as u8;
			src -= hh as u32 * HOUR_IN_SECONDS;
			hh
		}
		else { 0 };

	if src >= MINUTE_IN_SECONDS {
		let mm = ((src * 0x889) >> 17) as u8;
		src -= mm as u32 * MINUTE_IN_SECONDS;
		(hh, mm, src as u8)
	}
	else {
		(hh, 0, src as u8)
	}
}

/// # Parse 2 Digits.
///
/// This combines two ASCII `u8` values into a single `u8` integer, or dies
/// trying (if, i.e., one or both are non-numeric).
const fn parse_u8_str(one: u8, two: u8) -> Result<u8, Utc2kError> {
	if one.is_ascii_digit() && two.is_ascii_digit() {
		Ok((one & 0x0f) * 10 + (two & 0x0f))
	}
	else { Err(Utc2kError::Invalid) }
}



#[cfg(test)]
mod tests {
	use super::*;
	use brunch as _;
	use chrono::{
		Datelike,
		Timelike,
		TimeZone,
		Utc,
	};

	macro_rules! range_test {
		($buf:ident, $i:ident) => (
			let u = Utc2k::from($i);
			let c = Utc.timestamp($i as i64, 0);
			$buf.set_datetime(u);

			// Make sure the timestamp comes back the same.
			assert_eq!($i, u.unixtime(), "Timestamp out does not match timestamp in!");

			assert_eq!(u.year(), c.year() as u16, "Year mismatch for unixtime {}", $i);
			assert_eq!(u.month(), c.month() as u8, "Month mismatch for unixtime {}", $i);
			assert_eq!(u.day(), c.day() as u8, "Day mismatch for unixtime {}", $i);
			assert_eq!(u.hour(), c.hour() as u8, "Hour mismatch for unixtime {}", $i);
			assert_eq!(u.minute(), c.minute() as u8, "Minute mismatch for unixtime {}", $i);
			assert_eq!(u.second(), c.second() as u8, "Second mismatch for unixtime {}", $i);
			assert_eq!(u.ordinal(), c.ordinal() as u16, "Ordinal mismatch for unixtime {}", $i);

			assert_eq!($buf.as_str(), c.format("%Y-%m-%d %H:%M:%S").to_string(), "Date mismatch for unixtime {}", $i);
		);
	}

	#[test]
	#[ignore]
	/// # Test Full Unixtime Range for `FmtUtc2k` and `Utc2k`.
	///
	/// This compares our objects against `chrono` to ensure conversions line
	/// up as expected for the supported unixtime range.
	///
	/// Chrono's string formatting is ridiculously slow; if you run this test
	/// it will take a while to complete.
	fn unixtime_range() {
		let mut buf = FmtUtc2k::default();
		for i in Utc2k::MIN_UNIXTIME..=Utc2k::MAX_UNIXTIME {
			range_test!(buf, i);
		}
	}

	#[test]
	/// # Test Limited Unixtime Range for `FmtUtc2k` and `Utc2k`.
	///
	/// This covers about 1% of the `ignore`d full-range test, providing decent
	/// coverage and a reasonable runtime.
	fn limited_unixtime_range() {
		let mut buf = FmtUtc2k::default();
		for i in (Utc2k::MIN_UNIXTIME..=Utc2k::MAX_UNIXTIME).step_by(97) {
			range_test!(buf, i);
		}
	}

	#[test]
	/// # Leap Years.
	fn leap_years() {
		for y in 2000..2100 {
			let date = Utc2k::new(y, 1, 1, 0, 0, 0);
			assert_eq!(date.year(), y);
			assert_eq!(
				date.leap_year(),
				y.trailing_zeros() >= 2 && ((y % 100) != 0 || (y % 400) == 0)
			);
		}
	}

	#[test]
	/// # Test Ordering.
	fn ordering() {
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
}
