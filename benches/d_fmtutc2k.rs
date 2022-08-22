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
	Bench::new("utc2k::FmtUtc2k::try_from(valid datetime)")
		.run(|| FmtUtc2k::try_from("2019-04-10 18:18:55")),

	Bench::new("utc2k::FmtUtc2k::try_from(valid date)")
		.run(|| FmtUtc2k::try_from("2019-04-10")),

	Bench::new("utc2k::FmtUtc2k::from(Utc2k)")
		.run_seeded(Utc2k::new(2019, 4, 10, 18, 18, 55), FmtUtc2k::from),

	Bench::spacer(),

	Bench::new("utc2k::FmtUtc2k::to_string()")
		.run_seeded(FmtUtc2k::from(Utc2k::MAX_UNIXTIME), |x| x.to_string()),

	Bench::new("utc2k::FmtUtc2k::to_rfc2822()")
		.run_seeded(FmtUtc2k::from(Utc2k::MAX_UNIXTIME), |x| x.to_rfc2822()),

	Bench::new("utc2k::FmtUtc2k::to_rfc3339()")
		.run_seeded(FmtUtc2k::from(Utc2k::MAX_UNIXTIME), |x| x.to_rfc3339()),
);
