// public domain.
use std::fmt::Write;


extern {
	fn console_log_raw_str( _: *const u8, _: usize );
}

struct JsConsole;

impl std::fmt::Write for JsConsole {
	fn write_str( &mut self, s: &str ) -> Result<(), std::fmt::Error> {
		unsafe { console_log_raw_str( s.as_ptr(), s.len() ) };
		Ok( () )
	}
}

#[no_mangle]
pub fn main() {
	writeln!( JsConsole, "test." ).unwrap();
}
