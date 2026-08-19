//! The main entrance of Scrut.

use std::error::Error;

mod utils;
use utils::args::{CommonFlags, Parser, parse_arg};
use utils::logging::init_log_file;

fn main() -> Result<(), Box<dyn Error>> {
    init_log_file()?;
    let flags = CommonFlags::new(false, false);
    let arg_parser = Parser::new(&flags);
    parse_arg(arg_parser)
}
