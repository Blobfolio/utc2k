/*!
# Benchmark: Weekdays and Months.
*/

use brunch::{
	Bench,
	benches,
};
use utc2k::{
	Weekday,
	Month,
};



benches!(
	Bench::new("utc2k::Weekday::try_from(sat)")
		.run(|| Weekday::try_from("sat")),

	Bench::new("utc2k::Weekday::try_from(Saturday)")
		.run(|| Weekday::try_from("Saturday")),

	Bench::spacer(),

	Bench::new("utc2k::Weekday::Sunday + 1_u8")
		.run(|| Weekday::Sunday + 1_u8),

	Bench::new("utc2k::Weekday::Sunday + 77_u8")
		.run(|| Weekday::Sunday + 77_u8),

	Bench::new("utc2k::Weekday::Sunday - 1_u8")
		.run(|| Weekday::Sunday - 1_u8),

	Bench::new("utc2k::Weekday::Sunday - 77_u8")
		.run(|| Weekday::Sunday - 77_u8),

	Bench::spacer(),

	Bench::new("utc2k::Month::try_from(jun)")
		.run(|| Month::try_from("jun")),

	Bench::new("utc2k::Month::try_from(June)")
		.run(|| Month::try_from("June")),

	Bench::spacer(),

	Bench::new("utc2k::Month::January + 1_u8")
		.run(|| Month::January + 1_u8),

	Bench::new("utc2k::Month::January + 77_u8")
		.run(|| Month::January + 77_u8),

	Bench::new("utc2k::Month::January - 1_u8")
		.run(|| Month::January - 1_u8),

	Bench::new("utc2k::Month::January - 77_u8")
		.run(|| Month::January - 77_u8),
);
