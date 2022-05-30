/*!
# Utc2k: Local Offsets
*/

use crate::{
	FmtUtc2k,
	Utc2k,
};
use once_cell::sync::OnceCell;
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
/// timezone in a thread-safe manner, but **only for unix systems**.
///
/// Instantiation will never fail, though.
///
/// If the platform isn't supported or no offset can be determined, the
/// "offset" will simply be zero (i.e. as if it were UTC).
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

impl From<Utc2k> for LocalOffset {
	#[inline]
	/// # From `Utc2k`
	///
	/// Warning: this should only be used for `Utc2k` instances holding honest
	/// UTC datetimes. If you call this on a tricked/local instance, the offset
	/// will get applied twice!
	fn from(src: Utc2k) -> Self { Self::from(src.unixtime()) }
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

	#[must_use]
	/// # Local Timestamp.
	///
	/// Return the sum of the unix timestamp and the offset.
	pub const fn localtime(self) -> u32 {
		if self.offset < 0 {
			self.unixtime.saturating_sub(self.offset.abs() as u32)
		}
		else {
			self.unixtime.saturating_add(self.offset.abs() as u32)
		}
	}

	#[inline]
	#[must_use]
	/// # Offset.
	///
	/// Return the local offset (in seconds). Zero is returned if there is no
	/// offset, or no offset could be determined.
	pub const fn offset(self) -> i32 { self.offset }

	#[inline]
	#[must_use]
	/// # Unixtime.
	///
	/// Return the unix timestamp this instance applies to (i.e. the value it
	/// was seeded with).
	///
	/// ## Examples
	///
	/// ```
	/// use utc2k::LocalOffset;
	///
	/// let offset = LocalOffset::from(946_684_800_u32);
	/// assert_eq!(offset.unixtime(), 946_684_800_u32);
	/// ```
	pub const fn unixtime(self) -> u32 { self.unixtime }
}

impl From<LocalOffset> for i32 {
	fn from(src: LocalOffset) -> Self { src.offset }
}

impl From<LocalOffset> for FmtUtc2k {
	fn from(src: LocalOffset) -> Self { Self::from(Utc2k::from(src)) }
}

impl From<LocalOffset> for Utc2k {
	#[inline]
	fn from(src: LocalOffset) -> Self { Self::from(src.localtime()) }
}



/// # Parsed Timezone Details.
static TZ: OnceCell<TimeZone> = OnceCell::new();

/// # Offset From Unixtime.
///
/// The local timezone details are cached on the first run; subsequent method
/// calls will perform much faster.
fn offset(now: u32) -> i32 {
	TZ.get_or_init(|| TimeZone::local().unwrap_or_else(|_| TimeZone::utc()))
		.find_local_time_type(i64::from(now))
		.map_or(0, LocalTimeType::ut_offset)
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
