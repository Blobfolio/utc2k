# UTC2K

[![docs.rs](https://img.shields.io/docsrs/utc2k.svg?style=flat-square&label=docs.rs)](https://docs.rs/utc2k/)
[![changelog](https://img.shields.io/crates/v/utc2k.svg?style=flat-square&label=changelog&color=9b59b6)](https://github.com/Blobfolio/utc2k/blob/master/CHANGELOG.md)<br>
[![crates.io](https://img.shields.io/crates/v/utc2k.svg?style=flat-square&label=crates.io)](https://crates.io/crates/utc2k)
[![ci](https://img.shields.io/github/actions/workflow/status/Blobfolio/utc2k/ci.yaml?style=flat-square&label=ci)](https://github.com/Blobfolio/utc2k/actions)
[![deps.rs](https://deps.rs/repo/github/blobfolio/utc2k/status.svg?style=flat-square&label=deps.rs)](https://deps.rs/repo/github/blobfolio/utc2k)<br>
[![license](https://img.shields.io/badge/license-wtfpl-ff1493?style=flat-square)](https://en.wikipedia.org/wiki/WTFPL)
[![contributions welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square&label=contributions)](https://github.com/Blobfolio/utc2k/issues)

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

The main date object is `Utc2k`.

```rust
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

There is also `FmtUtc2k`, used for string representation.

```rust
use utc2k::{FmtUtc2k, Utc2k};
use std::convert::TryFrom;

// You can generate it from an existing Utc2k with either:
assert_eq!(Utc2k::default().formatted(), FmtUtc2k::from(Utc2k::default()));

// You could also skip `Utc2k` and seed directly from a timestamp or date/time
// string.
let fmt = FmtUtc2k::from(4_102_444_799_u32);
let fmt = FmtUtc2k::try_from("2099-12-31 23:59:59").unwrap();
```

Once you have a `FmtUtc2k`, you can turn it into a string with:

```rust
use utc2k::{FmtUtc2k, Utc2k};
use std::borrow::Borrow;

let fmt = FmtUtc2k::from(4_102_444_799_u32);

let s: &str = &fmt;
let s: &str = fmt.as_ref();
let s: &str = fmt.as_str();
let s: &str = fmt.borrow();
```



## Optional Crate Features

* `local`: Enables the `LocalOffset` struct. Refer to the documentation for important caveats and limitations.
* `serde`: Enables serialization/deserialization support.



## Installation

Add `utc2k` to your `dependencies` in `Cargo.toml`, like:

```
[dependencies]
utc2k = "0.9.*"
```



## License

Copyright © 2024 [Blobfolio, LLC](https://blobfolio.com) &lt;hello@blobfolio.com&gt;

This work is free. You can redistribute it and/or modify it under the terms of the Do What The Fuck You Want To Public License, Version 2.

    DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
    Version 2, December 2004
    
    Copyright (C) 2004 Sam Hocevar <sam@hocevar.net>
    
    Everyone is permitted to copy and distribute verbatim or modified
    copies of this license document, and changing it is allowed as long
    as the name is changed.
    
    DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
    TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
    
    0. You just DO WHAT THE FUCK YOU WANT TO.
