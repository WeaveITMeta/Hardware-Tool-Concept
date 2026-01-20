//! 3D Viewer trait and configuration.

use bevy::prelude::*;

/// Quality presets for rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum QualityPreset {
    /// Fast preview, basic shading
    Low,
    /// Balanced quality/performance
    #[default]
    Medium,
    /// Full detail, shadows, AO
    High,
    /// Raytracing, max quality
    Ultra,
}

/// View presets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewPreset {
    Top,
    Bottom,
    Front,
    Back,
    Left,
    Right,
    Isometric,
}

impl ViewPreset {
    /// Get the camera transform for this preset.
    pub fn camera_transform(&self, distance: f32) -> Transform {
        match self {
            Self::Top => Transform::from_xyz(0.0, distance, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
            Self::Bottom => {
                Transform::from_xyz(0.0, -distance, 0.0).looking_at(Vec3::ZERO, Vec3::NEG_Z)
            }
            Self::Front => Transform::from_xyz(0.0, 0.0, distance).looking_at(Vec3::ZERO, Vec3::Y),
            Self::Back => {
                Transform::from_xyz(0.0, 0.0, -distance).looking_at(Vec3::ZERO, Vec3::Y)
            }
            Self::Left => {
                Transform::from_xyz(-distance, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y)
            }
            Self::Right => Transform::from_xyz(distance, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            Self::Isometric => {
                let d = distance * 0.577; // 1/sqrt(3)
                Transform::from_xyz(d, d, d).looking_at(Vec3::ZERO, Vec3::Y)
            }
        }
    }
}

/// 3D Viewer configuration.
#[derive(Debug, Clone)]
pub struct Viewer3DConfig {
    /// Quality preset
    pub quality: QualityPreset,

    /// Enable shadows
    pub shadows: bool,

    /// Enable reflections
    pub reflections: bool,

    /// Enable ambient occlusion
    pub ambient_occlusion: bool,

    /// Background color
    pub background: Color,
}

impl Default for Viewer3DConfig {
    fn default() -> Self {
        Self {
            quality: QualityPreset::Medium,
            shadows: true,
            reflections: false,
            ambient_occlusion: true,
            background: Color::srgb(0.1, 0.1, 0.12),
        }
    }
}

/// Trait for 3D viewers (implemented by domain-specific viewers).
pub trait Viewer3D {
    /// Set the view preset.
    fn set_view(&mut self, preset: ViewPreset);

    /// Rotate the camera.
    fn rotate(&mut self, yaw: f32, pitch: f32);

    /// Pan the camera.
    fn pan(&mut self, dx: f32, dy: f32);

    /// Zoom the camera.
    fn zoom(&mut self, factor: f32);

    /// Set layer visibility.
    fn set_layer_visibility(&mut self, layer: &str, visible: bool);

    /// Highlight items.
    fn highlight(&mut self, items: &[&str]);

    /// Clear highlights.
    fn clear_highlights(&mut self);

    /// Take a screenshot.
    fn screenshot(&self, path: &str, width: u32, height: u32);

    /// Export to STEP format.
    fn export_step(&self, path: &str);

    /// Export to STL format.
    fn export_stl(&self, path: &str);
}
