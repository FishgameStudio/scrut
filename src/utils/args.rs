//! Parse command-line arguments and run their corresponding task.
//! Before initialization of log system, **do not** use log macros in [`crate::utils::logging`].
//! Note: The log system is initialized by [`crate::utils::logging::init_log_file`].

use std::fs::{exists, read_to_string};
use std::ops::Range;
use std::path::Path;
use std::{env, error};

use clap::{ArgMatches, Command};

use crate::utils::command::get_root_command_object;
use crate::utils::config::{
    Config, apply_one_config, get_config_content, init_config, is_config_inited, parse_config,
    set_config_file,
};
use crate::utils::confirm::set_confirm_flag;
use crate::utils::generate::{generate, str2enum};
use crate::utils::logging::{enable_verbose, fatal, init_log_file, verbose, warning};
use crate::utils::scan::scan_cwd;
use crate::utils::version::VERSION;

use dirs::home_dir;

use open::that;

use owo_colors::OwoColorize;

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
        Self { verbose, confirm }
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
        let root = get_root_command_object();

        Self { flags, root }
    }
}

/// Open local documentation files of scrut.
/// # Panics
/// If unable to access the documentation files.
pub fn open_local_docs() {
    match exists("./docs/index.html") {
        Ok(true) => {}
        Ok(false) => fatal!("Documentation html not found"),
        Err(e) => fatal!("Unable to access documentation html: {}", e),
    }
    let doc_path = Path::new("./docs/index.html").canonicalize().unwrap();
    println!(
        "{} {}",
        "Opening html page:".green(),
        doc_path.to_str().unwrap()
    );
    verbose!("Opening html page: {}", doc_path.to_str().unwrap());
    if let Err(e) = that(doc_path.as_os_str()) {
        fatal!(
            "Unable to open html page {}: {}",
            doc_path.to_str().unwrap(),
            e
        );
    }
}
/// Print log from the log file. The path of log is `~/scrut.log` on by default.
/// # Panics
/// If unable to access the log file.
pub fn print_log() {
    let home = match home_dir() {
        Some(path) => path,
        None => fatal!("Unable to get home directory"),
    };
    let log_path = format!("{}/scrut.log", home.display());
    verbose!("Printing log from log file {}", log_path);
    match exists(&log_path) {
        Err(e) => {
            fatal!("Unable to access log file '{}': {}", log_path, e);
        }
        Ok(option) => {
            if option {
                // Show contents of the log file by the default program.
                match read_to_string(&log_path) {
                    Ok(content) => {
                        println!("{}", content);
                    }
                    Err(e) => fatal!("Unable to read log file '{}': {}", log_path, e),
                }
            } else {
                fatal!("Log file not found: {}", log_path);
            }
        }
    }
}

/// Do basic scan actions with given argument matches.
/// # Panics
/// If failed to parse argument `-e --exclude`.
/// # Errors
/// If failed to scan current working directory.
pub fn parse_scan(sub_matches: &ArgMatches) -> Result<(), Box<dyn error::Error>> {
    let scan_all = sub_matches.get_flag("scan-all");
    let exclude: Vec<String> = match sub_matches.try_get_many("exclude") {
        Ok(Some(patterns)) => {
            // Exclusion list provided
            verbose!("Parsed argument -e, got {} pattern(s)", patterns.len());
            patterns.cloned().collect()
        }
        Ok(None) => {
            // No exclusion
            verbose!("Parsed argument -e, got 0 patterns");
            vec![]
        }
        Err(e) => {
            // Error when parsing
            fatal!("Error when parsing argument `exclude`: {}", e);
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
            warning!("No item provided, scanning all");
            scan_cwd(&exclude, scan_all, &[&"all".to_string()])?;
        }
        Err(e) => {
            // Error when parsing
            fatal!("Error when parsing argument `item`: {}", e);
        }
    };
    Ok(())
}

