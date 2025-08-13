/*!
# UTC2K

[![docs.rs](https://img.shields.io/docsrs/utc2k.svg?style=flat-square&label=docs.rs)](https://docs.rs/utc2k/)
[![changelog](https://img.shields.io/crates/v/utc2k.svg?style=flat-square&label=changelog&color=9b59b6)](https://github.com/Blobfolio/utc2k/blob/master/CHANGELOG.md)<br>
[![crates.io](https://img.shields.io/crates/v/utc2k.svg?style=flat-square&label=crates.io)](https://crates.io/crates/utc2k)
[![ci](https://img.shields.io/github/actions/workflow/status/Blobfolio/utc2k/ci.yaml?style=flat-square&label=ci)](https://github.com/Blobfolio/utc2k/actions)
[![deps.rs](https://deps.rs/crate/utc2k/latest/status.svg?style=flat-square&label=deps.rs)](https://deps.rs/crate/utc2k/)<br>
[![license](https://img.shields.io/badge/license-wtfpl-ff1493?style=flat-square)](https://en.wikipedia.org/wiki/WTFPL)
[![contributions welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square&label=contributions)](https://github.com/Blobfolio/utc2k/issues)

UTC2K is a heavily-optimized — and extremely niche — date/time library that **only supports UTC happenings in _this century_**.

For the moments between `2000-01-01 00:00:00..=2099-12-31 23:59:59`, it can run circles around crates like [`chrono`](https://crates.io/crates/chrono) and [`time`](https://crates.io/crates/time), while still being able to:

* Determine "now", at least until the final seconds of 2099;
* Convert to/from Unix timestamps;
* Convert to/from all sorts of different date/time strings;
* Perform checked and saturating addition/subtraction;
* Calculate ordinals, weekdays, leap years, etc.;



## Examples

The library's main export is [`Utc2k`], a `Copy`-friendly struct representing a specific UTC datetime.

```
use utc2k::{Utc2k, Weekday};

// Instantiation, four ways:
let date = Utc2k::now();                             // The current system time.
let date = Utc2k::new(2020, 1, 2, 12, 30, 30);       // From parts.
let date = Utc2k::from_unixtime(4_102_444_799);      // From a timestamp.
let date = Utc2k::from_ascii(b"2024-10-31 00:00:00") // From a datetime string.
               .unwrap();

// What day was Halloween 2024, anyway?
assert_eq!(
    date.weekday(),
    Weekday::Thursday,
);

// Ordinals are a kind of bird, right?
assert_eq!(
    date.ordinal(),
    305,
);

// Boss wants an RFC2822 for some reason?
assert_eq!(
    date.to_rfc2822(),
    "Thu, 31 Oct 2024 00:00:00 +0000",
);
```



## Optional Crate Features

* `local`: Enables the [`Local2k`]/[`FmtLocal2k`] structs. Refer to the documentation for important caveats and limitations.
* `serde`: Enables serialization/deserialization support.
*/

#![deny(
	clippy::allow_attributes_without_reason,
	clippy::correctness,
	unreachable_pub,
	unsafe_code,
)]

#![warn(
	clippy::complexity,
	clippy::nursery,
	clippy::pedantic,
	clippy::perf,
	clippy::style,

	clippy::allow_attributes,
	clippy::clone_on_ref_ptr,
	clippy::create_dir,
	clippy::filetype_is_file,
	clippy::format_push_string,
	clippy::get_unwrap,
	clippy::impl_trait_in_params,
	clippy::lossy_float_literal,
	clippy::missing_assert_message,
	clippy::missing_docs_in_private_items,
	clippy::needless_raw_strings,
	clippy::panic_in_result_fn,
	clippy::pub_without_shorthand,
	clippy::rest_pat_in_fully_bound_structs,
	clippy::semicolon_inside_block,
	clippy::str_to_string,
	clippy::string_to_string,
	clippy::todo,
	clippy::undocumented_unsafe_blocks,
	clippy::unneeded_field_pattern,
	clippy::unseparated_literal_suffix,
	clippy::unwrap_in_result,

	macro_use_extern_crate,
	missing_copy_implementations,
	missing_docs,
	non_ascii_idents,
	trivial_casts,
	trivial_numeric_casts,
	unused_crate_dependencies,
	unused_extern_crates,
	unused_import_braces,
)]

#![expect(clippy::redundant_pub_crate, reason = "Unresolvable.")]

#![cfg_attr(docsrs, feature(doc_cfg))]



mod chr;
mod date;
mod error;
mod month;
mod weekday;
mod year;

mod macros;

#[cfg(any(test, feature = "serde"))]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
mod serde;



use chr::DateChar;
pub use date::{
	FmtUtc2k,
	Utc2k,
};
pub use error::Utc2kError;
pub use month::Month;
pub use weekday::Weekday;
use year::Year;

#[cfg(feature = "local")]
#[cfg_attr(docsrs, doc(cfg(feature = "local")))]
pub use date::local::{
	FmtLocal2k,
	Local2k,
};

#[cfg(test)] use brunch as _;


/// # Seconds per Minute.
pub const MINUTE_IN_SECONDS: u32 = 60;

/// # Seconds per Hour.
pub const HOUR_IN_SECONDS: u32 = 3600;

/// # Seconds per Day.
pub const DAY_IN_SECONDS: u32 = 86_400;

/// # Seconds per Week.
pub const WEEK_IN_SECONDS: u32 = 604_800;

