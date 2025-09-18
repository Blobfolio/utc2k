/*!
# (De/)Serialization
*/

use crate::{
	FmtUtc2k,
	Month,
	Utc2k,
	Weekday,
};
use serde_core::{
	de,
	Deserialize,
	ser,
	Serialize,
};
use std::fmt;



impl<'de> Deserialize<'de> for FmtUtc2k {
	/// # Deserialize.
	///
	/// Use the optional `serde` crate feature to enable serialization support.
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where D: de::Deserializer<'de> {
		Utc2k::deserialize(deserializer).map(Self::from)
	}
}

impl Serialize for FmtUtc2k {
	#[inline]
	/// # Serialize.
	///
	/// Use the optional `serde` crate feature to enable serialization support.
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where S: ser::Serializer { serializer.serialize_str(self.as_str()) }
}



impl<'de> Deserialize<'de> for Utc2k {
	/// # Deserialize.
	///
	/// Use the optional `serde` crate feature to enable serialization support.
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where D: de::Deserializer<'de> {
		/// # Visitor Instance.
		struct Visitor;

		/// # Helper: Errors for Unsupported Formats.
		macro_rules! invalid {
			($fn:ident, $ty:ty) => (
				fn $fn<S>(self, _src: $ty) -> Result<Self::Value, S>
				where S: de::Error {
					Err(de::Error::custom(concat!(stringify!($ty), " is unsupported")))
				}
			);
		}

		impl de::Visitor<'_> for Visitor {
			type Value = Utc2k;

			fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
				f.write_str("a timestamp or datetime string")
			}

			fn visit_str<S>(self, src: &str) -> Result<Self::Value, S>
			where S: de::Error {
				Utc2k::try_from(src).map_err(|_| de::Error::custom("invalid datetime string"))
			}

			fn visit_bytes<S>(self, src: &[u8]) -> Result<Self::Value, S>
			where S: de::Error {
				Utc2k::try_from(src).map_err(|_| de::Error::custom("invalid datetime string"))
			}

			fn visit_i32<S>(self, src: i32) -> Result<Self::Value, S>
			where S: de::Error {
				// Fail on negative, otherwise parse as usual.
				u32::try_from(src)
					.map(Utc2k::from)
					.map_err(|_| de::Error::custom("invalid unix timestamp"))
			}

			fn visit_i64<S>(self, src: i64) -> Result<Self::Value, S>
			where S: de::Error {
				// Fail on negative, otherwise parse as usual.
				u32::try_from(src)
					.map(Utc2k::from)
					.map_err(|_| de::Error::custom("invalid unix timestamp"))
			}

			fn visit_u32<S>(self, src: u32) -> Result<Self::Value, S>
			where S: de::Error { Ok(Utc2k::from(src)) }

			fn visit_u64<S>(self, src: u64) -> Result<Self::Value, S>
			where S: de::Error {
				// Return the max value on failure because it's too big,
				// otherwise parse as normal.
				Ok(u32::try_from(src).map_or_else(|_| Utc2k::MAX, Utc2k::from))
			}

			// Too small to hold an in-range value.
			invalid!(visit_char, char);
			invalid!(visit_i8, i8);
			invalid!(visit_i16, i16);
			invalid!(visit_u8, u8);
			invalid!(visit_u16, u16);
		}

		deserializer.deserialize_any(Visitor)
	}
}

impl Serialize for Utc2k {
	#[inline]
	/// # Serialize.
	///
	/// Use the optional `serde` crate feature to enable serialization support.
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where S: ser::Serializer { serializer.serialize_u32(self.unixtime()) }
}



impl<'de> Deserialize<'de> for Month {
	#[inline]
	/// # Deserialize.
	///
	/// Use the optional `serde` crate feature to enable serialization support.
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where D: de::Deserializer<'de> {
		/// # Visitor Instance.
		struct Visitor;

		impl de::Visitor<'_> for Visitor {
			type Value = Month;

			fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
				f.write_str("a string representation like 'jan' or 'January'")
			}

			#[inline]
			fn visit_str<S>(self, src: &str) -> Result<Self::Value, S>
			where S: de::Error {
				Month::try_from(src).map_err(|_| de::Error::custom("invalid month string"))
			}

			#[inline]
			fn visit_bytes<S>(self, src: &[u8]) -> Result<Self::Value, S>
			where S: serde_core::de::Error {
				Month::try_from(src).map_err(|_| de::Error::custom("invalid month string"))
			}
		}

		deserializer.deserialize_str(Visitor)
	}
}

impl Serialize for Month {
	#[inline]
	/// # Serialize.
	///
	/// Use the optional `serde` crate feature to enable serialization support.
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where S: ser::Serializer { serializer.serialize_str(self.as_str()) }
}



impl<'de> Deserialize<'de> for Weekday {
	#[inline]
	/// # Deserialize.
	///
	/// Use the optional `serde` crate feature to enable serialization support.
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where D: de::Deserializer<'de> {
		/// # Visitor Instance.
		struct Visitor;

		impl de::Visitor<'_> for Visitor {
			type Value = Weekday;

			fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
				f.write_str("a string representation like 'mon' or 'Monday'")
			}

			#[inline]
			fn visit_str<S>(self, src: &str) -> Result<Self::Value, S>
			where S: de::Error {
				Weekday::try_from(src).map_err(|_| de::Error::custom("invalid weekday string"))
			}

			#[inline]
			fn visit_bytes<S>(self, src: &[u8]) -> Result<Self::Value, S>
			where S: serde_core::de::Error {
				Weekday::try_from(src).map_err(|_| de::Error::custom("invalid weekday string"))
			}
		}

		deserializer.deserialize_str(Visitor)
	}
}

impl Serialize for Weekday {
	#[inline]
	/// # Serialize.
	///
	/// Use the optional `serde` crate feature to enable serialization support.
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where S: ser::Serializer { serializer.serialize_str(self.as_str()) }
}



#[cfg(test)]
mod tests {
	use super::*;

	#[cfg(not(miri))]
	const SAMPLE_SIZE: usize = 1_000_000;

	#[cfg(miri)]
	const SAMPLE_SIZE: usize = 1000; // Miri runs way too slow for a million tests.

	#[test]
	/// # Test Serialization.
	fn t_serde() {
		const DATESTR: &str = "2021-07-08 11:33:16";
		const DATENUM: &str = "1625743996";
		const DATESTR_Q: &str = "\"2021-07-08 11:33:16\"";

		{
			// Formatted Version.
			let date = FmtUtc2k::try_from(DATESTR).unwrap();
			let serial = serde_json::to_string(&date)
				.expect("FmtUtc2k serialization failed.");
			assert_eq!(serial, DATESTR_Q);

			let mut date2: FmtUtc2k = serde_json::from_str(&serial)
				.expect("FmtUtc2k deserialization (str) failed.");
			assert_eq!(date, date2);

			// We should also be able to deserialize from a timestamp.
			date2 = serde_json::from_str(DATENUM)
				.expect("FmtUtc2k deserialization (u32) failed.");
			assert_eq!(date, date2);
		}

		{
			// Utc2k Version.
			let date = Utc2k::try_from(DATESTR).unwrap();
			let serial = serde_json::to_string(&date)
				.expect("Utc2k serialization failed.");
			assert_eq!(serial, DATENUM);

			let mut date2: Utc2k = serde_json::from_str(&serial)
				.expect("Utc2k deserialization (u32) failed.");
			assert_eq!(date, date2);

			// We should also be able to deserialize from a datetime string.
			date2 = serde_json::from_str(DATESTR_Q)
				.expect("Utc2k deserialization (str) failed.");
			assert_eq!(date, date2);
		}
	}

	#[test]
	fn t_serde_fmtutc2k_rng() {
		let mut rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.u32(Utc2k::MIN_UNIXTIME..=Utc2k::MAX_UNIXTIME)).take(SAMPLE_SIZE) {
			let date = FmtUtc2k::from(i);

			// Serialization should give us the date string with an extra pair
			// of quotes.
			let s = serde_json::to_string(&date).expect("Serialization failed.");
			assert_eq!(format!("{:?}", date.as_str()), s);

			// Deserialization should give us a copy of the original.
			let d = serde_json::from_str::<FmtUtc2k>(&s).expect("Deserialization failed.");
			assert_eq!(date, d);
		}
	}

	#[test]
	fn t_serde_utc2k_rng() {
		let mut rng = fastrand::Rng::new();
		for i in std::iter::repeat_with(|| rng.u32(Utc2k::MIN_UNIXTIME..=Utc2k::MAX_UNIXTIME)).take(SAMPLE_SIZE) {
			let date = Utc2k::from(i);

			// Serialization should give us the unixtime as a string.
			let s = serde_json::to_string(&date).expect("Serialization failed.");
			assert_eq!(s, i.to_string());

			// Deserialization should give us a copy of the original.
			let d = serde_json::from_str::<Utc2k>(&s).expect("Deserialization failed.");
			assert_eq!(date, d);
		}
	}

	#[test]
	fn t_serde_month() {
		for month in Month::ALL {
			let s = serde_json::to_string(&month).expect("Serialization failed.");
			assert_eq!(s, format!("\"{}\"", month.as_str()));

			let d = serde_json::from_str::<Month>(&s).expect("Deserialization failed.");
			assert_eq!(d, month);

			// From abbreviation.
			let d = serde_json::from_str::<Month>(&format!("\"{}\"", month.abbreviation()))
				.expect("Deserialization (abbr) failed.");
			assert_eq!(d, month);
		}
	}

	#[test]
	fn t_serde_weekday() {
		for day in Weekday::ALL {
			let s = serde_json::to_string(&day).expect("Serialization failed.");
			assert_eq!(s, format!("\"{}\"", day.as_str()));

			let d = serde_json::from_str::<Weekday>(&s).expect("Deserialization failed.");
			assert_eq!(d, day);

			// From abbreviation.
			let d = serde_json::from_str::<Weekday>(&format!("\"{}\"", day.abbreviation()))
				.expect("Deserialization (abbr) failed.");
			assert_eq!(d, day);
		}
	}
}
