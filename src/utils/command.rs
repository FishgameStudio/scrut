//! A private module to store the [`Command`] object.
//! To get the object, please call the function [`get_root_command_object``]
//! To see how it works, please see the [documentation](../../docs/index.html).

use clap::{self, Arg, Command, value_parser};

/// Get the main [`Command`] object for command-line parsing.
pub(crate) fn get_root_command_object() -> Command {
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
        )
        .arg(
            Arg::new("max-log-size")
                .long("max-log-size")
                .value_name("MiB")
                .help("Specify maximum size (MiB) of the log file.")
                .required(false)
                .value_parser(clap::value_parser!(u64)),
        )
        .arg(
            Arg::new("config-file")
                .long("config-file")
                .value_name("File")
                .help("Specify the path of the configuration file.")
                .required(false),
        )
        .arg(
            Arg::new("curr-dir")
                .long("curr-dir")
                .short('C') // Uppercase C
                .value_name("Directory")
                .help("Specify the directory in which scrut runs.")
                .required(false),
        );

    let version = Command::new("version");

    let docs = Command::new("docs");

    let log = Command::new("log");

    let scan = Command::new("scan")
        .arg(
            Arg::new("item") // Positional argument
                .help("Specify items to scan")
                .required(false)
                .value_name("Item")
                .num_args(1..), // One or more
        )
        .arg(
            Arg::new("exclude")
                .value_name("Glob-pattern")
                .short('e')
                .long("exclude")
                .help("Specify excluded files")
                .required(false)
                .num_args(0..),
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
                .value_name("Item")
                .help("Specify items to fix")
                .required(false), // Fix all issues in default.
        )
        .arg(
            Arg::new("fix-unsafe")
                .long("fix-unsafe") // No short names
                .help("Fix issues may modify code behavior.")
                .action(clap::ArgAction::SetTrue),
        );

    let generate = Command::new("generate")
        .alias("gen")
        .arg(
            // Potisional argument to specify item.
            Arg::new("item")
                .help("A positional argument to specify item to generate.")
                .value_name("Item")
                .required(true),
        )
        .arg(
            Arg::new("len")
                .long("len")
                .short('l')
                .help("Specify length of the password, if the item is `password`.")
                .value_parser(value_parser!(usize))
                .value_name("Length")
                .required(false),
        )
        .arg(
            Arg::new("range")
                .long("range")
                .short('r')
                .help("Specify range of the random number, if the item is `rand*`.")
                .required(false)
                .value_parser(value_parser!(f64))
                .value_names(["Start", "End"])
                .num_args(2), // Two argument: --range <Start> <End>
        )
        .arg(
            Arg::new("content")
                .long("content")
                .short('c')
                .help("Specify content to generate SHA256 if the item is `sha256`.")
                .value_name("String")
                .required(false),
        )
        .arg(
            Arg::new("from-file")
                .long("from-file")
                .visible_alias("file")
                .value_name("File")
                .help("Specify content from a valid file.")
                .required(false),
        );

    // Bind subcommands.

    root.subcommand(&version)
        .subcommand(&scan)
        .subcommand(&fix)
        .subcommand(&docs)
        .subcommand(&log)
        .subcommand(&generate)
}
