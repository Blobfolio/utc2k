/*!
# Benchmark: UTC2K
*/

use brunch::{
	Bench,
	benches,
};
use chrono::{
	Utc,
	TimeZone,
};
use std::convert::TryFrom;
use utc2k::Utc2k;



benches!(
	Bench::new("utc2k::Utc2k::from", "(u32)")
		.with(|| Utc2k::from(Utc2k::MAX_UNIXTIME)),

	Bench::new("chrono::Utc.timestamp", "(i64, _)")
		.with(|| Utc.timestamp(Utc2k::MAX_UNIXTIME as i64, 0)),

	Bench::new("utc2k::Utc2k::try_from", "(&str)")
		.with(|| Utc2k::try_from("2010-10-31 04:15:30").unwrap()),

	Bench::new("chrono::Utc.datetime_from_str", "(&str)")
		.with(|| Utc.datetime_from_str("2010-10-31 04:15:30", "%Y-%m-%d %H:%M:%S")),

	Bench::new("utc2k::Utc2k", "unixtime()")
		.with_setup(Utc2k::from(Utc2k::MAX_UNIXTIME), |u| u.unixtime()),

	Bench::new("chrono::DateTime<Utc>", "timestamp()")
		.with_setup(Utc.timestamp(Utc2k::MAX_UNIXTIME as i64, 0), |c| c.timestamp())
);
