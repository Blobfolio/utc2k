/*!
# UTC2K: Local Dates!
*/

use crate::{
	DateChar,
	DAY_IN_SECONDS,
	FmtUtc2k,
	HOUR_IN_SECONDS,
	macros,
	MINUTE_IN_SECONDS,
	Month,
	Utc2k,
	Weekday,
};
use std::{
	borrow::Cow,
	cmp::Ordering,
	fmt,
	hash,
	num::NonZeroI32,
	sync::OnceLock,
};
use super::Abacus;
use tz::timezone::TimeZone;



/// # Parsed Timezone Details.
static TZ: OnceLock<Option<TimeZone>> = OnceLock::new();



#[derive(Debug, Clone, Copy)]
/// # Formatted Local ~~UTC~~2K.
///
/// This is the formatted companion to [`Local2k`]. You can use it to obtain a
/// string version of the date, print it, etc.
///
/// While this acts essentially as a glorified `String`, it is sized exactly
/// and therefore requires less memory to represent. It also implements `Copy`.
///
/// It follows the simple Unix date format of `YYYY-MM-DD hh:mm:ss`.
///
/// Speaking of, you can obtain an `&str` using `AsRef<str>`,
/// `Borrow<str>`, or [`FmtLocal2k::as_str`].
///
/// If you only want the date or time half, call [`FmtLocal2k::date`] or
/// [`FmtLocal2k::time`] respectively.
///
/// See [`Local2k`] for limitations and gotchas.
pub struct FmtLocal2k {
	/// # Date/Time (w/ `offset`)
	inner: FmtUtc2k,

	/// # Local Offset (Seconds).
	offset: Option<NonZeroI32>,
}

impl AsRef<[u8]> for FmtLocal2k {
	#[inline]
	fn as_ref(&self) -> &[u8] { self.as_bytes() }
}

macros::as_ref_borrow_cast!(FmtLocal2k: as_str str);

impl fmt::Display for FmtLocal2k {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		<FmtUtc2k as fmt::Display>::fmt(&self.inner, f)
	}
}

impl Eq for FmtLocal2k {}

impl From<Local2k> for FmtLocal2k {
	#[inline]
	fn from(src: Local2k) -> Self { Self::from_local2k(src) }
}

impl From<FmtLocal2k> for String {
	#[inline]
	fn from(src: FmtLocal2k) -> Self { src.as_str().to_owned() }
}

impl hash::Hash for FmtLocal2k {
	#[inline]
	fn hash<H: hash::Hasher>(&self, state: &mut H) {
		<Local2k as hash::Hash>::hash(&Local2k::from_fmtlocal2k(*self), state);
	}
}

impl Ord for FmtLocal2k {
	#[inline]
	fn cmp(&self, other: &Self) -> Ordering {
		if self.offset == other.offset { self.inner.cmp(&other.inner) }
		else {
			Local2k::from_fmtlocal2k(*self).cmp(&Local2k::from_fmtlocal2k(*other))
		}
	}
}

impl PartialEq for FmtLocal2k {
	#[inline]
	fn eq(&self, other: &Self) -> bool {
		if self.offset == other.offset { self.inner == other.inner }
		else {
			Local2k::from_fmtlocal2k(*self) == Local2k::from_fmtlocal2k(*other)
		}
	}
}

impl PartialEq<str> for FmtLocal2k {
	#[inline]
	fn eq(&self, other: &str) -> bool { self.as_str() == other }
}
impl PartialEq<FmtLocal2k> for str {
	#[inline]
	fn eq(&self, other: &FmtLocal2k) -> bool { <FmtLocal2k as PartialEq<Self>>::eq(other, self) }
}

/// # Helper: Reciprocal `PartialEq`.
macro_rules! fmt_eq {
	($($ty:ty)+) => ($(
		impl PartialEq<$ty> for FmtLocal2k {
			#[inline]
			fn eq(&self, other: &$ty) -> bool { <Self as PartialEq<str>>::eq(self, other) }
		}
		impl PartialEq<FmtLocal2k> for $ty {
			#[inline]
			fn eq(&self, other: &FmtLocal2k) -> bool { <FmtLocal2k as PartialEq<str>>::eq(other, self) }
		}
	)+);
}
fmt_eq! { &str &String String &Cow<'_, str> Cow<'_, str> &Box<str> Box<str> }

impl PartialOrd for FmtLocal2k {
	#[inline]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

/// ## Instantiation.
impl FmtLocal2k {
	#[inline]
	#[must_use]
	/// # Now.
	///
	/// Create a new instance representing the current local time.
	///
	/// ```
	/// use utc2k::{FmtLocal2k, Local2k};
	///
	/// // Equivalent.
	/// assert_eq!(
	///     FmtLocal2k::now(),
	///     FmtLocal2k::from(Local2k::now()),
	/// );
	/// ```
	pub fn now() -> Self { Self::from_local2k(Local2k::now()) }
}

/// ## Getters.
impl FmtLocal2k {
	#[inline]
	#[must_use]
	/// # As Bytes.
	///
	/// Return a byte string slice in `YYYY-MM-DD hh:mm:ss` format.
	///
	/// A byte slice can also be obtained using [`FmtLocal2k::as_ref`].
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::{Local2k, Utc2k};
	///
	/// let fmt = Local2k::from(Utc2k::MAX).formatted();
	/// # let fmt = Local2k::fixed_from_utc2k(Utc2k::MAX, -28800).formatted();
	/// assert_eq!(
	///     fmt.as_bytes(),
	///     b"2099-12-31 15:59:59", // e.g. California.
	/// );
	/// ```
	pub const fn as_bytes(&self) -> &[u8] { self.inner.as_bytes() }

