/*!
# Benchmark: UTC2K
*/

use brunch::{
	Bench,
	benches,
};
use std::time::Duration;
use utc2k::Utc2k;



benches!(
	Bench::new("utc2k::Utc2k", "now()")
		.timed(Duration::from_secs(1))
		.with(|| Utc2k::now()),
	Bench::new("utc2k::Utc2k", "now_local()")
		.timed(Duration::from_secs(1))
		.with(|| Utc2k::now_local()),
);
