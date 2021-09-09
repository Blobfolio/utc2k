/*!
# (De/)Serialization
*/

use crate::{
	FmtUtc2k,
	Utc2k,
};
use serde::{
	de,
	Deserialize,
	ser,
	Serialize,
};
use std::convert::TryFrom;
use std::marker::PhantomData;
use std::fmt;



/// # Temporary Deserialization Value.
///
/// This is a stripped-down version of the private `Content` enum provided by
/// `serde_derive`. It allows us to deserialize our structs from timestamps or
/// datetime strings.
enum Value<'de> {
	Invalid,
	Int(u32),
	String(String),
	Str(&'de str),
	ByteBuf(Vec<u8>),
	Bytes(&'de [u8]),
}

impl<'de> Value<'de> {
	/// # As Integer.
	const fn as_int(&self) -> Option<u32> {
		if let Self::Int(x) = self { Some(*x) }
		else { None }
	}

	/// # As String Slice.
	fn as_str(&self) -> Option<&str> {
		match *self {
			Self::Str(x) => Some(x),
			Self::String(ref x) => Some(x),
			Self::Bytes(x) => std::str::from_utf8(x).ok(),
			Self::ByteBuf(ref x) => std::str::from_utf8(x).ok(),
			_ => None,
		}
	}
}

impl<'de> Deserialize<'de> for Value<'de> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: de::Deserializer<'de>,
	{
		// Untagged and internally tagged enums are only supported in
		// self-describing formats.
		let visitor = ValueVisitor { value: PhantomData };
		deserializer.deserialize_any(visitor)
	}
}

/// # Deserialization Visitor.
struct ValueVisitor<'de> {
	value: PhantomData<Value<'de>>,
}

/// # Helper: Generated Trait Methods for Unsupported Types.
///
/// Most of the types we do not support have the same signatures.
macro_rules! invalid_type {
	($(($fn:ident, $ty:ty)),+) => ($(
		fn $fn<F>(self, _v: $ty) -> Result<Self::Value, F>
		where F: de::Error { Ok(Value::Invalid) }
	)+);
}

impl<'de> de::Visitor<'de> for ValueVisitor<'de> {
	type Value = Value<'de>;

	fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		fmt.write_str("any value")
	}

	fn visit_u8<F>(self, value: u8) -> Result<Self::Value, F>
	where F: de::Error { Ok(Value::Int(u32::from(value))) }

	fn visit_u16<F>(self, value: u16) -> Result<Self::Value, F>
	where F: de::Error { Ok(Value::Int(u32::from(value))) }

	fn visit_u32<F>(self, value: u32) -> Result<Self::Value, F>
	where F: de::Error { Ok(Value::Int(value)) }

	fn visit_u64<F>(self, value: u64) -> Result<Self::Value, F>
	where F: de::Error
	{ Ok(Value::Int(u32::try_from(value).unwrap_or(Utc2k::MAX_UNIXTIME))) }

	fn visit_str<F>(self, value: &str) -> Result<Self::Value, F>
	where F: de::Error { Ok(Value::String(value.into())) }

	fn visit_borrowed_str<F>(self, value: &'de str) -> Result<Self::Value, F>
	where F: de::Error { Ok(Value::Str(value)) }

	fn visit_string<F>(self, value: String) -> Result<Self::Value, F>
	where F: de::Error { Ok(Value::String(value)) }

	fn visit_bytes<F>(self, value: &[u8]) -> Result<Self::Value, F>
	where F: de::Error { Ok(Value::ByteBuf(value.to_vec())) }

	fn visit_borrowed_bytes<F>(self, value: &'de [u8]) -> Result<Self::Value, F>
	where F: de::Error { Ok(Value::Bytes(value)) }

	fn visit_byte_buf<F>(self, value: Vec<u8>) -> Result<Self::Value, F>
	where F: de::Error { Ok(Value::ByteBuf(value)) }

	invalid_type!(
		(visit_bool, bool),
		(visit_i8, i8),
		(visit_i16, i16),
		(visit_i32, i32),
		(visit_i64, i64),
		(visit_f32, f32),
		(visit_f64, f64),
		(visit_char, char)
	);

	fn visit_unit<F>(self) -> Result<Self::Value, F>
	where F: de::Error { Ok(Value::Invalid) }

	fn visit_none<F>(self) -> Result<Self::Value, F>
	where F: de::Error { Ok(Value::Invalid) }

	fn visit_some<D>(self, _deserializer: D) -> Result<Self::Value, D::Error>
	where D: de::Deserializer<'de> { Ok(Value::Invalid) }

	fn visit_newtype_struct<D>(self, _deserializer: D) -> Result<Self::Value, D::Error>
	where D: de::Deserializer<'de> { Ok(Value::Invalid) }

	fn visit_seq<V>(self, _visitor: V) -> Result<Self::Value, V::Error>
	where V: de::SeqAccess<'de> { Ok(Value::Invalid) }

	fn visit_map<V>(self, _visitor: V) -> Result<Self::Value, V::Error>
	where V: de::MapAccess<'de> { Ok(Value::Invalid) }

	fn visit_enum<V>(self, _visitor: V) -> Result<Self::Value, V::Error>
	where V: de::EnumAccess<'de> { Ok(Value::Invalid) }
}



impl<'de> Deserialize<'de> for FmtUtc2k {
	/// # Deserialize.
	///
	/// Use the optional `serde` crate feature to enable serialization support.
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where D: de::Deserializer<'de> {
		let raw = Value::deserialize(deserializer)?;
		if let Some(v) = raw.as_str() {
			Self::try_from(v).map_err(|_| de::Error::custom("invalid date string"))
		}
		else if let Some(v) = raw.as_int() {
			Ok(Self::from(v))
		}
		else {
			Err(de::Error::custom("expected timestamp or datetime string"))
		}
	}
}

