//! Management of config file. (`~/.scrut/config.toml`)
//! You can use the logging macro ([`verbose`], [`warning`] and [`fatal`], etc.)
//! safely because the log file (`~/scrut.log`) has no overlapping
//! with the configuration directory (`~/.scrut`).

use dirs::home_dir;

use std::env::current_dir;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::sync::{LazyLock, Mutex};

use serde::{Deserialize, Serialize};

use crate::utils::confirm::{DefaultOpt, confirm};
use crate::utils::logging::{fatal, ok, ret, verbose, warning};

/// Returns `~/.scrut`.
fn get_config_dir() -> PathBuf {
    verbose!("Getting configuration dir");
    let home = home_dir().unwrap_or_else(|| fatal!("Couldn't get home directory"));
    verbose!("Done");
    ret!(home.join(".scrut"))
}

static CONFIG_PATH: LazyLock<Mutex<PathBuf>> =
    LazyLock::new(|| Mutex::new(get_config_dir().join("config.toml")));

/// Initialize configuration and return the absolute path of `~/.scrut`.
/// # Panics
/// If unable to access the configuration directory.
pub fn init_config(forcibly: bool) -> Result<(), Box<dyn Error>> {
    let path = get_config_dir();
    if forcibly && path.exists() {
        confirm(
            "Reinitialize configuration forcibly? All the config will be reset.",
            DefaultOpt::Yes,
        );
        fs::remove_dir_all(&path)?;
    }
    if is_config_inited() {
        ok!();
    }
    verbose!("Initializing configuration system...");
    if path.exists() && !path.is_dir() {
        // ~/.scrut exists but is not a directory, rename
        warning!("~/.scrut already exists but isn't a directory, renaming to '.scrut(2)' ...");
        confirm("Rename old `~/.scrut` to `.scrut(2)`?", DefaultOpt::Yes);
        fs::rename(&path, ".scrut(2)")?;
    }
    // If `~/scrut` doesn't exist
    if !match fs::exists(&path) {
        Err(e) => fatal!("Unable to access configuration directory: {e}"),
        Ok(option) => option,
    } {
        fs::create_dir(&path)?;
        // ~/.scrut/config.toml
        fs::write(path.join("config.toml"), "")?;
    }
    verbose!("Done");
    verbose!("Initializing config.toml ...");
    save_config(&Config::new()?)?;
    ok!()
}

/// Test whether the configuration system is initialized.
#[inline]
pub fn is_config_inited() -> bool {
    // return true if ~/.scrut exists and ~/.scrut is a directory.
    ret!(fs::exists(get_config_dir()).unwrap_or_default() || get_config_dir().is_dir())
}

/// Get content of the config file (`~/.scrut/config.toml`).
/// # Errors
/// If failed to read the file.
#[inline]
#[allow(unused)]
pub fn get_config_content() -> Result<String, Box<dyn Error>> {
    let path = &*CONFIG_PATH.lock()?;
    verbose!("Reading from config path {:?} ...", path);
    let content = fs::read_to_string(path)?;
    verbose!("Done");
    ok!(content)
}

/// Apply configuration from a different path.
/// # Errors
/// If failed to apply the config
/// # Panics
/// If the file doesn't exist.
#[inline]
pub fn set_config_file<P>(file: P) -> Result<(), Box<dyn Error>>
where
    P: Into<PathBuf>,
{
    let file: PathBuf = file.into();
    if !&file.exists() {
        fatal!("Config file '{}' doesn't exist", file.display());
    }
    let mut guard = CONFIG_PATH.lock()?;
    verbose!("Config file set to {file:#?}");
    *guard = file; // `file` moved here
    ok!()
}

/// Main configuration struct.
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub work_dir: PathBuf,  // -C --curr-dir <Directory>
    pub verbose: bool,      // -v --verbose
    pub confirm: bool,      // -c --confirm
    pub max_log_bytes: u64, // --max-log-size <MiB>
}
impl Config {
    /// Create a new [`Config`] object with default values.
    /// # Errors
    /// If unable to get the current working directory.
    pub fn new() -> Result<Self, Box<dyn Error>> {
        ok!(Self {
            work_dir: current_dir()?,
            verbose: false,
            confirm: false,
            max_log_bytes: 15 * 1024 * 1024,
        })
    }
    /// Create a new [`Config`] object with given arguments.
    #[allow(unused)]
    pub fn new_with(work_dir: PathBuf, verbose: bool, confirm: bool, max_log_bytes: u64) -> Self {
        ret!(Self {
            work_dir: work_dir.clone(),
            verbose,
            confirm,
            max_log_bytes,
        })
    }
}

/// Parse TOML from the configuration file.
/// # Errors
/// If failed to read config or failed to parse the TOML.
pub fn parse_config() -> Result<Config, Box<dyn Error>> {
    verbose!("Parsing config.toml...");
    let config: Config = toml::from_str(&get_config_content()?)?;
    verbose!("Done");
    ok!(config)
}

/// Save TOML to the configuration file with given [`Config`] object.
/// # Errors
pub fn save_config(config: &Config) -> Result<(), Box<dyn Error>> {
    confirm(
        &format!("Save config {config:?} to the config file?"),
        DefaultOpt::Yes,
    );
    verbose!("Parsing Config struct '{config:?}' ...");
    let s: String = toml::to_string_pretty(config)?;
    verbose!("Done. Writing Config struct '{config:?}' to config.toml ...");
    fs::write(&*CONFIG_PATH.lock()?, s)?;
    verbose!("Done");
    ok!()
}

/// Apply one config with given key & val.
/// # Panics
/// If the key is unknown.
/// # Errors
/// If failed to get / set the config, or the type of the value mismatch.
pub fn apply_one_config(key: &str, val: &str) -> Result<(), Box<dyn Error>> {
    confirm(
        &format!("Apply attribute '{key}' to value '{val}'?"),
        DefaultOpt::Yes,
    );
    verbose!("Applying config key '{key}' as value '{val}' ...");
    let mut config = parse_config()?;
    match key {
        "work_dir" => {
            config.work_dir = val.into();
        }
        "verbose" => {
            config.verbose = val.parse::<bool>()?;
        }
        "confirm" => {
            config.confirm = val.parse::<bool>()?;
        }
        "max_log_bytes" => {
            config.max_log_bytes = val.parse::<u64>()?;
        }
        other => fatal!("Unknown config key name: '{other}'"),
    }
    save_config(&config)?;
    ok!()
}
