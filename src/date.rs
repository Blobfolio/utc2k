/*!
# UTC2K
*/

use crate::{
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
	ops::Deref,
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

/// ## Constants.
impl FmtUtc2k {
	/// # Minimum Date/Time.
	pub const MIN: [u8; 19] = *b"2000-01-01 00:00:00";

	/// # Maximum Date/Time.
	pub const MAX: [u8; 19] = *b"2099-12-31 23:59:59";
}

/// ## Instantiation/Reuse.
impl FmtUtc2k {
	#[inline]
	#[must_use]
	/// # Now.
	///
	/// This returns an instance using the current unixtime as the seed.
	pub fn now() -> Self { Self::from(Utc2k::now()) }

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

	#[allow(clippy::cast_possible_truncation)] // It fits.
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

	#[allow(clippy::cast_possible_truncation)] // It fits.
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
		let (y, m, d, hh, mm, ss) = maybe_carry_over_parts(y, m, d, hh, mm, ss);
		self.set_parts_unchecked((y - 2000) as u8, m, d, hh, mm, ss);
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

	#[allow(clippy::cast_possible_truncation)] // It fits.
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
	/// fmt.set_timestamp(Utc2k::MAX_UNIXTIME);
	/// assert_eq!(fmt.as_str(), "2099-12-31 23:59:59");
	/// ```
	pub fn set_timestamp(&mut self, src: u32) { self.set_datetime(Utc2k::from(src)); }
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
		if src < Self::MIN_UNIXTIME { return Self::min(); }
		else if src > Self::MAX_UNIXTIME { return Self::max(); }

		// Tease out the date parts with a lot of terrible math.
		let (y, m, d) = parse_date_seconds(src / DAY_IN_SECONDS);
		let (hh, mm, ss) = parse_time_seconds(src % DAY_IN_SECONDS);

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

try_from_unixtime!(i32, u64, i64, usize, isize);

impl TryFrom<&str> for Utc2k {
	type Error = Utc2kError;

	fn try_from(src: &str) -> Result<Self, Self::Error> {
		// Work from bytes.
		let bytes = src.as_bytes();

		// It has to be at least 19 characters long.
		if bytes.len() >= 19 && bytes.is_ascii() {
			Ok(Self::new(
				bytes[..4].iter()
					.try_fold(0, |a, c|
						if c.is_ascii_digit() { Ok(a * 10 + u16::from(c & 0x0f)) }
						else { Err(Utc2kError::Invalid) }
					)?,
				parse_u8_str(bytes[5], bytes[6])?,
				parse_u8_str(bytes[8], bytes[9])?,
				parse_u8_str(bytes[11], bytes[12])?,
				parse_u8_str(bytes[14], bytes[15])?,
				parse_u8_str(bytes[17], bytes[18])?,
			))
		}
		else { Err(Utc2kError::Invalid) }
	}
}

/// ## Constants.
impl Utc2k {
	/// # Minimum Timestamp.
	pub const MIN_UNIXTIME: u32 = 946_684_800;

	/// # Maximum Timestamp.
	pub const MAX_UNIXTIME: u32 = 4_102_444_799;
}

/// ## Instantiation.
impl Utc2k {
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

	#[allow(clippy::cast_possible_truncation)] // It fits.
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
		let (y, m, d, hh, mm, ss) = maybe_carry_over_parts(y, m, d, hh, mm, ss);
		Self {
			y: (y - 2000) as u8,
			m, d, hh, mm, ss
		}
	}

	#[inline]
	#[must_use]
	/// # Now.
	///
	/// Create a new instance representing the current UTC time.
	pub fn now() -> Self { Self::from(unixtime()) }

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
}

/// ## Getters.
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
	/// let date = Utc2k::new(2010, 5, 5, 16, 30, 1);
	/// assert_eq!(date.parts(), (2010, 5, 5, 16, 30, 1));
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
		// Start with all the seconds prior to the year.
		let (mut time, leap) = year_size(self.year());

		// Add up all the seconds since the start of the year.
		time += u32::from(self.ss) +
			MINUTE_IN_SECONDS * u32::from(self.mm) +
			HOUR_IN_SECONDS * u32::from(self.hh) +
			DAY_IN_SECONDS * (u32::from(self.d) - 1) +
			month_seconds(self.m);

		// Factor in an extra leap day?
		if leap && self.m > 2 {
			time += DAY_IN_SECONDS;
		}

		time
	}

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
}



