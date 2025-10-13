/*!
# UTC2K: Fancy Formatting.

This module contains all the supporting infrastructure for
[`Utc2k::formatted_custom`].

Terrible, right?!

The performance isn't too bad, all things considered. The RFC2822 example is
only about 5x slower than the dedicated `Utc2k::to_rfc2822` helper, and 5-10x
_faster_ than custom formatting with the `time` crate.
*/

use crate::{
	Utc2k,
	Utc2kFormatError,
};



#[derive(Debug, Clone, Copy, Eq, PartialEq)]
/// # Formatting Components.
pub(super) enum Component {
	/// # Year.
	Year(Style, Padding),

	/// # Month.
	Month(Style, Padding),

	/// # Day.
	Day(Style, Padding),

	/// # Hour.
	Hour(Style, Padding),

	/// # Minute.
	Minute(Padding),

	/// # Second.
	Second(Padding),

	/// # Ordinal.
	Ordinal(Padding),

	/// # AM/PM.
	Period(Style),

	/// # Unix Timestamp.
	Unixtime,

	/// # Pass-Through (ASCII).
	Literal(u8),
}

impl Component {
	#[inline]
	/// # Format Date.
	///
	/// This method does all the work for [`Utc2k::formatted_custom`]. That
	/// module has enough on its plate already. Haha.
	pub(super) fn format_date(date: Utc2k, fmt: &str)
	-> Result<String, Utc2kFormatError> {
		if ! fmt.is_ascii() { return Err(Utc2kFormatError::NotAscii); }

		let mut out = String::with_capacity(64); // Magic number.
		let mut buf = U32DigitBuffer::DEFAULT;   // *Probably* needed.

		let mut fmt = fmt.as_bytes();
		while let Some((next, rest)) = Self::parse(fmt)? {
			fmt = rest;
			match next {
				Self::Year(style, pad) =>
					if matches!(style, Style::Main) {
						out.push_str(date.y.as_str_full());
					}
					else { buf.write2(date.y as u32, pad, &mut out); },

				Self::Month(style, pad) => match style {
					Style::Main => { buf.write2(u32::from(date.m), pad, &mut out); },
					Style::Alt1 => { out.push_str(date.m.as_str()); },
					Style::Alt2 => { out.push_str(date.m.abbreviation()); },
				},

				Self::Day(style, pad) => match style {
					Style::Main => { buf.write2(u32::from(date.d), pad, &mut out); },
					Style::Alt1 => { out.push_str(date.weekday().as_str()); },
					Style::Alt2 => { out.push_str(date.weekday().abbreviation()); },
				},

				Self::Hour(style, pad) => {
					let hh = u32::from(
						if matches!(style, Style::Main) { date.hh }
						else { date.hour_12() }
					);
					buf.write2(hh, pad, &mut out);
				},

				Self::Minute(pad) => {
					buf.write2(u32::from(date.mm), pad, &mut out);
				},

				Self::Second(pad) => {
					buf.write2(u32::from(date.ss), pad, &mut out);
				},

				Self::Ordinal(pad) => {
					buf.write3(u32::from(date.ordinal()), pad, &mut out);
				},

				Self::Period(style) => {
					let p = date.hour_period();
					match style {
						Style::Main => { out.push_str(p.as_str(false)); },
						Style::Alt1 => { out.push_str(p.as_str_ap()); },
						Style::Alt2 => { out.push_str(p.as_str(true)); },
					}
				},

				Self::Unixtime => { out.extend(buf.format(date.unixtime())); },

				Self::Literal(v) => { out.push(v as char); },
			}
		}

		Ok(out)
	}
}

