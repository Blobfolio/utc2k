/*!
# Utc2k - Macros
*/

/// # Helper: `AsRef` and `Borrow`.
macro_rules! as_ref_borrow_cast {
	($parent:ty: $($cast:ident $ty:ty),+ $(,)?) => ($(
		impl AsRef<$ty> for $parent {
			#[inline]
			fn as_ref(&self) -> &$ty { self.$cast() }
		}

		impl ::std::borrow::Borrow<$ty> for $parent {
			#[inline]
			fn borrow(&self) -> &$ty { self.$cast() }
		}
	)+);
}

/// # Helper: `Display`.
macro_rules! display_str {
	($cast:ident $ty:ty) => (
		impl ::std::fmt::Display for $ty {
			#[inline]
			fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
				<str as ::std::fmt::Display>::fmt(self.$cast(), f)
			}
		}
	);
}

/// # Helper: First in List.
macro_rules! first {
	// Slap a macro call around the answer.
	(@$mac:ident $first:tt $($_rest:tt)*) => ( $mac!($first) );

	// Passthrough.
	($first:tt $($_rest:tt)*) => ( $first );
}

/// # Helper: Last in List.
macro_rules! last {
	// Slap a macro call around the answer.
	(@$mac:ident $last:tt) => ( $mac!($last) );
	(@$mac:ident $_next:tt $($rest:tt)+) => ( $crate::macros::last!(@$mac $($rest)+) );

	// Passthrough.
	($last:tt) => ( $last );
	($_next:tt $($rest:tt)+) => ( $crate::macros::last!($($rest)+) );
}

/// # Helper: Pair Siblings.
///
/// This macro groups `Weekday`/`Month` variants with their siblings for
/// consumption by other macros (on this page).
macro_rules! pair {
	// Pair Down.
	(
		@previous $dst:ident { $($args:tt)* }
		$k1:ident $k2:ident $k3:ident $k4:ident $k5:ident $k6:ident $k7:ident
	) => (
		$crate::macros::$dst!(
			$($args)*
			($k1 $k7), ($k2 $k1), ($k3 $k2), ($k4 $k3), ($k5 $k4), ($k6 $k5),
			($k7 $k6),
		)
	);
	(
		@previous $dst:ident { $($args:tt)* }
		$k1:ident $k2:ident $k3:ident $k4:ident $k5:ident $k6:ident $k7:ident $k8:ident $k9:ident $k10:ident $k11:ident $k12:ident
	) => (
		$crate::macros::$dst!(
			$($args)*
			($k1 $k12), ($k2 $k1), ($k3 $k2), ($k4 $k3), ($k5 $k4), ($k6 $k5),
			($k7 $k6), ($k8 $k7), ($k9 $k8), ($k10 $k9), ($k11 $k10), ($k12 $k11),
		)
	);

	// Pair Up.
	(
		@next $dst:ident { $($args:tt)* }
		$k1:ident $k2:ident $k3:ident $k4:ident $k5:ident $k6:ident $k7:ident
	) => (
		$crate::macros::$dst!(
			$($args)*
			($k1 $k2), ($k2 $k3), ($k3 $k4), ($k4 $k5), ($k5 $k6), ($k6 $k7),
			($k7 $k1),
		)
	);
	(
		@next $dst:ident { $($args:tt)* }
		$k1:ident $k2:ident $k3:ident $k4:ident $k5:ident $k6:ident $k7:ident $k8:ident $k9:ident $k10:ident $k11:ident $k12:ident
	) => (
		$crate::macros::$dst!(
			$($args)*
			($k1 $k2), ($k2 $k3), ($k3 $k4), ($k4 $k5), ($k5 $k6), ($k6 $k7),
			($k7 $k8), ($k8 $k9), ($k9 $k10), ($k10 $k11), ($k11 $k12), ($k12 $k1),
		)
	);
}

