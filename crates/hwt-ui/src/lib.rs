//! # hwt-ui
//!
//! Slint-based UI shell for Hardware Tool.
//!
//! This crate provides the main application window with domain mode switching,
//! sidebars, and canvas integration.

pub mod app;
pub mod domain_mode;

pub use app::HardwareToolApp;
pub use domain_mode::DomainMode;
