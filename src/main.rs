//! The main entrance of Scrut.

use std::error::Error;

mod utils;
use utils::args::parse_arg;

fn main() -> Result<(), Box<dyn Error>> {
    parse_arg()
}
