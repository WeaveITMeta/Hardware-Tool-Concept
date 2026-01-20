//! Geometry primitives.
//!
//! Basic geometric types used throughout Hardware Tool.

use serde::{Deserialize, Serialize};

use crate::units::LengthUnit;

/// 2D point.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    /// Create a new 2D point.
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Distance to another point.
    pub fn distance(&self, other: &Point2D) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

/// 3D point.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    /// Create a new 3D point.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Distance to another point.
    pub fn distance(&self, other: &Point3D) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2))
            .sqrt()
    }
}

/// Position with optional Z and unit.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub z: Option<f64>,
    #[serde(default)]
    pub unit: LengthUnit,
}

impl Position {
    /// Create a new 2D position.
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            z: None,
            unit: LengthUnit::Mm,
        }
    }

    /// Create a new 3D position.
    pub fn new_3d(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z: Some(z),
            unit: LengthUnit::Mm,
        }
    }

    /// Convert to Point2D.
    pub fn to_point2d(&self) -> Point2D {
        Point2D::new(self.x, self.y)
    }

    /// Convert to Point3D.
    pub fn to_point3d(&self) -> Point3D {
        Point3D::new(self.x, self.y, self.z.unwrap_or(0.0))
    }
}

/// A bounding box.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BoundingBox {
    pub min: Point2D,
    pub max: Point2D,
}

impl BoundingBox {
    /// Create a new bounding box.
    pub fn new(min: Point2D, max: Point2D) -> Self {
        Self { min, max }
    }

    /// Width of the bounding box.
    pub fn width(&self) -> f64 {
        self.max.x - self.min.x
    }

    /// Height of the bounding box.
    pub fn height(&self) -> f64 {
        self.max.y - self.min.y
    }

    /// Center of the bounding box.
    pub fn center(&self) -> Point2D {
        Point2D::new(
            (self.min.x + self.max.x) / 2.0,
            (self.min.y + self.max.y) / 2.0,
        )
    }

    /// Check if a point is inside the bounding box.
    pub fn contains(&self, point: &Point2D) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_distance() {
        let a = Point2D::new(0.0, 0.0);
        let b = Point2D::new(3.0, 4.0);
        assert!((a.distance(&b) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_bounding_box() {
        let bbox = BoundingBox::new(Point2D::new(0.0, 0.0), Point2D::new(10.0, 20.0));
        assert_eq!(bbox.width(), 10.0);
        assert_eq!(bbox.height(), 20.0);
        assert!(bbox.contains(&Point2D::new(5.0, 10.0)));
        assert!(!bbox.contains(&Point2D::new(15.0, 10.0)));
    }
}