/// # Seconds per (Normal) Year.
pub const YEAR_IN_SECONDS: u32 = 31_536_000;

/// # ASCII Lower Mask.
///
/// This mask is used to unconditionally lowercase the last three bytes of a
/// (LE) `u32` so we can case-insensitively match (alphabetic-only) month,
/// weekday, and offset abbreviations.
const ASCII_LOWER: u32 = 0x2020_2000;

/// # Julian Day Offset.
///
/// The offset in days between JD0 and 1 March 1BC, necessary since _someone_
/// forgot to invent 0AD. Haha.
///
/// (Only used when calendarizing timestamps.)
const JULIAN_OFFSET: u32 = 2_440_588 - 1_721_119;

/// # Days per Year (Rounded to Two Decimals).
///
/// The average number of days per year, rounded to two decimal places (and
/// multiplied by 100).
///
/// (Only used when calendarizing timestamps.)
const YEAR_IN_DAYS_P2: u32 = 36_525; // 365.25

/// # Days per Year (Rounded to Four Decimals).
///
/// The average number of days per year, rounded to four decimal places (and
/// multiplied by 10,000).
///
/// (Only used when calendarizing timestamps.)
const YEAR_IN_DAYS_P4: u32 = 3_652_425; // 365.2425



#[expect(
	clippy::cast_lossless,
	clippy::cast_possible_truncation,
	reason = "False positive.",
)]
#[must_use]
/// # Now (Current Unixtime).
///
/// This returns the current unix timestamp as a `u32`.
///
/// Rather than panic on out-of-range values — in the event the system clock is
/// broken or an archaeologist is running this in the distant future — the
/// timetsamp will be saturated to [`Utc2k::MIN_UNIXTIME`] or
/// [`Utc2k::MAX_UNIXTIME`].
pub fn unixtime() -> u32 {
	use std::time::SystemTime;

	SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).map_or(
		Utc2k::MIN_UNIXTIME,
		|n| n.as_secs().clamp(Utc2k::MIN_UNIXTIME as u64, Utc2k::MAX_UNIXTIME as u64) as u32
	)
}

#[expect(clippy::cast_possible_truncation, reason = "False positive.")]
#[must_use]
/// # Now (Current Year).
///
/// This returns the current year as a `u16`.
///
/// See [`unixtime`] for notes about system clock error recovery.
///
/// ## Examples
///
/// ```
/// assert_eq!(
///     utc2k::Utc2k::now().year(),
///     utc2k::year(),
/// );
/// ```
pub fn year() -> u16 {
	// Same as Utc2k::now().year(), but stripped to the essentials.
	let z = unixtime().wrapping_div(DAY_IN_SECONDS) + JULIAN_OFFSET;
	let h: u32 = 100 * z - 25;
	let mut a: u32 = h.wrapping_div(YEAR_IN_DAYS_P4);
	a -= a.wrapping_div(4);
	let year: u32 = (100 * a + h).wrapping_div(YEAR_IN_DAYS_P2);
	a = a + z - 365 * year - year.wrapping_div(4);
	let month = (5 * a + 456).wrapping_div(153);

	year as u16 + u16::from(12 < month)
}



#[expect(clippy::inline_always, reason = "Foundational.")]
#[inline(always)]
#[must_use]
/// # Case-Insensitive Needle.
///
/// This method lower cases three (presumed letters) into a single `u32` for
/// lightweight comparison.
///
/// This is used for matching [`Month`] and [`Weekday`] abbreviations, and
/// `"UTC"`/`"GMT"` offset markers.
const fn needle3(a: u8, b: u8, c: u8) -> u32 {
	u32::from_le_bytes([0, a, b, c]) | ASCII_LOWER
}



#[cfg(test)]
mod test {
	use super::*;
	use std::time::SystemTime;

	#[test]
	fn t_needle3() {
		// The ASCII lower bit mask is meant to apply to the last three bytes
		// (LE).
		assert_eq!(
			ASCII_LOWER.to_le_bytes(),
			[0, 0b0010_0000, 0b0010_0000, 0b0010_0000],
		);

		// We lowercase month/weekday abbreviation search needles
		// unconditionally — non-letters won't match regardless — so just need
		// to make sure it works for upper/lower letters.
		assert_eq!(
			needle3(b'J', b'E', b'B'),
			u32::from_le_bytes([0, b'j', b'e', b'b']),
		);
		assert_eq!(
			needle3(b'j', b'e', b'b'),
			u32::from_le_bytes([0, b'j', b'e', b'b']),
		);
	}

	#[test]
	fn t_unixtime() {
		// Our method.
		let our_secs = unixtime();

		// Manual construction via SystemTime.
		let secs: u32 = SystemTime::now()
			.duration_since(SystemTime::UNIX_EPOCH)
			.expect("The system time is set to the deep past!")
			.as_secs()
			.try_into()
			.expect("The system clock is set to the distant future!");

		// The SystemTime version should fall within our range.
		assert!(
			(Utc2k::MIN_UNIXTIME..=Utc2k::MAX_UNIXTIME).contains(&secs),
			"Bug: the system clock is completely wrong!",
		);

		// It should also match the `unixtime` output, but let's allow a tiny
		// ten-second cushion in case the runner is _really_ slow.
		assert!(
			our_secs.abs_diff(secs) <= 10,
			"SystemTime and unixtime are more different than expected!",
		);
	}
}
