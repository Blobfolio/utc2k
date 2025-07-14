/*!
# UTC2K - Year
*/

#![expect(clippy::inline_always, reason = "Foundational.")]

use crate::Weekday;
use std::cmp::Ordering;



/// # Helper: Define Enum.
macro_rules! year {
	(
		$($k:ident $v:tt $v16:tt $d1:ident $d2:ident $sec:literal),+,
		@last $last_k:ident $last_v:tt $last_v16:tt $last_d1:ident $last_d2:ident $last_sec:literal
		$(,)?
	) => (
		#[repr(u8)]
		#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
		/// # Year.
		///
		/// This enum holds the years between `2000..=2099`, mainly to ensure
		/// we can't possibly accidentally operate on a 1990 or whatever.
		pub(crate) enum Year {
			$($k = $v),+,
			$last_k = $last_v
		}

		impl Year {
			/// # As String.
			///
			/// Return the year as a string with both a leading and trailing
			/// space, since this is only actually used for the RFC2822
			/// methods.
			pub(crate) const fn as_str(self) -> &'static str {
				match self {
					$( Self::$k => concat!(" ", stringify!($v16), " ") ),+,
					Self::$last_k => concat!(" ", stringify!($last_v16), " "),
				}
			}

			/// # From `u8` (Saturating).
			///
			/// Return the year corresponding to the `u8`, or `Self::Y2k99` if
			/// out of range.
			pub(crate) const fn from_u8(src: u8) -> Self {
				match src {
					$($v => Self::$k),+,
					_ => Self::$last_k,
				}
			}

			/// # From `u16` (Checked).
			///
			/// Return the year corresponding to the `u16`, or `None` if
			/// out of range.
			pub(crate) const fn from_u16_checked(src: u16) -> Option<Self> {
				match src {
					$($v16 => Some(Self::$k)),+,
					$last_v16 => Some(Self::$last_k),
					_ => None,
				}
			}

			#[inline(always)]
			/// # Cumulative Unixtime.
			///
			/// Return the total number of unix timestamp seconds before the
			/// current year.
			///
			/// (This gives us a baseline to work with when converting a
			/// parsed date/time back to unixtime.)
			pub(crate) const fn unixtime(self) -> u32 {
				match self {
					$(Self::$k => $sec),+,
					Self::$last_k => $last_sec,
				}
			}
		}
	);
}