	#[inline]
	#[must_use]
	/// # As Str.
	///
	/// Return a string slice in `YYYY-MM-DD hh:mm:ss` format.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::{Local2k, Utc2k};
	///
	/// let fmt = Local2k::from(Utc2k::MAX).formatted();
	/// # let fmt = Local2k::fixed_from_utc2k(Utc2k::MAX, -28800).formatted();
	/// assert_eq!(
	///     fmt.as_str(),
	///     "2099-12-31 15:59:59", // e.g. California.
	/// );
	/// ```
	pub const fn as_str(&self) -> &str { self.inner.as_str() }

	#[inline]
	#[must_use]
	/// # Just the Date Bits.
	///
	/// This returns the date as a string slice in `YYYY-MM-DD` format.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::{Local2k, Utc2k};
	///
	/// let utc = Utc2k::new(2025, 6, 19, 18, 57, 12);
	/// let fmt = Local2k::from(utc).formatted();
	/// # let fmt = Local2k::fixed_from_utc2k(utc, -25200).formatted();
	/// assert_eq!(
	///     fmt.as_str(),
	///     "2025-06-19 11:57:12", // e.g. California.
	/// );
	/// assert_eq!(fmt.date(), "2025-06-19");
	/// ```
	pub const fn date(&self) -> &str { self.inner.date() }

	#[inline]
	#[must_use]
	/// # Just the Year Bit.
	///
	/// This returns the year as a string slice.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::{Local2k, Utc2k};
	///
	/// let utc = Utc2k::new(2025, 6, 19, 18, 57, 12);
	/// let fmt = Local2k::from(utc).formatted();
	/// # let fmt = Local2k::fixed_from_utc2k(utc, -25200).formatted();
	/// assert_eq!(
	///     fmt.as_str(),
	///     "2025-06-19 11:57:12", // e.g. California.
	/// );
	/// assert_eq!(fmt.year(), "2025");
	/// ```
	pub const fn year(&self) -> &str { self.inner.year() }

