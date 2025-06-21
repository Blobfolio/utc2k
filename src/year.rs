/*!
# UTC2K - Year
*/

#![expect(clippy::inline_always, reason = "Foundational.")]

use crate::DateChar;



/// # Helper: Define Enum.
macro_rules! year {
	(@as_str 0) => ( " 2000 " );
	(@as_str 1) => ( " 2001 " );
	(@as_str 2) => ( " 2002 " );
	(@as_str 3) => ( " 2003 " );
	(@as_str 4) => ( " 2004 " );
	(@as_str 5) => ( " 2005 " );
	(@as_str 6) => ( " 2006 " );
	(@as_str 7) => ( " 2007 " );
	(@as_str 8) => ( " 2008 " );
	(@as_str 9) => ( " 2009 " );
	(@as_str $tt:tt) => ( concat!(" 20", $tt, " ") );

	(
		$($k:ident $v:tt $d1:ident $d2:ident $sec:literal),+,
		@last $last_k:ident $last_v:tt $last_d1:ident $last_d2:ident $last_sec:literal
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
					$( Self::$k => year!(@as_str $v) ),+,
					Self::$last_k => year!(@as_str $last_v),
				}
			}

			#[inline(always)]
			/// # Double Digit.
			///
			/// Return the year as two digits.
			pub(crate) const fn dd(self) -> [DateChar; 2] {
				match self {
					$(Self::$k => [DateChar::$d1, DateChar::$d2]),+,
					Self::$last_k => [DateChar::$last_d1, DateChar::$last_d2],
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
	Y2k00  0 Digit0 Digit0 946_684_800,
	Y2k01  1 Digit0 Digit1 978_307_200,
	Y2k02  2 Digit0 Digit2 1_009_843_200,
	Y2k03  3 Digit0 Digit3 1_041_379_200,
	Y2k04  4 Digit0 Digit4 1_072_915_200,
	Y2k05  5 Digit0 Digit5 1_104_537_600,
	Y2k06  6 Digit0 Digit6 1_136_073_600,
	Y2k07  7 Digit0 Digit7 1_167_609_600,
	Y2k08  8 Digit0 Digit8 1_199_145_600,
	Y2k09  9 Digit0 Digit9 1_230_768_000,
	Y2k10 10 Digit1 Digit0 1_262_304_000,
	Y2k11 11 Digit1 Digit1 1_293_840_000,
	Y2k12 12 Digit1 Digit2 1_325_376_000,
	Y2k13 13 Digit1 Digit3 1_356_998_400,
	Y2k14 14 Digit1 Digit4 1_388_534_400,
	Y2k15 15 Digit1 Digit5 1_420_070_400,
	Y2k16 16 Digit1 Digit6 1_451_606_400,
	Y2k17 17 Digit1 Digit7 1_483_228_800,
	Y2k18 18 Digit1 Digit8 1_514_764_800,
	Y2k19 19 Digit1 Digit9 1_546_300_800,
	Y2k20 20 Digit2 Digit0 1_577_836_800,
	Y2k21 21 Digit2 Digit1 1_609_459_200,
	Y2k22 22 Digit2 Digit2 1_640_995_200,
	Y2k23 23 Digit2 Digit3 1_672_531_200,
	Y2k24 24 Digit2 Digit4 1_704_067_200,
	Y2k25 25 Digit2 Digit5 1_735_689_600,
	Y2k26 26 Digit2 Digit6 1_767_225_600,
	Y2k27 27 Digit2 Digit7 1_798_761_600,
	Y2k28 28 Digit2 Digit8 1_830_297_600,
	Y2k29 29 Digit2 Digit9 1_861_920_000,
	Y2k30 30 Digit3 Digit0 1_893_456_000,
	Y2k31 31 Digit3 Digit1 1_924_992_000,
	Y2k32 32 Digit3 Digit2 1_956_528_000,
	Y2k33 33 Digit3 Digit3 1_988_150_400,
	Y2k34 34 Digit3 Digit4 2_019_686_400,
	Y2k35 35 Digit3 Digit5 2_051_222_400,
	Y2k36 36 Digit3 Digit6 2_082_758_400,
	Y2k37 37 Digit3 Digit7 2_114_380_800,
	Y2k38 38 Digit3 Digit8 2_145_916_800,
	Y2k39 39 Digit3 Digit9 2_177_452_800,
	Y2k40 40 Digit4 Digit0 2_208_988_800,
	Y2k41 41 Digit4 Digit1 2_240_611_200,
	Y2k42 42 Digit4 Digit2 2_272_147_200,
	Y2k43 43 Digit4 Digit3 2_303_683_200,
	Y2k44 44 Digit4 Digit4 2_335_219_200,
	Y2k45 45 Digit4 Digit5 2_366_841_600,
	Y2k46 46 Digit4 Digit6 2_398_377_600,
	Y2k47 47 Digit4 Digit7 2_429_913_600,
	Y2k48 48 Digit4 Digit8 2_461_449_600,
	Y2k49 49 Digit4 Digit9 2_493_072_000,
	Y2k50 50 Digit5 Digit0 2_524_608_000,
	Y2k51 51 Digit5 Digit1 2_556_144_000,
	Y2k52 52 Digit5 Digit2 2_587_680_000,
	Y2k53 53 Digit5 Digit3 2_619_302_400,
	Y2k54 54 Digit5 Digit4 2_650_838_400,
	Y2k55 55 Digit5 Digit5 2_682_374_400,
	Y2k56 56 Digit5 Digit6 2_713_910_400,
	Y2k57 57 Digit5 Digit7 2_745_532_800,
	Y2k58 58 Digit5 Digit8 2_777_068_800,
	Y2k59 59 Digit5 Digit9 2_808_604_800,
	Y2k60 60 Digit6 Digit0 2_840_140_800,
	Y2k61 61 Digit6 Digit1 2_871_763_200,
	Y2k62 62 Digit6 Digit2 2_903_299_200,
	Y2k63 63 Digit6 Digit3 2_934_835_200,
	Y2k64 64 Digit6 Digit4 2_966_371_200,
	Y2k65 65 Digit6 Digit5 2_997_993_600,
	Y2k66 66 Digit6 Digit6 3_029_529_600,
	Y2k67 67 Digit6 Digit7 3_061_065_600,
	Y2k68 68 Digit6 Digit8 3_092_601_600,
	Y2k69 69 Digit6 Digit9 3_124_224_000,
	Y2k70 70 Digit7 Digit0 3_155_760_000,
	Y2k71 71 Digit7 Digit1 3_187_296_000,
	Y2k72 72 Digit7 Digit2 3_218_832_000,
	Y2k73 73 Digit7 Digit3 3_250_454_400,
	Y2k74 74 Digit7 Digit4 3_281_990_400,
	Y2k75 75 Digit7 Digit5 3_313_526_400,
	Y2k76 76 Digit7 Digit6 3_345_062_400,
	Y2k77 77 Digit7 Digit7 3_376_684_800,
	Y2k78 78 Digit7 Digit8 3_408_220_800,
	Y2k79 79 Digit7 Digit9 3_439_756_800,
	Y2k80 80 Digit8 Digit0 3_471_292_800,
	Y2k81 81 Digit8 Digit1 3_502_915_200,
	Y2k82 82 Digit8 Digit2 3_534_451_200,
	Y2k83 83 Digit8 Digit3 3_565_987_200,
	Y2k84 84 Digit8 Digit4 3_597_523_200,
	Y2k85 85 Digit8 Digit5 3_629_145_600,
	Y2k86 86 Digit8 Digit6 3_660_681_600,
	Y2k87 87 Digit8 Digit7 3_692_217_600,
	Y2k88 88 Digit8 Digit8 3_723_753_600,
	Y2k89 89 Digit8 Digit9 3_755_376_000,
	Y2k90 90 Digit9 Digit0 3_786_912_000,
	Y2k91 91 Digit9 Digit1 3_818_448_000,
	Y2k92 92 Digit9 Digit2 3_849_984_000,
	Y2k93 93 Digit9 Digit3 3_881_606_400,
	Y2k94 94 Digit9 Digit4 3_913_142_400,
	Y2k95 95 Digit9 Digit5 3_944_678_400,
	Y2k96 96 Digit9 Digit6 3_976_214_400,
	Y2k97 97 Digit9 Digit7 4_007_836_800,
	Y2k98 98 Digit9 Digit8 4_039_372_800,
	@last Y2k99 99 Digit9 Digit9 4_070_908_800,
}

impl Year {
	#[inline(always)]
	/// # Full Year.
	///
	/// Convert our two-digit year abbreviation back to a proper
	/// four-digit masterpiece.
	pub(crate) const fn full(self) -> u16 { self as u16 + 2000 }

	#[inline(always)]
	/// # Leap Year?
	pub(crate) const fn leap(self) -> bool {
		matches!(
			self,
			Self::Y2k00 | Self::Y2k04 | Self::Y2k08 | Self::Y2k12 | Self::Y2k16 |
			Self::Y2k20 | Self::Y2k24 | Self::Y2k28 | Self::Y2k32 | Self::Y2k36 |
			Self::Y2k40 | Self::Y2k44 | Self::Y2k48 | Self::Y2k52 | Self::Y2k56 |
			Self::Y2k60 | Self::Y2k64 | Self::Y2k68 | Self::Y2k72 | Self::Y2k76 |
			Self::Y2k80 | Self::Y2k84 | Self::Y2k88 | Self::Y2k92 | Self::Y2k96
		)
	}
}
