//! Unit definitions.
//!
//! Hardware Tool supports multiple unit systems for different domains.

use serde::{Deserialize, Serialize};

/// Length units.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum LengthUnit {
    /// Millimeters (default for PCB)
    #[default]
    Mm,
    /// Mils (thousandths of an inch)
    Mil,
    /// Micrometers (default for IC)
    Um,
    /// Nanometers (for advanced IC)
    Nm,
    /// Inches
    Inch,
}

impl LengthUnit {
    /// Convert a value from this unit to millimeters.
    pub fn to_mm(&self, value: f64) -> f64 {
        match self {
            Self::Mm => value,
            Self::Mil => value * 0.0254,
            Self::Um => value * 0.001,
            Self::Nm => value * 0.000001,
            Self::Inch => value * 25.4,
        }
    }

    /// Convert a value from millimeters to this unit.
    pub fn from_mm(&self, value: f64) -> f64 {
        match self {
            Self::Mm => value,
            Self::Mil => value / 0.0254,
            Self::Um => value / 0.001,
            Self::Nm => value / 0.000001,
            Self::Inch => value / 25.4,
        }
    }

    /// Get the display suffix.
    pub fn suffix(&self) -> &'static str {
        match self {
            Self::Mm => "mm",
            Self::Mil => "mil",
            Self::Um => "μm",
            Self::Nm => "nm",
            Self::Inch => "in",
        }
    }
}

/// Angle units.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum AngleUnit {
    /// Degrees (default)
    #[default]
    Degrees,
    /// Radians
    Radians,
}

impl AngleUnit {
    /// Convert to degrees.
    pub fn to_degrees(&self, value: f64) -> f64 {
        match self {
            Self::Degrees => value,
            Self::Radians => value.to_degrees(),
        }
    }

    /// Convert to radians.
    pub fn to_radians(&self, value: f64) -> f64 {
        match self {
            Self::Degrees => value.to_radians(),
            Self::Radians => value,
        }
    }
}

/// Frequency units.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum FrequencyUnit {
    /// Hertz
    Hz,
    /// Kilohertz
    KHz,
    /// Megahertz
    #[default]
    MHz,
    /// Gigahertz
    GHz,
}

impl FrequencyUnit {
    /// Convert to Hz.
    pub fn to_hz(&self, value: f64) -> f64 {
        match self {
            Self::Hz => value,
            Self::KHz => value * 1e3,
            Self::MHz => value * 1e6,
            Self::GHz => value * 1e9,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_conversion() {
        assert!((LengthUnit::Mil.to_mm(1000.0) - 25.4).abs() < 1e-10);
        assert!((LengthUnit::Um.to_mm(1000.0) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_frequency_conversion() {
        assert!((FrequencyUnit::GHz.to_hz(1.0) - 1e9).abs() < 1e-10);
    }
}