	#[inline]
	#[must_use]
	/// # Just the Time Bits.
	///
	/// This returns the time as a string slice in `hh:mm:ss` format.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::{Local2k, Utc2k};
	///
	/// let utc = Utc2k::new(2025, 6, 19, 18, 57, 12);
	/// let fmt = Local2k::from(utc).formatted();
	/// # let fmt = Local2k::fixed_from_utc2k(utc, -25200).formatted();
	/// assert_eq!(
	///     fmt.as_str(),
	///     "2025-06-19 11:57:12", // e.g. California.
	/// );
	/// assert_eq!(fmt.time(), "11:57:12");
	/// ```
	pub const fn time(&self) -> &str { self.inner.time() }
}

/// ## Conversion.
impl FmtLocal2k {
	#[must_use]
	/// # To RFC2822.
	///
	/// Return a string formatted according to [RFC2822](https://datatracker.ietf.org/doc/html/rfc2822).
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::{Local2k, Utc2k};
	///
	/// // A proper UTC date in RFC2822.
	/// let utc = Utc2k::new(2021, 12, 13, 04, 56, 1);
	/// assert_eq!(
	///     utc.to_rfc2822(),
	///     "Mon, 13 Dec 2021 04:56:01 +0000",
	/// );
	///
	/// // The same date localized to, say, California.
	/// let local = Local2k::from(utc).formatted();
	/// # let local = Local2k::fixed_from_utc2k(utc, -28800).formatted();
	/// assert_eq!(
	///     local.to_rfc2822(),
	///     "Sun, 12 Dec 2021 20:56:01 -0800",
	/// );
	/// ```
	///
	/// The RFC2822 date/time format is portable, whether local or UTC.
	///
	/// ```
	/// # use utc2k::{Local2k, Utc2k};
	/// # let utc = Utc2k::new(2003, 7, 1, 10, 52, 37);
	/// # let local = Local2k::from(utc).formatted();
	/// let utc_2822 = utc.to_rfc2822();
	/// let local_2822 = local.to_rfc2822();
	///
	/// // The RFC2822 representations will vary if there's an offset, but
	/// // if parsed back into a Utc2k, that'll get sorted and they'll match!
	/// assert_eq!(
	///     Utc2k::from_rfc2822(utc_2822.as_bytes()),
	///     Some(utc),
	/// );
	/// assert_eq!(
	///     Utc2k::from_rfc2822(local_2822.as_bytes()),
	///     Some(utc),
	/// );
	/// ```
	pub fn to_rfc2822(&self) -> String {
		let local = Local2k::from_fmtlocal2k(*self);

		let mut out = String::with_capacity(31);
		out.push_str(local.weekday().abbreviation());
		out.push_str(", ");
		out.push(self.inner.0[8].as_char());
		out.push(self.inner.0[9].as_char());
		out.push(' ');
		out.push_str(local.month().abbreviation());
		out.push(' ');
		out.push_str(self.year());
		out.push(' ');
		out.push_str(self.time());
		if let Some(offset) = offset_suffix(self.offset) {
			out.push(' ');
			out.push_str(DateChar::as_str(offset.as_slice()));
		}
		else { out.push_str(" +0000"); }

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
	/// use utc2k::{Local2k, Utc2k};
	///
	/// // A proper UTC date in RFC3339.
	/// let utc = Utc2k::new(2021, 12, 13, 11, 56, 1);
	/// assert_eq!(utc.to_rfc3339(), "2021-12-13T11:56:01Z");
	///
	/// // The same date localized to, say, California.
	/// let local = Local2k::from(utc).formatted();
	/// # let local = Local2k::fixed_from_utc2k(utc, -28800).formatted();
	/// assert_eq!(local.to_rfc3339(), "2021-12-13T03:56:01-0800");
	/// ```
	///
	/// The RFC3339 date/time format is portable, whether local or UTC.
	///
	/// ```
	/// # use utc2k::{Local2k, Utc2k};
	/// # let utc = Utc2k::new(2021, 12, 13, 11, 56, 1);
	/// # let local = Local2k::from(utc).formatted();
	/// let utc_3339 = utc.to_rfc3339();
	/// let local_3339 = local.to_rfc3339();
	///
	/// // The RFC3339 representations will vary if there's an offset, but
	/// // if parsed back into a Utc2k, that'll get sorted and they'll match!
	/// assert_eq!(
	///     Utc2k::from_ascii(utc_3339.as_bytes()),
	///     Some(utc),
	/// );
	/// assert_eq!(
	///     Utc2k::from_ascii(local_3339.as_bytes()),
	///     Some(utc),
	/// );
	/// ```
	pub fn to_rfc3339(&self) -> String {
		let mut out = String::with_capacity(if self.offset.is_some() { 24 } else { 20 });
		out.push_str(self.date());
		out.push('T');
		out.push_str(self.time());
		if let Some(offset) = offset_suffix(self.offset) {
			out.push_str(DateChar::as_str(offset.as_slice()));
		}
		else { out.push('Z'); }
		out
	}
}

/// ## Internal.
impl FmtLocal2k {
	#[must_use]
	/// # From [`Local2k`].
	const fn from_local2k(src: Local2k) -> Self {
		Self {
			inner: FmtUtc2k::from_utc2k(src.inner),
			offset: src.offset,
		}
	}
}



#[derive(Debug, Clone, Copy)]
/// # Local ~~UTC~~2K.
///
/// This struct brings barebones locale awareness to [`Utc2k`], allowing
/// date/time digits to be carved up according to the user's local time zone
/// instead of the usual UTC.
///
/// Time zone detection is automatic, but only supported on unix platforms.
/// If the lookup fails or the user is running something weird like Windows,
/// it'll stick with UTC.
///
/// UTC is also used in cases where the local offset would cause the date/time
/// to be clamped to the `2000..=2099` range. (This is only applicable to the
/// first and final hours of the century, so shouldn't come up very often!)
///
/// To keep things simple, `Local2k` is effectively read-only, requiring
/// [`Utc2k`] as a go-between for both [instantiation](Local2k::from_utc2k)
/// and [modification](Local2k::to_utc2k), except for a few convenience methods
/// like [`Local2k::now`], [`Local2k::tomorrow`], and [`Local2k::yesterday`].
///
/// Note that offsets, or the lack thereof, have no effect on date/time
/// equality, hashing, or ordering. `Local2k` objects can be freely compared
/// with one another and/or [`Utc2k`] date/times.
pub struct Local2k {
	/// # Date/Time (w/ `offset`)
	inner: Utc2k,

