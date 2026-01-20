//! Project management.
//!
//! Project file format and management for Hardware Tool.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::domain::HardwareDomain;

/// A Hardware Tool project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    /// Project metadata
    pub project: ProjectMetadata,

    /// Schematic files
    #[serde(default)]
    pub schematics: SchematicConfig,

    /// PCB/Layout files
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pcb: Option<PcbConfig>,

    /// Library configuration
    #[serde(default)]
    pub libraries: LibraryConfig,

    /// Output configuration
    #[serde(default)]
    pub output: OutputConfig,
}

impl Project {
    /// Create a new project.
    pub fn new(name: impl Into<String>, domain: HardwareDomain) -> Self {
        Self {
            project: ProjectMetadata {
                name: name.into(),
                version: "0.1.0".to_string(),
                format_version: "1".to_string(),
                domain,
                author: None,
                description: None,
                license: None,
            },
            schematics: SchematicConfig::default(),
            pcb: None,
            libraries: LibraryConfig::default(),
            output: OutputConfig::default(),
        }
    }

    /// Load from TOML string.
    pub fn from_toml(toml_str: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(toml_str)
    }

    /// Serialize to TOML string.
    pub fn to_toml(&self) -> Result<String, toml::ser::Error> {
        toml::to_string_pretty(self)
    }
}

/// Project metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetadata {
    /// Project name
    pub name: String,

    /// Project version (semver)
    #[serde(default = "default_version")]
    pub version: String,

    /// Format version
    #[serde(default = "default_format_version")]
    pub format_version: String,

    /// Hardware domain
    #[serde(default)]
    pub domain: HardwareDomain,

    /// Author
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,

    /// Description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// License
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
}

fn default_version() -> String {
    "0.1.0".to_string()
}

fn default_format_version() -> String {
    "1".to_string()
}

/// Schematic configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SchematicConfig {
    /// Main schematic file
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub main: Option<PathBuf>,

    /// Additional schematic sheets
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sheets: Vec<PathBuf>,
}

/// PCB configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PcbConfig {
    /// Layout file
    pub layout: PathBuf,

    /// Stackup preset
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stackup: Option<String>,
}

/// Library configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LibraryConfig {
    /// Local library paths
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub local: Vec<PathBuf>,

    /// Remote library URLs
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub remote: Vec<String>,
}

/// Output configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    /// Generate Gerber files
    #[serde(default = "default_true")]
    pub gerber: bool,

    /// Generate IPC-2581
    #[serde(default)]
    pub ipc2581: bool,

    /// Generate ODB++
    #[serde(default)]
    pub odbpp: bool,

    /// Generate STEP 3D model
    #[serde(default)]
    pub step: bool,
}

fn default_true() -> bool {
    true
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            gerber: true,
            ipc2581: false,
            odbpp: false,
            step: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_toml_roundtrip() {
        let project = Project::new("Test Project", HardwareDomain::Pcb);
        let toml = project.to_toml().unwrap();
        let parsed = Project::from_toml(&toml).unwrap();
        assert_eq!(parsed.project.name, "Test Project");
    }
}
