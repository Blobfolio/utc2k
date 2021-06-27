/*!
# UTC2K

UTC2K is a fast and lean date/time library that only cares about UTC happenings in _this century_ (between `2000-01-01 00:00:00` and `2099-12-31 23:59:59`).

With that very significant constraint in mind, UTC2K can:

* Convert to/from Unix timestamps (`u32`);
* Convert to/from date strings of the `YYYY-MM-DD` and `YYYY-MM-DD hh:mm:ss` varieties;
* Perform addition/subtraction (in seconds), checked or saturating;
* Calculate the date's ordinal;
* Calculate the number of seconds from midnight;

That's it!

Compared to more robust libraries like [`chrono`](https://crates.io/crates/chrono) and [`time`](https://crates.io/crates/time), UTC2K can be magnitudes faster, particularly in regards to string parsing and printing.

This library is still a work in progress and there is certainly room to improve performance further.

If you have any suggestions for improvement, feel free to open [an issue](https://github.com/Blobfolio/utc2k/issues) on Github!



## Examples

The main date object is [`Utc2k`].

```
use utc2k::Utc2k;
use std::convert::TryFrom;

let date = Utc2k::default(); // 2000-01-01 00:00:00
let date = Utc2k::now(); // The current time.
let date = Utc2k::from(4_102_444_799_u32); // 2099-12-31 23:59:59
let date = Utc2k::new(2010, 10, 31, 15, 30, 0); // 2010-10-31 15:30:00

// String parsing is fallible, but flexible. So long as the numbers we
// need are in the right place, it will be fine. (At least, it won't error
// out; if the date string is trying to communicate a time zone, that won't
// be listened to.)
assert!(Utc2k::try_from("2099-12-31 23:59:59").is_ok()); // Fine.
assert!(Utc2k::try_from("2099-12-31T23:59:59.0000Z").is_ok()); // Also fine.
assert!(Utc2k::try_from("January 1, 2010 @ Eleven O'Clock").is_err()); // Nope!
```

There is also [`FmtUtc2k`], used for string representation.

```
use utc2k::{FmtUtc2k, Utc2k};
use std::convert::TryFrom;

// You can generate it from an existing Utc2k with either:
assert_eq!(Utc2k::default().formatted(), FmtUtc2k::from(Utc2k::default()));

// You could also skip `Utc2k` and seed directly from a timestamp or date/time
// string.
let fmt = FmtUtc2k::from(4_102_444_799_u32);
let fmt = FmtUtc2k::try_from("2099-12-31 23:59:59").unwrap();
```

Once you have a [`FmtUtc2k`], you can turn it into a string with:

```
use utc2k::{FmtUtc2k, Utc2k};
use std::borrow::Borrow;

let fmt = FmtUtc2k::from(4_102_444_799_u32);

let s: &str = &fmt;
let s: &str = fmt.as_ref();
let s: &str = fmt.as_str();
let s: &str = fmt.borrow();
```


## Installation

Add `utc2k` to your `dependencies` in `Cargo.toml`, like:

```ignore,text
[dependencies]
utc2k = "0.1.*"
```
*/

#![warn(clippy::filetype_is_file)]
#![warn(clippy::integer_division)]
#![warn(clippy::needless_borrow)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![warn(clippy::perf)]
#![warn(clippy::suboptimal_flops)]
#![warn(clippy::unneeded_field_pattern)]
#![warn(macro_use_extern_crate)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(non_ascii_idents)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unreachable_pub)]
#![warn(unused_crate_dependencies)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]

#![allow(clippy::module_name_repetitions)]



mod date;
mod error;

pub use date::{
	FmtUtc2k,
	Utc2k,
};
pub use error::Utc2kError;



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

/// # Seconds Between 1970 and 2000.
pub const OLD_SECONDS: u32 = 946_684_800;

/// # Julian Day Epoch.
pub const JULIAN_EPOCH: u32 = 2_440_588;



#[allow(clippy::cast_possible_truncation)] // It fits.
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
		|n| n.as_secs().min(4_102_444_799) as u32
	)
}
