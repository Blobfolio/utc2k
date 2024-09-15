/*!
# UTC2K: Parsing
*/

use crate::{
	Abacus,
	HOUR_IN_SECONDS,
	JULIAN_EPOCH,
	MINUTE_IN_SECONDS,
	Month,
	Utc2k,
	Utc2kError,
};



#[expect(clippy::cast_possible_truncation, reason = "False positive.")]
/// # Parse Date From Seconds.
///
/// This parses the date portion of a date/time timestamp using the same
/// approach as [`time`](https://crates.io/crates/time), which is based on
/// algorithms by [Peter Baum](https://www.researchgate.net/publication/316558298_Date_Algorithms).
///
/// (Our version is a little simpler as we aren't worried about old times.)
pub(crate) const fn date_seconds(mut z: u32) -> (u8, u8, u8) {
	z += JULIAN_EPOCH - 1_721_119;
	let h: u32 = 100 * z - 25;
	let mut a: u32 = h.wrapping_div(3_652_425);
	a -= a >> 2;
	let year: u32 = (100 * a + h).wrapping_div(36_525);
	a = a + z - 365 * year - (year >> 2);
	let month: u32 = (5 * a + 456).wrapping_div(153);
	let day: u8 = (a - (153 * month - 457).wrapping_div(5)) as u8;

	if month > 12 {
		((year - 1999) as u8, month as u8 - 12, day)
	}
	else {
		((year - 2000) as u8, month as u8, day)
	}
}

/// # HMS.
///
/// Parse out the hours, minutes, and seconds from a byte slice like
/// `HH:MM:SS`.
pub(super) const fn hms(src: &[u8]) -> Result<(u8, u8, u8), Utc2kError> {
	if 8 <= src.len() {
		if let Ok(hh) = parse2(src[0], src[1]) {
			if let Ok(mm) = parse2(src[3], src[4]) {
				if let Ok(ss) = parse2(src[6], src[7]) {
					return Ok((hh, mm, ss));
				}
			}
		}
	}

	Err(Utc2kError::Invalid)
}

/// # Parse 2 Digits.
///
/// This combines two ASCII `u8` values into a single `u8` integer, or dies
/// trying (if, i.e., one or both are non-numeric).
pub(super) const fn parse2(a: u8, b: u8) -> Result<u8, Utc2kError> {
	let a = a ^ b'0';
	let b = b ^ b'0';
	if a < 10 && b < 10 {
		Ok(a * 10 + b)
	}
	else { Err(Utc2kError::Invalid) }
}

/// # Parse 4 Digits.
///
/// This combines four ASCII `u8` values into a single `16` integer, or dies
/// trying (if, i.e., any are non-numeric).
pub(super) const fn parse4(a: u8, b: u8, c: u8, d: u8) -> Result<u16, Utc2kError> {
	let a = (a ^ b'0') as u16;
	let b = (b ^ b'0') as u16;
	let c = (c ^ b'0') as u16;
	let d = (d ^ b'0') as u16;
	if a < 10 && b < 10 && c < 10 && d < 10 {
		Ok(a * 1000 + b * 100 + c * 10 + d)
	}
	else { Err(Utc2kError::Invalid) }
}

/// # Parse Parts From Date.
///
/// This attempts to extract the year, month, and day from a `YYYY-MM-DD` byte
/// slice. Only the numeric ranges are parsed — separators can be whatever.
pub(super) fn parts_from_date(src: &[u8; 10]) -> Result<Utc2k, Utc2kError> {
	let tmp = Abacus::new(
		parse4(src[0], src[1], src[2], src[3])?,
		parse2(src[5], src[6])?,
		parse2(src[8], src[9])?,
		0, 0, 0
	);

	Ok(Utc2k::from(tmp))
}

/// # Parse Parts From Date.
///
/// This attempts to extract the year, month, and day from a `YYYYMMDD` byte
/// slice.
pub(super) fn parts_from_smooshed_date(src: [u8; 8]) -> Result<Utc2k, Utc2kError> {
	let tmp = Abacus::new(
		parse4(src[0], src[1], src[2], src[3])?,
		parse2(src[4], src[5])?,
		parse2(src[6], src[7])?,
		0, 0, 0
	);

	Ok(Utc2k::from(tmp))
}

