/*!
# Benchmark: UTC2K
*/

use brunch::{
	Bench,
	benches,
};
use utc2k::{
	Local2k,
	Utc2k,
};



benches!(
	Bench::new("utc2k::Utc2k::now()").run(|| Utc2k::now()),
	Bench::new("utc2k::Local2k::now()").run(|| Local2k::now()),

	Bench::spacer(),

	Bench::new("String::from(Local2k)")
		.run_seeded(
			Local2k::from_utc2k(Utc2k::new(2025, 6, 19, 19, 31, 22)),
			|u| String::from(u),
		),

	Bench::new("utc2k::Local2k::to_rfc2822()")
		.run_seeded(
			Local2k::from_utc2k(Utc2k::new(2025, 6, 19, 19, 31, 22)),
			|u| u.to_rfc2822(),
		),

	Bench::new("utc2k::Local2k::to_rfc3339()")
		.run_seeded(
			Local2k::from_utc2k(Utc2k::new(2025, 6, 19, 19, 31, 22)),
			|u| u.to_rfc3339(),
		),
);
