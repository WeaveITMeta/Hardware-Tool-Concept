//! Layout data structures.
//!
//! Physical layout information for PCB, IC, and other domains.

use serde::{Deserialize, Serialize};

use crate::geometry::{Point2D, Position};
use crate::units::LengthUnit;

/// Layout data for a design.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Layout {
    /// Board/die outline
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outline: Option<Outline>,

    /// Layer stack
    #[serde(default)]
    pub layers: Vec<Layer>,

    /// Traces/routes
    #[serde(default)]
    pub traces: Vec<Trace>,

    /// Vias
    #[serde(default)]
    pub vias: Vec<Via>,

    /// Copper zones/fills
    #[serde(default)]
    pub zones: Vec<Zone>,
}

/// Board/die outline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Outline {
    /// Outline type
    #[serde(rename = "type")]
    pub outline_type: OutlineType,

    /// Points defining the outline (for polygon)
    #[serde(default)]
    pub points: Vec<Point2D>,

    /// Width (for rectangle)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,

    /// Height (for rectangle)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,

    /// Unit
    #[serde(default)]
    pub unit: LengthUnit,
}

/// Outline type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum OutlineType {
    /// Rectangular outline
    #[default]
    Rectangle,
    /// Polygon outline
    Polygon,
    /// Circular outline
    Circle,
}

/// A layer in the stack.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    /// Layer name (e.g., "F.Cu", "B.Cu", "In1.Cu")
    pub name: String,

    /// Layer type
    pub layer_type: LayerType,

    /// Layer thickness
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thickness: Option<f64>,

    /// Material (e.g., "copper", "FR4", "prepreg")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub material: Option<String>,

    /// Visibility
    #[serde(default = "default_true")]
    pub visible: bool,
}

fn default_true() -> bool {
    true
}

/// Layer type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum LayerType {
    /// Copper layer
    #[default]
    Copper,
    /// Dielectric layer
    Dielectric,
    /// Solder mask
    SolderMask,
    /// Silkscreen
    Silkscreen,
    /// Paste
    Paste,
    /// Courtyard
    Courtyard,
    /// Fabrication
    Fabrication,
}

/// A trace/route segment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trace {
    /// Net name
    pub net: String,

    /// Layer name
    pub layer: String,

    /// Start position
    pub start: Position,

    /// End position
    pub end: Position,

    /// Trace width
    pub width: f64,

    /// Unit
    #[serde(default)]
    pub unit: LengthUnit,
}

/// A via.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Via {
    /// Net name
    pub net: String,

    /// Position
    pub position: Position,

    /// Via type
    #[serde(default)]
    pub via_type: ViaType,

    /// Drill diameter
    pub drill: f64,

    /// Pad diameter
    pub pad: f64,

    /// Start layer
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_layer: Option<String>,

    /// End layer
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_layer: Option<String>,

    /// Unit
    #[serde(default)]
    pub unit: LengthUnit,
}

/// Via type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ViaType {
    /// Through-hole via
    #[default]
    Through,
    /// Blind via
    Blind,
    /// Buried via
    Buried,
    /// Micro via
    Micro,
}

/// A copper zone/fill.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Zone {
    /// Net name
    pub net: String,

    /// Layer name
    pub layer: String,

    /// Zone outline points
    pub points: Vec<Point2D>,

    /// Fill type
    #[serde(default)]
    pub fill_type: ZoneFillType,

    /// Clearance
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clearance: Option<f64>,

    /// Minimum width
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_width: Option<f64>,

    /// Unit
    #[serde(default)]
    pub unit: LengthUnit,
}

/// Zone fill type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ZoneFillType {
    /// Solid fill
    #[default]
    Solid,
    /// Hatched fill
    Hatched,
    /// No fill (outline only)
    None,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_default() {
        let layout = Layout::default();
        assert!(layout.layers.is_empty());
        assert!(layout.traces.is_empty());
    }
}