#[must_use]
/// # Maybe Carry Over Parts.
///
/// This checks to see if all of the date/time parts are within range. If they
/// are, it simply passes them back. If they aren't, it sends them to
/// [`carry_over_parts`] so they can be adjusted.
const fn maybe_carry_over_parts(y: u16, m: u8, d: u8, hh: u8, mm: u8, ss: u8)
-> (u16, u8, u8, u8, u8, u8) {
	// Everything is in range!
	if
		1999 < y && y < 2100 &&
		0 < m && m < 13 &&
		0 < d &&
		hh < 24 &&
		mm < 60 &&
		ss < 60 &&
		d <= month_days(y, m)
	{ (y, m, d, hh, mm, ss) }
	else {
		carry_over_parts(y, m as u16, d as u16, hh as u16, mm as u16, ss as u16)
	}
}

#[allow(clippy::cast_possible_truncation)] // It fits.
#[allow(clippy::integer_division)] // It's OK.
#[inline]
#[must_use]
/// # Carry Over Parts.
///
/// This makes sure years, months, etc., are in range. In cases where there are
/// 13 months, say, that becomes 1 year and 1 month.
///
/// Dates outside the century will be capped accordingly.
const fn carry_over_parts(y: u16, m: u16, mut d: u16, mut hh: u16, mut mm: u16, mut ss: u16)
-> (u16, u8, u8, u8, u8, u8) {
	// Seconds to minutes.
	if ss > 59 {
		let div = ss / 60;
		mm += div;
		ss -= div * 60;
	}
	// Minutes to hours.
	if mm > 59 {
		let div = mm / 60;
		hh += div;
		mm -= div * 60;
	}
	// Hours to days.
	if hh > 23 {
		let div = hh / 24;
		d += div;
		hh -= div * 24;
	}

	// Fix the date bits, which is little trickier.
	let (y, m, d) = carry_over_date_parts(y, m, d);

	// Did we overflow?
	if y > 2099 { (2099, 12, 31, 23, 59, 59) }
	else if y < 2000 { (2000, 1, 1, 0, 0, 0) }
	else { (y, m, d, hh as u8, mm as u8, ss as u8) }
}

#[allow(clippy::cast_possible_truncation)] // It fits.
#[allow(clippy::integer_division)] // It's OK.
/// # Carry Over Date Parts.
///
/// This recurses in cases where days overflow as each new month brings a new
/// maximum number of days.
const fn carry_over_date_parts(mut y: u16, mut m: u16, mut d: u16) -> (u16, u8, u8) {
	// There has to be a month.
	if m == 0 { m = 1; }
	// Months to Years.
	else if m > 12 {
		let div = m / 12;
		y += div;
		m -= div * 12;
	}

	// There has to be a day.
	if d == 0 { d = 1; }
	else {
		// Days to Months.
		let size = month_days(y, m as u8) as u16;
		if d > size {
			m += 1;
			d -= size;

			// Recurse.
			return carry_over_date_parts(y, m, d);
		}
	}

	(y, m as u8, d as u8)
}

#[inline]
#[must_use]
/// # Is Leap Year?
const fn leap_year(y: u16) -> bool { (y % 4 == 0 && y % 100 != 0) || y % 400 == 0 }

#[must_use]
/// # Days in Month.
///
/// This returns the number of days in a given year/month. (Year is included to
/// make this leap-aware.)
const fn month_days(y: u16, m: u8) -> u8 {
	match m {
		1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
		4 |6 |9 | 11 => 30,
		2 =>
			if leap_year(y) { 29 }
			else { 28 },
		_ => 0,
	}
}

#[must_use]
/// # Month Size.
///
/// This returns the number of seconds from the start of the year up to the
/// given month.
///
/// Note: this does not consider February leap days.
const fn month_seconds(m: u8) -> u32 {
	match m {
		2 => 2_678_400,
		3 => 5_097_600,
		4 => 7_776_000,
		5 => 10_368_000,
		6 => 13_046_400,
		7 => 15_638_400,
		8 => 18_316_800,
		9 => 20_995_200,
		10 => 23_587_200,
		11 => 26_265_600,
		12 => 28_857_600,
		_ => 0,
	}
}

/// # Parse 4 Digits.
///
/// This parses a 4-digit numeric string slice into `u16`, or dies trying.
const fn parse_u8_str(one: u8, two: u8) -> Result<u8, Utc2kError> {
	if one.is_ascii_digit() && two.is_ascii_digit() {
		Ok((one & 0x0f) * 10 + (two & 0x0f))
	}
	else { Err(Utc2kError::Invalid) }
}