impl Serialize for FmtUtc2k {
	#[inline]
	/// # Serialize.
	///
	/// Use the optional `serde` crate feature to enable serialization support.
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where S: ser::Serializer {
		serializer.serialize_str(self.as_str())
	}
}



impl<'de> Deserialize<'de> for Utc2k {
	/// # Deserialize.
	///
	/// Use the optional `serde` crate feature to enable serialization support.
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where D: de::Deserializer<'de> {
		let raw = Value::deserialize(deserializer)?;
		if let Some(v) = raw.as_int() {
			Ok(Self::from(v))
		}
		else if let Some(v) = raw.as_str() {
			Self::try_from(v).map_err(|_| de::Error::custom("invalid date string"))
		}
		else {
			Err(de::Error::custom("expected timestamp or datetime string"))
		}
	}
}

impl Serialize for Utc2k {
	#[inline]
	/// # Serialize.
	///
	/// Use the optional `serde` crate feature to enable serialization support.
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where S: ser::Serializer {
		serializer.serialize_u32(self.unixtime())
	}
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	/// # Test Serialization.
	fn t_serde() {
		const DATESTR: &str = "2021-07-08 11:33:16";
		const DATENUM: &str = "1625743996";
		const DATESTR_Q: &str = "\"2021-07-08 11:33:16\"";

		const YSERIALNUM: &str = "---\n1625743996\n";
		const YSERIALSTR: &str = "---\n\"2021-07-08 11:33:16\"\n";

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

			// Try it with serde_yaml, which is a bit stricter.
			date2 = serde_yaml::from_str(&serial)
				.expect("FmtUtc2k deserialization (str) failed.");
			assert_eq!(date, date2);

			date2 = serde_yaml::from_str(DATENUM)
				.expect("FmtUtc2k deserialization (u32) failed.");
			assert_eq!(date, date2);

			// Serialize it with serde_yaml too.
			let serial2 = serde_yaml::to_string(&date)
				.expect("FmtUtc2k serialization failed.");
			assert_eq!(serial2, YSERIALSTR);

			// And make sure deserialization from YAML format works.
			date2 = serde_yaml::from_str(YSERIALSTR)
				.expect("FmtUtc2k deserialization (str) failed.");
			assert_eq!(date, date2);

			date2 = serde_yaml::from_str(YSERIALNUM)
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

			// Again, retry with serde_yaml.
			date2 = serde_yaml::from_str(&serial)
				.expect("Utc2k deserialization (u32) failed.");
			assert_eq!(date, date2);

			// We should also be able to deserialize from a datetime string.
			date2 = serde_yaml::from_str(DATESTR_Q)
				.expect("Utc2k deserialization (str) failed.");
			assert_eq!(date, date2);

			let serial2 = serde_yaml::to_string(&date)
				.expect("Utc2k serialization failed.");
			assert_eq!(serial2, YSERIALNUM);

			date2 = serde_yaml::from_str(YSERIALSTR)
				.expect("Utc2k deserialization (str) failed.");
			assert_eq!(date, date2);

			date2 = serde_yaml::from_str(YSERIALNUM)
				.expect("Utc2k deserialization (u32) failed.");
			assert_eq!(date, date2);
		}
	}
}
