/*!
# Benchmark: UTC2K
*/

use brunch::{
	Bench,
	benches,
};
use utc2k::Utc2k;



benches!(
	Bench::new("utc2k::Utc2k::now()").run(|| Utc2k::now()),
	Bench::new("utc2k::Utc2k::now_local()").run(|| Utc2k::now_local()),
);
