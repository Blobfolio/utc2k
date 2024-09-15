/*!
# UTC2K - Abacus
*/

use crate::{
	DAY_IN_SECONDS,
	HOUR_IN_SECONDS,
	MINUTE_IN_SECONDS,
	Utc2k,
};
use std::ops::{
	Add,
	AddAssign,
};



#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
/// # Abacus.
///
/// This is essentially a 32-bit version of [`Utc2k`], which allows individual
/// date/time parts to temporarily hold large values so that they can be
/// "rebalanced" without worrying about overflow.
///
/// This is used by [`Utc2k`] when dealing with parts to ensure the unit values
/// make sense, for example during string parsing or addition operations. If
/// 1000 seconds are added to a time, that's totally fine, but we don't want
/// 1000 seconds; we want 16 minutes and 40 seconds.
///
/// Because this is a transient struct, the functionality could be represented
/// as a top-level function or something instead, but this approach appears to
/// be faster.
pub(super) struct Abacus {
	/// # Year.
	y: u32,

	/// # Month.
	m: u32,

	/// # Day.
	d: u32,

	/// # Hour.
	hh: u32,

	/// # Minute.
	mm: u32,

	/// # Second.
	ss: u32,
}

impl Add<u32> for Abacus {
	type Output = Self;
	fn add(self, other: u32) -> Self {
		let mut out = Self {
			y: self.y,
			m: self.m,
			d: self.d,
			hh: self.hh,
			mm: self.mm,
			ss: self.ss.saturating_add(other),
		};
		out.rebalance();
		out
	}
}

impl AddAssign<u32> for Abacus {
	#[inline]
	fn add_assign(&mut self, other: u32) {
		self.ss = self.ss.saturating_add(other);
		self.rebalance();
	}
}

impl From<Utc2k> for Abacus {
	#[inline]
	fn from(src: Utc2k) -> Self {
		let (y, m, d, hh, mm, ss) = src.parts();
		Self::new(y, m, d, hh, mm, ss)
	}
}

impl Abacus {
	#[must_use]
	/// # New.
	pub(super) fn new(y: u16, m: u8, d: u8, hh: u8, mm: u8, ss: u8) -> Self {
		let mut out = Self {
			y: y.into(),
			m: m.into(),
			d: d.into(),
			hh: hh.into(),
			mm: mm.into(),
			ss: ss.into(),
		};
		out.rebalance();
		out
	}

	#[expect(clippy::cast_possible_truncation, reason = "False positive.")]
	#[must_use]
	/// # Parts.
	///
	/// Return the individual parts, nice and balanced, ready for consumption
	/// by [`Utc2k`]. (Only the last two digits of the year are returned.)
	pub(super) const fn parts(&self) -> (u8, u8, u8, u8, u8, u8) {
		if self.y < 2000 { (0, 1, 1, 0, 0, 0) }
		else if 2099 < self.y { (99, 12, 31, 23, 59, 59) }
		else {
			(
				(self.y - 2000) as u8,
				self.m as u8,
				self.d as u8,
				self.hh as u8,
				self.mm as u8,
				self.ss as u8,
			)
		}
	}
}

impl Abacus {
	/// # Rebalance.
	///
	/// Shift overflowing small units to larger units, like seconds to minutes,
	/// minutes to hours, etc.
	fn rebalance(&mut self) {
		if 23 < self.hh || 59 < self.mm || 59 < self.ss {
			self.rebalance_ss();
			self.rebalance_mm();
			self.rebalance_hh();
		}

		if
			0 == self.m || 12 < self.m || 0 == self.d ||
			(28 < self.d && self.month_days() < self.d)
		{

			self.rebalance_date();
		}
	}

