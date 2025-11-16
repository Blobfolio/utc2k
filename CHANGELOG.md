# Changelog



## [0.18.1](https://github.com/Blobfolio/utc2k/releases/tag/v0.18.1) - 2025-11-16

### Changed

* Make `sqlx-mysql` feature traits generic over `sqlx::Database`



## [0.18.0](https://github.com/Blobfolio/utc2k/releases/tag/v0.18.0) - 2025-10-30

### New

* `Utc2k::formatted_custom`
* `Utc2k::hour_12`
* `Utc2k::hour_period`

### Changed

* Bump `tz-rs` to `0.7.1`
* Miscellaneous code cleanup and lints



## [0.17.1](https://github.com/Blobfolio/utc2k/releases/tag/v0.17.1) - 2025-09-18

### Changed

* Replace `serde` w/ `serde_core`
* Miscellaneous code cleanup and lints



## [0.17.0](https://github.com/Blobfolio/utc2k/releases/tag/v0.17.0) - 2025-08-12

### New

* Optional crate feature `sqlx-mysql`

### Changed

* Miscellaneous code cleanup and lints



## [0.16.0](https://github.com/Blobfolio/utc2k/releases/tag/v0.16.0) - 2025-07-13

### New

* `Utc2k::cmp` (const comparison)
* `Month::cmp` (const comparison)
* `Weekday::cmp` (const comparison)
* `Utc2k::from_ascii` now supports `±hh:mm`-style (with colon) fixed offsets

### Breaking

* `Utc2k::cmp_date`/`cmp_time` now take `(a: Self, b: Self)` (instead of `(&self, b: Self)`)
* `Weekday::first/last/nth_in_month` now take a `Month` (instead of a `u8`)



## [0.15.0](https://github.com/Blobfolio/utc2k/releases/tag/v0.15.0) - 2025-06-26

Date/time string parsing has been completely refactored for this release, vastly improving the behavioral consistency — and correctness — across methods and formats.

Behavioral changes can be _tricky_, though, so warrant special attention.

It is recommended that projects using `utc2k`'s string-parsing features retest their integrations after upgrading to make sure everything still works as expected.

