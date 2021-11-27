/*!
# Benchmark: UTC2K
*/

use brunch::{
	Bench,
	benches,
};
use utc2k::Utc2k;



benches!(
	Bench::new("utc2k::Utc2k::from", "(u32)")
		.with(|| Utc2k::from(1_624_593_661_u32)),

	Bench::new("utc2k::Utc2k::try_from", "(&str)")
		.with(|| Utc2k::try_from("2010-10-31 04:15:30").unwrap()),

	Bench::new("utc2k::Utc2k", "unixtime()")
		.with_setup(Utc2k::from(1_624_593_661_u32), |u| u.unixtime())
);