#[allow(clippy::cast_possible_truncation)] // It fits.
#[allow(clippy::integer_division)]
#[allow(clippy::many_single_char_names)]
/// # Parse Date From Seconds.
///
/// This parses the date portion of a date/time timestamp using the same
/// approach as [`time`](https://crates.io/crates/time), which is based on
/// algorithms by [Peter Baum](https://www.researchgate.net/publication/316558298_Date_Algorithms).
///
/// (Our version is a little simpler as we aren't worried about old times.)
///
/// It is not quite as fast as the Gregorian approach used by [`chrono`](https://crates.io/crates/chrono),
/// but is significantly simpler.
const fn parse_date_seconds(mut z: u32) -> (u8, u8, u8) {
	z += JULIAN_EPOCH - 1_721_119;
	let h = 100 * z - 25;
	let mut a = h / 3_652_425;
	a -= a / 4;
	let mut year = (100 * a + h) / 36_525;
	a += z - 365 * year - year / 4;
	let mut month = (5 * a + 456) / 153;
	let day = a - (153 * month - 457) / 5;

	if month > 12 {
		year += 1;
		month -= 12;
	}

	((year - 2000) as u8, month as u8, day as u8)
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

	let mm =
		if src >= MINUTE_IN_SECONDS {
			let mm = ((src * 0x889) >> 17) as u8;
			src -= mm as u32 * MINUTE_IN_SECONDS;
			mm
		}
		else { 0 };

	(hh, mm, src as u8)
}