/// # Parse Parts From Date/Time.
///
/// This attempts to extract the year, month, day, hour, minute and second from
/// a `YYYY-MM-DD HH:MM:SS` byte slice. Only the numeric ranges are parsed —
/// separators can be whatever.
pub(super) fn parts_from_datetime(src: &[u8; 19]) -> Result<Utc2k, Utc2kError> {
	let (hh, mm, ss) = hms(&src[11..])?;
	let tmp = Abacus::new(
		parse4(src[0], src[1], src[2], src[3])?,
		parse2(src[5], src[6])?,
		parse2(src[8], src[9])?,
		hh, mm, ss,
	);

	Ok(Utc2k::from(tmp))
}

/// # Parse Parts From Date/Time.
///
/// This attempts to extract the year, month, day, hour, minute and second from
/// a `YYYYMMDDHHMMSS` byte slice.
pub(super) fn parts_from_smooshed_datetime(src: &[u8; 14]) -> Result<Utc2k, Utc2kError> {
	let tmp = Abacus::new(
		parse4(src[0], src[1], src[2], src[3])?,
		parse2(src[4], src[5])?,
		parse2(src[6], src[7])?,
		parse2(src[8], src[9])?,
		parse2(src[10], src[11])?,
		parse2(src[12], src[13])?,
	);

	Ok(Utc2k::from(tmp))
}

/// # Parse RFC2822 Day.
///
/// This method represents the second stage of [`Utc2k::from_rfc2822`]. It
/// parses the month-day component from the string, moves the pointer, and
/// passes it along to [`parse_rfc2822_datetime`] to finish it up.
pub(super) fn rfc2822_day(src: &[u8]) -> Option<Utc2k> {
	if 19 <= src.len() {
		let a = src[0] ^ b'0';
		if a < 10 {
			if src[1] == b' ' {
				return rfc2822_datetime(&src[2..], a);
			}

			let b = src[1] ^ b'0';
			if b < 10 {
				return rfc2822_datetime(
					&src[3..],
					a * 10 + b,
				);
			}
		}
	}

	None
}

#[expect(clippy::cast_possible_truncation, reason = "False positive.")]
/// # Parse Time From Seconds.
///
/// This parses the time portion of a date/time timestamp. It works the same
/// way a naive div/mod approach would, except it uses multiplication and bit
/// shifts to avoid actually having to div/mod.
///
/// (This only works because time values stop at 23 or 59; rounding errors
/// would creep in if the full u8 range was used.)
pub(super) const fn time_seconds(mut src: u32) -> (u8, u8, u8) {
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



/// # Parse RFC2822 Date/Time.
///
/// This method represents the third stage of [`Utc2k::from_rfc2822`]. It
/// parses the remaining date/time components from the string, applies the
/// offset (if any), and returns the desired `Utc2k` object.
fn rfc2822_datetime(src: &[u8], d: u8) -> Option<Utc2k> {
	// Grab the time bits.
	let (src, time) = src.split_first_chunk::<9>()?;
	let (hh, mm, ss) = hms(time).ok()?;

	// Parse out the rest!
	let tmp = Abacus::new(
		parse4(src[4], src[5], src[6], src[7]).ok()?,
		Month::from_abbreviation(src.as_slice())? as u8,
		d,
		hh, mm, ss,
	);

	// Apply an offset?
	if let Some((plus, offset_ss)) = rfc2822_offset(time) {
		// The offset is beyond UTC; we need to subtract.
		if plus { Some(Utc2k::from(tmp) - offset_ss) }
		// The offset is earlier than UTC; we need to add.
		else { Some(Utc2k::from(tmp + offset_ss)) }
	}
	// Pass through as-is!
	else { Some(Utc2k::from(tmp)) }
}

/// # Parse RFC2822 Offset.
///
/// This tries to tease out the UTC offset from the end of an RFC2822 string.
/// If present, it returns a bool representing the sign and the offset as
/// seconds.
const fn rfc2822_offset(src: &[u8]) -> Option<(bool, u32)> {
	let len: usize = src.len();
	if len > 6 && src[len - 6] == b' ' {
		let plus: bool = match src[len - 5] {
			b'+' => true,
			b'-' => false,
			_ => return None,
		};

		if let Ok(hh) = parse2(src[len - 4], src[len - 3]) {
			if let Ok(mm) = parse2(src[len - 2], src[len - 1]) {
				if 0 < hh || 0 < mm {
					return Some((
						plus,
						(hh as u32 * HOUR_IN_SECONDS + mm as u32 * MINUTE_IN_SECONDS)
					));
				}
			}
		}
	}

	None
}
