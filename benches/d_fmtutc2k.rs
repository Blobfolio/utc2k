/*!
# Benchmark: Formatted UTC2K
*/

use brunch::{
	Bench,
	benches,
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

	Bench::new("utc2k::FmtUtc2k", "from(Utc2k)")
		.with_setup(Utc2k::new(2019, 4, 10, 18, 18, 55), FmtUtc2k::from),

	Bench::new("utc2k::FmtUtc2k", "to_string()")
		.with_setup(FmtUtc2k::from(Utc2k::MAX_UNIXTIME), |x| x.to_string()),

	Bench::new("utc2k::FmtUtc2k", "to_rfc2822()")
		.with_setup(FmtUtc2k::from(Utc2k::MAX_UNIXTIME), |x| x.to_rfc2822()),

	Bench::new("utc2k::FmtUtc2k", "to_rfc3339()")
		.with_setup(FmtUtc2k::from(Utc2k::MAX_UNIXTIME), |x| x.to_rfc3339()),
);