year!{
	Y2k00  0 2000 Digit0 Digit0 946_684_800,
	Y2k01  1 2001 Digit0 Digit1 978_307_200,
	Y2k02  2 2002 Digit0 Digit2 1_009_843_200,
	Y2k03  3 2003 Digit0 Digit3 1_041_379_200,
	Y2k04  4 2004 Digit0 Digit4 1_072_915_200,
	Y2k05  5 2005 Digit0 Digit5 1_104_537_600,
	Y2k06  6 2006 Digit0 Digit6 1_136_073_600,
	Y2k07  7 2007 Digit0 Digit7 1_167_609_600,
	Y2k08  8 2008 Digit0 Digit8 1_199_145_600,
	Y2k09  9 2009 Digit0 Digit9 1_230_768_000,
	Y2k10 10 2010 Digit1 Digit0 1_262_304_000,
	Y2k11 11 2011 Digit1 Digit1 1_293_840_000,
	Y2k12 12 2012 Digit1 Digit2 1_325_376_000,
	Y2k13 13 2013 Digit1 Digit3 1_356_998_400,
	Y2k14 14 2014 Digit1 Digit4 1_388_534_400,
	Y2k15 15 2015 Digit1 Digit5 1_420_070_400,
	Y2k16 16 2016 Digit1 Digit6 1_451_606_400,
	Y2k17 17 2017 Digit1 Digit7 1_483_228_800,
	Y2k18 18 2018 Digit1 Digit8 1_514_764_800,
	Y2k19 19 2019 Digit1 Digit9 1_546_300_800,
	Y2k20 20 2020 Digit2 Digit0 1_577_836_800,
	Y2k21 21 2021 Digit2 Digit1 1_609_459_200,
	Y2k22 22 2022 Digit2 Digit2 1_640_995_200,
	Y2k23 23 2023 Digit2 Digit3 1_672_531_200,
	Y2k24 24 2024 Digit2 Digit4 1_704_067_200,
	Y2k25 25 2025 Digit2 Digit5 1_735_689_600,
	Y2k26 26 2026 Digit2 Digit6 1_767_225_600,
	Y2k27 27 2027 Digit2 Digit7 1_798_761_600,
	Y2k28 28 2028 Digit2 Digit8 1_830_297_600,
	Y2k29 29 2029 Digit2 Digit9 1_861_920_000,
	Y2k30 30 2030 Digit3 Digit0 1_893_456_000,
	Y2k31 31 2031 Digit3 Digit1 1_924_992_000,
	Y2k32 32 2032 Digit3 Digit2 1_956_528_000,
	Y2k33 33 2033 Digit3 Digit3 1_988_150_400,
	Y2k34 34 2034 Digit3 Digit4 2_019_686_400,
	Y2k35 35 2035 Digit3 Digit5 2_051_222_400,
	Y2k36 36 2036 Digit3 Digit6 2_082_758_400,
	Y2k37 37 2037 Digit3 Digit7 2_114_380_800,
	Y2k38 38 2038 Digit3 Digit8 2_145_916_800,
	Y2k39 39 2039 Digit3 Digit9 2_177_452_800,
	Y2k40 40 2040 Digit4 Digit0 2_208_988_800,
	Y2k41 41 2041 Digit4 Digit1 2_240_611_200,
	Y2k42 42 2042 Digit4 Digit2 2_272_147_200,
	Y2k43 43 2043 Digit4 Digit3 2_303_683_200,
	Y2k44 44 2044 Digit4 Digit4 2_335_219_200,
	Y2k45 45 2045 Digit4 Digit5 2_366_841_600,
	Y2k46 46 2046 Digit4 Digit6 2_398_377_600,
	Y2k47 47 2047 Digit4 Digit7 2_429_913_600,
	Y2k48 48 2048 Digit4 Digit8 2_461_449_600,
	Y2k49 49 2049 Digit4 Digit9 2_493_072_000,
	Y2k50 50 2050 Digit5 Digit0 2_524_608_000,
	Y2k51 51 2051 Digit5 Digit1 2_556_144_000,
	Y2k52 52 2052 Digit5 Digit2 2_587_680_000,
	Y2k53 53 2053 Digit5 Digit3 2_619_302_400,
	Y2k54 54 2054 Digit5 Digit4 2_650_838_400,
	Y2k55 55 2055 Digit5 Digit5 2_682_374_400,
	Y2k56 56 2056 Digit5 Digit6 2_713_910_400,
	Y2k57 57 2057 Digit5 Digit7 2_745_532_800,
	Y2k58 58 2058 Digit5 Digit8 2_777_068_800,
	Y2k59 59 2059 Digit5 Digit9 2_808_604_800,
	Y2k60 60 2060 Digit6 Digit0 2_840_140_800,
	Y2k61 61 2061 Digit6 Digit1 2_871_763_200,
	Y2k62 62 2062 Digit6 Digit2 2_903_299_200,
	Y2k63 63 2063 Digit6 Digit3 2_934_835_200,
	Y2k64 64 2064 Digit6 Digit4 2_966_371_200,
	Y2k65 65 2065 Digit6 Digit5 2_997_993_600,
	Y2k66 66 2066 Digit6 Digit6 3_029_529_600,
	Y2k67 67 2067 Digit6 Digit7 3_061_065_600,
	Y2k68 68 2068 Digit6 Digit8 3_092_601_600,
	Y2k69 69 2069 Digit6 Digit9 3_124_224_000,
	Y2k70 70 2070 Digit7 Digit0 3_155_760_000,
	Y2k71 71 2071 Digit7 Digit1 3_187_296_000,
	Y2k72 72 2072 Digit7 Digit2 3_218_832_000,
	Y2k73 73 2073 Digit7 Digit3 3_250_454_400,
	Y2k74 74 2074 Digit7 Digit4 3_281_990_400,
	Y2k75 75 2075 Digit7 Digit5 3_313_526_400,
	Y2k76 76 2076 Digit7 Digit6 3_345_062_400,
	Y2k77 77 2077 Digit7 Digit7 3_376_684_800,
	Y2k78 78 2078 Digit7 Digit8 3_408_220_800,
	Y2k79 79 2079 Digit7 Digit9 3_439_756_800,
	Y2k80 80 2080 Digit8 Digit0 3_471_292_800,
	Y2k81 81 2081 Digit8 Digit1 3_502_915_200,
	Y2k82 82 2082 Digit8 Digit2 3_534_451_200,
	Y2k83 83 2083 Digit8 Digit3 3_565_987_200,
	Y2k84 84 2084 Digit8 Digit4 3_597_523_200,
	Y2k85 85 2085 Digit8 Digit5 3_629_145_600,
	Y2k86 86 2086 Digit8 Digit6 3_660_681_600,
	Y2k87 87 2087 Digit8 Digit7 3_692_217_600,
	Y2k88 88 2088 Digit8 Digit8 3_723_753_600,
	Y2k89 89 2089 Digit8 Digit9 3_755_376_000,
	Y2k90 90 2090 Digit9 Digit0 3_786_912_000,
	Y2k91 91 2091 Digit9 Digit1 3_818_448_000,
	Y2k92 92 2092 Digit9 Digit2 3_849_984_000,
	Y2k93 93 2093 Digit9 Digit3 3_881_606_400,
	Y2k94 94 2094 Digit9 Digit4 3_913_142_400,
	Y2k95 95 2095 Digit9 Digit5 3_944_678_400,
	Y2k96 96 2096 Digit9 Digit6 3_976_214_400,
	Y2k97 97 2097 Digit9 Digit7 4_007_836_800,
	Y2k98 98 2098 Digit9 Digit8 4_039_372_800,
	@last
	Y2k99 99 2099 Digit9 Digit9 4_070_908_800,
}

