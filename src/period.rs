/*!
# UTC2K - Period
*/

use crate::macros;



/// # Helper: Enum Definition.
macro_rules! period {
	(@byte A) => ( b'A' );
	(@byte M) => ( b'M' );
	(@byte P) => ( b'P' );
	(@byte a) => ( b'a' );
	(@byte m) => ( b'm' );
	(@byte p) => ( b'p' );

	(@char A) => ( 'A' );
	(@char M) => ( 'M' );
	(@char P) => ( 'P' );
	(@char a) => ( 'a' );
	(@char m) => ( 'm' );
	(@char p) => ( 'p' );

	(@str $ty1:tt $ty2:tt) =>   ( concat!(period!(@char $ty1), period!(@char $ty2)) );
	(@str_p $ty1:tt $ty2:tt) => ( concat!(period!(@char $ty1), ".", period!(@char $ty2), ".") );

	( $(
		$( #[doc = $doc:expr] )+
		$k:ident $lower1:tt $lower2:tt $upper1:tt $upper2:tt,
	)+ ) => (
		#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
		/// # Period (AM/PM).
		///
		/// This enum holds the the periods of the day required to
		/// contextualize 12-hour clock representations of time.
		///
		/// See also [`Utc2k::hour_period`](crate::Utc2k::hour_period).
		pub enum Period {
			$(
				$( #[doc = $doc])+
				$k,
			)+
		}

		impl Period {
			#[must_use]
			/// # As String Slice.
			///
			/// Return the period as a string slice in UPPER or lower case.
			///
			/// ## Examples
			///
			/// ```
			/// use utc2k::Period;
			///
			$(
				#[doc = concat!(
					"assert_eq!(Period::", stringify!($k), ".as_str(false), \"", period!(@str $lower1 $lower2), "\");\n",
					"assert_eq!(Period::", stringify!($k), ".as_str(true), \"", period!(@str $upper1 $upper2), "\");\n",
					"# assert_eq!(Period::", stringify!($k), ", \"", period!(@str $upper1 $upper2), "\");\n",
					"# assert_eq!(Period::", stringify!($k), ", \"", period!(@str_p $upper1 $upper2), "\");\n",
					"# assert_eq!(\"", period!(@str $upper1 $upper2), "\", Period::", stringify!($k), ");\n",
					"# assert_eq!(\"", period!(@str_p $upper1 $upper2), "\", Period::", stringify!($k), ");\n",
					"# assert_eq!(Period::", stringify!($k), ", \"", period!(@str $lower1 $lower2), "\");\n",
					"# assert_eq!(Period::", stringify!($k), ", \"", period!(@str_p $lower1 $lower2), "\");\n",
					"# assert_eq!(\"", period!(@str $lower1 $lower2), "\", Period::", stringify!($k), ");\n",
					"# assert_eq!(\"", period!(@str_p $lower1 $lower2), "\", Period::", stringify!($k), ");\n",
				)]
			)+
			/// ```
			pub const fn as_str(self, upper: bool) -> &'static str {
				match self {
					$(
						Self::$k =>
							if upper { period!(@str $upper1 $upper2) }
							else { period!(@str $lower1 $lower2) },
					)+
				}
			}

			#[inline]
			#[must_use]
			/// # As Lowercase String.
			const fn as_str_default(self) -> &'static str { self.as_str(false) }

			#[must_use]
			/// # As String Slice (AP Style).
			///
			/// Return the period as an AP Style-formatted string slice.
			///
			/// ## Examples
			///
			/// ```
			/// use utc2k::Period;
			///
			$(
				#[doc = concat!(
					"assert_eq!(Period::", stringify!($k), ".as_str_ap(), \"", period!(@str_p $lower1 $lower2), "\");\n",
				)]
			)+
			/// ```
			pub const fn as_str_ap(self) -> &'static str {
				match self {
					$(
						Self::$k => period!(@str_p $lower1 $lower2),
					)+
				}
			}

			#[must_use]
			/// # Try From Bytes.
			///
			/// Match both naked and punctuated styles, case-insensitively.
			const fn from_bytes(raw: &[u8]) -> Option<Self> {
				match raw.trim_ascii() {
					$(
						[ period!(@byte $lower1), period!(@byte $lower2) ] |
						[ period!(@byte $lower1), b'.', period!(@byte $lower2), b'.' ] |
						[ period!(@byte $upper1), period!(@byte $upper2) ] |
						[ period!(@byte $upper1), b'.', period!(@byte $upper2), b'.' ] => Some(Self::$k),
					)+
					_ => None,
				}
			}
		}
	);
}

period! {
	/// # Ante Meridiem.
	///
	/// The hours before noon.
	Am a m A M,

	/// # Post Meridiem.
	///
	/// Midday and beyond.
	Pm p m P M,
}

macros::as_ref_borrow_cast!(Period: as_str_default str);
macros::display_str!(as_str_default Period);

impl PartialEq<str> for Period {
	#[inline]
	fn eq(&self, other: &str) -> bool {
		Self::from_bytes(other.as_bytes()) == Some(*self)
	}
}
impl PartialEq<&str> for Period {
	#[inline]
	fn eq(&self, other: &&str) -> bool {
		Self::from_bytes(other.as_bytes()) == Some(*self)
	}
}
impl PartialEq<Period> for str {
	#[inline]
	fn eq(&self, other: &Period) -> bool {
		Period::from_bytes(self.as_bytes()) == Some(*other)
	}
}
impl PartialEq<Period> for &str {
	#[inline]
	fn eq(&self, other: &Period) -> bool {
		Period::from_bytes(self.as_bytes()) == Some(*other)
	}
}
