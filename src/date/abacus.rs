/*!
# UTC2K - Abacus
*/

use crate::{
	DAY_IN_SECONDS,
	HOUR_IN_SECONDS,
	MINUTE_IN_SECONDS,
	Month,
	Utc2k,
	Utc2kError,
};



/// # Helper: Merge Digits.
///
/// This macro acts like `concat` for digit literals residing within a
/// slice/array (at the specified indices). If the slice has values `1` and `2`,
/// for example, this makes `12` (rather than `1+2=3`).
macro_rules! merge_digits {
	// Two digits to u8.
	($src:ident $idx1:literal $idx2:literal) => ( $src[$idx1] * 10 + $src[$idx2] );

	// Four digits to u16.
	($src:ident $idx1:literal $idx2:literal $idx3:literal $idx4:literal) => (
		$src[$idx1] as u16 * 1000 +
		$src[$idx2] as u16 * 100 +
		$src[$idx3] as u16 * 10 +
		$src[$idx4] as u16
	);
}



#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
/// # Abacus.
///
/// This struct contains the same parts as [`Utc2k`], but with room for
/// temporary over/underflow so that parts can be added, subtracted, and/or
/// realigned.
///
/// It is used for opportunisitic maths against an established [`Utc2k`], or
/// as an intermediary when parsing a date/time string.
pub(super) struct Abacus {
	/// # Year.
	y: u16,

	/// # Month.
	m: u16,

	/// # Day.
	d: u16,

	/// # Hour.
	hh: u16,

	/// # Minute.
	mm: u16,

	/// # Second.
	ss: u16,
}

impl Abacus {
	/// # Max Seconds.
	///
	/// Trying to add more than this many seconds to any in-range date/time
	/// would pull it _out_ of range, so there's no point.
	const MAX_SECONDS: u32 = Utc2k::MAX_UNIXTIME - Utc2k::MIN_UNIXTIME + 1;
}

impl Abacus {
	#[must_use]
	/// # New.
	///
	/// Create a new (and balanced) instance from raw parts, usually from a
	/// valid [`Utc2k`].
	pub(super) const fn new(y: u16, m: u8, d: u8, hh: u8, mm: u8, ss: u8) -> Self {
		let mut out = Self {
			y,
			m: m as u16,
			d: d as u16,
			hh: hh as u16,
			mm: mm as u16,
			ss: ss as u16,
		};
		out.rebalance();
		out
	}

	#[must_use]
	/// # From `Utc2k`
	pub(super) const fn from_utc2k(src: Utc2k) -> Self {
		let (y, m, d, hh, mm, ss) = src.parts();
		Self::new(y, m, d, hh, mm, ss)
	}

	#[expect(clippy::cast_possible_truncation, reason = "False positive.")]
	#[must_use]
	/// # Parts (Saturating).
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

	#[expect(clippy::cast_possible_truncation, reason = "False positive.")]
	/// # Parts (Checked).
	///
	/// Return the individual parts, nice and balanced, ready for consumption
	/// by [`Utc2k`], unless out of range.
	pub(super) const fn parts_checked(&self)
	-> Result<(u8, u8, u8, u8, u8, u8), Utc2kError> {
		if self.y < 2000 { Err(Utc2kError::Underflow) }
		else if 2099 < self.y { Err(Utc2kError::Overflow) }
		else {
			Ok((
				(self.y - 2000) as u8,
				self.m as u8,
				self.d as u8,
				self.hh as u8,
				self.mm as u8,
				self.ss as u8,
			))
		}
	}
}

impl Abacus {
	#[expect(clippy::cast_possible_truncation, reason = "False positive.")]
	/// # Rebalance.
	///
	/// Shift overflowing small units to larger units, like seconds to minutes,
	/// minutes to hours, etc.
	const fn rebalance(&mut self) {
		// Time parts can only ever trickle upward, so they're best tackled
		// first, and in ascending order.
		if 59 < self.ss {
			self.mm += self.ss.wrapping_div(MINUTE_IN_SECONDS as u16);
			self.ss %= MINUTE_IN_SECONDS as u16;
		}
		if 59 < self.mm {
			self.hh += self.mm.wrapping_div(60);
			self.mm %= 60;
		}
		if 23 < self.hh {
			self.d += self.hh.wrapping_div(24);
			self.hh %= 24;
		}

		// Day balancing can require recursion, so the date parts need to be
		// tackled as a group.
		if
			0 == self.m || 12 < self.m || 0 == self.d ||
			(28 < self.d && self.month_days() < self.d)
		{
			self.rebalance_date();
		}
	}