	/// # Local Offset (Seconds).
	offset: Option<NonZeroI32>,
}

impl fmt::Display for Local2k {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		<FmtLocal2k as fmt::Display>::fmt(&FmtLocal2k::from_local2k(*self), f)
	}
}

impl Eq for Local2k {}

impl From<&FmtLocal2k> for Local2k {
	#[inline]
	fn from(src: &FmtLocal2k) -> Self { Self::from_fmtlocal2k(*src) }
}

impl From<FmtLocal2k> for Local2k {
	#[inline]
	fn from(src: FmtLocal2k) -> Self { Self::from_fmtlocal2k(src) }
}

impl From<&Utc2k> for Local2k {
	#[inline]
	fn from(src: &Utc2k) -> Self { Self::from_utc2k(*src) }
}

impl From<Utc2k> for Local2k {
	#[inline]
	fn from(src: Utc2k) -> Self { Self::from_utc2k(src) }
}

impl From<Local2k> for String {
	#[inline]
	fn from(src: Local2k) -> Self { Self::from(FmtLocal2k::from_local2k(src)) }
}

impl From<&Local2k> for Utc2k {
	#[inline]
	fn from(src: &Local2k) -> Self { src.to_utc2k() }
}

impl From<Local2k> for Utc2k {
	#[inline]
	fn from(src: Local2k) -> Self { src.to_utc2k() }
}

impl hash::Hash for Local2k {
	#[inline]
	fn hash<H: hash::Hasher>(&self, state: &mut H) {
		<Utc2k as hash::Hash>::hash(&self.to_utc2k(), state);
	}
}

impl Ord for Local2k {
	#[inline]
	fn cmp(&self, other: &Self) -> Ordering {
		if self.offset == other.offset { self.inner.cmp(&other.inner) }
		else { self.unixtime().cmp(&other.unixtime()) }
	}
}

impl PartialEq for Local2k {
	#[inline]
	fn eq(&self, other: &Self) -> bool {
		if self.offset == other.offset { self.inner == other.inner }
		else { self.unixtime() == other.unixtime() }
	}
}

impl PartialEq<Utc2k> for Local2k {
	#[inline]
	/// # Cross-Offset Equality.
	///
	/// Local and UTC dates are compared as unix timestamps, so should always
	/// match up.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::{Local2k, Utc2k};
	///
	/// let utc = Utc2k::new(2025, 1, 1, 0, 0, 0);
	/// let local = Local2k::from(utc);
	/// assert_eq!(utc, local);
	///
	/// // String representations, however, will only be equal if there's
	/// // no offset.
	/// assert_eq!(
	///     utc.to_string() == local.to_string(),
	///     local.offset().is_none(),
	/// );
	/// ```
	fn eq(&self, other: &Utc2k) -> bool { self.unixtime() == other.unixtime() }
}
impl PartialEq<Local2k> for Utc2k {
	#[inline]
	fn eq(&self, other: &Local2k) -> bool { <Local2k as PartialEq<Self>>::eq(other, self) }
}

impl PartialOrd for Local2k {
	#[inline]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

/// # Instantiation.
impl Local2k {
	#[must_use]
	/// # From UTC.
	///
	/// Convert a UTC date/time into a local one.
	///
	/// Refer to the main [`Local2k`] for limitations and gotchas.
	pub fn from_utc2k(src: Utc2k) -> Self {
		// If we have an offset, we need to do some things.
		let unixtime = src.unixtime();

		// Is there an offset?
		if let Some(offset) = unixtime_offset(unixtime) {
			let localtime = unixtime.saturating_add_signed(offset.get());
			if (Utc2k::MIN_UNIXTIME..=Utc2k::MAX_UNIXTIME).contains(&localtime) {
				return Self {
					inner: Utc2k::from_unixtime(localtime),
					offset: Some(offset),
				};
			}
		}

		// Keep it UTC.
		Self { inner: src, offset: None }
	}

	#[doc(hidden)]
	#[must_use]
	/// # From UTC w/ Fixed Offset.
	///
	/// Same as [`Local2k::from_utc2k`], but localized with a fixed offset
	/// instead of the system one.
	///
	/// Offsets can be positive or negative, but must break down evenly into
	/// hours and/or minutes, and must be (absolutely) less than one day.
	///
	/// Note: this method is used internally for debugging/testing and is not
	/// intended for broader use.
	///
	/// ```
	/// use std::num::NonZeroI32;
	/// use utc2k::{Local2k, Utc2k};
	///
	/// let one = Local2k::now();
	/// let two = Local2k::fixed_from_utc2k(
	///     one.to_utc2k(),
	///     one.offset().map_or(0, NonZeroI32::get),
	/// );
	///
	/// assert_eq!(one, two);
	/// ```
	pub fn fixed_from_utc2k(src: Utc2k, offset: i32) -> Self {
		// If we have an offset, we need to do some things.
		let unixtime = src.unixtime();

		// Is there an offset?
		if let Some(offset) = nonzero_offset(offset) {
			let localtime = unixtime.saturating_add_signed(offset.get());
			if (Utc2k::MIN_UNIXTIME..=Utc2k::MAX_UNIXTIME).contains(&localtime) {
				return Self {
					inner: Utc2k::from_unixtime(localtime),
					offset: Some(offset),
				};
			}
		}

		// Keep it UTC.
		Self { inner: src, offset: None }
	}

	#[inline]
	#[must_use]
	/// # Now.
	///
	/// Create a new instance representing the current local time.
	///
	/// ```
	/// use utc2k::{Local2k, Utc2k};
	///
	/// // Equivalent.
	/// assert_eq!(
	///     Local2k::now(),
	///     Local2k::from(Utc2k::now()),
	/// );
	/// ```
	pub fn now() -> Self { Self::from_utc2k(Utc2k::now()) }

