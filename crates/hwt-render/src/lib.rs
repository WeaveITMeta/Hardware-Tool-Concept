//! # hwt-render
//!
//! Bevy-based 3D rendering engine for Hardware Tool.
//!
//! This crate provides the shared 3D visualization infrastructure
//! used across all hardware domains.

pub mod camera;
pub mod plugin;
pub mod viewer;

pub use plugin::Viewer3DPlugin;
pub use viewer::{ViewPreset, Viewer3D, Viewer3DConfig};
