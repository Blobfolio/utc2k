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

/*
/// # Helper: Last in List.
macro_rules! last {
	// Slap a macro call around the answer.
	(@$mac:ident $last:tt) => ( $mac!($last) );
	(@$mac:ident $_next:tt $($rest:tt)+) => ( $crate::macros::last!(@$mac $($rest)+) );

	// Passthrough.
	($last:tt) => ( $mac!($last) );
	($_next:tt $($rest:tt)+) => ( $crate::macros::last!($($rest)+) );
}
*/

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

/// # Helper: `Weekday`/`Month` Iterators.
///
/// This macro generates both the iterator struct and `IntoIterator` enum
/// implementation.
macro_rules! weekmonth_iter {
	// Example Pairs (Back/Next).
	(@doc @pairs $ty:ident $fn:literal $(($from:ident $to:ident)),+ $(,)?) => (concat!(
		$(
			"assert_eq!(",
			stringify!($ty), "::", stringify!($from), ".", $fn, "(), ",
			stringify!($ty), "::", stringify!($to),
			");\n",
		)+
	));

	// Example Assertions.
	(@doc $ty:tt $(($k:ident $com:literal)),+ $(,)?) => (concat!(
		$(
			"assert_eq!(iter.next(), Some(", stringify!($ty), "::", stringify!($k), ")); ", $com, "\n",
		)+
	));

	// Example (Weekday).
	(@doc $ty:tt $k1:ident $k2:ident $k3:ident $k4:ident $k5:ident $k6:ident $k7:ident) => (concat!(
		"let mut iter = ", stringify!($ty), "::", stringify!($k1), ".into_iter();\n",
		$crate::macros::weekmonth_iter!(
			@doc $ty
			($k1 ""), ($k2 ""), ($k3 ""), ($k4 ""), ($k5 ""), ($k6 ""), ($k7 ""),
			($k1 "// Wrap."), ($k2 "// Wrap."), ($k3 "// Wrap."),
		),
		"// …\n\n",
		"// Or like Ginger, backwards and in high heels.\n",
		"let mut iter = ", stringify!($ty), "::", stringify!($k7), ".into_iter().rev();\n",
		$crate::macros::weekmonth_iter!(
			@doc $ty
			($k7 ""), ($k6 ""), ($k5 ""), ($k4 ""), ($k3 ""), ($k2 ""), ($k1 ""),
			($k7 "// Wrap."), ($k6 "// Wrap."), ($k5 "// Wrap."),
		),
		"// …\n",
	));

	// Example (Month).
	(@doc $ty:tt $k1:ident $k2:ident $k3:ident $k4:ident $k5:ident $k6:ident $k7:ident $k8:ident $k9:ident $k10:ident $k11:ident $k12:ident) => (concat!(
		"let mut iter = ", stringify!($ty), "::", stringify!($k1), ".into_iter();\n",
		$crate::macros::weekmonth_iter!(
			@doc $ty
			($k1 ""), ($k2 ""), ($k3 ""), ($k4 ""), ($k5 ""), ($k6 ""), ($k7 ""),
			($k8 ""), ($k9 ""), ($k10 ""), ($k11 ""), ($k12 ""),
			($k1 "// Wrap."), ($k2 "// Wrap."), ($k3 "// Wrap."),
		),
		"// …\n\n",
		"// Or like Ginger:\n",
		"let mut iter = ", stringify!($ty), "::", stringify!($k12), ".into_iter().rev();\n",
		$crate::macros::weekmonth_iter!(
			@doc $ty
			($k12 ""), ($k11 ""), ($k10 ""), ($k9 ""), ($k8 ""), ($k7 ""), ($k6 ""),
			($k5 ""), ($k4 ""), ($k3 ""), ($k2 ""), ($k1 ""),
			($k12 "// Wrap."), ($k11 "// Wrap."), ($k10 "// Wrap."),
		),
		"// …\n",
	));

	// Match Pairs.
	(@pairs $src:ident $(($from:ident $to:ident)),+ $(,)?) => (
		match $src {
			$( Self::$from => Self::$to, )+
		}
	);

	// Entrypoint.
	(
		$ty:tt $lower:literal $iter:ident
		$($k:ident)+
	) => (
		impl IntoIterator for $ty {
			type Item = Self;
			type IntoIter = $iter;

			#[inline]
			#[doc = concat!(
				"# Endless `", stringify!($ty), "` Iterator.\n\n",

				"Return an iterator that will cycle endlessly through the ", $lower, "s, \
				in order, forward or backward, starting with `self`.\n\n",

				"## Examples\n\n",

				"```\n",
				"use utc2k::", stringify!($ty), ";\n\n",
				$crate::macros::weekmonth_iter!(@doc $ty $($k)+),
				"```",
			)]
			fn into_iter(self) -> Self::IntoIter { $iter(self) }
		}

		impl $ty {
			#[inline]
			#[must_use]
			#[doc = concat!(
				"# Previous ", stringify!($ty), " (Wrapping).\n\n",

				"Return the previous [`", stringify!($ty), "`].\n\n",

				"## Examples\n\n",

				"```\n",
				"use utc2k::", stringify!($ty), ";\n\n",
				$crate::macros::pair!(@previous weekmonth_iter { @doc @pairs $ty "previous" } $($k)+), "\n",
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
				$crate::macros::pair!(@previous weekmonth_iter { @pairs self } $($k)+)
			}

			#[inline]
			#[must_use]
			#[doc = concat!(
				"# Next ", stringify!($ty), " (Wrapping).\n\n",

				"Return the next [`", stringify!($ty), "`].\n\n",

				"## Examples\n\n",

				"```\n",
				"use utc2k::", stringify!($ty), ";\n\n",
				$crate::macros::pair!(@next weekmonth_iter { @doc @pairs $ty "next" } $($k)+), "\n",
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
				$crate::macros::pair!(@next weekmonth_iter { @pairs self } $($k)+)
			}
		}

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
	// last,
	pair,
	weekmonth_iter,
};
