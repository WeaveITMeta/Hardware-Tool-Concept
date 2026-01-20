//! Design constraint definitions.
//!
//! Constraints define rules for the design (clearances, trace widths, etc.).

use serde::{Deserialize, Serialize};

use crate::units::LengthUnit;

/// A design constraint.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Constraint {
    /// Clearance constraint
    Clearance {
        /// Scope (e.g., "net:VCC", "class:power", "all")
        scope: String,
        /// Minimum clearance value
        value: f64,
        /// Unit
        unit: LengthUnit,
    },

    /// Trace width constraint
    TraceWidth {
        /// Scope
        scope: String,
        /// Minimum width
        min: f64,
        /// Maximum width
        max: f64,
        /// Unit
        unit: LengthUnit,
    },

    /// Differential pair constraint
    DifferentialPair {
        /// Positive net
        net_positive: String,
        /// Negative net
        net_negative: String,
        /// Target impedance (ohms)
        impedance: f64,
        /// Tolerance (%)
        tolerance: f64,
    },

    /// Length matching constraint
    LengthMatch {
        /// Nets to match
        nets: Vec<String>,
        /// Maximum length difference
        max_difference: f64,
        /// Unit
        unit: LengthUnit,
    },

    /// Via constraint
    Via {
        /// Scope
        scope: String,
        /// Via type (e.g., "through", "blind", "buried")
        via_type: String,
        /// Drill diameter
        drill: f64,
        /// Annular ring
        annular_ring: f64,
        /// Unit
        unit: LengthUnit,
    },
}

impl Constraint {
    /// Create a clearance constraint.
    pub fn clearance(scope: impl Into<String>, value: f64, unit: LengthUnit) -> Self {
        Self::Clearance {
            scope: scope.into(),
            value,
            unit,
        }
    }

    /// Create a trace width constraint.
    pub fn trace_width(scope: impl Into<String>, min: f64, max: f64, unit: LengthUnit) -> Self {
        Self::TraceWidth {
            scope: scope.into(),
            min,
            max,
            unit,
        }
    }

    /// Create a differential pair constraint.
    pub fn differential_pair(
        net_positive: impl Into<String>,
        net_negative: impl Into<String>,
        impedance: f64,
        tolerance: f64,
    ) -> Self {
        Self::DifferentialPair {
            net_positive: net_positive.into(),
            net_negative: net_negative.into(),
            impedance,
            tolerance,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constraint_serialization() {
        let constraint = Constraint::clearance("net:VCC", 0.2, LengthUnit::Mm);
        let json = serde_json::to_string(&constraint).unwrap();
        assert!(json.contains("clearance"));
    }
}
