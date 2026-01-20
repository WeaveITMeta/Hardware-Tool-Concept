//! Domain mode definitions.
//!
//! Each hardware domain has different view modes (tabs).

use hwt_core::HardwareDomain;

/// View modes within a domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DomainMode {
    /// Schematic editor
    #[default]
    Schematic,
    /// Layout editor (PCB, IC, etc.)
    Layout,
    /// 3D viewer
    ThreeD,
    /// Code editor
    Code,
    /// RTL editor (IC only)
    Rtl,
    /// Circuit editor (Quantum only)
    Circuit,
    /// Device editor (MEMS only)
    Device,
    /// Die map editor (Packaging only)
    DieMap,
    /// S-parameter viewer (RF only)
    SParams,
    /// Simulation results
    Simulation,
    /// Thermal viewer (Packaging only)
    Thermal,
}

impl DomainMode {
    /// Get the default mode for a domain.
    pub fn default_for_domain(domain: HardwareDomain) -> Self {
        match domain {
            HardwareDomain::Pcb => Self::Schematic,
            HardwareDomain::Ic => Self::Rtl,
            HardwareDomain::Quantum => Self::Circuit,
            HardwareDomain::Mems => Self::Device,
            HardwareDomain::Rf => Self::Schematic,
            HardwareDomain::Packaging => Self::DieMap,
        }
    }

    /// Get available modes for a domain.
    pub fn modes_for_domain(domain: HardwareDomain) -> &'static [DomainMode] {
        match domain {
            HardwareDomain::Pcb => &[
                Self::Schematic,
                Self::Layout,
                Self::ThreeD,
                Self::Code,
            ],
            HardwareDomain::Ic => &[
                Self::Rtl,
                Self::Schematic,
                Self::Layout,
                Self::ThreeD,
                Self::Code,
            ],
            HardwareDomain::Quantum => &[
                Self::Circuit,
                Self::Layout,
                Self::ThreeD,
                Self::Simulation,
                Self::Code,
            ],
            HardwareDomain::Mems => &[
                Self::Device,
                Self::Layout,
                Self::ThreeD,
                Self::Simulation,
                Self::Code,
            ],
            HardwareDomain::Rf => &[
                Self::Schematic,
                Self::Layout,
                Self::ThreeD,
                Self::SParams,
                Self::Code,
            ],
            HardwareDomain::Packaging => &[
                Self::DieMap,
                Self::Layout,
                Self::ThreeD,
                Self::Thermal,
                Self::Code,
            ],
        }
    }

    /// Get the display name.
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Schematic => "Schematic",
            Self::Layout => "Layout",
            Self::ThreeD => "3D",
            Self::Code => "Code",
            Self::Rtl => "RTL",
            Self::Circuit => "Circuit",
            Self::Device => "Device",
            Self::DieMap => "Die Map",
            Self::SParams => "S-Params",
            Self::Simulation => "Simulation",
            Self::Thermal => "Thermal",
        }
    }

    /// Get the icon.
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Schematic => "📐",
            Self::Layout => "🔲",
            Self::ThreeD => "🎲",
            Self::Code => "💻",
            Self::Rtl => "📝",
            Self::Circuit => "⚛️",
            Self::Device => "📡",
            Self::DieMap => "📦",
            Self::SParams => "📊",
            Self::Simulation => "📈",
            Self::Thermal => "🌡️",
        }
    }

    /// Get the keyboard shortcut.
    pub fn shortcut(&self) -> &'static str {
        match self {
            Self::Schematic | Self::Rtl | Self::Circuit | Self::Device | Self::DieMap => "F5",
            Self::Layout => "F6",
            Self::Code => "F7",
            Self::ThreeD => "F8",
            Self::Simulation | Self::SParams | Self::Thermal => "F9",
        }
    }
}

impl std::fmt::Display for DomainMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.icon(), self.display_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modes_for_domain() {
        let pcb_modes = DomainMode::modes_for_domain(HardwareDomain::Pcb);
        assert_eq!(pcb_modes.len(), 4);
        assert!(pcb_modes.contains(&DomainMode::Schematic));
    }

    #[test]
    fn test_default_mode() {
        assert_eq!(
            DomainMode::default_for_domain(HardwareDomain::Pcb),
            DomainMode::Schematic
        );
        assert_eq!(
            DomainMode::default_for_domain(HardwareDomain::Ic),
            DomainMode::Rtl
        );
    }
}