	/// # Rebalance Seconds.
	///
	/// While the other time-rebalancing methods focus on just a single shift,
	/// this will move seconds to days, hours, and/or minutes as necessary. The
	/// extra effort here is primarily a short-circuit for addition operations,
	/// which only increment seconds.
	///
	/// The bitshift wizardry was inspired by [this post](https://johnnylee-sde.github.io/Fast-unsigned-integer-to-time-string/).
	fn rebalance_ss(&mut self) {
		if self.ss >= DAY_IN_SECONDS {
			let div = self.ss.wrapping_div(DAY_IN_SECONDS);
			self.d += div;
			self.ss -= div * DAY_IN_SECONDS;
		}
		if self.ss >= HOUR_IN_SECONDS {
			let div = (self.ss * 0x91A3) >> 27;
			self.hh += div;
			self.ss -= div * HOUR_IN_SECONDS;
		}
		if self.ss >= MINUTE_IN_SECONDS {
			let div = (self.ss * 0x889) >> 17;
			self.mm += div;
			self.ss -= div * MINUTE_IN_SECONDS;
		}
	}

	/// # Rebalance Minutes.
	///
	/// This moves overflowing minutes to hours.
	fn rebalance_mm(&mut self) {
		if self.mm > 59 {
			let div = (self.mm * 0x889) >> 17;
			self.hh += div;
			self.mm -= div * 60;
		}
	}

	/// # Rebalance Hours.
	///
	/// This moves overflowing hours to days.
	fn rebalance_hh(&mut self) {
		if self.hh > 23 {
			let div = self.hh.wrapping_div(24);
			self.d += div;
			self.hh -= div * 24;
		}
	}

	/// # Rebalance Date.
	///
	/// This handles the shifting of both days to months and months to years.
	///
	/// In cases where a value is zero, the higher unit is rewound. For
	/// example, a year/month of `2000-00` becomes `1999-12`; a month/day of
	/// '06-00' becomes '05-31'.
	///
	/// Because months have different numbers of days from one another, and
	/// even from their selves year-to-year, this method recurses to simplify
	/// handling.
	fn rebalance_date(&mut self) {
		// No amount of rebalancing can bring this within range.
		if self.y < 1500 {
			self.y = 1500;
			self.m = 1;
			self.d = 1;
			return;
		}

		// Rewind the year.
		if self.m == 0 {
			self.y -= 1;
			self.m = 12;
		}
		// Carry excess months over to years.
		else if 12 < self.m {
			let div = (self.m - 1).wrapping_div(12);
			self.y += div;
			self.m -= div * 12;
		}

		// Rewind the month.
		if self.d == 0 {
			// If the month was January, we need to rewind the year too. Might
			// as well handle all rewinds in one go.
			if self.m == 1 {
				self.y -= 1;
				self.m = 12;
				self.d = 31;
			}
			else {
				self.m -= 1;
				self.d = self.month_days();
			}
		}
		// We know we're fine if the day is less than 28, but if it is greater,
		// we have to do some additional checking.
		else if 28 < self.d {
			let size = self.month_days();
			if size < self.d {
				self.m += 1;
				self.d -= size;
				self.rebalance_date();
			}
		}
	}

	/// # Last Day of Month.
	///
	/// This returns the last day of the month, or the number of days in a
	/// month, whichever way you want to think of it.
	///
	/// This is leap-aware.
	const fn month_days(&self) -> u32 {
		match self.m {
			1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
			4 | 6 | 9 | 11 => 30,
			2 if self.y.trailing_zeros() >= 2 && ((self.y % 100) != 0 || (self.y % 400) == 0) => 29,
			_ => 28,
		}
	}
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	/// # Addition.
	fn addition() {
		macro_rules! add {
			($($start:ident + $num:literal = ($y2:literal, $m2:literal, $d2:literal, $hh2:literal, $mm2:literal, $ss2:literal)),+) => ($(
				assert_eq!(
					($start + $num).parts(),
					($y2, $m2, $d2, $hh2, $mm2, $ss2)
				);

				// Make sure add/assign is the same as adding. It's obviously
				// fine now, but could get broken later. Who knows!
				{
					let mut tmp = $start;
					tmp += $num;
					assert_eq!($start + $num, tmp);
				}
			)+);
		}

		// Add nothing.
		let start = Abacus::new(2000, 1, 1, 0, 0, 0);
		add!(
			start + 0 = (0, 1, 1, 0, 0, 0),
			start + 1 = (0, 1, 1, 0, 0, 1),
			start + 60 = (0, 1, 1, 0, 1, 0),
			start + 3600 = (0, 1, 1, 1, 0, 0),
			start + 3661 = (0, 1, 1, 1, 1, 1),
			start + 31_622_400 = (1, 1, 1, 0, 0, 0),
			start + 4_294_967_295 = (99, 12, 31, 23, 59, 59)
		);
	}