/// # Helper: Set Up `Weekday`/`Month` Enums.
///
/// This monster enum generates all the common code for the two enums, ensuring
/// consistent documentation, unit tests, etc.
///
/// Specifically:
///
/// * The main `enum` definition and derived traits:
///   * `Clone`
///   * `Copy`
///   * `Debug`
///   * `Display`
///   * `Eq` / `PartialEq`
///   * `Hash`
///   * `Ord` / `PartialOrd`
/// * `AsRef<str>`
/// * `Borrow<str>`
/// * `From<Utc2k>`
/// * `FromStr`
/// * `IntoIterator`
/// * `TryFrom`
///   * `&str`
///   * `Box<str>` (and ref)
///   * `Cow<str>` (and ref)
///   * `String` (and ref)
/// * `Self::ALL`
/// * `Self::abbreviation`
/// * `Self::as_str`
/// * `Self::from_u8` (private)
/// * `Self::next`
/// * `Self::previous`
/// * The iterator struct and its impls
///
/// This also handles the following cross-type implementations for `u8`, `u16`,
/// `u32`, `u64`, and `usize`:
///
/// * `Add` / `AddAssign`
/// * `From` (both ways)
/// * `PartialEq` (both ways)
/// * `Sub` / `SubAssign`
///
/// Big as this list is, there are three common components _not_ handled here:
///
/// * `Self::from_abbreviation` (one-off vars and weird sorting)
/// * `Self::now`               (only `Weekday` has yesterday/tomorrow)
/// * `TryFrom<&[u8]>`          (overly specific documentation)
macro_rules! weekmonth {
	// Docs: print the type's numerical range and first entry.
	(@ex @range $ty:tt $($k:ident $v:literal)+) => (concat!(
		stringify!($ty), "s range from `",
		$crate::macros::first!(@stringify $($v)+),
		"..=",
		$crate::macros::last!(@stringify $($v)+),
		"`, starting with ",
		$crate::macros::first!(@stringify $($k)+),
		".",
	));

	// Docs: addition/subtraction (code).
	(@ex @addsub $uint:tt $ty:tt $op:literal $(($v:literal $k:ident $com:literal)),+ $(,)?) => (
		concat!(
			"let start = ", stringify!($ty), "::", $crate::macros::first!(@stringify $($k)+), ";\n",
			$(
				"assert_eq!(start ", $op, " ", $v, "_", stringify!($uint), ", ", stringify!($ty), "::", stringify!($k), "); ", $com, "\n",
			)+
			"// …\n",
		)
	);
	(@ex @add $uint:tt $ty:tt $k1:ident $k2:ident $k3:ident $k4:ident $k5:ident $k6:ident $k7:ident) => (
		$crate::macros::weekmonth!(
			@ex @addsub
			$uint $ty "+"
			(0 $k1 "// Noop."),
			(1 $k2 ""), (2 $k3 ""), (3 $k4 ""), (4 $k5 ""), (5 $k6 ""), (6 $k7 ""),
			(7 $k1 "// Wrap."), (8 $k2 "// Wrap."), (9 $k3 "// Wrap."),
		)
	);
	(@ex @add $uint:tt $ty:tt $k1:ident $k2:ident $k3:ident $k4:ident $k5:ident $k6:ident $k7:ident $k8:ident $k9:ident $k10:ident $k11:ident $k12:ident) => (
		$crate::macros::weekmonth!(
			@ex @addsub
			$uint $ty "+"
			(0 $k1 "// Noop."),
			(1 $k2 ""), (2 $k3 ""), (3 $k4 ""), (4 $k5 ""), (5 $k6 ""), (6 $k7 ""),
			(7 $k8 ""), (8 $k9 ""), (9 $k10 ""), (10 $k11 ""), (11 $k12 ""),
			(12 $k1 "// Wrap."), (13 $k2 "// Wrap."), (14 $k3 "// Wrap."),
		)
	);
	(@ex @sub $uint:tt $ty:tt $k1:ident $k2:ident $k3:ident $k4:ident $k5:ident $k6:ident $k7:ident) => (
		$crate::macros::weekmonth!(
			@ex @addsub
			$uint $ty "-"
			(0 $k1 "// Noop."),
			(1 $k7 ""), (2 $k6 ""), (3 $k5 ""), (4 $k4 ""), (5 $k3 ""), (6 $k2 ""),
			(7 $k1 "// Wrap."), (8 $k7 "// Wrap."), (9 $k6 "// Wrap."),
		)
	);
	(@ex @sub $uint:tt $ty:tt $k1:ident $k2:ident $k3:ident $k4:ident $k5:ident $k6:ident $k7:ident $k8:ident $k9:ident $k10:ident $k11:ident $k12:ident) => (
		$crate::macros::weekmonth!(
			@ex @addsub
			$uint $ty "-"
			(0 $k1 "// Noop."),
			(1 $k12 ""), (2 $k11 ""), (3 $k10 ""), (4 $k9 ""), (5 $k8 ""), (6 $k7 ""),
			(7 $k6 ""), (8 $k5 ""), (9 $k4 ""), (10 $k3 ""), (11 $k2 ""),
			(12 $k1 "// Wrap."), (13 $k12 "// Wrap."), (14 $k11 "// Wrap."),
		)
	);

	// Docs: extra (wrapping) from-int (code).
	(@ex @from_a $uint:tt $ty:tt $k1:ident $k2:ident $k3:ident $k4:ident $k5:ident $k6:ident $k7:ident) => (
		concat!(
			"assert_eq!(", stringify!($ty), "::from(8_", stringify!($uint), "), ", stringify!($ty), "::", stringify!($k1), "); // Wrap.\n",
			"assert_eq!(", stringify!($ty), "::from(9_", stringify!($uint), "), ", stringify!($ty), "::", stringify!($k2), "); // Wrap.\n",
			"assert_eq!(", stringify!($ty), "::from(10_", stringify!($uint), "), ", stringify!($ty), "::", stringify!($k3), "); // Wrap.\n",
			"// …\n",
		)
	);
	(@ex @from_a $uint:tt $ty:tt $k1:ident $k2:ident $k3:ident $k4:ident $k5:ident $k6:ident $k7:ident $k8:ident $k9:ident $k10:ident $k11:ident $k12:ident) => (
		concat!(
			"assert_eq!(", stringify!($ty), "::from(13_", stringify!($uint), "), ", stringify!($ty), "::", stringify!($k1), "); // Wrap.\n",
			"assert_eq!(", stringify!($ty), "::from(14_", stringify!($uint), "), ", stringify!($ty), "::", stringify!($k2), "); // Wrap.\n",
			"assert_eq!(", stringify!($ty), "::from(15_", stringify!($uint), "), ", stringify!($ty), "::", stringify!($k3), "); // Wrap.\n",
			"// …\n",
		)
	);

	// Docs: `Self::previous`/`Self::next` (partial code).
	(@ex @pairs $ty:ident $fn:literal $(($from:ident $to:ident)),+ $(,)?) => (concat!(
		$(
			"assert_eq!(",
			stringify!($ty), "::", stringify!($from), ".", $fn, "(), ",
			stringify!($ty), "::", stringify!($to),
			");\n",
		)+
	));

	// Docs: `iter.next()` (code).
	(@ex @next $ty:tt $(($k:ident $com:literal)),+ $(,)?) => (concat!(
		$(
			"assert_eq!(iter.next(), Some(", stringify!($ty), "::", stringify!($k), ")); ", $com, "\n",
		)+
	));

	// Docs: `Self::previous`/`Self::next` (full code).
	(@ex @iter $ty:tt $k1:ident $k2:ident $k3:ident $k4:ident $k5:ident $k6:ident $k7:ident) => (concat!(
		"let mut iter = ", stringify!($ty), "::", stringify!($k1), ".into_iter();\n",
		$crate::macros::weekmonth!(
			@ex @next $ty
			($k1 ""), ($k2 ""), ($k3 ""), ($k4 ""), ($k5 ""), ($k6 ""), ($k7 ""),
			($k1 "// Wrap."), ($k2 "// Wrap."), ($k3 "// Wrap."),
		),
		"// …\n\n",
		"// Or like Ginger, backwards and in high heels.\n",
		"let mut iter = ", stringify!($ty), "::", stringify!($k7), ".into_iter().rev();\n",
		$crate::macros::weekmonth!(
			@ex @next $ty
			($k7 ""), ($k6 ""), ($k5 ""), ($k4 ""), ($k3 ""), ($k2 ""), ($k1 ""),
			($k7 "// Wrap."), ($k6 "// Wrap."), ($k5 "// Wrap."),
		),
		"// …\n",
	));
	(@ex @iter $ty:tt $k1:ident $k2:ident $k3:ident $k4:ident $k5:ident $k6:ident $k7:ident $k8:ident $k9:ident $k10:ident $k11:ident $k12:ident) => (concat!(
		"let mut iter = ", stringify!($ty), "::", stringify!($k1), ".into_iter();\n",
		$crate::macros::weekmonth!(
			@ex @next $ty
			($k1 ""), ($k2 ""), ($k3 ""), ($k4 ""), ($k5 ""), ($k6 ""), ($k7 ""),
			($k8 ""), ($k9 ""), ($k10 ""), ($k11 ""), ($k12 ""),
			($k1 "// Wrap."), ($k2 "// Wrap."), ($k3 "// Wrap."),
		),
		"// …\n\n",
		"// Or like Ginger:\n",
		"let mut iter = ", stringify!($ty), "::", stringify!($k12), ".into_iter().rev();\n",
		$crate::macros::weekmonth!(
			@ex @next $ty
			($k12 ""), ($k11 ""), ($k10 ""), ($k9 ""), ($k8 ""), ($k7 ""), ($k6 ""),
			($k5 ""), ($k4 ""), ($k3 ""), ($k2 ""), ($k1 ""),
			($k12 "// Wrap."), ($k11 "// Wrap."), ($k10 "// Wrap."),
		),
		"// …\n",
	));

	// Wrong Word.
	(@wrong Month) =>   ( "Janissary" );
	(@wrong Weekday) => ( "Sunlight" );

	// Match Pairs.
	(@pairs $src:ident $(($from:ident $to:ident)),+ $(,)?) => (
		match $src {
			$( Self::$from => Self::$to, )+
		}
	);

	// Add.
	(@add $uint:tt $ty:tt $( $k:ident $v:literal)+) => (
		impl ::std::ops::Add<$uint> for $ty {
			type Output = Self;

			#[inline]
			#[doc = concat!(
				"# Wrapping `", stringify!($uint), "` Addition.\n\n",

				$crate::macros::weekmonth!(@ex @range $ty $($k $v)+), "\n\n",

				"## Examples\n\n",

				"```\n",
				"use utc2k::", stringify!($ty), ";\n\n",
				$crate::macros::weekmonth!(@ex @add $uint $ty $($k)+),
				"```",
			)]
			fn add(self, other: $uint) -> Self {
				Self::from(self as $uint + other % $crate::macros::last!($($v)+),)
			}
		}

		impl ::std::ops::AddAssign<$uint> for $ty {
			#[inline]
			fn add_assign(&mut self, other: $uint) { *self = *self + other; }
		}
	);

	// PartialEq.
	(@eq $uint:ident $ty:ident $( $k:ident $v:literal)+) => (
		impl PartialEq<$uint> for $ty {
			#[inline]
			#[doc = concat!(
				"# `", stringify!($ty), "`/`", stringify!($uint), "` Equality.\n\n",
				"```\n",
				"use utc2k::", stringify!($ty), ";\n\n",
				$(
					"assert_eq!(", stringify!($ty), "::", stringify!($k), ", ", stringify!($v), "_", stringify!($uint), ");\n",
				)+
				"\n// Nope.\n",
				"assert_ne!(", stringify!($ty), "::", $crate::macros::first!(@stringify $($k)+), ", ", stringify!($uint), "::MIN);\n",
				"```",
			)]
			fn eq(&self, other: &$uint) -> bool { (*self as $uint) == *other }
		}

		impl PartialEq<$ty> for $uint {
			#[inline]
			#[doc = concat!(
				"# `", stringify!($uint), "`/`", stringify!($ty), "` Equality.\n\n",
				"```\n",
				"use utc2k::", stringify!($ty), ";\n\n",
				$(
					"assert_eq!(", stringify!($v), "_", stringify!($uint), ", ", stringify!($ty), "::", stringify!($k), ");\n",
				)+
				"```",
				"\n// Nope.\n",
				"assert_ne!(", stringify!($uint), "::MIN, ", stringify!($ty), "::", $crate::macros::first!(@stringify $($k)+), ");\n",
			)]
			fn eq(&self, other: &$ty) -> bool { <$ty as PartialEq<$uint>>::eq(other, self) }
		}
	);

	// From.
	(@from $uint:ident $ty:ident $( $k:ident $v:literal )+ @last $k_last:ident $v_last:literal) => (
		impl From<$uint> for $ty {
			#[inline]
			#[doc = concat!(
				"# `", stringify!($ty), "` From `", stringify!($uint), "` (Wrapping).\n\n",

				$crate::macros::weekmonth!(@ex @range $ty $($k $v)+ $k_last $v_last), "\n\n",

				"## Examples\n\n",

				"```\n",
				"use utc2k::", stringify!($ty), ";\n\n",
				"assert_eq!(", stringify!($ty), "::from(0_", stringify!($uint), "), ", stringify!($ty), "::", stringify!($k_last), "); // Wrap.\n",
				$(
					"assert_eq!(", stringify!($ty), "::from(", stringify!($v), "_", stringify!($uint), "), ", stringify!($ty), "::", stringify!($k), ");\n",
				)+
				"assert_eq!(", stringify!($ty), "::from(", stringify!($v_last), "_", stringify!($uint), "), ", stringify!($ty), "::", stringify!($k_last), ");\n",
				$crate::macros::weekmonth!(@ex @from_a $uint $ty $($k)+ $k_last ),
				"```",
			)]
			fn from(src: $uint) -> Self {
				match src % $v_last {
					$( $v => Self::$k, )+
					_ => Self::$k_last,
				}
			}
		}

		impl From<$ty> for $uint {
			#[inline]
			#[doc = concat!(
				"# `", stringify!($uint), "` From `", stringify!($ty), "`.\n\n",

				$crate::macros::weekmonth!(@ex @range $ty $($k $v)+ $k_last $v_last), "\n\n",

				"## Examples\n\n",

				"```\n",
				"use utc2k::", stringify!($ty), ";\n\n",
				$(
					"assert_eq!(", stringify!($uint), "::from(", stringify!($ty), "::", stringify!($k), "), ", stringify!($v), ");\n",
				)+
				"assert_eq!(", stringify!($uint), "::from(", stringify!($ty), "::", stringify!($k_last), "), ", stringify!($v_last), ");\n\n",
				"// Same as `as` casting.\n",
				"for v in ", stringify!($ty), "::ALL {\n",
				"    assert_eq!(", stringify!($uint), "::from(v), v as ", stringify!($uint), ");\n",
				"}\n",
				"```",
			)]
			fn from(src: $ty) -> Self {
				match src {
					$( <$ty>::$k => $v, )+
					<$ty>::$k_last => $v_last,
				}
			}
		}
	);
	(
		@from $uint:ident $ty:ident
		$k1:ident $v1:literal $k2:ident $v2:literal $k3:ident $v3:literal $k4:ident $v4:literal $k5:ident $v5:literal $k6:ident $v6:literal $k7:ident $v7:literal
	) => (
		$crate::macros::weekmonth!(
			@from $uint $ty
			$k1 $v1 $k2 $v2 $k3 $v3 $k4 $v4 $k5 $v5 $k6 $v6
			@last $k7 $v7
		);
	);
	(
		@from $uint:ident $ty:ident
		$k1:ident $v1:literal $k2:ident $v2:literal $k3:ident $v3:literal $k4:ident $v4:literal $k5:ident $v5:literal $k6:ident $v6:literal
		$k7:ident $v7:literal $k8:ident $v8:literal $k9:ident $v9:literal $k10:ident $v10:literal $k11:ident $v11:literal $k12:ident $v12:literal
	) => (
		$crate::macros::weekmonth!(
			@from $uint $ty
			$k1 $v1 $k2 $v2 $k3 $v3 $k4 $v4 $k5 $v5 $k6 $v6
			$k7 $v7 $k8 $v8 $k9 $v9 $k10 $v10 $k11 $v11
			@last $k12 $v12
		);
	);

	// Constant From U8.
	(@from_u8 $ty:ident $($k:ident $v:literal)+ @last $k_last:ident $v_last:literal) => (
		impl $ty {
			#[inline]
			#[must_use]
			/// # From `u8`.
			pub(crate) const fn from_u8(src: u8) -> Self {
				match src % $v_last {
					$( $v => Self::$k, )+
					_ => Self::$k_last,
				}
			}
		}
	);
	(
		@from_u8 $ty:ident
		$k1:ident $v1:literal $k2:ident $v2:literal $k3:ident $v3:literal $k4:ident $v4:literal $k5:ident $v5:literal $k6:ident $v6:literal $k7:ident $v7:literal
	) => (
		$crate::macros::weekmonth!(
			@from_u8 $ty
			$k1 $v1 $k2 $v2 $k3 $v3 $k4 $v4 $k5 $v5 $k6 $v6
			@last $k7 $v7
		);
	);
	(
		@from_u8 $ty:ident
		$k1:ident $v1:literal $k2:ident $v2:literal $k3:ident $v3:literal $k4:ident $v4:literal $k5:ident $v5:literal $k6:ident $v6:literal
		$k7:ident $v7:literal $k8:ident $v8:literal $k9:ident $v9:literal $k10:ident $v10:literal $k11:ident $v11:literal $k12:ident $v12:literal
	) => (
		$crate::macros::weekmonth!(
			@from_u8 $ty
			$k1 $v1 $k2 $v2 $k3 $v3 $k4 $v4 $k5 $v5 $k6 $v6
			$k7 $v7 $k8 $v8 $k9 $v9 $k10 $v10 $k11 $v11
			@last $k12 $v12
		);
	);

	// Subtract.
	(@sub $uint:tt $ty:tt $k_first:ident $v_first:literal ($sub1_first:literal $sub2_first:literal), $( $k:ident $v:literal ($sub1:literal $sub2:literal) ),+ $(,)?) => (
		impl ::std::ops::Sub<$uint> for $ty {
			type Output = Self;

			#[inline]
			#[doc = concat!(
				"# Wrapping `", stringify!($uint), "` Subtraction.\n\n",

				$crate::macros::weekmonth!(@ex @range $ty $k_first $v_first $($k $v)+), "\n\n",

				"## Examples\n\n",

				"```\n",
				"use utc2k::", stringify!($ty), ";\n\n",
				$crate::macros::weekmonth!(@ex @sub $uint $ty $k_first $($k)+),
				"```",
			)]
			fn sub(self, other: $uint) -> Self {
				match (self as u8 - 1).wrapping_sub((other % $crate::macros::last!($($v)+)) as u8) {
					0 => Self::$k_first,
					$( $sub1 | $sub2 => Self::$k, )+
					_ => unreachable!(),
				}
			}
		}

		impl ::std::ops::SubAssign<$uint> for $ty {
			#[inline]
			fn sub_assign(&mut self, other: $uint) { *self = *self - other; }
		}
	);

	// TryFrom Reference.
	(@try_from @as_bytes $ty:tt $($from:ty)+) => ($(
		impl TryFrom<$from> for $ty {
			type Error = Utc2kError;

			#[inline]
			fn try_from(src: $from) -> Result<Self, Self::Error> {
				Self::try_from(src.as_bytes())
			}
		}
	)+);

	// Integer implementations.
	(@int $uint:ident $ty:ident $( $k:ident $v:literal ($sub1:literal $sub2:literal) ),+ $(,)?) => (
		$crate::macros::weekmonth!(@add  $uint $ty $($k $v)+);
		$crate::macros::weekmonth!(@eq   $uint $ty $($k $v)+);
		$crate::macros::weekmonth!(@from $uint $ty $($k $v)+);
		$crate::macros::weekmonth!(@sub  $uint $ty $($k $v ($sub1 $sub2)),+);
	);

	// Entrypoint.
	($ty:tt $lower:ident $iter:ident $($k:ident $v:literal $abbr:literal ($sub1:literal $sub2:literal)),+ $(,)?) => (
		#[expect(missing_docs, reason = "Redundant.")]
		#[repr(u8)]
		#[derive(Debug, Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
		#[doc = concat!(
			"# ", stringify!($ty), ".\n\n",

			"This enum is used by [`Utc2k`] to differentiate between calendar ", stringify!($lower), "s.\n\n",

			"## Examples\n\n",

			"```\n",
			"use utc2k::", stringify!($ty), ";\n\n",
			"// The first.\n",
			"# assert_eq!(", stringify!($ty), "::default(), ", stringify!($ty), "::", $crate::macros::first!(@stringify $($k)+), ");\n",
			"assert_eq!(", stringify!($ty) ,"::", $crate::macros::first!(@stringify $($k)+), " as u8, 1_u8);\n",
			"assert_eq!(", stringify!($ty) ,"::", $crate::macros::first!(@stringify $($k)+), ".as_str(), \"", $crate::macros::first!(@stringify $($k)+), "\");\n",
			"assert_eq!(", stringify!($ty) ,"::", $crate::macros::first!(@stringify $($k)+), ".abbreviation(), \"", $crate::macros::first!($($abbr)+), "\");\n\n",
			"// The last.\n",
			"assert_eq!(", stringify!($ty) ,"::", $crate::macros::last!(@stringify $($k)+), " as u8, ", $crate::macros::last!(@stringify $($v)+), "_u8);\n",
			"assert_eq!(", stringify!($ty) ,"::", $crate::macros::last!(@stringify $($k)+), ".as_str(), \"", $crate::macros::last!(@stringify $($k)+), "\");\n",
			"assert_eq!(", stringify!($ty) ,"::", $crate::macros::last!(@stringify $($k)+), ".abbreviation(), \"", $crate::macros::last!($($abbr)+), "\");\n",
			"```",
		)]
		pub enum $ty {
			#[default]
			$( $k = $v ),+
		}

		$crate::macros::as_ref_borrow_cast!($ty: as_str str);
		$crate::macros::display_str!(as_str $ty);
		$crate::macros::weekmonth!(
			@try_from @as_bytes $ty
			&str &String String &std::borrow::Cow<'_, str>
			std::borrow::Cow<'_, str> &Box<str> Box<str>
		);

		impl From<Utc2k> for $ty {
			#[inline]
			#[doc = concat!(
				"# From [`Utc2k`].\n\n",

				"This is equivalent to calling [`Utc2k::", stringify!($lower), "`].\n\n",

				"## Examples\n\n",

				"```\n",
				"use utc2k::{", stringify!($ty), ", Utc2k};\n\n",
				"let utc = Utc2k::new(2030, 1, 6, 0, 0, 0);\n",
				"assert_eq!(", stringify!($ty), "::", $crate::macros::first!(@stringify $($k)+), ", ", stringify!($ty), "::from(utc));\n",
				"assert_eq!(", stringify!($ty), "::", $crate::macros::first!(@stringify $($k)+), ", utc.", stringify!($lower), "());\n",
				"```",
			)]
			fn from(src: Utc2k) -> Self { src.$lower() }
		}

		impl ::std::str::FromStr for $ty {
			type Err = Utc2kError;

			#[inline]
			#[doc = concat!(
				"# Parse From String.\n\n",

				"Parse a `", stringify!($ty), "` from the first three letters \
				of a string, case-insensitively.\n\n",

				"## Examples\n\n",

				"```\n",
				"use utc2k::", stringify!($ty), ";\n\n",
				"for v in ", stringify!($ty), "::ALL {\n",
				"    assert_eq!(v.as_str().parse::<", stringify!($ty), ">(), Ok(v));\n",
				"    assert_eq!(v.abbreviation().parse::<", stringify!($ty), ">(), Ok(v));\n",
				"}\n\n",
				"// Remember that only the first three letters count!\n",
				"assert_eq!(\"", $crate::macros::weekmonth!(@wrong $ty), "\".parse::<", stringify!($ty), ">(), Ok(", stringify!($ty), "::", $crate::macros::first!(@stringify $($k)+), "));\n",
				"```",
			)]
			fn from_str(src: &str) -> Result<Self, Self::Err> { Self::try_from(src) }
		}

		impl IntoIterator for $ty {
			type Item = Self;
			type IntoIter = $iter;

			#[inline]
			#[doc = concat!(
				"# Endless `", stringify!($ty), "` Iterator.\n\n",

				"Return an iterator that will cycle endlessly through the ", stringify!($lower), "s, \
				in order, forward or backward, starting with `self`.\n\n",

				"## Examples\n\n",

				"```\n",
				"use utc2k::", stringify!($ty), ";\n\n",
				$crate::macros::weekmonth!(@ex @iter $ty $($k)+),
				"```",
			)]
			fn into_iter(self) -> Self::IntoIter { $iter(self) }
		}

		$crate::macros::weekmonth!(@int u8    $ty $($k $v ($sub1 $sub2)),+);
		$crate::macros::weekmonth!(@int u16   $ty $($k $v ($sub1 $sub2)),+);
		$crate::macros::weekmonth!(@int u32   $ty $($k $v ($sub1 $sub2)),+);
		$crate::macros::weekmonth!(@int u64   $ty $($k $v ($sub1 $sub2)),+);
		$crate::macros::weekmonth!(@int usize $ty $($k $v ($sub1 $sub2)),+);

		impl $ty {
			#[doc = concat!(
				"# All ", stringify!($ty), "s.\n\n",
				"This array contains all of the ", stringify!($lower), "s, in order.\n\n",
				"## Examples\n\n",
				"```\n",
				"# assert!(utc2k::", stringify!($ty), "::ALL.is_sorted());\n",
				"for pair in utc2k::", stringify!($ty), "::ALL.windows(2) {\n",
				"    assert!(pair[0] < pair[1]);\n",
				"}\n",
				"```",
			)]
			pub const ALL: [Self; $crate::macros::last!($($v)+)] = [ $( Self::$k ),+ ];

			#[inline]
			#[must_use]
			#[doc = concat!(
				"# As String Slice (Abbreviated).\n\n",

				"Return the three-letter abbreviation for a given ", stringify!($lower), " as a static string slice.\n\n",

				"## Examples\n\n",

				"```\n",
				"use utc2k::", stringify!($ty), ";\n\n",
				$(
					"assert_eq!(", stringify!($ty), "::", stringify!($k), ".abbreviation(), \"", $abbr, "\");\n",
				)+
				"# for v in ", stringify!($ty), "::ALL {\n",
				"#    assert_eq!(v.abbreviation(), &v.as_str()[..3]);\n",
				"# }\n",
				"```",
			)]
			pub const fn abbreviation(self) -> &'static str {
				match self {
					$( Self::$k => $abbr ),+
				}
			}

			#[inline]
			#[must_use]
			#[doc = concat!(
				"# As String Slice.\n\n",

				"Return the name of a given ", stringify!($lower), " as a static string slice.\n\n",

				"## Examples\n\n",

				"```\n",
				"use utc2k::", stringify!($ty), ";\n\n",
				$(
					"assert_eq!(", stringify!($ty), "::", stringify!($k), ".as_str(), \"", stringify!($k), "\");\n",
				)+
				"```",
			)]
			pub const fn as_str(self) -> &'static str {
				match self {
					$( Self::$k => stringify!($k) ),+
				}
			}

			#[inline]
			#[must_use]
			#[doc = concat!(
				"# Previous ", stringify!($ty), " (Wrapping).\n\n",

				"Return the previous [`", stringify!($ty), "`].\n\n",

				"## Examples\n\n",

				"```\n",
				"use utc2k::", stringify!($ty), ";\n\n",
				$crate::macros::pair!(@previous weekmonth { @ex @pairs $ty "previous" } $($k)+), "\n",
				"// Same as math:\n",
				"assert_eq!(", stringify!($ty), "::", $crate::macros::first!(@stringify $($k)+), ".previous(), ", stringify!($ty), "::", $crate::macros::first!(@stringify $($k)+), " - 1_u8);\n\n",
				"// Same as the proper iterator too (provided you skip the first value):\n",
				"assert_eq!(\n",
				"    Some(", stringify!($ty), "::", $crate::macros::first!(@stringify $($k)+), ".previous()),\n",
				"    ", stringify!($ty), "::", $crate::macros::first!(@stringify $($k)+), ".into_iter().rev().skip(1).next(),\n",
				");\n",
				"```",
			)]
			pub const fn previous(self) -> Self {
				$crate::macros::pair!(@previous weekmonth { @pairs self } $($k)+)
			}

			#[inline]
			#[must_use]
			#[doc = concat!(
				"# Next ", stringify!($ty), " (Wrapping).\n\n",

				"Return the next [`", stringify!($ty), "`].\n\n",

				"## Examples\n\n",

				"```\n",
				"use utc2k::", stringify!($ty), ";\n\n",
				$crate::macros::pair!(@next weekmonth { @ex @pairs $ty "next" } $($k)+), "\n",
				"// Same as math:\n",
				"assert_eq!(", stringify!($ty), "::", $crate::macros::first!(@stringify $($k)+), ".next(), ", stringify!($ty), "::", $crate::macros::first!(@stringify $($k)+), " + 1_u8);\n\n",
				"// Same as the proper iterator too (provided you skip the first value):\n",
				"assert_eq!(\n",
				"    Some(", stringify!($ty), "::", $crate::macros::first!(@stringify $($k)+), ".next()),\n",
				"    ", stringify!($ty), "::", $crate::macros::first!(@stringify $($k)+), ".into_iter().skip(1).next(),\n",
				");\n",
				"```",
			)]
			pub const fn next(self) -> Self {
				$crate::macros::pair!(@next weekmonth { @pairs self } $($k)+)
			}

			#[inline]
			#[must_use]
			#[doc = concat!(
				"# Compare Two `", stringify!($ty), "`s.\n\n",

				"Same as `Ord`/`PartialOrd`, but constant.\n\n",

				"## Examples\n\n",

				"```\n",
				"use utc2k::", stringify!($ty), ";\n\n",
				"assert_eq!(\n",
				"    ", stringify!($ty), "::", $crate::macros::first!(@stringify $($k)+), ".cmp(&", stringify!($ty), "::", $crate::macros::first!(@stringify $($k)+), "),\n",
				"    ", stringify!($ty), "::cmp(", stringify!($ty), "::", $crate::macros::first!(@stringify $($k)+), ", ", stringify!($ty), "::", $crate::macros::first!(@stringify $($k)+), "), // Ordering::Equal\n",
				");\n",
				"assert_eq!(\n",
				"    ", stringify!($ty), "::", $crate::macros::first!(@stringify $($k)+), ".cmp(&", stringify!($ty), "::", $crate::macros::last!(@stringify $($k)+), "),\n",
				"    ", stringify!($ty), "::cmp(", stringify!($ty), "::", $crate::macros::first!(@stringify $($k)+), ", ", stringify!($ty), "::", $crate::macros::last!(@stringify $($k)+), "), // Ordering::Less\n",
				");\n",
				"assert_eq!(\n",
				"    ", stringify!($ty), "::", $crate::macros::last!(@stringify $($k)+), ".cmp(&", stringify!($ty), "::", $crate::macros::first!(@stringify $($k)+), "),\n",
				"    ", stringify!($ty), "::cmp(", stringify!($ty), "::", $crate::macros::last!(@stringify $($k)+), ", ", stringify!($ty), "::", $crate::macros::first!(@stringify $($k)+), "), // Ordering::Greater\n",
				");\n",
				"```",
			)]
			pub const fn cmp(a: Self, b: Self) -> ::std::cmp::Ordering {
				let a = a as u8;
				let b = b as u8;
				if a == b { ::std::cmp::Ordering::Equal }
				else if a < b { ::std::cmp::Ordering::Less }
				else { ::std::cmp::Ordering::Greater }
			}
		}

		$crate::macros::weekmonth!(@from_u8 $ty $($k $v)+);

		#[derive(Debug, Clone)]
		#[doc = concat!(
			"# Endless `", stringify!($ty), "` Iterator.\n\n",

			"This iterator yields infinite [`", stringify!($ty), "`]s, \
			in order, forward or backward, starting with any arbitrary variant.\n\n",

			"See [`", stringify!($ty), "::into_iter`] for more details.",
		)]
		pub struct $iter($ty);

		impl Iterator for $iter {
			type Item = $ty;

			#[inline]
			#[doc = concat!("# Next [`", stringify!($ty), "`].")]
			fn next(&mut self) -> Option<Self::Item> {
				let next = self.0;
				self.0 = next + 1_u8;
				Some(next)
			}

			#[inline]
			/// # Infinity.
			///
			/// This iterator never stops!
			fn size_hint(&self) -> (usize, Option<usize>) { (usize::MAX, None) }
		}

		impl DoubleEndedIterator for $iter {
			#[inline]
			#[doc = concat!("# Previous [`", stringify!($ty), "`].")]
			fn next_back(&mut self) -> Option<Self::Item> {
				let next = self.0;
				self.0 = next - 1_u8;
				Some(next)
			}
		}
	);
}



pub(super) use {
	as_ref_borrow_cast,
	display_str,
	first,
	last,
	pair,
	weekmonth,
};
