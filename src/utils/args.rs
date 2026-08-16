//! Parse command-line arguments.

use std::{error, ffi::OsStr};

use clap::{self, Arg, Command};

use crate::utils::scan::scan_cwd;
use crate::utils::version::VERSION;

use open::that;

#[allow(unused)]
#[derive(Debug)]
/// Common flags, like -v --verbose.
pub struct CommonFlags {
    verbose: bool, // -v --verbose
    confirm: bool, // -c --confirm
}

#[allow(unused)]
impl CommonFlags {
    /// Create new CommonFlags object.
    pub fn new(verbose: bool, confirm: bool) -> Self {
        Self {
            verbose: verbose,
            confirm: confirm,
        }
    }
}

#[allow(unused)]
#[derive(Debug)]
/// Several objects of command parsers.
pub struct Parser<'a> {
    /// Common flags
    flags: &'a CommonFlags,
    /// Root command
    root: Command,
}
#[allow(unused)]
impl<'a> Parser<'a> {
    /// Create a new `Parser` object with specified CommonFlags object.
    pub fn new(flags: &'a CommonFlags) -> Self {
        // Root command
        let root = Command::new("scrut")
            .arg(
                Arg::new("verbose")
                    .short('v')
                    .long("verbose")
                    .help("Enable verbose logging")
                    .action(clap::ArgAction::SetTrue),
            )
            .arg(
                Arg::new("confirm")
                    .short('c')
                    .long("confirm")
                    .help("Confirm before actions")
                    .action(clap::ArgAction::SetTrue),
            );

        let version = Command::new("version");

        let docs = Command::new("docs");

        let scan = Command::new("scan")
            .arg(
                Arg::new("item") // Positional argument
                    .help("Specify items to scan")
                    .required(true),
            )
            .arg(
                Arg::new("exclude")
                    .short('e')
                    .long("exclude")
                    .help("Specify excluded files")
                    .required(false),
            )
            .arg(
                Arg::new("scan-all")
                    .short('a')
                    .long("scan-all")
                    .help("Scan all files in current working directory")
                    .action(clap::ArgAction::SetTrue),
            );

        let fix = Command::new("fix")
            .arg(
                Arg::new("item") // Positional argument
                    .help("Specify items to fix")
                    .required(false), // Fix all issues in default.
            )
            .arg(
                Arg::new("fix-unsafe")
                    .long("fix-unsafe") // No short names
                    .help("Fix issues may modify code behavior.")
                    .action(clap::ArgAction::SetTrue),
            );

        // Add subcommands.
        let root = root.subcommand(&version).subcommand(&scan).subcommand(&fix);

        Self {
            flags: flags,
            root: root,
        }
    }
}

/// Parse arguments and run their corresponding task.
/// # Panics
/// If no command provided or unknown command.
#[allow(unused)]
pub fn parse_arg(parser: Parser) -> Result<(), Box<dyn error::Error>> {
    let matches = parser.root.clone().get_matches();
    // Match provided subcommand.
    match matches.subcommand() {
        Some(("version", _)) => {
            // Command `version`
            println!("scrut version {}", VERSION)
        }
        Some(("docs", _)) => {
            // Command `docs`
            const DOC_URL: &str = "file:///docs/index.html";
            println!("Opening html page: {DOC_URL}");
            match that(OsStr::new(DOC_URL)) {
                Ok(()) => {
                    println!("")
                }
                Err(e) => {
                    panic!("Unable to open html page: {DOC_URL}")
                }
            }
        }
        Some(("scan", sub_matches)) => {
            // Command `scan`
            let scan_all = sub_matches.get_flag("scan-all");
            let exclude: Vec<String> = match sub_matches.try_get_many("exclude") {
                Ok(Some(patterns)) => {
                    // Exclusion list provided
                    patterns.map(|s: &String| s.clone()).collect()
                }
                Ok(None) => {
                    // No exclusion
                    vec![]
                }
                Err(e) => {
                    // Error when parsing
                    panic!("fatal: Error when parsing argument `exclude`: {}", e);
                }
            };
            match matches.try_get_many::<String>("item") {
                Ok(Some(items)) => {
                    // Item provided
                    let items: Vec<&String> = items.collect();
                    scan_cwd(&exclude, scan_all, &items);
                }
                Ok(None) => {
                    // No item provided
                    let items: Vec<&String> = vec![&String::from("all")];
                    eprintln!("warning: No item provided, scanning all");
                }
                Err(e) => {
                    // Error when parsing
                    panic!("fatal: Error when parsing argument `item`")
                }
            }
        }
        Some(("fix", sub_matches)) => {
            // Command `fix`
        }
        None => panic!("fatal: Must provide a command"),
        _ => panic!("fatal: Unknown command"),
    }
    Ok(())
}