impl Component {
	/// # Parse Component.
	///
	/// This method parses the next character or sequence, returning it
	/// along with what remains of the source slice, if anything.
	const fn parse(mut raw: &[u8]) -> Result<Option<(Self, &[u8])>, Utc2kFormatError> {
		let mut style = Style::Main;
		let mut padding = Padding::Zero;

		macro_rules! parse_props {
			( $fn:ident ) => (
				loop {
					match Modifier::$fn(raw) {
						Ok((Some(Modifier::Padding(v)), rest)) => {
							padding = v;
							raw = rest;
						},
						Ok((Some(Modifier::Style(v)), rest)) => {
							style = v;
							raw = rest;
						},
						Ok((None, rest)) => {
							raw = rest;
							break;
						},
						Err(e) => return Err(e),
					}
				}
			);
		}

		// Check to see what's next.
		match raw {
			// Literal bracket.
			[ b'[', b'[', rest @ .. ] => return Ok(Some((Self::Literal(b'['), rest))),

			// Start of component.
			[ b'[', rest @ .. ] => {
				raw = rest;
			},

			// Other literal.
			[ n, rest @ .. ] => return Ok(Some((Self::Literal(*n), rest))),

			// Nothing.
			[] => return Ok(None),
		}

		#[expect(unused_assignments, reason = "Macro made me do it.")]
		// Parse component.
		match raw.trim_ascii_start() {
			[ b'y', b'e', b'a', b'r', rest @ .. ] => {
				raw = rest;
				parse_props!(parse_year);
				Ok(Some((Self::Year(style, padding), raw)))
			},

			[ b'm', b'o', b'n', b't', b'h', rest @ .. ] => {
				raw = rest;
				parse_props!(parse_month);
				Ok(Some((Self::Month(style, padding), raw)))
			},

			[ b'd', b'a', b'y', rest @ .. ] => {
				raw = rest;
				parse_props!(parse_day);
				Ok(Some((Self::Day(style, padding), raw)))
			},

			[ b'h', b'o', b'u', b'r', rest @ .. ] => {
				raw = rest;
				parse_props!(parse_hour);
				Ok(Some((Self::Hour(style, padding), raw)))
			},

			[ b'm', b'i', b'n', b'u', b't', b'e', rest @ .. ] => {
				raw = rest;
				parse_props!(parse_minute);
				Ok(Some((Self::Minute(padding), raw)))
			},

			[ b's', b'e', b'c', b'o', b'n', b'd', rest @ .. ] => {
				raw = rest;
				parse_props!(parse_second);
				Ok(Some((Self::Second(padding), raw)))
			},

			[ b'o', b'r', b'd', b'i', b'n', b'a', b'l', rest @ .. ] => {
				raw = rest;
				parse_props!(parse_ordinal);
				Ok(Some((Self::Ordinal(padding), raw)))
			},

			[ b'p', b'e', b'r', b'i', b'o', b'd', rest @ .. ] => {
				raw = rest;
				parse_props!(parse_period);
				Ok(Some((Self::Period(style), raw)))
			},

			[ b'u', b'n', b'i', b'x', b't', b'i', b'm', b'e', rest @ .. ] =>  match rest.trim_ascii_start() {
				[ b']', rest @ .. ] => Ok(Some((Self::Unixtime, rest))),
				[ b'@', .. ] => Err(Utc2kFormatError::InvalidModifier("The [unixtime] component has no modifiers.")),
				_ => Err(Utc2kFormatError::Eof),
			},

			_ => Err(Utc2kFormatError::InvalidComponent),
		}
	}
}



#[derive(Debug, Clone, Copy, Eq, PartialEq)]
/// # Modifier.
///
/// This is only used as an either/or during property parsing. The components
/// hold both types discretely.
enum Modifier {
	/// # Padding.
	Padding(Padding),

	/// # Style.
	Style(Style),
}

/// # Helper: Property Parsing.
macro_rules! parse_prop {
	(
		$fn:ident
		$( [ $( $byte:literal ),+ ] $kind:ident $flag:ident, )+
		$comp:literal $( $prop:literal )+,
		$expected:ident $(,)?
	) => (
		/// # Parse Property.
		///
		/// Parse a single @modifier, returning the associated flag (if any),
		/// `None` if not, or an error if there's something unexpected.
		const fn $fn(raw: &[u8])
		-> Result<(Option<Self>, &[u8]), Utc2kFormatError> {
			match raw.trim_ascii_start() {
				$(
					[ $( $byte, )+ rest @ .. ] => Ok((Some(Self::$kind($kind::$flag)), rest)),
				)+

				// Done!
				[ b']', rest @ .. ] => Ok((None, rest)),

				// Wrong modifier.
				[ b'@', .. ] => Err(Utc2kFormatError::InvalidModifier(concat!(
					"Expected [", $comp, "… ",
					$( concat!($prop, ", "), )+
					" …].",
				))),

				// Missing closing tag.
				_ => Err(Utc2kFormatError::Eof),
			}
		}
	);
}

impl Modifier {
	parse_prop! {
		parse_year
		[ b'@', b'2' ]                         Style Alt1,
		[ b'@', b's', b'p', b'a', b'c', b'e' ] Padding Space,
		[ b'@', b't', b'r', b'i', b'm' ]       Padding Trim,
		"year" "@2" "@space" "@trim",
		Year,
	}

	parse_prop! {
		parse_month
		[ b'@', b'n', b'a', b'm', b'e' ]       Style Alt1,
		[ b'@', b'a', b'b', b'b', b'r' ]       Style Alt2,
		[ b'@', b's', b'p', b'a', b'c', b'e' ] Padding Space,
		[ b'@', b't', b'r', b'i', b'm' ]       Padding Trim,
		"month" "@name" "@abbr" "@space" "@trim",
		Month,
	}

	parse_prop! {
		parse_day
		[ b'@', b'n', b'a', b'm', b'e' ]       Style Alt1,
		[ b'@', b'a', b'b', b'b', b'r' ]       Style Alt2,
		[ b'@', b's', b'p', b'a', b'c', b'e' ] Padding Space,
		[ b'@', b't', b'r', b'i', b'm' ]       Padding Trim,
		"day" "@name" "@abbr" "@space" "@trim",
		Day,
	}

