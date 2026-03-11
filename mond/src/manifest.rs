use std::{collections::HashMap, path::PathBuf};

use eyre::Context;
use semver::Version;
use serde::{Deserialize, Serialize};

use crate::{MANIFEST_NAME, VERSION};

#[derive(Serialize, Deserialize)]
pub(crate) struct MondManifest {
    pub(crate) package: Package,
    pub(crate) dependencies: HashMap<String, DependencySpec>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Package {
    pub(crate) name: String,
    pub(crate) version: Version,
    pub(crate) mond_version: Version,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct DependencySpec {
    pub(crate) git: String,
    #[serde(flatten)]
    pub(crate) reference: GitReference,
}

#[derive(Serialize, Deserialize)]
pub(crate) enum GitReference {
    #[serde(rename = "tag")]
    Tag(String),
    #[serde(rename = "branch")]
    Branch(String),
    #[serde(rename = "rev")]
    Rev(String),
}

impl MondManifest {
    fn new(name: String) -> Self {
        Self {
            package: Package {
                name,
                version: Version::new(0, 1, 0),
                mond_version: Version::parse(VERSION).unwrap(),
            },
            dependencies: Default::default(),
        }
    }
}

pub(crate) fn read_manifest(root: PathBuf) -> eyre::Result<MondManifest> {
    let manifest_file_path = root.join(MANIFEST_NAME);
    let file = std::fs::read(&manifest_file_path).context(format!(
        "failed to read {MANIFEST_NAME} at {manifest_file_path:?}"
    ))?;
    let manifest: MondManifest =
        toml::from_slice(&file).context(format!("failed to parse {MANIFEST_NAME}"))?;
    Ok(manifest)
}

pub(crate) fn create_new_manifest(name: String) -> MondManifest {
    MondManifest::new(name)
}

pub(crate) fn write_manifest(manifest: &MondManifest, path: &PathBuf) -> eyre::Result<()> {
    let manifest_as_string =
        toml::to_string_pretty(&manifest).context("failed to write {MANIFEST_NAME} to string")?;

    std::fs::write(path, manifest_as_string).context("failed to write {MANIFEST_NAME} to disk")?;
    Ok(())
}
