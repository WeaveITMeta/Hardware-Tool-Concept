//! Circuit JSON Intermediate Representation.
//!
//! The universal data model for Hardware Tool, providing a standardized
//! intermediate representation for schematics, layouts, nets, and constraints.

use serde::{Deserialize, Serialize};

use crate::component::Component;
use crate::constraint::Constraint;
use crate::layout::Layout;
use crate::net::Net;

/// Root document for Circuit JSON IR.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitJson {
    /// Schema version (semver)
    pub version: String,

    /// Project metadata
    pub metadata: CircuitMetadata,

    /// All components in the design
    #[serde(default)]
    pub components: Vec<Component>,

    /// All nets (connections) in the design
    #[serde(default)]
    pub nets: Vec<Net>,

    /// Design constraints
    #[serde(default)]
    pub constraints: Vec<Constraint>,

    /// Layout data (optional, may not exist for schematic-only)
    #[serde(default)]
    pub layout: Option<Layout>,
}

impl CircuitJson {
    /// Create a new empty circuit.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            version: "1.0.0".to_string(),
            metadata: CircuitMetadata {
                name: name.into(),
                created: chrono::Utc::now(),
                modified: chrono::Utc::now(),
                tool_version: env!("CARGO_PKG_VERSION").to_string(),
                description: None,
                author: None,
            },
            components: Vec::new(),
            nets: Vec::new(),
            constraints: Vec::new(),
            layout: None,
        }
    }

    /// Load from JSON string.
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Serialize to JSON string.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Serialize to JSON string (compact).
    pub fn to_json_compact(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

impl Default for CircuitJson {
    fn default() -> Self {
        Self::new("Untitled")
    }
}

/// Metadata for a circuit design.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitMetadata {
    /// Design name
    pub name: String,

    /// Creation timestamp
    pub created: chrono::DateTime<chrono::Utc>,

    /// Last modified timestamp
    pub modified: chrono::DateTime<chrono::Utc>,

    /// Tool version that created this file
    pub tool_version: String,

    /// Optional description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Optional author
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit_json_roundtrip() {
        let circuit = CircuitJson::new("Test Circuit");
        let json = circuit.to_json().unwrap();
        let parsed = CircuitJson::from_json(&json).unwrap();
        assert_eq!(parsed.metadata.name, "Test Circuit");
    }
}
