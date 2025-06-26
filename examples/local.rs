/*!
# Local2k.

**This requires the `local` crate feature.**
*/

#[cfg(not(feature = "local"))]
fn main() {
	panic!("This example requires the 'local' crate feature.");
}

#[cfg(feature = "local")]
fn main() {
	use utc2k::{
		Local2k,
		Utc2k,
	};

	// The current time.
	let utc = Utc2k::now();         // UTC.
	let local = Local2k::from(utc); // Local.

	println!(
		"\
Simple:
    \x1b[2mUTC\x1b[0m {utc}
  \x1b[2mLocal\x1b[0m {local}

RFC2822:
    \x1b[2mUTC\x1b[0m {utc_2822}
  \x1b[2mLocal\x1b[0m {local_2822}

RFC3339:
    \x1b[2mUTC\x1b[0m {utc_3339}
  \x1b[2mLocal\x1b[0m {local_3339}\
		",
		utc_2822=utc.to_rfc2822(),
		local_2822=local.to_rfc2822(),
		utc_3339=utc.to_rfc3339(),
		local_3339=local.to_rfc3339(),
	);

	// Same thing but from the formatted structs.
	let fmt_utc = utc.formatted();
	let fmt_local = local.formatted();

	println!(
		"
\x1b[2m-----\x1b[0m

Simple:
    \x1b[2mUTC\x1b[0m {fmt_utc}
  \x1b[2mLocal\x1b[0m {fmt_local}

RFC2822:
    \x1b[2mUTC\x1b[0m {fmt_utc_2822}
  \x1b[2mLocal\x1b[0m {fmt_local_2822}

RFC3339:
    \x1b[2mUTC\x1b[0m {fmt_utc_3339}
  \x1b[2mLocal\x1b[0m {fmt_local_3339}\
		",
		fmt_utc_2822=fmt_utc.to_rfc2822(),
		fmt_local_2822=fmt_local.to_rfc2822(),
		fmt_utc_3339=fmt_utc.to_rfc3339(),
		fmt_local_3339=fmt_local.to_rfc3339(),
	);
}
