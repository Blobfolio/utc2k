/*!
# Sqlx/Mysql.
*/

use sqlx::{
	Database,
	Decode,
	Encode,
	encode::IsNull,
	error::BoxDynError,
	Type,
};
use super::{
	Utc2k,
	Utc2kError,
};



impl<DB> Type<DB> for Utc2k
where DB: Database, i64: Type<DB> {
	#[inline]
	/// # Database Type For `Utc2k`.
	///
	/// Use the optional `sqlx-mysql` crate feature to enable Mysql database
	/// support for [`Utc2k`]s.
	///
	/// To keep things simple, `Utc2k` values are mapped to Mysql's (signed)
	/// `BIGINT` type to match the input/output signatures of `FROM_UNIXTIME`
	/// and `UNIX_TIMESTAMP` respectively.
	///
	/// Refer to the `Decode`/`Encode` impls for example usage.
	fn type_info() -> <DB as Database>::TypeInfo {
		<i64 as Type<DB>>::type_info()
	}

	/// # Compatibility.
	fn compatible(ty: &<DB as Database>::TypeInfo) -> bool {
		<i64 as Type<DB>>::compatible(ty)
	}
}

impl<'r, DB> Decode<'r, DB> for Utc2k
where DB: Database, i64: Decode<'r, DB> {
	/// # Decode `Utc2k`.
	///
	/// Use the optional `sqlx-mysql` crate feature to decode Mysql (signed)
	/// `BIGINT` unix timestamps as [`Utc2k`] objects.
	///
	/// For schemas with proper `TIMESTAMP` column types, you'll need to
	/// leverage Mysql's `UNIX_TIMESTAMP` and `FROM_UNIXTIME` functions to
	/// convert to/from the intermediary `BIGINT`, like:
	///
	/// ```ignore
	/// query!(
	///     "
	///     SELECT
	///         UNIX_TIMESTAMP(date_last) AS `date_last!: Utc2k`,
	///         first_name,
	///         last_name
	///     FROM mailing_list
	///     WHERE date_last < FROM_UNIXTIME(?)
	///     ",
	///     Utc2k::yesterday()
	/// )
	///     .fetch_all(&pool)
	///     .await?
	/// ```
	///
	/// ## Errors
	///
	/// Decoding uses [`Utc2k::checked_from_unixtime`] under the hood, so
	/// values outside the 2000s will fail with an error.
	fn decode(value: <DB as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
		let raw = <i64 as Decode<'r, DB>>::decode(value)?;
		u32::try_from(raw)
			.map_err(|_|
				if raw < 0 { Utc2kError::Underflow }
				else { Utc2kError::Overflow }
			)
			.and_then(Self::checked_from_unixtime)
			.map_err(Into::into)
	}
}

impl<'q, DB> Encode<'q, DB> for Utc2k
where DB: Database, i64: Encode<'q, DB> {
	#[inline]
	/// # Encode `Utc2k`.
	///
	/// Use the optional `sqlx-mysql` crate feature to encode [`Utc2k`]
	/// objects as unix timestamps mapped to Mysql's (signed) `BIGINT` type.
	///
	/// For schemas with proper `TIMESTAMP` column types, you'll need to
	/// leverage Mysql's `UNIX_TIMESTAMP` and `FROM_UNIXTIME` functions to
	/// convert to/from the intermediary `BIGINT`, like:
	///
	/// ```ignore
	/// query!(
	///     "
	///     SELECT
	///         UNIX_TIMESTAMP(date_last) AS `date_last!: Utc2k`,
	///         first_name,
	///         last_name
	///     FROM mailing_list
	///     WHERE date_last < FROM_UNIXTIME(?)
	///     ",
	///     Utc2k::yesterday()
	/// )
	///     .fetch_all(&pool)
	///     .await?
	/// ```
	fn encode_by_ref(
		&self,
		buf: &mut <DB as Database>::ArgumentBuffer<'q>,
	) -> Result<IsNull, BoxDynError> {
		<i64 as Encode::<'q, DB>>::encode_by_ref(&i64::from(self.unixtime()), buf)
	}
}
