/*!
# Benchmark: Formatted UTC2K
*/

use brunch::{
	Bench,
	benches,
};
use chrono::{
	Utc,
	TimeZone,
};
use utc2k::{
	FmtUtc2k,
	Utc2k,
};



benches!(
	Bench::new("utc2k::FmtUtc2k", "try_from(valid datetime)")
		.with(|| FmtUtc2k::try_from("2019-04-10 18:18:55")),

	Bench::new("utc2k::FmtUtc2k", "try_from(valid date)")
		.with(|| FmtUtc2k::try_from("2019-04-10")),

	Bench::new("utc2k::FmtUtc2k", "to_string()")
		.with(|| FmtUtc2k::from(Utc2k::MAX_UNIXTIME).to_string()),

	Bench::new("chrono::Utc", "to_string()")
		.with(||
			Utc.timestamp(Utc2k::MAX_UNIXTIME as i64, 0)
				.format("%Y-%m-%d %H:%M:%S")
				.to_string()
		)
);
