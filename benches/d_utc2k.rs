/*!
# Benchmark: UTC2K
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
	Bench::new("utc2k::Utc2k::from_unixtime(u32)")
		.run(|| Utc2k::from_unixtime(1_624_593_661_u32)),

	Bench::spacer(),

	Bench::new("utc2k::Utc2k::from_ascii(date)")
		.run(|| Utc2k::from_ascii(b"2010-10-31").unwrap()),

	Bench::new("utc2k::Utc2k::checked_from_ascii(date)")
		.run(|| Utc2k::checked_from_ascii(b"2010-10-31").unwrap()),

	Bench::spacer(),

	Bench::new("utc2k::Utc2k::from_ascii(datetime)")
		.run(|| Utc2k::from_ascii(b"2010-10-31 04:15:30").unwrap()),

	Bench::new("utc2k::Utc2k::checked_from_ascii(datetime)")
		.run(|| Utc2k::checked_from_ascii(b"2010-10-31 04:15:30").unwrap()),

	Bench::spacer(),

	Bench::new("utc2k::Utc2k::from(FmtUtc2k)")
		.run_seeded(FmtUtc2k::from_ascii(b"2019-04-10 18:18:55").unwrap(), Utc2k::from),

	Bench::spacer(),

	Bench::new("utc2k::Utc2k::unixtime()")
		.run_seeded(Utc2k::from_unixtime(1_624_593_661_u32), Utc2k::unixtime),

	Bench::spacer(),

	Bench::new("String::from(Utc2k)")
		.run_seeded(Utc2k::MAX, String::from),

	Bench::new("utc2k::Utc2k::to_string()")
		.run_seeded(Utc2k::MAX, |u| u.to_string()),

	Bench::spacer(),

	Bench::new("utc2k::Utc2k::formatted_custom()")
		.run_seeded(Utc2k::MAX, |u| u.formatted_custom(
			"[day @abbr], [day] [month @abbr] [year] [hour]:[minute]:[second] +0000"
		)),

	Bench::new("utc2k::Utc2k::to_rfc2822()")
		.run_seeded(Utc2k::MAX, |u| u.to_rfc2822()),

	Bench::new("utc2k::Utc2k::from_rfc2822(Tue, 10 Jul 2003 10:52:37 +0000)")
		.run(|| Utc2k::from_rfc2822(b"Tue, 10 Jul 2003 10:52:37 +0000")),
);