	/// # Rebalance Date.
	///
	/// Shift over/underflowing days to months, and months to years.
	///
	/// In the case of underflows — a day or month of zero — the bigger pieces
	/// will "rewind" to accommodate. For example, a year/month of `2000-00`
	/// becomes `1999-12`; a month/day of '06-00' becomes '05-31'.
	const fn rebalance_date(&mut self) {
		loop {
			// Short circuit if we're way off base!
			if self.y <= 1900 { return self.rebalance_underflow(); }
			if 2200 <= self.y { return self.rebalance_overflow(); }

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
			// All months can hold at least 28 days, but if we've got 29+,
			// we'll need to do some sanity checks and maybe recurse.
			else if 28 < self.d {
				let size = self.month_days();
				if size < self.d {
					self.m += 1;
					self.d -= size;
					continue; // Recurse.
				}
			}

			return;
		}
	}

	/// # Rebalance to Overflow.
	///
	/// Set all components to an out-of-range date on the upper end.
	const fn rebalance_overflow(&mut self) {
		self.y = 2200;
		self.m = 1;
		self.d = 1;
		self.hh = 0;
		self.mm = 0;
		self.ss = 0;
	}

	/// # Rebalance to Underflow.
	///
	/// Set all components to an out-of-range date on the lower end.
	const fn rebalance_underflow(&mut self) {
		self.y = 1900;
		self.m = 1;
		self.d = 1;
		self.hh = 0;
		self.mm = 0;
		self.ss = 0;
	}

	#[must_use]
	/// # Last Day of Month.
	///
	/// This returns the last day of the month, or the number of days in that
	/// month, whichever way you want to think of it.
	///
	/// This is leap-aware.
	const fn month_days(&self) -> u16 {
		match self.m {
			1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
			4 | 6 | 9 | 11 => 30,
			2 if self.y.trailing_zeros() >= 2 && (! self.y.is_multiple_of(100) || self.y.is_multiple_of(400)) => 29,
			_ => 28,
		}
	}
}


impl Abacus {
	#[expect(clippy::cast_possible_truncation, reason = "False positive.")]
	#[must_use]
	/// # Add Seconds.
	///
	/// Create a new (and balanced) instance from `self + offset`.
	///
	/// This is only ever used when adding seconds to an existing (valid)
	/// [`Utc2k`] instance, so the starting values will be in range.
	pub(super) const fn plus_seconds(mut self, mut offset: u32) -> Self {
		// To make the big bad `u32` fit our `u16` units, we need to spread
		// the misery around into bigger buckets.
		if DAY_IN_SECONDS <= offset {
			// If the offset itself is too big for `Utc2k`, overflow is
			// inevitable. Let's just skip to the end!
			if Self::MAX_SECONDS < offset {
				self.rebalance_overflow();
				return self;
			}

			self.d += offset.wrapping_div(DAY_IN_SECONDS) as u16;
			offset %= DAY_IN_SECONDS;
		}
		if HOUR_IN_SECONDS <= offset {
			self.hh += offset.wrapping_div(HOUR_IN_SECONDS) as u16;
			offset %= HOUR_IN_SECONDS;
		}
		if MINUTE_IN_SECONDS <= offset {
			self.mm += offset.wrapping_div(MINUTE_IN_SECONDS) as u16;
			offset %= MINUTE_IN_SECONDS;
		}
		self.ss += offset as u16;
		self.rebalance();
		self
	}