	#[test]
	/// # Test Carry-Over.
	///
	/// This helps ensure we're doing the math correctly.
	fn carries() {
		macro_rules! carry {
			($(($y:literal, $m:literal, $d:literal, $hh:literal, $mm:literal, $ss:literal) ($y2:literal, $m2:literal, $d2:literal, $hh2:literal, $mm2:literal, $ss2:literal) $fail:literal),+) => ($(
				assert_eq!(
					Abacus::new($y, $m, $d, $hh, $mm, $ss).parts(),
					($y2, $m2, $d2, $hh2, $mm2, $ss2),
					$fail
				);
			)+);
		}

		carry!(
			(2000, 13, 32, 24, 60, 60) (01, 2, 2, 1, 1, 0) "Overage of one everywhere.",
			(2000, 25, 99, 1, 1, 1) (02, 4, 9, 1, 1, 1) "Large month/day overages.",
			(2000, 1, 1, 99, 99, 99) (00, 1, 5, 4, 40, 39) "Large time overflows.",
			(2000, 255, 255, 255, 255, 255) (21, 11, 20, 19, 19, 15) "Max overflows.",
			(1970, 25, 99, 1, 1, 1) (00, 1, 1, 0, 0, 0) "Saturating low.",
			(3000, 25, 99, 1, 1, 1) (99, 12, 31, 23, 59, 59) "Saturating high #1.",
			(2099, 25, 99, 1, 1, 1) (99, 12, 31, 23, 59, 59) "Saturating high #2.",
			(2010, 0, 0, 1, 1, 1) (09, 11, 30, 1, 1, 1) "Zero month, zero day.",
			(2010, 0, 32, 1, 1, 1) (10, 1, 1, 1, 1, 1) "Zero month, overflowing day.",
			(2010, 1, 0, 1, 1, 1) (09, 12, 31, 1, 1, 1) "Zero day into zero month.",
			(2010, 2, 30, 1, 1, 1) (10, 3, 2, 1, 1, 1) "Too many days for month.",
			(2010, 24, 1, 1, 1, 1) (11, 12, 1, 1, 1, 1) "Exactly 24 months."
		);
	}

	#[test]
	/// # Bitshifts.
	///
	/// Make sure our bitshift tickery doesn't lead to any rounding errors.
	///
	/// While the time fields are themselves `u32`, because of how
	/// instantiation works, they are all effectively limited to the `u8` range
	/// except for seconds, which (by the time shifting matters) maxes out at
	/// `86_399`.
	///
	/// The range to check is therefore `0..DAY_IN_SECONDS`.
	fn shifting() {
		fn divvy(mut ss: u32) -> (u32, u32, u32) {
			let mut hh = 0;
			let mut mm = 0;

			if ss >= HOUR_IN_SECONDS {
				let div = ss / HOUR_IN_SECONDS;
				hh += div;
				ss -= div * HOUR_IN_SECONDS;
			}
			if ss >= MINUTE_IN_SECONDS {
				let div = ss / MINUTE_IN_SECONDS;
				mm += div;
				ss -= div * MINUTE_IN_SECONDS;
			}

			(hh, mm, ss)
		}

		fn shifty(mut ss: u32) -> (u32, u32, u32) {
			let mut hh = 0;
			let mut mm = 0;

			if ss >= HOUR_IN_SECONDS {
				let div = (ss * 0x91A3) >> 27;
				hh += div;
				ss -= div * HOUR_IN_SECONDS;
			}
			if ss >= MINUTE_IN_SECONDS {
				let div = (ss * 0x889) >> 17;
				mm += div;
				ss -= div * MINUTE_IN_SECONDS;
			}

			(hh, mm, ss)
		}

		for i in 0..DAY_IN_SECONDS {
			assert_eq!(divvy(i), shifty(i));
		}
	}
}
