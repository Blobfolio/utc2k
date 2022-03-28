# Changelog



## [0.4.2](https://github.com/Blobfolio/utc2k/releases/tag/v0.4.2) - TBD

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