	#[expect(clippy::cast_possible_truncation, reason = "False positive.")]
	/// # Handle Positive/Negative Offset.
	///
	/// Apply a parsed `±hhmm`-style UTC offset to self, avoiding rebalance
	/// if at all possible.
	///
	/// This is only used during ASCII parsing, and adjustments will always
	/// be less than one day.
	const fn apply_offset(&mut self, signed_offset: i32) {
		if signed_offset == 0 { return; }

		let mut offset = signed_offset.unsigned_abs();
		debug_assert!(
			offset < DAY_IN_SECONDS,
			"BUG: parsed offsets are supposed to be capped to a day!",
		);

		// Positive offsets require subtraction, which is super annoying.
		if 0 < signed_offset {
			// The time parts are easiest to shift, so let's figure out how
			// many seconds that buys us, and (temporarily) shift the lot to
			// a working variable.
			let mut balance =
				self.ss as u32 +
				self.mm as u32 * MINUTE_IN_SECONDS +
				self.hh as u32 * HOUR_IN_SECONDS;

			self.ss = 0;
			self.hh = 0;
			self.mm = 0;

			// If the offset is bigger, we'll need to steal a day too.
			if balance < offset {
				// If we don't have any days, rebalancing (just the date) will
				// always yield at least one.
				if 0 == self.d { self.rebalance_date(); }
				balance += DAY_IN_SECONDS;
				self.d -= 1;
			}

			// The difference between our starting balance and the offset needs
			// to be added back to `self` to complete the operation. By
			// assigning the difference to "offset", the negative handling
			// below will take care of that for us!
			offset = balance - offset;
		}

		// Negative offsets require addition; much easier!
		if DAY_IN_SECONDS <= offset {
			self.d += offset.wrapping_div(DAY_IN_SECONDS) as u16;
			offset %= DAY_IN_SECONDS;
		}
		if HOUR_IN_SECONDS <= offset {
			self.hh += offset.wrapping_div(HOUR_IN_SECONDS) as u16;
			offset %= HOUR_IN_SECONDS;
		}
		if MINUTE_IN_SECONDS <= offset {
			self.mm += offset.wrapping_div(MINUTE_IN_SECONDS) as u16;
			offset %= MINUTE_IN_SECONDS;
		}
		self.ss += offset as u16;
	}
}

impl Abacus {
	#[must_use]
	/// # From ASCII Date/Time Slice.
	///
	/// Try to parse the date/time parts from an ASCII string, returning a new
	/// balanced instance if successful.
	pub(super) const fn from_ascii(src: &[u8]) -> Option<Self> {
		if let Some(mut out) = Self::parse_ascii_raw(src) {
			out.rebalance();
			Some(out)
		}
		else { None }
	}

	#[must_use]
	/// # From RFC2822 Date/Time Slice.
	///
	/// Try to parse the date/time parts from an RFC2822-formatted string,
	/// returning a new balanced instance if successful.
	pub(super) const fn from_rfc2822(src: &[u8]) -> Option<Self> {
		if let Some(mut out) = Self::parse_rfc822_raw(src) {
			out.rebalance();
			Some(out)
		}
		else { None }
	}

	#[must_use]
	/// # From ASCII Date/Time Slice (Raw).
	///
	/// This method does all the hard work for `Self::from_ascii`.
	///
	/// Note the return value may not be balanced.
	const fn parse_ascii_raw(src: &[u8]) -> Option<Self> {
		match src {
			// Date.
			[ y1, y2, y3, y4,    m1 @ b'0'..=b'9', m2,    d1, d2, ] |
			[ y1, y2, y3, y4, _, m1,               m2, _, d1, d2, ] => {
				// By temporarily re-imagining the eight date bytes as a `u64`,
				// we can flip the ASCII bits and verify the results en masse.
				let chunk = u64::from_le_bytes([
					*y1, *y2, *y3, *y4, *m1, *m2, *d1, *d2,
				]) ^ 0x3030_3030_3030_3030_u64;
				let chk = chunk.wrapping_add(0x7676_7676_7676_7676_u64);
				if (chunk & 0xf0f0_f0f0_f0f0_f0f0_u64) | (chk & 0x8080_8080_8080_8080_u64) == 0 {
					let chunk = chunk.to_le_bytes();
					return Some(Self {
						y: merge_digits!(chunk 0 1 2 3),
						m: merge_digits!(chunk 4 5) as u16,
						d: merge_digits!(chunk 6 7) as u16,
						hh: 0, mm: 0, ss: 0,
					});
			    }
			},

			// Datetime.
			[ y1, y2, y3, y4,    m1 @ b'0'..=b'9', m2,    d1, d2,    hh1, hh2,    mm1, mm2,    ss1, ss2, rest @ .. ] |
			[ y1, y2, y3, y4, _, m1,               m2, _, d1, d2, _, hh1, hh2, _, mm1, mm2, _, ss1, ss2, rest @ .. ] => {
				// Same as before, but scaled up to u128 to accommodate an
				// additional six time bytes (and two bytes for filler).
				let chunk = u128::from_le_bytes([
					*y1, *y2, *y3, *y4, *m1, *m2, *d1, *d2,
					*hh1, *hh2, *mm1, *mm2, *ss1, *ss2,
					0, 0, // Filler.
				]) ^ 0x3030_3030_3030_3030_3030_3030_3030_u128;
				let chk = chunk.wrapping_add(0x7676_7676_7676_7676_7676_7676_7676_u128);
				if (chunk & 0xf0f0_f0f0_f0f0_f0f0_f0f0_f0f0_f0f0_u128) | (chk & 0x8080_8080_8080_8080_8080_8080_8080_u128) == 0 {
					let chunk = chunk.to_le_bytes();
					let mut out = Self {
						y:  merge_digits!(chunk 0 1 2 3),
						m:  merge_digits!(chunk 4 5) as u16,
						d:  merge_digits!(chunk 6 7) as u16,
						hh: merge_digits!(chunk 8 9) as u16,
						mm: merge_digits!(chunk 10 11) as u16,
						ss: merge_digits!(chunk 12 13) as u16,
					};

					// Check/apply the UTC offset, if any, and make sure the
					// slice ends where it's supposed to.
					if let Some(offset) = parse_offset(rest) {
						out.apply_offset(offset);
						return Some(out);
					}
				}
			},
			_ => {},
		}

		None
	}

