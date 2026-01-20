//! Hardware domain definitions.
//!
//! Hardware Tool supports 6 hardware domains, each with specialized
//! tools, views, and workflows.

use serde::{Deserialize, Serialize};

/// The 6 hardware domains supported by Hardware Tool.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum HardwareDomain {
    /// Printed Circuit Board design (default)
    #[default]
    Pcb,
    /// Integrated Circuit design
    Ic,
    /// Quantum hardware design
    Quantum,
    /// MEMS and sensor design
    Mems,
    /// RF, microwave, and photonics design
    Rf,
    /// Advanced packaging (2.5D/3D, chiplets)
    Packaging,
}

impl HardwareDomain {
    /// Get the display name for this domain.
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Pcb => "PCB Design",
            Self::Ic => "IC Design",
            Self::Quantum => "Quantum Hardware",
            Self::Mems => "MEMS & Sensors",
            Self::Rf => "RF & Photonics",
            Self::Packaging => "Advanced Packaging",
        }
    }

    /// Get the icon for this domain.
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Pcb => "🔲",
            Self::Ic => "🔷",
            Self::Quantum => "⚛️",
            Self::Mems => "📡",
            Self::Rf => "📶",
            Self::Packaging => "📦",
        }
    }

    /// Get the accent color for this domain (hex).
    pub fn accent_color(&self) -> &'static str {
        match self {
            Self::Pcb => "#B87333",      // Copper
            Self::Ic => "#3498DB",       // Blue
            Self::Quantum => "#9B59B6",  // Purple
            Self::Mems => "#1ABC9C",     // Teal
            Self::Rf => "#E67E22",       // Orange
            Self::Packaging => "#7F8C8D", // Gray
        }
    }

    /// Get all domains.
    pub fn all() -> &'static [HardwareDomain] {
        &[
            Self::Pcb,
            Self::Ic,
            Self::Quantum,
            Self::Mems,
            Self::Rf,
            Self::Packaging,
        ]
    }
}

impl std::fmt::Display for HardwareDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_serialization() {
        let domain = HardwareDomain::Pcb;
        let json = serde_json::to_string(&domain).unwrap();
        assert_eq!(json, "\"pcb\"");
    }

    #[test]
    fn test_all_domains() {
        assert_eq!(HardwareDomain::all().len(), 6);
    }
}