	#[inline]
	#[must_use]
	/// # Tomorrow.
	///
	/// Create a new instance representing one day from now (present time).
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::{Local2k, Utc2k};
	///
	/// // Equivalent.
	/// assert_eq!(
	///     Local2k::tomorrow(),
	///     Local2k::from(Utc2k::tomorrow()),
	/// );
	/// ```
	pub fn tomorrow() -> Self { Self::from_utc2k(Utc2k::tomorrow()) }

	#[inline]
	#[must_use]
	/// # Yesterday.
	///
	/// Create a new instance representing one day ago (present time).
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::{Local2k, Utc2k};
	///
	/// // Equivalent.
	/// assert_eq!(
	///     Local2k::yesterday(),
	///     Local2k::from(Utc2k::yesterday()),
	/// );
	/// ```
	pub fn yesterday() -> Self { Self::from_utc2k(Utc2k::yesterday()) }
}

/// # Conversion.
impl Local2k {
	#[inline]
	#[must_use]
	/// # Formatted.
	///
	/// This returns a [`FmtLocal2k`] and is equivalent to calling
	/// `FmtLocal2k::from(self)`.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::{FmtLocal2k, Local2k, Utc2k};
	///
	/// let utc = Utc2k::new(2010, 5, 15, 16, 30, 1);
	/// let local = Local2k::from(utc);
	/// assert_eq!(local.formatted(), FmtLocal2k::from(local));
	/// ```
	pub const fn formatted(self) -> FmtLocal2k { FmtLocal2k::from_local2k(self) }

	#[must_use]
	/// # To RFC2822.
	///
	/// Return a string formatted according to [RFC2822](https://datatracker.ietf.org/doc/html/rfc2822).
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::{Local2k, Utc2k};
	///
	/// // A proper UTC date in RFC2822.
	/// let utc = Utc2k::new(2021, 12, 13, 04, 56, 1);
	/// assert_eq!(
	///     utc.to_rfc2822(),
	///     "Mon, 13 Dec 2021 04:56:01 +0000",
	/// );
	///
	/// // The same date localized to, say, California.
	/// let local = Local2k::from(utc);
	/// # let local = Local2k::fixed_from_utc2k(utc, -28800);
	/// assert_eq!(
	///     local.to_rfc2822(),
	///     "Sun, 12 Dec 2021 20:56:01 -0800",
	/// );
	/// ```
	///
	/// The RFC2822 date/time format is portable, whether local or UTC.
	///
	/// ```
	/// # use utc2k::{Local2k, Utc2k};
	/// # let utc = Utc2k::new(2003, 7, 1, 10, 52, 37);
	/// # let local = Local2k::from(utc);
	/// let utc_2822 = utc.to_rfc2822();
	/// let local_2822 = local.to_rfc2822();
	///
	/// // The RFC2822 representations will vary if there's an offset, but
	/// // if parsed back into a Utc2k, that'll get sorted and they'll match!
	/// assert_eq!(
	///     Utc2k::from_rfc2822(utc_2822.as_bytes()),
	///     Some(utc),
	/// );
	/// assert_eq!(
	///     Utc2k::from_rfc2822(local_2822.as_bytes()),
	///     Some(utc),
	/// );
	/// ```
	pub fn to_rfc2822(&self) -> String {
		let mut out = String::with_capacity(31);

		out.push_str(self.weekday().abbreviation());
		out.push_str(", ");
		out.push_str(DateChar::dd_str(self.inner.d));
		out.push(' ');
		out.push_str(self.month().abbreviation());
		out.push_str(" 20");
		out.push_str(DateChar::as_str(self.inner.y.dd().as_slice()));
		out.push(' ');
		out.push_str(DateChar::dd_str(self.inner.hh));
		out.push(':');
		out.push_str(DateChar::dd_str(self.inner.mm));
		out.push(':');
		out.push_str(DateChar::dd_str(self.inner.ss));

		if let Some(offset) = offset_suffix(self.offset) {
			out.push(' ');
			out.push_str(DateChar::as_str(offset.as_slice()));
		}
		else { out.push_str(" +0000"); }

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
	/// use utc2k::{Local2k, Utc2k};
	///
	/// // A proper UTC date in RFC3339.
	/// let utc = Utc2k::new(2021, 12, 13, 11, 56, 1);
	/// assert_eq!(utc.to_rfc3339(), "2021-12-13T11:56:01Z");
	///
	/// // The same date localized to, say, California.
	/// let local = Local2k::from(utc);
	/// # let local = Local2k::fixed_from_utc2k(utc, -28800);
	/// assert_eq!(local.to_rfc3339(), "2021-12-13T03:56:01-0800");
	/// ```
	///
	/// The RFC3339 date/time format is portable, whether local or UTC.
	///
	/// ```
	/// # use utc2k::{Local2k, Utc2k};
	/// # let utc = Utc2k::new(2021, 12, 13, 11, 56, 1);
	/// # let local = Local2k::from(utc);
	/// let utc_3339 = utc.to_rfc3339();
	/// let local_3339 = local.to_rfc3339();
	///
	/// // The RFC3339 representations will vary if there's an offset, but
	/// // if parsed back into a Utc2k, that'll get sorted and they'll match!
	/// assert_eq!(
	///     Utc2k::from_ascii(utc_3339.as_bytes()),
	///     Some(utc),
	/// );
	/// assert_eq!(
	///     Utc2k::from_ascii(local_3339.as_bytes()),
	///     Some(utc),
	/// );
	/// ```
	pub fn to_rfc3339(&self) -> String {
		FmtLocal2k::from_local2k(*self).to_rfc3339()
	}

	#[must_use]
	/// # Into UTC.
	///
	/// Convert a local date/time back into UTC one.
	///
	/// ```
	/// use utc2k::{Utc2k, Local2k};
	///
	/// let utc = Utc2k::now();
	/// let local = Local2k::from(utc);
	/// assert_eq!(
	///     local.to_utc2k(),
	///     utc,
	/// );
	/// ```
	pub const fn to_utc2k(&self) -> Utc2k {
		if let Some(offset) = self.offset {
			let (y, m, d, hh, mm, ss) = self.parts();
			Utc2k::from_abacus(Abacus::new_with_offset(y, m, d, hh, mm, ss, offset.get()))
		}
		else { self.inner }
	}

	#[inline]
	#[must_use]
	/// # Unixtime.
	///
	/// Return the (original) unix timestamp used to create this instance.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::{Local2k, Utc2k};
	///
	/// let utc = Utc2k::from_unixtime(1_434_765_671_u32);
	/// let local = Local2k::from(utc);
	///
	/// assert_eq!(utc.unixtime(),   1_434_765_671);
	/// assert_eq!(local.unixtime(), 1_434_765_671, "local {:?}", local.offset());
	/// ```
	pub const fn unixtime(&self) -> u32 {
		let unixtime = self.inner.unixtime();
		if let Some(offset) = self.offset {
			unixtime.saturating_add_signed(0 - offset.get())
		}
		else { unixtime }
	}
}

/// # Get Parts.
impl Local2k {
	#[inline]
	#[must_use]
	/// # Is UTC?
	///
	/// Returns `true` if there is no offset applied to the "local" date/time.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::Local2k;
	///
	/// let date = Local2k::now();
	/// assert_eq!(
	///     date.is_utc(),
	///     date.offset().is_none(),
	/// );
	/// ```
	pub const fn is_utc(&self) -> bool { self.offset.is_none() }