	#[must_use]
	/// # From RFC2822 Date/Time Slice (Raw).
	///
	/// This method does all the hard work for `Self::from_rfc2822`.
	///
	/// Note the return value may not be balanced.
	const fn parse_rfc822_raw(src: &[u8]) -> Option<Self> {
		// Start with the date, as that's rather annoying and variable.
		if let Some((y, m, d, src)) = parse_rfc2822_date(src) {
			let mut out = Self {
				y,
				m: m as u16,
				d: d as u16,
				hh: 0, mm: 0, ss: 0,
			};

			// Is there more to parse?
			if let [ _, a, b, _, c, d, _, e, f, src @ .. ] = src {
				// By temporarily re-imagining the six date bytes as a `u64`,
				// we can flip the ASCII bits and verify the results en masse.
				let chunk = u64::from_le_bytes([
					*a, *b, *c, *d, *e, *f, 0, 0 // Two for filler.
				]) ^ 0x3030_3030_3030_u64;

				let chk = chunk.wrapping_add(0x7676_7676_7676_u64);
				if (chunk & 0xf0f0_f0f0_f0f0_u64) | (chk & 0x8080_8080_8080_u64) == 0 {
					let chunk = chunk.to_le_bytes();
					out.hh = merge_digits!(chunk 0 1) as u16;
					out.mm = merge_digits!(chunk 2 3) as u16;
					out.ss = merge_digits!(chunk 4 5) as u16;

					// Check/apply the UTC offset, if any, and make sure the
					// slice ends where it's supposed to.
					if let Some(offset) = parse_offset(src) {
						out.apply_offset(offset);
						return Some(out);
					}
				}
			}
			else if src.is_empty() { return Some(out); }
		}

		None
	}
}



#[expect(clippy::cast_possible_wrap, reason = "False positive.")]
#[must_use]
/// # Parse End.
///
/// Parse and return the UTC offset, if any, while also making sure there isn't
/// any other unexpected data lingering at the end.
///
/// Returns `None` if the remainder is non-empty.
const fn parse_offset(src: &[u8]) -> Option<i32> {
	// Zero will be the default.
	let mut offset = 0_i32;

	// What do we have?
	let src = match src {
		// Empty? Done!
		[] => return Some(offset),

		// A fixed offset?
		[ rest @ .., sign @ (b'+' | b'-'), a, b, c, d ] => {
			// By temporarily re-imagining the four offset bytes as a `u32`,
			// we can flip the ASCII bits and verify the results en masse.
			let chunk = u32::from_le_bytes([*a, *b, *c, *d]) ^ 0x3030_3030;
			if (chunk & 0xf0f0_f0f0_u32) | (chunk.wrapping_add(0x7676_7676_u32) & 0x8080_8080_u32) != 0 {
				return None;
			}

			let chunk = chunk.to_le_bytes();
			offset += merge_digits!(chunk 0 1) as i32 * HOUR_IN_SECONDS as i32;
			offset += merge_digits!(chunk 2 3) as i32 * MINUTE_IN_SECONDS as i32;

			// Normalize to within a day.
			offset %= DAY_IN_SECONDS as i32;

			// If the sign was negative, invert it.
			if *sign == b'-' { offset = 0_i32 - offset; }

			rest.trim_ascii_end()
		},

		// Redundant identifiers? (Z, UT, GMT, UTC)
		[ rest @ ..,                           b'Z' | b'z' ] |
		[ rest @ ..,              b'U' | b'u', b'T' | b't' ] |
		[ rest @ .., b'G' | b'g', b'M' | b'm', b'T' | b't' ] |
		[ rest @ .., b'U' | b'u', b'T' | b't', b'C' | b'c' ] => { rest.trim_ascii_end() },

		// Dunno?
		_ => src,
	};

	// Empty's good!
	if src.is_empty() { Some(offset) }

	// Fractional seconds are not supported, but aren't disqualifying either.
	// If the first thing is a dot and everything else is a digit, we'll allow
	// it.
	else if let [ b'.', rest @ .. ] = src {
		let mut src = rest;
		while let [ b'0'..=b'9', rest @ .. ] = src { src = rest; }
		if src.is_empty() { Some(offset) }
		else { None }
	}

	// Anything else is a deal breaker.
	else { None }
}

