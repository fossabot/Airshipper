// Networking

pub const DOWNLOAD_SERVER: &str = "https://download.veloren.net";

// Filesystem

#[cfg(windows)]
pub const DOWNLOAD_FILE: &str = "veloren.zip";
#[cfg(unix)]
pub const DOWNLOAD_FILE: &str = "veloren";

#[cfg(windows)]
pub const VOXYGEN_FILE: &str = "veloren-voxygen.exe";
#[cfg(unix)]
pub const VOXYGEN_FILE: &str = "veloren-voxygen";

#[cfg(windows)]
pub const SERVER_CLI_FILE: &str = "veloren-server-cli.exe";
#[cfg(unix)]
pub const SERVER_CLI_FILE: &str = "veloren-server-cli";

pub const LOG_FILE: &str = "airshipper.log";

// Veloren

pub const VOXYGEN_LOG_ENV: &str = "VOXYGEN_LOG";