	#[inline]
	#[must_use]
	/// # Offset.
	///
	/// Return the UTC offset in seconds, if any.
	///
	/// ## Examples
	///
	/// ```
	/// use std::num::NonZeroI32;
	/// use utc2k::{Local2k, Utc2k};
	///
	/// let utc = Utc2k::new(2005, 1, 1, 12, 0, 0);
	/// let local = Local2k::from(utc);
	/// # let local = Local2k::fixed_from_utc2k(utc, -28800);
	/// assert_eq!(
	///     local.offset(),
	///     NonZeroI32::new(-28_800), // e.g. California.
	/// );
	///
	/// // Don't forget about Daylight Saving! 🕱
	/// let utc = Utc2k::new(2005, 6, 1, 12, 0, 0);
	/// let local = Local2k::from(utc);
	/// # let local = Local2k::fixed_from_utc2k(utc, -25200);
	/// assert_eq!(
	///     local.offset(),
	///     NonZeroI32::new(-25_200), // e.g. California.
	/// );
	///
	/// // Remember, 1999 and 2100 DO NOT EXIST. To prevent a loss of
	/// // precision, UTC will be used to represent the first or final hours
	/// // of the century to prevent precision loss.
	/// let local = Local2k::from(Utc2k::MIN);
	/// # let local = Local2k::fixed_from_utc2k(Utc2k::MIN, -28800);
	/// assert!(local.offset().is_none()); // Can't apply a -0800 offset
	///                                    // without leaving the century!
	///
	/// let local = Local2k::from(Utc2k::MAX);
	/// # let local = Local2k::fixed_from_utc2k(Utc2k::MAX, -28800);
	/// assert!(local.offset().is_some()); // The -0800 is no problem on the
	///                                    // other end, though.
	/// # // In Moscow, it'd be the other way around.
	/// # let local = Local2k::fixed_from_utc2k(Utc2k::MIN, 14400);
	/// # assert_eq!(local.offset(), NonZeroI32::new(14400));
	/// # let local = Local2k::fixed_from_utc2k(Utc2k::MAX, 14400);
	/// # assert!(local.offset().is_none());
	/// ```
	pub const fn offset(&self) -> Option<NonZeroI32> { self.offset }

