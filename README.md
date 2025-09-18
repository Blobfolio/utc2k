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

The library's main export is `Utc2k`, a `Copy`-friendly struct representing a specific UTC datetime.

```rust
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

* `local`: Enables the `Local2k`/`FmtLocal2k` structs. Refer to the documentation for important caveats and limitations.
* `serde`: Enables serialization/deserialization support.
* `sqlx-mysql`: Enables [`sqlx`](https://crates.io/crates/sqlx) (mysql) support for `Utc2k`.



## Installation

Add `utc2k` to your `dependencies` in `Cargo.toml`, like:

```toml
[dependencies]
utc2k = "0.17.*"
```