	parse_prop! {
		parse_hour
		[ b'@', b'1', b'2' ]                   Style Alt1,
		[ b'@', b's', b'p', b'a', b'c', b'e' ] Padding Space,
		[ b'@', b't', b'r', b'i', b'm' ]       Padding Trim,
		"hour" "@12" "@space" "@trim",
		Hour,
	}

	parse_prop! {
		parse_minute
		[ b'@', b's', b'p', b'a', b'c', b'e' ] Padding Space,
		[ b'@', b't', b'r', b'i', b'm' ]       Padding Trim,
		"minute" "@space" "@trim",
		Minute,
	}

	parse_prop! {
		parse_second
		[ b'@', b's', b'p', b'a', b'c', b'e' ] Padding Space,
		[ b'@', b't', b'r', b'i', b'm' ]       Padding Trim,
		"second" "@space" "@trim",
		Second,
	}

	parse_prop! {
		parse_ordinal
		[ b'@', b's', b'p', b'a', b'c', b'e' ] Padding Space,
		[ b'@', b't', b'r', b'i', b'm' ]       Padding Trim,
		"ordinal" "@space" "@trim",
		Ordinal,
	}

	parse_prop! {
		parse_period
		[ b'@', b'a', b'p' ]                   Style Alt1,
		[ b'@', b'u', b'p', b'p', b'e', b'r' ] Style Alt2,
		"period" "@ap" "@upper",
		Period,
	}
}



#[derive(Debug, Clone, Copy, Eq, PartialEq)]
/// # (Left) Padding.
///
/// This enum represents the different padding options for numeric values, with
/// the default being zeroes, since that's the most common.
pub(super) enum Padding {
	/// # Zero.
	Zero,

	/// # Space.
	Space,

	/// # N/A.
	Trim,
}

impl Padding {
	/// # Padding Character.
	const fn as_char(self) -> Option<char> {
		match self {
			Self::Zero => Some('0'),
			Self::Space => Some(' '),
			Self::Trim => None,
		}
	}
}



#[derive(Debug, Clone, Copy, Eq, PartialEq)]
/// # Format Style.
///
/// This enum represents the different, well, _representations_ values might
/// have. The logical meaning and applicability vary by component, but `Main`
/// always serves as the default.
pub(super) enum Style {
	/// # Default.
	Main,

	/// # Alternate One.
	///
	/// Used for:
	/// * `[year @2]`
	/// * `[month @name]`
	/// * `[day @name]`
	/// * `[hour @12]`
	/// * `[period @ap]`
	Alt1,

	/// # Alternate Two.
	///
	/// Used for:
	/// * `[month @abbr]`
	/// * `[day @abbr]`
	/// * `[period @upper]`
	Alt2,
}



#[derive(Debug, Clone, Copy)]
/// # Digit Buffer.
///
/// This struct offers a cheap way to stringify numbers up to `u32::MAX`.
///
/// We don't need much — most of our numbers are `u8` — but need _something_
/// to avoid the (needless) fallibility of `fmt::Write`.
struct U32DigitBuffer([char; 10]);

impl U32DigitBuffer {
	/// # Default Buffer.
	const DEFAULT: Self = Self(['0'; 10]);

	#[expect(clippy::cast_possible_truncation, reason = "False positive.")]
	/// # Digitize a Number.
	///
	/// Return a slice containing each digit represented as an ASCII `char`.
	const fn format(&mut self, mut num: u32) -> &[char] {
		// Fill the buffer, right to left.
		let mut from = self.0.len();
		while 9 < num && 0 < from {
			from -= 1;
			self.0[from] = ((num % 10) as u8 ^ b'0') as char;
			num /= 10;
		}
		from -= 1;
		self.0[from] = (num as u8 ^ b'0') as char;

		// Split off and return the relevant part.
		let (_, b) = self.0.split_at(from);
		b
	}

	#[inline]
	/// # Write to String.
	///
	/// Write at least 2 digits, unless padding is set to trim, in which
	/// case we might write as few as one.
	fn write2(&mut self, num: u32, pad: Padding, out: &mut String) {
		// Stringify.
		let num = self.format(num);

		// Pad?
		if num.len() == 1 {
			match pad {
				Padding::Zero => { out.push('0'); },
				Padding::Space => { out.push(' '); },
				Padding::Trim => {},
			}
		}

		// Write number.
		out.extend(num);
	}

	/// # Write to String.
	///
	/// Write at least 3 digits, unless padding is set to trim, in which
	/// case we might write as few as one.
	fn write3(&mut self, num: u32, pad: Padding, out: &mut String) {
		// Stringify.
		let num = self.format(num);

		// Pad?
		let diff = 3_usize.saturating_sub(num.len());
		if diff != 0 && let Some(pad) = pad.as_char() {
			out.push(pad);
			if diff == 2 { out.push(pad); } // Unlikely, but possible!
		}

		// Write number.
		out.extend(num);
	}
}