If you get stuck, feel free to open an [issue](https://github.com/Blobfolio/utc2k/issues).

The rest of the changes are more straightforward:

### New

* `FmtUtc2k::LEN`
* `FmtUtc2k::from_ascii`
* `FmtUtc2k::from_unixtime`
* `Local2k` (**local** crate feature)
* `Month::previous`/`next`
* `Weekday::previous`/`next`
* `Utc2k::checked_from_ascii`
* `Utc2k::from_ascii`
* impl `DoubleEndedIterator` for `RepeatingMonthIter`
* impl `DoubleEndedIterator` for `RepeatingWeekdayIter`
* impl `From<FmtUtc2k>` for `String`
* impl `From<Utc2k>` for `String`

### Changed

* Bump `brunch` to `0.11` (dev)
* Bump MSRV to `1.88`
* `FmtUtc2k::from_rfc822`/`Utc2k::from_rfc2822` now support date-only variations
* `FmtUtc2k::set_datetime` is now const
* `FmtUtc2k::set_parts` is now const
* `Utc2k::formatted` is now const
* Remove all but two `unsafe` blocks!
* Miscellaneous code cleanup, lints, and test/doc improvements

### Breaking

* The `FmtUtc2k` and `Utc2k` `FromStr`/`TryFrom<&[u8]>`/`TryFrom<&str>` impls are now equivalent to `Utc2k::from_ascii`, changing some previous behaviors:
  * Random trailing data (after the parsed date/time parts) is no longer allowed and will result in an error;
  * Squished date/time formats like `YYYYMMDD` can now be parsed;
  * Trailing `±hhmm` UTC offsets are now supported;
* `Utc2k::month` now returns a `Month` instead of a `u8`
* Removed `FmtUtc2k::now_local` (use `Local2k::now` instead);
* Removed `FmtUtc2k::try_from<i32, i64, isize, u64, usize>` (use `From<u32>` instead)
* Removed `LocalOffset` (use `Local2k` instead);
* Removed `Utc2k::from_datetime_str` (use `Utc2k::from_ascii` instead)
* Removed `Utc2k::from_smooshed_datetime_str` (use `Utc2k::from_ascii` instead)
* Removed `Utc2k::from_date_str` (use `Utc2k::from_ascii` instead)
* Removed `Utc2k::from_smooshed_date_str` (use `Utc2k::from_ascii` instead)
* Removed `Utc2k::month_abbreviation` (use `Utc2k::month().abbreviation` instead)
* Removed `Utc2k::month_enum` (use `Utc2k::month` instead)
* Removed `Utc2k::month_name` (use `Utc2k::month().as_str` instead)
* Removed `Utc2k::now_local` (use `Local2k::now` instead);
* Removed `Utc2k::parse_time_str`
* Removed `Utc2k::try_from<i32, i64, isize, u64, usize>` (use `From<u32>` instead)



## [0.14.0](https://github.com/Blobfolio/utc2k/releases/tag/v0.14.0) - 2025-05-31

### New

* `Utc2k::from_unixtime` (const alternative to `From<u32>`)

### Changed

* `Utc2k::checked_add` is now const
* `Utc2k::checked_from_unixtime` is now const
* `Utc2k::checked_sub` is now const
* `Utc2k::new` is now const
* `Utc2k::weekday` is now const
* `Utc2k::with_time` is now const
* `Weekday::first_in_month` is now const
* `Weekday::last_in_month` is now const
* `Weekday::nth_in_month` is now const

### Breaking

* `Month::all` (method) is now `Month::ALL` (constant)
* `Weekday::all` (method) is now `Weekday::ALL` (constant)
* Removed `Deref` impl for `FmtUtc2k`
* Removed `Deref` impl for `Month`
* Removed `Deref` impl for `Weekday`



## [0.13.0](https://github.com/Blobfolio/utc2k/releases/tag/v0.13.0) - 2025-05-15

### Changed

* Bump `brunch` to `0.10` (dev)
* Bump MSRV to `1.87`
* Miscellaneous code cleanup and lints



## [0.12.1](https://github.com/Blobfolio/utc2k/releases/tag/v0.12.1) - 2025-04-03

### Changed

* Miscellaneous code cleanup and lints



## [0.12.0](https://github.com/Blobfolio/utc2k/releases/tag/v0.12.0) - 2025-02-25

### Changed

* Bump `brunch` to `0.9` (dev)
* Bump MSRV to `1.85`
* Bump Rust edition to `2024`
* Miscellaneous code cleanup and lints



## [0.11.2](https://github.com/Blobfolio/utc2k/releases/tag/v0.11.2) - 2025-01-09

### Changed

* Bump `brunch` to `0.8` (dev)
* Miscellaneous code cleanup and lints



## [0.11.1](https://github.com/Blobfolio/utc2k/releases/tag/v0.11.1) - 2024-11-28

### Changed

* Bump `brunch` to `0.7` (dev)
* `FmtUtf2k::date` is now const
* `FmtUtf2k::time` is now const
* `FmtUtf2k::year` is now const
* Miscellaneous code cleanup and lints



## [0.11.0](https://github.com/Blobfolio/utc2k/releases/tag/v0.11.0) - 2024-10-25

### New

* `FmtUtc2k::MIN`
* `FmtUtc2k::MAX`
* `Utc2k::MIN`
* `Utc2k::MAX`

### Fixed

* Clamp `utc2k::unixtime` to supported min/max range in case the system clock is the right kind of wrong

### Changed

* Make `Utc2k::cmp_date` const
* Make `Utc2k::cmp_time` const

### Replaced

* `FmtUtc2k::min` (use `FmtUtc2k::MIN` instead)
* `FmtUtc2k::max` (use `FmtUtc2k::MAX` instead)
* `Utc2k::min` (use `Utc2k::MIN` instead)
* `Utc2k::max` (use `Utc2k::MAX` instead)



## [0.10.0](https://github.com/Blobfolio/utc2k/releases/tag/v0.10.0) - 2024-09-14

### Changed

* Bump `tz-rs` to `0.7`
* Bump MSRV to `1.81`
* Miscellaneous code lints



## [0.9.1](https://github.com/Blobfolio/utc2k/releases/tag/v0.9.1) - 2024-09-05

### Changed

* Miscellaneous code cleanup and lints
* Add `visit_bytes` to `Month`/`Weekday` deserializers
* Bump `brunch` to `0.6`



## [0.9.0](https://github.com/Blobfolio/utc2k/releases/tag/v0.9.0) - 2024-08-03

### New

* `Month::all`
* `Month::into_iter` (repeating month iterator)
* Enable de/serialization for `Month` (with `serde` crate feature)
* impl `FromStr` for `Month`
* impl `TryFrom<&[u8]>` for `Month`
* `Weekday::all`
* `Weekday::into_iter` (repeating week iterator)
* Enable de/serialization for `Weekday` (with `serde` crate feature)
* impl `FromStr` for `Weekday`
* impl `TryFrom<&[u8]>` for `Weekday`

### Breaking

* Bump MSRV to `1.80`

### Changed

* `Utc2k::unixtime` is now const
* `Utc2k::abs_diff` is now const



## [0.8.1](https://github.com/Blobfolio/utc2k/releases/tag/v0.8.1) - 2024-07-25

### Changed

* Miscellaneous code lints



## [0.8.0](https://github.com/Blobfolio/utc2k/releases/tag/v0.8.0) - 2024-02-08

### Removed

* `Borrow<[u8]>` for `FmtUtc2k`

### Other

* Miscellaneous doc/script cleanup



## [0.7.0](https://github.com/Blobfolio/utc2k/releases/tag/v0.7.0) - 2023-10-05

### New

* `Weekday::first_in_month`
* `Weekday::last_in_month`
* `Weekday::nth_in_month`



## [0.6.1](https://github.com/Blobfolio/utc2k/releases/tag/v0.6.1) - 2023-07-13

### Changed

* Update dev dependencies



## [0.6.0](https://github.com/Blobfolio/utc2k/releases/tag/v0.6.0) - 2023-06-01

### Changed

* Bump MSRV `1.70`
* Drop `once_cell` (in favor of new built-in types)
* Replace various `unsafe` blocks with safe alternatives
* Add debug/assertions for logical redundancy
* CI: test in debug and release modes
* CI: test MSRV



## [0.5.15](https://github.com/Blobfolio/utc2k/releases/tag/v0.5.15) - 2023-02-15

### Changed

* Improve `Deserialize` handling

### New

* impl `FromStr` for `Utc2k` (same as `TryFrom<&str>`)
* impl `FromStr` for `FmtUtc2k` (same as `TryFrom<&str>`)



## [0.5.14](https://github.com/Blobfolio/utc2k/releases/tag/v0.5.14) - 2023-02-04

### Changed

* Improve docs.rs environment detection
* Declare "serde" feature explicitly



## [0.5.13](https://github.com/Blobfolio/utc2k/releases/tag/v0.5.13) - 2023-01-26

### Changed

* Bump brunch `0.4`



## [0.5.12](https://github.com/Blobfolio/utc2k/releases/tag/v0.5.12) - 2023-01-01

### Fixed

* `utc2k::year` sometimes off by one!



## [0.5.11](https://github.com/Blobfolio/utc2k/releases/tag/v0.5.11) - 2022-12-29

### Changed

* Bump once_cell
* Update ci badge syntax (docs)



## [0.5.10](https://github.com/Blobfolio/utc2k/releases/tag/v0.5.10) - 2022-11-03

### Changed

* Bump once_cell



## [0.5.9](https://github.com/Blobfolio/utc2k/releases/tag/v0.5.9) - 2022-09-22

### Changed

* Update dependencies
* Improve docs



## [0.5.8](https://github.com/Blobfolio/utc2k/releases/tag/v0.5.8) - 2022-09-02

### Changed

* Update dependencies



## [0.5.7](https://github.com/Blobfolio/utc2k/releases/tag/v0.5.7) - 2022-08-19

### Changed

* Lower once_cell version specificity



## [0.5.6](https://github.com/Blobfolio/utc2k/releases/tag/v0.5.6) - 2022-08-14

### Changed

* Bump tz-rs `=0.6.14`



## [0.5.5](https://github.com/Blobfolio/utc2k/releases/tag/v0.5.5) - 2022-08-11

### Changed

* Bump tz-rs `=0.6.12`
* Bump fastrand `1.8.0`
* Remove `serde_yaml` dev dependency



## [0.5.4](https://github.com/Blobfolio/utc2k/releases/tag/v0.5.4) - 2022-07-22

### Added

* `Utc2k::abs_diff`
* `Utc2k::cmp_date`
* `Utc2k::cmp_time`
* `Utc2k::from_smooshed_date_str`
* `Utc2k::from_smooshed_datetime_str`
* `Utc2k::to_midnight`



## [0.5.3](https://github.com/Blobfolio/utc2k/releases/tag/v0.5.3) - 2022-07-14

### Changed

* Bump once_cell `=1.13.0`



## [0.5.2](https://github.com/Blobfolio/utc2k/releases/tag/v0.5.2) - 2022-07-04

### Changed

* Bump MSRV `1.62`.
* Bump tz-rs `=0.6.11`
* Bump once_cell `=1.12.1`



## [0.5.1](https://github.com/Blobfolio/utc2k/releases/tag/v0.5.1) - 2022-06-27

### Changed

* Bump tz-rs `=0.6.10`.




## [0.5.0](https://github.com/Blobfolio/utc2k/releases/tag/v0.5.0) - 2022-05-30

### Changed

* Bump once_cell to `=1.12.0`.
* Minor localization cache performance improvements.



## [0.4.5](https://github.com/Blobfolio/utc2k/releases/tag/v0.4.5) - 2022-05-19

### Changed

* Lock third-party dependency versions



## [0.4.4](https://github.com/Blobfolio/utc2k/releases/tag/v0.4.4) - 2022-05-02

### Added

* `LocalOffset::unixtime` (for returning the unix timestamp the offset applies to)
* `LocalOffset::localtime` (for returning the adjusted timestamp)

### Changed

* Timezone details are now statically cached after parsing, improving performance when multiple `LocalOffset` objects are created



## [0.4.3](https://github.com/Blobfolio/utc2k/releases/tag/v0.4.3) - 2022-04-30

### Added

* Optional crate feature `local`
* `LocalOffset` (for obtaining local UTC offset)

### Changed

* Bump MSRV to `1.57`
* Various doc and lint tweaks



## [0.4.2](https://github.com/Blobfolio/utc2k/releases/tag/v0.4.2) - 2022-03-27

### Added

* `utc2k::year` (fetch current year)

### Changed

* `Utc2k::parse_time_str` now accepts any `AsRef<[u8]>`
* `Utc2k::from_datetime_str` now accepts any `AsRef<[u8]>`
* `Utc2k::from_date_str` now accepts any `AsRef<[u8]>`
* impl `TryFrom<&[u8]>` for `Utc2k` and `FmtUtc2k`



## [0.4.1](https://github.com/Blobfolio/utc2k/releases/tag/v0.4.1) - 2022-03-20

### Added

* `FmtUtc2k::year`
* `Utc2k::parse_time_str`
* impl `Hash` for `Month`
* impl `Hash` for `Weekday`

### Misc

* Clean up parsing helpers;
* Improve string parsing performance;
* Improve `to_rfc2822` performance;



## [0.4.0](https://github.com/Blobfolio/utc2k/releases/tag/v0.4.0) - 2022-03-03

### Fixed

* Mistaken compiler optimizations can lead to overflow.

### Removed

* `Weekday::as_u8`



## [0.3.4](https://github.com/Blobfolio/utc2k/releases/tag/v0.3.4) - 2022-02-15

### Fixed

* `FmtUtc2k::to_rfc2822` and `Utc2k::to_rfc2822` now zero-pads days



## [0.3.3](https://github.com/Blobfolio/utc2k/releases/tag/v0.3.3) - 2022-01-06

### Added

* New enum: `utc2k::Month`
* `FmtUtc2k::eq::<&str>`
* `FmtUtc2k::eq::<&String>`
* `FmtUtc2k::eq::<String>`
* `FmtUtc2k::to_rfc2822`
* `Utc2k::from::<FmtUtc2k>`
* `Utc2k::from_rfc2822`
* `Utc2k::month_enum`
* `Utc2k::to_rfc2822`
* `Weekday::add::<u8>`..`Weekday::add::<u64>` and corresponding `AddAssign`s
* `Weekday::eq::<u8>`..`Weekday::eq::<u64>`
* `Weekday::from::<u8>`..`Weekday::from::<u64>`
* `Weekday::now`
* `Weekday::sub::<u8>`..`Weekday::sub::<u64>` and corresponding `SubAssign`s
* `Weekday::tomorrow`
* `Weekday::try_from::<&str>`
* `Weekday::try_from::<String>`
* `Weekday::yesterday`
  
### Changed

* Performance optimizations for `Utc2k::sub::<u32>`
* `Weekday` is now represented as a `u8`

### Deprecated

* `Weekday::as_u8`



## [0.3.2](https://github.com/Blobfolio/utc2k/releases/tag/v0.3.2) - 2021-12-13

### Added

* `Utc2k::to_rfc3339`
* `FmtUtc2k::to_rfc3339`



## [0.3.1](https://github.com/Blobfolio/utc2k/releases/tag/v0.3.1) - 2021-11-27

### Changed

* Replace the dev-dependency `chrono` with `time`.



## [0.3.0](https://github.com/Blobfolio/utc2k/releases/tag/v0.3.0) - 2021-10-21

### Added

* This changelog! Haha.

### Changed

* Use Rust edition 2021.
