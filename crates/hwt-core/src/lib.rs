//! # hwt-core
//!
//! Core data model and Circuit JSON IR for Hardware Tool.
//!
//! This crate provides the universal data structures for representing
//! hardware designs across all domains: PCB, IC, Quantum, MEMS, RF, and Packaging.

pub mod circuit;
pub mod component;
pub mod constraint;
pub mod domain;
pub mod geometry;
pub mod layout;
pub mod net;
pub mod project;
pub mod units;

pub use circuit::CircuitJson;
pub use component::Component;
pub use constraint::Constraint;
pub use domain::HardwareDomain;
pub use geometry::{Point2D, Point3D, Position};
pub use layout::Layout;
pub use net::Net;
pub use project::{Project, ProjectMetadata};
