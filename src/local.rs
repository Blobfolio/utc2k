/*!
# Utc2k: Local Offsets
*/

use crate::{
	FmtUtc2k,
	Utc2k,
};
use std::ops::Neg;
use tz::timezone::{
	LocalTimeType,
	TimeZone,
};



#[cfg_attr(feature = "docsrs", doc(cfg(feature = "local")))]
#[derive(Debug, Clone, Copy, Default, Eq, Hash, PartialEq)]
/// # Local Offset.
///
/// This struct attempts to determine the appropriate UTC offset for the local
/// timezone in a thread-safe manner.
///
/// This currently **only works for unix systems**. For everybody else, it will act
/// as if there is no local offset (i.e. as if it were UTC).
///
/// ## Examples
///
/// To obtain the _current_ local offset, use [`LocalOffset::now`].
///
/// To obtain the local offset for a specific unix timestamp, use its `From<u32>`
/// implementation instead.
///
/// ```
/// use utc2k::LocalOffset;
///
/// // The current offset.
/// let now = LocalOffset::now();
///
/// // The offset for a specific time.
/// let then = LocalOffset::from(946_684_800_u32);
/// ```
///
/// If all you want is the offset, you can grab that by calling
/// [`LocalOffset::offset`] afterward.
///
/// This struct can, however, also be used to _trick_ [`FmtUtc2k`] and [`Utc2k`]
/// into representing _local_ rather than UTC datetimes by using their
/// `From<LocalOffset>` implementations, or the [`FmtUtc2k::now_local`]/[`Utc2k::now_local`]
/// shorthands.
///
/// If you do this, however, it is worth noting that comparing tricked and
/// untricked objects makes no logical sense, so you shouldn't mix-and-match if
/// you need to test for equality or ordering.
///
/// Additionally, you should avoid localizing a datetime if you plan on using the
/// `to_rfc2822` or `to_rfc3339` formatting helpers. They always assume they're
/// representing a UTC timestamp, so will add the wrong suffix to their output
/// if your local offset is non-zero.
///
/// Other than that, the trick works perfectly well. ;)
///
/// ```
/// use utc2k::{LocalOffset, Utc2k};
///
/// let now_utc = Utc2k::now();
///
/// // These two are equivalent.
/// let now_local = Utc2k::from(LocalOffset::now());
/// let now_local = Utc2k::now_local();
/// ```
///
/// If you need to convert from a local timestamp into a UTC one, you can
/// leverage [`std::ops::Neg`] to invert the offset, like:
///
/// ```
/// use utc2k::{LocalOffset, Utc2k};
///
/// let offset = LocalOffset::from(946_684_800_u32);
/// let utc = Utc2k::from(-offset);
/// ```
pub struct LocalOffset {
	unixtime: u32,
	offset: i32,
}

impl From<u32> for LocalOffset {
	#[inline]
	fn from(unixtime: u32) -> Self {
		let offset = offset(unixtime);
		Self { unixtime, offset }
	}
}

impl Neg for LocalOffset {
	type Output = Self;

	fn neg(self) -> Self::Output {
		if self.offset == i32::MIN {
			Self {
				unixtime: self.unixtime,
				offset: i32::MAX,
			}
		}
		else {
			Self {
				unixtime: self.unixtime,
				offset: self.offset.wrapping_neg(),
			}
		}
	}
}

impl LocalOffset {
	#[inline]
	#[must_use]
	/// # Now.
	///
	/// Return the current, local offset. If no offset can be determined, UTC
	/// will be assumed.
	///
	/// ## Examples
	///
	/// ```
	/// let now = utc2k::LocalOffset::now();
	/// ```
	pub fn now() -> Self { Self::from(crate::unixtime()) }

	#[inline]
	#[must_use]
	/// # Offset.
	///
	/// Return the local offset (in seconds). Zero is returned if there is no
	/// offset, or no offset could be determined.
	pub const fn offset(self) -> i32 { self.offset }
}

impl From<LocalOffset> for FmtUtc2k {
	fn from(src: LocalOffset) -> Self { Self::from(Utc2k::from(src)) }
}

impl From<LocalOffset> for Utc2k {
	fn from(src: LocalOffset) -> Self {
		let abs = src.offset.abs() as u32;

		if src.offset < 0 { Self::from(src.unixtime.saturating_sub(abs)) }
		else { Self::from(src.unixtime.saturating_add(abs)) }
	}
}



#[cfg(not(feature = "local_cache"))]
/// # Offset From Time.
fn offset(now: u32) -> i32 {
	if let Ok(x) = TimeZone::local() {
		x.find_local_time_type(i64::from(now)).map_or(0, LocalTimeType::ut_offset)
	}
	else { 0 }
}

#[cfg(feature = "local_cache")]
/// # Offset From Time.
///
/// This version caches the parsed timezone information, allowing for much
/// faster repeated use.
fn offset(now: u32) -> i32 {
	use once_cell::sync::Lazy;
	static TZ: Lazy<Option<TimeZone>> = Lazy::new(|| TimeZone::local().ok());

	TZ.as_ref().map_or(0, |x|
		x.find_local_time_type(i64::from(now)).map_or(0, LocalTimeType::ut_offset)
	)
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn neg() {
		let off = LocalOffset::now();
		let off2 = -off;
		assert_eq!(off, -off2); // We should be back to the original.
	}

	#[test]
	fn now() {
		// Unless we're one second away from a DST-type change, the offsets
		// should match!
		let now = crate::unixtime();
		assert_eq!(LocalOffset::now().offset, LocalOffset::from(now).offset);
	}
}
