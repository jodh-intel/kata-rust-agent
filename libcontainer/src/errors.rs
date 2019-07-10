// define errors here

error_chain! {
	types {
		Error, ErrorKind, ResultExt, Result;
	}
	// foreign error conv to chain error
	foreign_links {
		Io(std::io::Error);
		Nix(nix::Error);
		Ffi(std::ffi::NulError);
		Caps(caps::errors::Error);
		Serde(serde_json::Error);
		UTF8(std::string::FromUtf8Error);
		Parse(std::num::ParseIntError);
		Scanfmt(scan_fmt::parse::ScanError);
	}
	// define new errors
	errors {
		ErrorCode(t: String) {
			description("Error Code")
			display("Error Code: '{}'", t)
		}
	}
}
