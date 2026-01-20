//! Net (connection) definitions.
//!
//! Nets represent electrical connections between component pins.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// A net (electrical connection) in the design.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Net {
    /// Unique identifier
    pub id: Uuid,

    /// Net name (e.g., "VCC", "GND", "CLK")
    pub name: String,

    /// Net class reference
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,

    /// Net type
    #[serde(default)]
    pub net_type: NetType,

    /// Connections to component pins
    #[serde(default)]
    pub connections: Vec<Connection>,

    /// Net properties
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub properties: HashMap<String, String>,
}

impl Net {
    /// Create a new net.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            class: None,
            net_type: NetType::Signal,
            connections: Vec::new(),
            properties: HashMap::new(),
        }
    }

    /// Set the net type.
    pub fn with_type(mut self, net_type: NetType) -> Self {
        self.net_type = net_type;
        self
    }

    /// Set the net class.
    pub fn with_class(mut self, class: impl Into<String>) -> Self {
        self.class = Some(class.into());
        self
    }

    /// Add a connection.
    pub fn with_connection(mut self, component_id: Uuid, pin: impl Into<String>) -> Self {
        self.connections.push(Connection {
            component_id,
            pin: pin.into(),
        });
        self
    }
}

/// Net type classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum NetType {
    /// General signal
    #[default]
    Signal,
    /// Power net (VCC, VDD)
    Power,
    /// Ground net
    Ground,
    /// Clock signal
    Clock,
    /// Differential pair
    Differential,
}

/// A connection to a component pin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    /// Component UUID
    pub component_id: Uuid,

    /// Pin identifier
    pub pin: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_net_builder() {
        let component_id = Uuid::new_v4();
        let net = Net::new("VCC")
            .with_type(NetType::Power)
            .with_class("power")
            .with_connection(component_id, "VDD");

        assert_eq!(net.name, "VCC");
        assert_eq!(net.net_type, NetType::Power);
        assert_eq!(net.connections.len(), 1);
    }
}