impl Year {
	#[inline]
	#[must_use]
	/// # Constant Compare.
	pub(crate) const fn cmp(a: Self, b: Self) -> Ordering {
		let a = a as u8;
		let b = b as u8;
		if a == b { Ordering::Equal }
		else if a < b { Ordering::Less }
		else { Ordering::Greater }
	}

	#[inline(always)]
	/// # Full Year.
	///
	/// Convert our two-digit year abbreviation back to a proper
	/// four-digit masterpiece.
	pub(crate) const fn full(self) -> u16 { self as u16 + 2000 }

	#[inline(always)]
	/// # Leap Year?
	///
	/// Thanks to the type constraints, we can ignore the 100/400 nuance; all
	/// multiples of four are leaped.
	pub(crate) const fn leap(self) -> bool { (self as u8).is_multiple_of(4) }

	/// # Weekday.
	///
	/// Return the weekday the year starts on.
	pub(crate) const fn weekday(self) -> Weekday {
		match self {
			Self::Y2k00 | Self::Y2k05 | Self::Y2k11 | Self::Y2k22 | Self::Y2k28 | Self::Y2k33 | Self::Y2k39 | Self::Y2k50 | Self::Y2k56 | Self::Y2k61 | Self::Y2k67 | Self::Y2k78 | Self::Y2k84 | Self::Y2k89 | Self::Y2k95 => Weekday::Saturday,
			Self::Y2k01 | Self::Y2k07 | Self::Y2k18 | Self::Y2k24 | Self::Y2k29 | Self::Y2k35 | Self::Y2k46 | Self::Y2k52 | Self::Y2k57 | Self::Y2k63 | Self::Y2k74 | Self::Y2k80 | Self::Y2k85 | Self::Y2k91 => Weekday::Monday,
			Self::Y2k02 | Self::Y2k08 | Self::Y2k13 | Self::Y2k19 | Self::Y2k30 | Self::Y2k36 | Self::Y2k41 | Self::Y2k47 | Self::Y2k58 | Self::Y2k64 | Self::Y2k69 | Self::Y2k75 | Self::Y2k86 | Self::Y2k92 | Self::Y2k97 => Weekday::Tuesday,
			Self::Y2k03 | Self::Y2k14 | Self::Y2k20 | Self::Y2k25 | Self::Y2k31 | Self::Y2k42 | Self::Y2k48 | Self::Y2k53 | Self::Y2k59 | Self::Y2k70 | Self::Y2k76 | Self::Y2k81 | Self::Y2k87 | Self::Y2k98 => Weekday::Wednesday,
			Self::Y2k04 | Self::Y2k09 | Self::Y2k15 | Self::Y2k26 | Self::Y2k32 | Self::Y2k37 | Self::Y2k43 | Self::Y2k54 | Self::Y2k60 | Self::Y2k65 | Self::Y2k71 | Self::Y2k82 | Self::Y2k88 | Self::Y2k93 | Self::Y2k99 => Weekday::Thursday,
			Self::Y2k06 | Self::Y2k12 | Self::Y2k17 | Self::Y2k23 | Self::Y2k34 | Self::Y2k40 | Self::Y2k45 | Self::Y2k51 | Self::Y2k62 | Self::Y2k68 | Self::Y2k73 | Self::Y2k79 | Self::Y2k90 | Self::Y2k96 => Weekday::Sunday,
			_ => Weekday::Friday,
		}
	}
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	/// # Test First of Year.
	fn t_year_start() {
		for y in 0..=99_u8 {
			let c = time::Date::from_calendar_date(y as i32 + 2000, time::Month::January, 1)
				.expect("Unable to create time::Date.");
			assert_eq!(
				Year::from_u8(y).weekday().as_str(),
				c.weekday().to_string(),
				"Failed with year {y}"
			);
		}
	}
}
