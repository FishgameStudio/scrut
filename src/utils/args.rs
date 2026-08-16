//! Parse command-line arguments.

use std::path::Path;
use std::{error, fs::exists};

use clap::{self, Arg, Command};

use crate::utils::logging::enable_verbose;
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
                    .required(false)
                    .num_args(1..),
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
        let root = root
            .subcommand(&version)
            .subcommand(&scan)
            .subcommand(&fix)
            .subcommand(&docs);

        Self {
            flags: flags,
            root: root,
        }
    }
}

/// Parse arguments and run their corresponding task.
/// # Panics
/// If no command provided or unknown command.
pub fn parse_arg(parser: Parser) -> Result<(), Box<dyn error::Error>> {
    let matches = parser.root.clone().get_matches();
    if matches.get_flag("verbose") {
        // Enable verbose logging.
        enable_verbose().expect("fatal: Cannot open log file");
    }
    if matches.get_flag("confirm") {
        todo!("Enable confirm before actions")
    }
    // Match provided subcommand.
    match matches.subcommand() {
        Some(("version", _)) => {
            // Command `version`
            println!("scrut version {}", VERSION)
        }
        Some(("docs", _)) => {
            // Command `docs`
            match exists("./docs/index.html") {
                Ok(true) => {}
                Ok(false) => panic!("fatal: Documentation html not found"),
                Err(e) => panic!("fatal: Unable to access documentation html: {}", e),
            }
            let doc_path = Path::new("./docs/index.html").canonicalize().unwrap();
            println!("Opening html page: {}", doc_path.to_str().unwrap());
            match that(doc_path.as_os_str()) {
                Ok(()) => {}
                Err(e) => {
                    panic!(
                        "fatal: Unable to open html page {}: {}",
                        doc_path.to_str().unwrap(),
                        e
                    );
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
            match sub_matches.try_get_many::<String>("item") {
                Ok(Some(items)) => {
                    // Item provided
                    let items: Vec<&String> = items.collect();
                    scan_cwd(&exclude, scan_all, &items)?;
                }
                Ok(None) => {
                    // No item provided
                    eprintln!("warning: No item provided, scanning all");
                    scan_cwd(&exclude, scan_all, &vec![&"all".to_string()])?;
                }
                Err(e) => {
                    // Error when parsing
                    panic!("fatal: Error when parsing argument `item`: {}", e);
                }
            }
        }
        #[allow(unused)]
        Some(("fix", sub_matches)) => {
            // Command `fix`
            todo!("Implement command `fix`")
        }
        None => panic!("fatal: Must provide a command"),
        _ => panic!("fatal: Unknown command"),
    }
    Ok(())
}
