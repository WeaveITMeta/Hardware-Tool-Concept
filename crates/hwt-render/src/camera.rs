//! Camera controls for 3D viewer.

use bevy::prelude::*;

/// Camera controller component.
#[derive(Component)]
pub struct CameraController {
    /// Rotation sensitivity
    pub rotate_sensitivity: f32,

    /// Pan sensitivity
    pub pan_sensitivity: f32,

    /// Zoom sensitivity
    pub zoom_sensitivity: f32,

    /// Minimum zoom distance
    pub min_distance: f32,

    /// Maximum zoom distance
    pub max_distance: f32,

    /// Current distance from target
    pub distance: f32,

    /// Target point
    pub target: Vec3,

    /// Current yaw angle (radians)
    pub yaw: f32,

    /// Current pitch angle (radians)
    pub pitch: f32,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            rotate_sensitivity: 0.005,
            pan_sensitivity: 0.01,
            zoom_sensitivity: 0.1,
            min_distance: 1.0,
            max_distance: 1000.0,
            distance: 100.0,
            target: Vec3::ZERO,
            yaw: std::f32::consts::FRAC_PI_4,
            pitch: std::f32::consts::FRAC_PI_4,
        }
    }
}

impl CameraController {
    /// Calculate the camera transform from current state.
    pub fn calculate_transform(&self) -> Transform {
        let rotation = Quat::from_euler(EulerRot::YXZ, self.yaw, self.pitch, 0.0);
        let position = self.target + rotation * Vec3::new(0.0, 0.0, self.distance);
        Transform::from_translation(position).looking_at(self.target, Vec3::Y)
    }

    /// Rotate the camera.
    pub fn rotate(&mut self, delta_x: f32, delta_y: f32) {
        self.yaw -= delta_x * self.rotate_sensitivity;
        self.pitch -= delta_y * self.rotate_sensitivity;

        // Clamp pitch to avoid gimbal lock
        self.pitch = self.pitch.clamp(-1.5, 1.5);
    }

    /// Pan the camera.
    pub fn pan(&mut self, delta_x: f32, delta_y: f32) {
        let rotation = Quat::from_euler(EulerRot::YXZ, self.yaw, self.pitch, 0.0);
        let right = rotation * Vec3::X;
        let up = rotation * Vec3::Y;

        self.target += right * delta_x * self.pan_sensitivity * self.distance;
        self.target += up * delta_y * self.pan_sensitivity * self.distance;
    }

    /// Zoom the camera.
    pub fn zoom(&mut self, delta: f32) {
        self.distance *= 1.0 - delta * self.zoom_sensitivity;
        self.distance = self.distance.clamp(self.min_distance, self.max_distance);
    }

    /// Reset to default view.
    pub fn reset(&mut self) {
        self.target = Vec3::ZERO;
        self.distance = 100.0;
        self.yaw = std::f32::consts::FRAC_PI_4;
        self.pitch = std::f32::consts::FRAC_PI_4;
    }
}
