//! Deals with all filesystem specific details

use crate::config;
use std::path::PathBuf;

lazy_static::lazy_static! {
    // Base for config, profiles, ...
    pub static ref BASE_PATH: PathBuf = base();
    // Base for the assets
    pub static ref ASSETS_PATH: PathBuf = assets();
}

// TODO: Use goddamn env vars loaded from .env to avoid polluting main install while developing!

/// Returns the base path where all airshipper files like config, profiles belong.
///
/// |Platform | Example                                                       |
/// | ------- | ------------------------------------------------------------- |
/// | Linux   | /home/alice/.local/share/barapp                               |
/// | macOS   | /Users/Alice/Library/Application Support/com.Foo-Corp.Bar-App |
/// | Windows | C:\Users\Alice\AppData\Roaming                                |
fn base() -> PathBuf {
    let path = dirs::data_dir()
        .expect("Couldn't locate where to put launcher data!")
        .join("airshipper");
    std::fs::create_dir_all(&path).expect("failed to create data directory!");
    path
}

/// Tries to locate the static assets at various places.
/// Priorities relative over absolute paths (e.g. next to the executable before checking /usr/share/airshipper/.. etc)
fn assets() -> PathBuf {
    let mut paths = Vec::new();

    // Executable path
    if let Ok(mut path) = std::env::current_exe() {
        path.pop();
        paths.push(path);
    }

    // current working directory
    if let Ok(path) = std::env::current_dir() {
        paths.push(path);
    }

    // System paths
    #[cfg(target_os = "linux")]
    paths.push("/usr/share/airshipper/assets".into());

    for path in paths.clone() {
        match find_folder::Search::ParentsThenKids(3, 1).of(path).for_folder("assets") {
            Ok(assets_path) => return assets_path,
            Err(_) => continue,
        }
    }

    panic!(
        "Airshipper assets could not be found! Searched folders:\n{})",
        paths.iter().fold(String::new(), |mut a, path| {
            a += &path.to_string_lossy();
            a += "\n";
            a
        }),
    );
}

/// Returns path to where the assets are stored
pub fn get_assets_path(name: &str) -> String {
    ASSETS_PATH.join(name).display().to_string()
}

/// Returns path to a profile while creating the folder
pub fn get_profile_path(profile_name: &str) -> Result<std::path::PathBuf, std::io::Error> {
    let path = BASE_PATH.join("profiles").join(profile_name);
    std::fs::create_dir_all(&path)?;
    Ok(path)
}

/// Returns path to the file where the logs will be stored
pub fn get_log_path() -> PathBuf {
    BASE_PATH.join(config::LOG_FILE)
}
