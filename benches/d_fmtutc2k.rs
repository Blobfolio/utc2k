/*!
# Benchmark: FMTUTC2K
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
	Bench::new("utc2k::FmtUtc2k::from_unixtime(u32)")
		.run(|| FmtUtc2k::from_unixtime(1_624_593_661_u32)),

	Bench::spacer(),

	Bench::new("utc2k::FmtUtc2k::from_ascii(date)")
		.run(|| FmtUtc2k::from_ascii(b"2010-10-31").unwrap()),

	Bench::new("utc2k::FmtUtc2k::from_ascii(datetime)")
		.run(|| FmtUtc2k::from_ascii(b"2010-10-31 04:15:30").unwrap()),

	Bench::spacer(),

	Bench::new("utc2k::FmtUtc2k::from(Utc2k)")
		.run_seeded(Utc2k::from_ascii(b"2019-04-10 18:18:55").unwrap(), FmtUtc2k::from),

	Bench::spacer(),

	Bench::new("String::from(Utc2k)")
		.run_seeded(FmtUtc2k::from(Utc2k::MAX_UNIXTIME), String::from),

	Bench::new("utc2k::FmtUtc2k::to_string()")
		.run_seeded(FmtUtc2k::MAX, |u| u.to_string()),

	Bench::spacer(),

	Bench::new("utc2k::FmtUtc2k::to_rfc2822()")
		.run_seeded(FmtUtc2k::MAX, |u| u.to_rfc2822()),

	Bench::new("utc2k::FmtUtc2k::from_rfc2822(Tue, 10 Jul 2003 10:52:37 +0000)")
		.run(|| FmtUtc2k::from_rfc2822(b"Tue, 10 Jul 2003 10:52:37 +0000")),
);
