//! Main application.

use hwt_core::HardwareDomain;

use crate::domain_mode::DomainMode;

/// Hardware Tool application state.
pub struct HardwareToolApp {
    /// Current hardware domain
    pub domain: HardwareDomain,

    /// Current domain mode
    pub mode: DomainMode,

    /// Project name
    pub project_name: Option<String>,

    /// Unsaved changes flag
    pub unsaved: bool,

    /// Left sidebar collapsed
    pub left_sidebar_collapsed: bool,

    /// Right sidebar collapsed
    pub right_sidebar_collapsed: bool,
}

impl Default for HardwareToolApp {
    fn default() -> Self {
        Self {
            domain: HardwareDomain::Pcb,
            mode: DomainMode::Schematic,
            project_name: None,
            unsaved: false,
            left_sidebar_collapsed: false,
            right_sidebar_collapsed: false,
        }
    }
}

impl HardwareToolApp {
    /// Create a new application.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the hardware domain.
    pub fn set_domain(&mut self, domain: HardwareDomain) {
        self.domain = domain;
        // Reset to default mode for this domain
        self.mode = DomainMode::default_for_domain(domain);
    }

    /// Set the current mode.
    pub fn set_mode(&mut self, mode: DomainMode) {
        self.mode = mode;
    }

    /// Mark as having unsaved changes.
    pub fn mark_unsaved(&mut self) {
        self.unsaved = true;
    }

    /// Mark as saved.
    pub fn mark_saved(&mut self) {
        self.unsaved = false;
    }

    /// Get the window title.
    pub fn window_title(&self) -> String {
        let unsaved_indicator = if self.unsaved { " •" } else { "" };
        match &self.project_name {
            Some(name) => format!("{}{} - Hardware Tool", name, unsaved_indicator),
            None => "Hardware Tool".to_string(),
        }
    }

    /// Toggle left sidebar.
    pub fn toggle_left_sidebar(&mut self) {
        self.left_sidebar_collapsed = !self.left_sidebar_collapsed;
    }

    /// Toggle right sidebar.
    pub fn toggle_right_sidebar(&mut self) {
        self.right_sidebar_collapsed = !self.right_sidebar_collapsed;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_default() {
        let app = HardwareToolApp::new();
        assert_eq!(app.domain, HardwareDomain::Pcb);
        assert_eq!(app.mode, DomainMode::Schematic);
    }

    #[test]
    fn test_window_title() {
        let mut app = HardwareToolApp::new();
        assert_eq!(app.window_title(), "Hardware Tool");

        app.project_name = Some("My Project".to_string());
        assert_eq!(app.window_title(), "My Project - Hardware Tool");

        app.mark_unsaved();
        assert_eq!(app.window_title(), "My Project • - Hardware Tool");
    }
}
