//! The main entrance of Scrut.

use std::error::Error;

mod utils;
use utils::args::{CommonFlags, Parser, parse_arg};

fn main() -> Result<(), Box<dyn Error>> {
    let flags = CommonFlags::new(false, false);
    let arg_parser = Parser::new(&flags);
    parse_arg(arg_parser)
}
