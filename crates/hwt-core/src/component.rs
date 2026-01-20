//! Component definitions.
//!
//! Components are the building blocks of hardware designs.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::geometry::Position;

/// A component in the design.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    /// Unique identifier
    pub id: Uuid,

    /// Component type (e.g., "resistor", "capacitor", "ic")
    #[serde(rename = "type")]
    pub component_type: String,

    /// Reference designator (e.g., "R1", "U1", "C5")
    pub reference: String,

    /// Component value (e.g., "10k", "100nF", "STM32F405")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// Symbol library reference
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Footprint/cell library reference
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub footprint: Option<String>,

    /// Position in the design
    #[serde(default)]
    pub position: Position,

    /// Rotation in degrees (0-360)
    #[serde(default)]
    pub rotation: f64,

    /// Component pins
    #[serde(default)]
    pub pins: Vec<Pin>,

    /// Custom properties
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub properties: HashMap<String, String>,
}

impl Component {
    /// Create a new component.
    pub fn new(reference: impl Into<String>, component_type: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            component_type: component_type.into(),
            reference: reference.into(),
            value: None,
            symbol: None,
            footprint: None,
            position: Position::default(),
            rotation: 0.0,
            pins: Vec::new(),
            properties: HashMap::new(),
        }
    }

    /// Set the component value.
    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Set the footprint.
    pub fn with_footprint(mut self, footprint: impl Into<String>) -> Self {
        self.footprint = Some(footprint.into());
        self
    }

    /// Set the position.
    pub fn with_position(mut self, x: f64, y: f64) -> Self {
        self.position = Position { x, y, z: None, unit: crate::units::LengthUnit::Mm };
        self
    }

    /// Add a pin.
    pub fn with_pin(mut self, pin: Pin) -> Self {
        self.pins.push(pin);
        self
    }

    /// Set a custom property.
    pub fn with_property(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.properties.insert(key.into(), value.into());
        self
    }
}

/// A pin on a component.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pin {
    /// Pin identifier (e.g., "1", "2", "VCC")
    pub id: String,

    /// Pin name (human-readable)
    pub name: String,

    /// Net this pin is connected to (if any)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub net: Option<String>,

    /// Pin type
    #[serde(default)]
    pub pin_type: PinType,
}

impl Pin {
    /// Create a new pin.
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            net: None,
            pin_type: PinType::Passive,
        }
    }

    /// Connect this pin to a net.
    pub fn connected_to(mut self, net: impl Into<String>) -> Self {
        self.net = Some(net.into());
        self
    }
}

/// Pin electrical type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum PinType {
    /// Input pin
    Input,
    /// Output pin
    Output,
    /// Bidirectional pin
    Bidirectional,
    /// Power input (VCC, VDD)
    PowerInput,
    /// Power output
    PowerOutput,
    /// Ground
    Ground,
    /// Passive (resistor, capacitor)
    #[default]
    Passive,
    /// No connect
    NoConnect,
    /// Open collector
    OpenCollector,
    /// Open emitter
    OpenEmitter,
    /// Tri-state
    TriState,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_builder() {
        let resistor = Component::new("R1", "resistor")
            .with_value("10k")
            .with_footprint("0603")
            .with_position(100.0, 50.0)
            .with_pin(Pin::new("1", "1").connected_to("VCC"))
            .with_pin(Pin::new("2", "2").connected_to("NODE_A"));

        assert_eq!(resistor.reference, "R1");
        assert_eq!(resistor.value, Some("10k".to_string()));
        assert_eq!(resistor.pins.len(), 2);
    }
}