#[must_use]
/// # Parse RFC2822 Date.
///
/// This method parses the year, month, and day components from an
/// RFC2822-formatted string, returning them along with the remainder of the
/// source slice.
const fn parse_rfc2822_date(mut src: &[u8]) -> Option<(u16, Month, u8, &[u8])> {
	// Strip the leading weekday, if any; it's pointless.
	if let [ _, _, _, b',', b' ', rest @ .. ] = src { src = rest; }

	// The day could have one digit with or without a leading space, or two
	// digits, so is easiest to figure out on its own.
	let d = match src {
		[                  b @ b'0'..=b'9', b' ', rest @ .. ] |
		[ b' ',            b @ b'0'..=b'9', b' ', rest @ .. ] => {
			src = rest;
			*b ^ b'0'
		},
		[ a @ b'0'..=b'9', b @ b'0'..=b'9', b' ', rest @ .. ] => {
			src = rest;
			(*a ^ b'0') * 10 + (*b ^ b'0')
		},
		_ => return None,
	};

	// What remains should always look like "Mon YYYY".
	if let [ m1, m2, m3, b' ', y1, y2, y3, y4, rest @ .. ] = src {
		if let Some(m) = Month::from_abbreviation(&[*m1, *m2, *m3]) {
			// By temporarily re-imagining the four year bytes as a `u32`,
			// we can flip the ASCII bits and verify the results en masse.
			let chunk = u32::from_le_bytes([*y1, *y2, *y3, *y4]) ^ 0x3030_3030;
			if (chunk & 0xf0f0_f0f0_u32) | (chunk.wrapping_add(0x7676_7676_u32) & 0x8080_8080_u32) == 0 {
				let chunk = chunk.to_le_bytes();
				return Some((merge_digits!(chunk 0 1 2 3), m, d, rest));
			}
		}
	}

	None
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	/// # Addition.
	fn t_addition() {
		macro_rules! add {
			($($start:ident + $num:literal = ($y2:literal, $m2:literal, $d2:literal, $hh2:literal, $mm2:literal, $ss2:literal)),+) => ($(
				assert_eq!(
					$start.plus_seconds($num).parts(),
					($y2, $m2, $d2, $hh2, $mm2, $ss2)
				);
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

		// Let's verify nothing explodes if we try to add the most number of
		// seconds we can (without short-circuiting) to the biggest possible
		// (valid) starting point.
		let start = Abacus::from_utc2k(Utc2k::MAX);
		let end = start.plus_seconds(Abacus::MAX_SECONDS);
		assert_eq!(
			end.parts(),
			(99, 12, 31, 23, 59, 59)
		);

		// Similarly, let's verify that the biggest possible offset added to
		// the biggest possible day doesn't cause any problems.
		let mut start = Abacus {
			y: 9999,
			m: 99,
			d: 99,
			hh: 99,
			mm: 99,
			ss: 99,
		};
		start.apply_offset(-86_399);
		start.rebalance();
		assert_eq!(
			start.parts(),
			(99, 12, 31, 23, 59, 59)
		);

		// And the reverse, a nothing date with the smallest possible offset.
		start.y = 0;
		start.m = 0;
		start.d = 0;
		start.hh = 0;
		start.mm = 0;
		start.ss = 0;
		start.apply_offset(86_399);
		start.rebalance();
		assert_eq!(
			start.parts(),
			(0, 1, 1, 0, 0, 0)
		);
	}

	#[test]
	/// # Test Carry-Over.
	///
	/// This helps ensure we're doing the math correctly.
	fn t_carries() {
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
	/// # Month Days.
	///
	/// Test all years to make sure leaps are lept.
	fn t_month_days() {
		let mut abacus = Abacus::new(2000, 2, 15, 0, 0, 0);
		for i in 2000..=2099_u16 {
			abacus.y = i;
			let days = abacus.month_days();
			let leap = Utc2k::from_abacus(abacus).leap_year();
			assert_eq!(
				28_u16 + u16::from(leap),
				days,
				"Disagreement over February {i}: {days} ({leap})",
			);
		}
	}
}