	#[inline]
	#[must_use]
	/// # Parts.
	///
	/// Return the individual numerical components of the datetime, from years
	/// down to seconds.
	///
	/// Alternatively, if you only want the date bits, use [`Local2k::ymd`], or
	/// if you only want the time bits, use [`Local2k::hms`].
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::{Local2k, Utc2k};
	///
	/// let utc = Utc2k::new(2010, 5, 4, 16, 30, 1);
	/// assert_eq!(
	///     utc.parts(),
	///     (2010, 5, 4, 16, 30, 1),
	/// );
	///
	/// let local = Local2k::from(utc);
	/// # let local = Local2k::fixed_from_utc2k(utc, -25200);
	/// assert_eq!(
	///     local.parts(),
	///     (2010, 5, 4, 9, 30, 1), // e.g. California.
	/// //               ^ -0700
	/// );
	/// ```
	pub const fn parts(&self) -> (u16, u8, u8, u8, u8, u8) { self.inner.parts() }

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
	/// use utc2k::{Local2k, Utc2k};
	///
	/// let utc = Utc2k::new(2010, 5, 5, 16, 30, 1);
	/// let local = Local2k::from(utc);
	/// # let local = Local2k::fixed_from_utc2k(utc, -25200);
	/// assert_eq!(local.ymd(), (2010, 5, 5)); // e.g. California.
	/// ```
	pub const fn ymd(&self) -> (u16, u8, u8) { self.inner.ymd() }

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
	/// use utc2k::{Local2k, Utc2k};
	///
	/// let utc = Utc2k::new(2010, 5, 5, 16, 30, 1);
	/// let local = Local2k::from(utc);
	/// # let local = Local2k::fixed_from_utc2k(utc, -25200);
	/// assert_eq!(local.hms(), (9, 30, 1)); // e.g. California.
	/// ```
	pub const fn hms(&self) -> (u8, u8, u8) { self.inner.hms() }

	#[inline]
	#[must_use]
	/// # Year.
	///
	/// This returns the year value.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::{Local2k, Utc2k};
	///
	/// let utc = Utc2k::new(2010, 5, 15, 16, 30, 1);
	/// let local = Local2k::from(utc);
	/// # let local = Local2k::fixed_from_utc2k(utc, -25200);
	/// assert_eq!(local.year(), 2010);
	/// ```
	pub const fn year(&self) -> u16 { self.inner.year() }

	#[inline]
	#[must_use]
	/// # Month (enum).
	///
	/// This returns the month value as a [`Month`].
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::{Local2k, Month, Utc2k};
	///
	/// let utc = Utc2k::new(2010, 5, 15, 16, 30, 1);
	/// let local = Local2k::from(utc);
	/// # let local = Local2k::fixed_from_utc2k(utc, -25200);
	/// assert_eq!(local.month(), Month::May);
	/// ```
	pub const fn month(&self) -> Month { self.inner.month() }

	#[inline]
	#[must_use]
	/// # Day.
	///
	/// This returns the day value.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::{Local2k, Utc2k};
	///
	/// let utc = Utc2k::new(2010, 5, 15, 16, 30, 1);
	/// let local = Local2k::from(utc);
	/// # let local = Local2k::fixed_from_utc2k(utc, -25200);
	/// assert_eq!(local.day(), 15);
	/// ```
	pub const fn day(&self) -> u8 { self.inner.day() }

	#[inline]
	#[must_use]
	/// # Hour.
	///
	/// This returns the hour value.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::{Local2k, Utc2k};
	///
	/// let utc = Utc2k::new(2010, 5, 15, 16, 30, 1);
	/// let local = Local2k::from(utc);
	/// # let local = Local2k::fixed_from_utc2k(utc, -25200);
	/// assert_eq!(local.hour(), 9); // e.g. California.
	/// ```
	pub const fn hour(&self) -> u8 { self.inner.hour() }

	#[inline]
	#[must_use]
	/// # Minute.
	///
	/// This returns the minute value.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::{Local2k, Utc2k};
	///
	/// let utc = Utc2k::new(2010, 5, 15, 16, 30, 1);
	/// let local = Local2k::from(utc);
	/// # let local = Local2k::fixed_from_utc2k(utc, -25200);
	/// assert_eq!(local.minute(), 30);
	/// ```
	pub const fn minute(&self) -> u8 { self.inner.minute() }

	#[inline]
	#[must_use]
	/// # Second.
	///
	/// This returns the second value.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::{Local2k, Utc2k};
	///
	/// let utc = Utc2k::new(2010, 5, 15, 16, 30, 1);
	/// let local = Local2k::from(utc);
	/// # let local = Local2k::fixed_from_utc2k(utc, -25200);
	/// assert_eq!(local.second(), 1);
	/// ```
	pub const fn second(&self) -> u8 { self.inner.second() }
}

/// ## Other Getters.
impl Local2k {
	#[inline]
	#[must_use]
	/// # Is Leap Year?
	///
	/// This returns `true` if this date is/was in a leap year.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::{Local2k, Utc2k};
	///
	/// let date = Local2k::from(
	///     Utc2k::try_from("2020-05-10").unwrap()
	/// );
	/// assert!(date.leap_year());
	///
	/// let date = Local2k::from(
	///     Utc2k::try_from("2021-03-15").unwrap()
	/// );
	/// assert!(! date.leap_year());
	/// ```
	pub const fn leap_year(&self) -> bool { self.inner.leap_year() }