#[allow(clippy::too_many_lines)] // We have a century to cover!
#[must_use]
/// # Year Size.
///
/// This returns the number of seconds from 1970 up to a given year, and a
/// bool indicating whether or not it is/was a leap year.
///
/// Because we're only looking at a single century, a simple pre-computed table
/// lookup is faster than any realtime calculation.
const fn year_size(y: u16) -> (u32, bool) {
	match y {
		2000 => (946_684_800, true),
		2001 => (978_307_200, false),
		2002 => (1_009_843_200, false),
		2003 => (1_041_379_200, false),
		2004 => (1_072_915_200, true),
		2005 => (1_104_537_600, false),
		2006 => (1_136_073_600, false),
		2007 => (1_167_609_600, false),
		2008 => (1_199_145_600, true),
		2009 => (1_230_768_000, false),
		2010 => (1_262_304_000, false),
		2011 => (1_293_840_000, false),
		2012 => (1_325_376_000, true),
		2013 => (1_356_998_400, false),
		2014 => (1_388_534_400, false),
		2015 => (1_420_070_400, false),
		2016 => (1_451_606_400, true),
		2017 => (1_483_228_800, false),
		2018 => (1_514_764_800, false),
		2019 => (1_546_300_800, false),
		2020 => (1_577_836_800, true),
		2021 => (1_609_459_200, false),
		2022 => (1_640_995_200, false),
		2023 => (1_672_531_200, false),
		2024 => (1_704_067_200, true),
		2025 => (1_735_689_600, false),
		2026 => (1_767_225_600, false),
		2027 => (1_798_761_600, false),
		2028 => (1_830_297_600, true),
		2029 => (1_861_920_000, false),
		2030 => (1_893_456_000, false),
		2031 => (1_924_992_000, false),
		2032 => (1_956_528_000, true),
		2033 => (1_988_150_400, false),
		2034 => (2_019_686_400, false),
		2035 => (2_051_222_400, false),
		2036 => (2_082_758_400, true),
		2037 => (2_114_380_800, false),
		2038 => (2_145_916_800, false),
		2039 => (2_177_452_800, false),
		2040 => (2_208_988_800, true),
		2041 => (2_240_611_200, false),
		2042 => (2_272_147_200, false),
		2043 => (2_303_683_200, false),
		2044 => (2_335_219_200, true),
		2045 => (2_366_841_600, false),
		2046 => (2_398_377_600, false),
		2047 => (2_429_913_600, false),
		2048 => (2_461_449_600, true),
		2049 => (2_493_072_000, false),
		2050 => (2_524_608_000, false),
		2051 => (2_556_144_000, false),
		2052 => (2_587_680_000, true),
		2053 => (2_619_302_400, false),
		2054 => (2_650_838_400, false),
		2055 => (2_682_374_400, false),
		2056 => (2_713_910_400, true),
		2057 => (2_745_532_800, false),
		2058 => (2_777_068_800, false),
		2059 => (2_808_604_800, false),
		2060 => (2_840_140_800, true),
		2061 => (2_871_763_200, false),
		2062 => (2_903_299_200, false),
		2063 => (2_934_835_200, false),
		2064 => (2_966_371_200, true),
		2065 => (2_997_993_600, false),
		2066 => (3_029_529_600, false),
		2067 => (3_061_065_600, false),
		2068 => (3_092_601_600, true),
		2069 => (3_124_224_000, false),
		2070 => (3_155_760_000, false),
		2071 => (3_187_296_000, false),
		2072 => (3_218_832_000, true),
		2073 => (3_250_454_400, false),
		2074 => (3_281_990_400, false),
		2075 => (3_313_526_400, false),
		2076 => (3_345_062_400, true),
		2077 => (3_376_684_800, false),
		2078 => (3_408_220_800, false),
		2079 => (3_439_756_800, false),
		2080 => (3_471_292_800, true),
		2081 => (3_502_915_200, false),
		2082 => (3_534_451_200, false),
		2083 => (3_565_987_200, false),
		2084 => (3_597_523_200, true),
		2085 => (3_629_145_600, false),
		2086 => (3_660_681_600, false),
		2087 => (3_692_217_600, false),
		2088 => (3_723_753_600, true),
		2089 => (3_755_376_000, false),
		2090 => (3_786_912_000, false),
		2091 => (3_818_448_000, false),
		2092 => (3_849_984_000, true),
		2093 => (3_881_606_400, false),
		2094 => (3_913_142_400, false),
		2095 => (3_944_678_400, false),
		2096 => (3_976_214_400, true),
		2097 => (4_007_836_800, false),
		2098 => (4_039_372_800, false),
		_ => (4_070_908_800, false),
	}
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
			let u = Utc2k::from(i);
			let c = Utc.timestamp(i as i64, 0);
			buf.set_datetime(u);

			// Make sure the timestamp comes back the same.
			assert_eq!(i, u.unixtime());

			assert_eq!(u.year(), c.year() as u16, "Year mismatch for unixtime {}", i);
			assert_eq!(u.month(), c.month() as u8, "Month mismatch for unixtime {}", i);
			assert_eq!(u.day(), c.day() as u8, "Day mismatch for unixtime {}", i);
			assert_eq!(u.hour(), c.hour() as u8, "Hour mismatch for unixtime {}", i);
			assert_eq!(u.minute(), c.minute() as u8, "Minute mismatch for unixtime {}", i);
			assert_eq!(u.second(), c.second() as u8, "Second mismatch for unixtime {}", i);

			assert_eq!(buf.as_str(), c.format("%Y-%m-%d %H:%M:%S").to_string(), "Date mismatch for unixtime {}", i);
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
			let u = Utc2k::from(i);
			let c = Utc.timestamp(i as i64, 0);
			buf.set_datetime(u);

			// Make sure the timestamp comes back the same.
			assert_eq!(i, u.unixtime());

			assert_eq!(u.year(), c.year() as u16, "Year mismatch for unixtime {}", i);
			assert_eq!(u.month(), c.month() as u8, "Month mismatch for unixtime {}", i);
			assert_eq!(u.day(), c.day() as u8, "Day mismatch for unixtime {}", i);
			assert_eq!(u.hour(), c.hour() as u8, "Hour mismatch for unixtime {}", i);
			assert_eq!(u.minute(), c.minute() as u8, "Minute mismatch for unixtime {}", i);
			assert_eq!(u.second(), c.second() as u8, "Second mismatch for unixtime {}", i);

			assert_eq!(buf.as_str(), c.format("%Y-%m-%d %H:%M:%S").to_string(), "Date mismatch for unixtime {}", i);
		}
	}

	#[test]
	/// # Test Carry-Over.
	///
	/// This helps ensure we're doing the math correctly.
	fn carries() {
		// Overage of one everywhere.
		assert_eq!(
			carry_over_parts(2000, 13, 32, 24, 60, 60),
			(2001, 2, 2, 1, 1, 0)
		);

		// Large month/day overages.
		assert_eq!(
			carry_over_parts(2000, 25, 99, 1, 1, 1),
			(2002, 4, 9, 1, 1, 1)
		);

		// Large time overflows.
		assert_eq!(
			carry_over_parts(2000, 1, 1, 99, 99, 99),
			(2000, 1, 5, 4, 40, 39)
		);

		// Saturating low.
		assert_eq!(
			carry_over_parts(1970, 25, 99, 1, 1, 1),
			(2000, 1, 1, 0, 0, 0)
		);

		// Saturating high.
		assert_eq!(
			carry_over_parts(2099, 25, 99, 1, 1, 1),
			(2099, 12, 31, 23, 59, 59)
		);
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
