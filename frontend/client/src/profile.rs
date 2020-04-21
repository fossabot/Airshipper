use crate::{config, filesystem};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Profiles(Vec<Profile>);

impl Profiles {
    pub fn latest(&self) -> &Profile {
        &self.0[0]
    }
}

impl Default for Profiles {
    fn default() -> Self {
        Self(vec![Profile::default()])
    }
}

/// Represents a version with channel, name and path.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub channel: Channel,

    pub directory: PathBuf,
    pub version: String,
}

impl Default for Profile {
    fn default() -> Self {
        Profile::new("default", Channel::Nightly, "default").expect("Couldn't create default profile!")
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Channel {
    Nightly,
    /* TODO: Release,
     * TODO: Source, */
}

impl Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nightly => write!(f, "nightly"),
        }
    }
}

impl Profile {
    /// Creates a new profile and downloads the correct files into the target directory.
    pub fn new<T: ToString>(name: T, channel: Channel, version: T) -> Result<Self, std::io::Error> {
        Ok(Self {
            directory: filesystem::get_profile_path(&name.to_string())?,
            name: name.to_string(),
            channel,
            version: version.to_string(),
        })
    }

    /// Returns path to voxygen binary.
    /// e.g. <base>/profiles/latest/veloren-voxygen.exe
    pub fn voxygen_path(&self) -> PathBuf {
        self.directory.join(config::VOXYGEN_FILE)
    }

    pub fn version_uri(&self) -> String {
        format!(
            "{}/version/{}/{}",
            config::DOWNLOAD_SERVER,
            std::env::consts::OS,
            self.channel
        )
    }
}