/// Parse given `ArgMatches` object and do generate.
/// # Panics
/// If the item is unknown, or error during parsing.
pub fn parse_generate(sub_matches: &ArgMatches) -> Result<(), Box<dyn error::Error>> {
    let item: &String = match sub_matches.try_get_one("item") {
        Ok(Some(name)) => name,
        Ok(None) => {
            fatal!("This command requires a positional argument 'item' but 0 was given")
        }
        Err(e) => fatal!("Error when parsing positional argument 'item': {e}"),
    };
    let range_float: Option<Range<f64>> = match sub_matches.try_get_many::<f64>("range") {
        Ok(Some(range)) => {
            let range: Vec<f64> = range.cloned().collect();
            if range.len() < 2 {
                fatal!(
                    "This argument requires 2 values but {} was given",
                    range.len()
                );
            }
            Some(range[0]..range[1])
        }
        Ok(None) => None,
        Err(e) => {
            fatal!("Error when parsing argument '--range': {e}");
        }
    };
    let range_int: Option<Range<i32>> = range_float
        .as_ref()
        .map(|range| (range.start as i32)..(range.end as i32));
    let len: Option<usize> = match sub_matches.try_get_one::<usize>("len") {
        Ok(Some(val)) => Some(*val),
        Ok(None) => None,
        Err(e) => fatal!("Error when parsing argument 'len': {e}"),
    };
    let mut content: Option<String> = match sub_matches.try_get_one::<String>("content") {
        Ok(Some(s)) => Some(s.to_owned()),
        Ok(None) => None,
        Err(e) => fatal!("Error when parsing argument 'content': {e}"),
    };
    match sub_matches.try_get_one::<String>("from-file") {
        Ok(Some(s)) => {
            // Read the content of the file if the file is specified.
            let s = read_to_string(s)?;
            content = Some(s);
        }
        Ok(None) => {}
        Err(e) => fatal!("Error when parsing argument 'from-file': {e}"),
    }

    let item = str2enum(item, len, range_int, range_float, content);
    println!("{}", generate(item));
    Ok(())
}

/// Parse arguments and run their corresponding task.
/// # Panics
/// If no command provided or unknown command.
pub fn parse_arg(parser: Parser) -> Result<(), Box<dyn error::Error>> {
    let orig_dir = env::current_dir()?;
    // Parse
    let matches = parser.root.clone().get_matches();

    /* Set the config file.
     * Note: This initialization of the configuration system has
     * logged but the log system doesn't has initialized. The log
     * of this part will be discarded.
     */
    match matches.get_one::<String>("config-file") {
        Some(path) => {
            set_config_file(path)?;
        }
        None => {
            fatal!("The path wasn't given for the argument --config-file");
        }
    }
    let config = if is_config_inited() {
        parse_config()?
    } else {
        Config::new()?
    };

    /*
     * Initialize log system.
     * Be sure that all the functions called above will
     * not log anything. All of the log before this
     * initialization will be discarded.
     * You can log below this match statement!
     */
    match matches.get_one::<u64>("max-log-size") {
        Some(val) => {
            init_log_file(Some(val * 1024 * 1024))?; // MiB
        }
        None => {
            init_log_file(None)?;
        }
    }
    verbose!("Log system initialized");

    if matches.get_flag("verbose") || config.verbose {
        // Enable verbose logging.
        enable_verbose();
    }
    if matches.get_flag("confirm") || config.confirm {
        // Enable confirmation mode.
        set_confirm_flag(true);
    }

    // Change to the specified directory.
    match matches.get_one::<String>("curr-dir") {
        Some(path) => {
            // No threads spawned at this time,
            // this change of current directory is safe.
            env::set_current_dir(path)?;
        }
        None => {
            fatal!("The path wasn't given for the argument --curr-dir (aka -C)");
        }
    }

    // Match provided subcommand.
    match matches.subcommand() {
        Some(("version", _)) => {
            // Command `version`
            verbose!("print version of scrut ({})", VERSION);
            println!("scrut version {}", VERSION);
        }
        Some(("docs", _)) => {
            // Command `docs`
            open_local_docs();
        }
        Some(("log", _)) => {
            // Command `log`
            print_log();
        }
        Some(("scan", sub_matches)) => {
            // Command `scan`
            parse_scan(sub_matches)?;
        }
        #[allow(unused)]
        Some(("fix", sub_matches)) => {
            // Command `fix`
            todo!("Implement command `fix`")
        }
        Some(("generate", sub_matches)) => {
            // Command `generate` or its alias `gen`
            parse_generate(sub_matches)?;
        }
        Some(("config", sub_matches)) => match sub_matches.subcommand() {
            Some(("print", _)) => {
                // Print content of `~/.scrut/config.toml`.
                println!("{}", get_config_content()?);
            }
            Some(("show", _)) => {
                // Show attributes of the config file.
                println!("{config:#?}");
            }
            Some(("set", m)) => match m.get_many::<String>("attr") {
                Some(args) => {
                    let args: Vec<String> = args.cloned().collect();
                    if args.len() != 2 {
                        fatal!("Expect 2 positional arguments but {} provided", args.len());
                    }
                    let key = &args[0];
                    let val = &args[1];
                    apply_one_config(key, val)?;
                }
                None => fatal!("No such values provided"),
            },
            Some(("init", m)) => init_config(m.get_flag("forcibly"))?,
            Some((cmd, _)) => fatal!("Unknown sub command: '{cmd}'"),
            None => fatal!("Must provide a sub command of the command `config`"),
        },
        None => fatal!("Must provide a command"),
        Some((cmd, _)) => fatal!("Unknown command: {}", cmd),
    }

    // Change back to the original directory.
    env::set_current_dir(orig_dir)?;

    Ok(())
}