	#[inline]
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
	/// use utc2k::{Local2k, Utc2k};
	///
	/// let utc = Utc2k::new(2021, 7, 8, 0, 0, 0);
	/// let local = Local2k::from(utc);
	/// assert_eq!(local.month_size(), 31);
	///
	/// let utc = Utc2k::new(2020, 2, 20, 0, 0, 0);
	/// let local = Local2k::from(utc);
	/// assert_eq!(local.month_size(), 29); // Leap!
	/// ```
	pub const fn month_size(&self) -> u8 { self.inner.month_size() }

	#[inline]
	#[must_use]
	/// # Ordinal.
	///
	/// Return the day-of-year value. This will be between `1..=365` (or `1..=366`
	/// for leap years).
	///
	/// ## Examples
	///
	/// ```no_run
	/// use utc2k::{Local2k, Utc2k};
	///
	/// let utc = Utc2k::new(2020, 5, 10, 12, 0, 0);
	/// let local = Local2k::from(utc);
	/// # let local = Local2k::fixed_from_utc2k(utc, -25200);
	/// assert_eq!(local.ordinal(), 131);
	/// ```
	pub const fn ordinal(&self) -> u16 { self.inner.ordinal() }

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
	/// use utc2k::{DAY_IN_SECONDS, Local2k, Utc2k};
	///
	/// let utc = Utc2k::new(2010, 11, 01, 0, 0, 0);
	/// assert_eq!(utc.seconds_from_midnight(), 0); // It _is_ midnight!
	///
	/// // In California, though, it's still Halloween!
	/// let local = Local2k::from(utc);
	/// # let local = Local2k::fixed_from_utc2k(utc, -28800);
	/// assert_eq!(
	///     local.parts(),
	///     (2010, 10, 31, 16, 0, 0),
	/// );
	///
	/// // The distance from _its_ midnight is very different!
	/// assert_eq!(local.seconds_from_midnight(), 57_600);
	/// ```
	pub const fn seconds_from_midnight(&self) -> u32 {
		self.inner.seconds_from_midnight()
	}

	#[inline]
	#[must_use]
	/// # Weekday.
	///
	/// Return the [`Weekday`] corresponding to the given date.
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::{Local2k, Utc2k, Weekday};
	///
	/// let utc = Utc2k::new(2021, 7, 8, 5, 22, 1);
	/// assert_eq!(utc.weekday(), Weekday::Thursday);
	///
	/// // Local date/times may differ. In California, for example, it'd
	/// // still be the night before.
	/// let local = Local2k::from(utc);
	/// # let local = Local2k::fixed_from_utc2k(utc, -25200);
	/// assert_eq!(local.weekday(), Weekday::Wednesday);
	/// ```
	pub const fn weekday(&self) -> Weekday { self.inner.weekday() }
}

/// ## Internal Helpers.
impl Local2k {
	#[must_use]
	/// # From `FmtLocal2k`.
	const fn from_fmtlocal2k(src: FmtLocal2k) -> Self {
		Self {
			inner: Utc2k::from_fmtutc2k(src.inner),
			offset: src.offset,
		}
	}
}



#[expect(clippy::cast_possible_truncation, reason = "False positive.")]
/// # Offset Suffix.
///
/// Convert an offset back to `±hhmm` format.
const fn offset_suffix(offset: Option<NonZeroI32>) -> Option<[DateChar; 5]> {
	if let Some(offset) = offset {
		let sign =
			if offset.get() < 0 { DateChar::Dash }
			else { DateChar::Plus };

		let offset = offset.get().unsigned_abs();

		let hh = DateChar::dd(offset.wrapping_div(HOUR_IN_SECONDS) as u8);
		let mm = DateChar::dd((offset % HOUR_IN_SECONDS).wrapping_div(MINUTE_IN_SECONDS) as u8);

		Some([sign, hh[0], hh[1], mm[0], mm[1]])
	}
	else { None }
}

#[expect(clippy::cast_possible_wrap, reason = "False positive.")]
/// # Sanitize Offset.
///
/// Strip multi-day bullshit, make sure it is a multiple of sixty, and return
/// if nonzero.
const fn nonzero_offset(offset: i32) -> Option<NonZeroI32> {
	let offset = offset % DAY_IN_SECONDS as i32;
	if offset.unsigned_abs().is_multiple_of(MINUTE_IN_SECONDS) {
		NonZeroI32::new(offset)
	}
	else { None }
}

#[inline]
#[must_use]
/// # Offset From Unixtime.
///
/// Return the local offset details for a given UTC date/time, ensuring it is
/// less than a day (absolutely) and limited to hour/minute precision.
///
/// The local time zone details are cached on the first call; subsequent runs
/// should be much faster.
fn unixtime_offset(unixtime: u32) -> Option<NonZeroI32> {
	TZ.get_or_init(|| TimeZone::local().ok())
		.as_ref()
		.and_then(|tz|
			tz.find_local_time_type(i64::from(unixtime))
				.ok()
				.and_then(|tz| nonzero_offset(tz.ut_offset()))
		)
}
