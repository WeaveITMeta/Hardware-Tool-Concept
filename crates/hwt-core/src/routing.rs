//! PCB Routing Engine.
//!
//! Implements point-to-point routing with support for multiple routing modes,
//! corner styles, and layer transitions.

use serde::{Deserialize, Serialize};

use crate::geometry::Position;
use crate::layout::{Layout, Trace, Via, ViaType};
use crate::units::LengthUnit;

/// Routing result type.
pub type RoutingResult<T> = Result<T, RoutingError>;

/// Routing errors.
#[derive(Debug, Clone)]
pub enum RoutingError {
    /// No valid path found
    NoPath(String),
    /// DRC violation would occur
    DrcViolation(String),
    /// Invalid start/end point
    InvalidEndpoint(String),
    /// Layer not found
    LayerNotFound(String),
    /// Route cancelled
    Cancelled,
}

impl std::fmt::Display for RoutingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RoutingError::NoPath(msg) => write!(f, "No path found: {}", msg),
            RoutingError::DrcViolation(msg) => write!(f, "DRC violation: {}", msg),
            RoutingError::InvalidEndpoint(msg) => write!(f, "Invalid endpoint: {}", msg),
            RoutingError::LayerNotFound(layer) => write!(f, "Layer not found: {}", layer),
            RoutingError::Cancelled => write!(f, "Route cancelled"),
        }
    }
}

impl std::error::Error for RoutingError {}

/// Corner style for routing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum CornerStyle {
    /// 90° sharp corners
    #[default]
    Sharp,
    /// 45° mitered corners
    Mitered45,
    /// Rounded arc corners
    Rounded,
}

/// Routing mode (direction preference).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum RoutingMode {
    /// Horizontal first, then vertical
    #[default]
    HorizontalFirst,
    /// Vertical first, then horizontal
    VerticalFirst,
    /// Diagonal (45°) routing
    Diagonal,
    /// Free angle routing
    FreeAngle,
}

impl RoutingMode {
    /// Toggle between horizontal-first and vertical-first.
    pub fn toggle(&self) -> Self {
        match self {
            RoutingMode::HorizontalFirst => RoutingMode::VerticalFirst,
            RoutingMode::VerticalFirst => RoutingMode::HorizontalFirst,
            other => *other,
        }
    }
}

/// Routing configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingConfig {
    /// Default trace width (mm)
    pub trace_width: f64,
    
    /// Default via drill diameter (mm)
    pub via_drill: f64,
    
    /// Default via pad diameter (mm)
    pub via_pad: f64,
    
    /// Corner style
    pub corner_style: CornerStyle,
    
    /// Routing mode
    pub routing_mode: RoutingMode,
    
    /// Minimum clearance (mm)
    pub clearance: f64,
    
    /// Enable DRC during routing
    pub drc_enabled: bool,
    
    /// Snap to grid
    pub snap_to_grid: bool,
    
    /// Grid size (mm)
    pub grid_size: f64,
    
    /// Available trace widths for quick selection
    pub width_presets: Vec<f64>,
}

impl Default for RoutingConfig {
    fn default() -> Self {
        Self {
            trace_width: 0.25,
            via_drill: 0.3,
            via_pad: 0.6,
            corner_style: CornerStyle::Mitered45,
            routing_mode: RoutingMode::HorizontalFirst,
            clearance: 0.2,
            drc_enabled: true,
            snap_to_grid: true,
            grid_size: 0.1,
            width_presets: vec![0.15, 0.2, 0.25, 0.3, 0.4, 0.5, 0.8, 1.0],
        }
    }
}

/// A route segment (part of a trace being constructed).
#[derive(Debug, Clone)]
pub struct RouteSegment {
    /// Start position
    pub start: Position,
    /// End position
    pub end: Position,
    /// Layer name
    pub layer: String,
    /// Trace width
    pub width: f64,
}

/// Active routing session state.
#[derive(Debug, Clone)]
pub struct RoutingSession {
    /// Net being routed
    pub net: String,
    
    /// Current layer
    pub current_layer: String,
    
    /// Route segments created so far
    pub segments: Vec<RouteSegment>,
    
    /// Vias inserted during routing
    pub vias: Vec<Position>,
    
    /// Current cursor position
    pub cursor: Position,
    
    /// Starting position
    pub start: Position,
    
    /// Current trace width
    pub width: f64,
    
    /// Routing mode
    pub mode: RoutingMode,
    
    /// Is routing active
    pub active: bool,
}

impl RoutingSession {
    /// Create a new routing session.
    pub fn new(net: String, start: Position, layer: String, width: f64) -> Self {
        Self {
            net,
            current_layer: layer,
            segments: Vec::new(),
            vias: Vec::new(),
            cursor: start.clone(),
            start,
            width,
            mode: RoutingMode::default(),
            active: true,
        }
    }
    
    /// Get the current endpoint (last segment end or start).
    pub fn current_point(&self) -> &Position {
        self.segments.last()
            .map(|s| &s.end)
            .unwrap_or(&self.start)
    }
    
    /// Add a segment to the route.
    pub fn add_segment(&mut self, end: Position) {
        let start = self.current_point().clone();
        self.segments.push(RouteSegment {
            start,
            end: end.clone(),
            layer: self.current_layer.clone(),
            width: self.width,
        });
        self.cursor = end;
    }
    
    /// Undo the last segment.
    pub fn undo_segment(&mut self) -> bool {
        if let Some(segment) = self.segments.pop() {
            self.cursor = segment.start;
            true
        } else {
            false
        }
    }
    
    /// Insert a via at the current position and switch layers.
    pub fn insert_via(&mut self, new_layer: String) {
        self.vias.push(self.cursor.clone());
        self.current_layer = new_layer;
    }
    
    /// Change trace width.
    pub fn set_width(&mut self, width: f64) {
        self.width = width;
    }
    
    /// Toggle routing mode.
    pub fn toggle_mode(&mut self) {
        self.mode = self.mode.toggle();
    }
    
    /// Cancel the routing session.
    pub fn cancel(&mut self) {
        self.active = false;
        self.segments.clear();
        self.vias.clear();
    }
    
    /// Get total route length.
    pub fn total_length(&self) -> f64 {
        self.segments.iter()
            .map(|s| {
                let dx = s.end.x - s.start.x;
                let dy = s.end.y - s.start.y;
                (dx * dx + dy * dy).sqrt()
            })
            .sum()
    }
}

/// Point-to-point router.
pub struct Router<'a> {
    layout: &'a mut Layout,
    config: RoutingConfig,
}

impl<'a> Router<'a> {
    /// Create a new router.
    pub fn new(layout: &'a mut Layout, config: RoutingConfig) -> Self {
        Self { layout, config }
    }
    
    /// Start a new routing session.
    pub fn start_route(&self, net: String, start: Position, layer: String) -> RoutingResult<RoutingSession> {
        // Validate layer exists
        if !self.layout.layers.iter().any(|l| l.name == layer) {
            return Err(RoutingError::LayerNotFound(layer));
        }
        
        let start = self.snap_to_grid(start);
        Ok(RoutingSession::new(net, start, layer, self.config.trace_width))
    }
    
    /// Calculate route segments from current point to target.
    pub fn calculate_segments(
        &self,
        session: &RoutingSession,
        target: Position,
    ) -> Vec<RouteSegment> {
        let target = self.snap_to_grid(target);
        let current = session.current_point();
        
        match session.mode {
            RoutingMode::HorizontalFirst => {
                self.route_orthogonal(current, &target, &session.current_layer, session.width, true)
            }
            RoutingMode::VerticalFirst => {
                self.route_orthogonal(current, &target, &session.current_layer, session.width, false)
            }
            RoutingMode::Diagonal => {
                self.route_diagonal(current, &target, &session.current_layer, session.width)
            }
            RoutingMode::FreeAngle => {
                vec![RouteSegment {
                    start: current.clone(),
                    end: target,
                    layer: session.current_layer.clone(),
                    width: session.width,
                }]
            }
        }
    }
    
    /// Route using orthogonal segments (horizontal/vertical).
    fn route_orthogonal(
        &self,
        start: &Position,
        end: &Position,
        layer: &str,
        width: f64,
        horizontal_first: bool,
    ) -> Vec<RouteSegment> {
        let dx = end.x - start.x;
        let dy = end.y - start.y;
        
        // If already aligned, single segment
        if dx.abs() < 0.001 || dy.abs() < 0.001 {
            return vec![RouteSegment {
                start: start.clone(),
                end: end.clone(),
                layer: layer.to_string(),
                width,
            }];
        }
        
        // Create corner point based on mode
        let corner = if horizontal_first {
            Position { x: end.x, y: start.y, z: None, unit: start.unit }
        } else {
            Position { x: start.x, y: end.y, z: None, unit: start.unit }
        };
        
        match self.config.corner_style {
            CornerStyle::Sharp => {
                vec![
                    RouteSegment {
                        start: start.clone(),
                        end: corner.clone(),
                        layer: layer.to_string(),
                        width,
                    },
                    RouteSegment {
                        start: corner,
                        end: end.clone(),
                        layer: layer.to_string(),
                        width,
                    },
                ]
            }
            CornerStyle::Mitered45 => {
                self.route_mitered_45(start, end, layer, width, horizontal_first)
            }
            CornerStyle::Rounded => {
                // For now, use mitered as placeholder
                self.route_mitered_45(start, end, layer, width, horizontal_first)
            }
        }
    }
    
    /// Route with 45° mitered corners.
    fn route_mitered_45(
        &self,
        start: &Position,
        end: &Position,
        layer: &str,
        width: f64,
        horizontal_first: bool,
    ) -> Vec<RouteSegment> {
        let dx = end.x - start.x;
        let dy = end.y - start.y;
        
        // Calculate the 45° miter distance
        let miter_dist = dx.abs().min(dy.abs());
        
        if miter_dist < 0.1 {
            // Too short for miter, use sharp corner
            let corner = if horizontal_first {
                Position { x: end.x, y: start.y, z: None, unit: start.unit }
            } else {
                Position { x: start.x, y: end.y, z: None, unit: start.unit }
            };
            return vec![
                RouteSegment {
                    start: start.clone(),
                    end: corner.clone(),
                    layer: layer.to_string(),
                    width,
                },
                RouteSegment {
                    start: corner,
                    end: end.clone(),
                    layer: layer.to_string(),
                    width,
                },
            ];
        }
        
        // Create 3-segment route with 45° diagonal
        let (p1, p2) = if horizontal_first {
            let x_sign = if dx > 0.0 { 1.0 } else { -1.0 };
            let y_sign = if dy > 0.0 { 1.0 } else { -1.0 };
            
            let p1 = Position {
                x: end.x - x_sign * miter_dist,
                y: start.y,
                z: None,
                unit: start.unit,
            };
            let p2 = Position {
                x: end.x,
                y: start.y + y_sign * miter_dist,
                z: None,
                unit: start.unit,
            };
            (p1, p2)
        } else {
            let x_sign = if dx > 0.0 { 1.0 } else { -1.0 };
            let y_sign = if dy > 0.0 { 1.0 } else { -1.0 };
            
            let p1 = Position {
                x: start.x,
                y: end.y - y_sign * miter_dist,
                z: None,
                unit: start.unit,
            };
            let p2 = Position {
                x: start.x + x_sign * miter_dist,
                y: end.y,
                z: None,
                unit: start.unit,
            };
            (p1, p2)
        };
        
        vec![
            RouteSegment {
                start: start.clone(),
                end: p1.clone(),
                layer: layer.to_string(),
                width,
            },
            RouteSegment {
                start: p1,
                end: p2.clone(),
                layer: layer.to_string(),
                width,
            },
            RouteSegment {
                start: p2,
                end: end.clone(),
                layer: layer.to_string(),
                width,
            },
        ]
    }
    
    /// Route using 45° diagonal segments.
    fn route_diagonal(
        &self,
        start: &Position,
        end: &Position,
        layer: &str,
        width: f64,
    ) -> Vec<RouteSegment> {
        let dx = end.x - start.x;
        let dy = end.y - start.y;
        
        // Calculate diagonal distance
        let diag_dist = dx.abs().min(dy.abs());
        
        if diag_dist < 0.1 {
            // Nearly aligned, single segment
            return vec![RouteSegment {
                start: start.clone(),
                end: end.clone(),
                layer: layer.to_string(),
                width,
            }];
        }
        
        let x_sign = if dx > 0.0 { 1.0 } else { -1.0 };
        let y_sign = if dy > 0.0 { 1.0 } else { -1.0 };
        
        // Diagonal first, then straight
        let diag_end = Position {
            x: start.x + x_sign * diag_dist,
            y: start.y + y_sign * diag_dist,
            z: None,
            unit: start.unit,
        };
        
        vec![
            RouteSegment {
                start: start.clone(),
                end: diag_end.clone(),
                layer: layer.to_string(),
                width,
            },
            RouteSegment {
                start: diag_end,
                end: end.clone(),
                layer: layer.to_string(),
                width,
            },
        ]
    }
    
    /// Snap position to grid.
    fn snap_to_grid(&self, pos: Position) -> Position {
        if !self.config.snap_to_grid {
            return pos;
        }
        
        let grid = self.config.grid_size;
        Position {
            x: (pos.x / grid).round() * grid,
            y: (pos.y / grid).round() * grid,
            z: pos.z,
            unit: pos.unit,
        }
    }
    
    /// Commit a routing session to the layout.
    pub fn commit_route(&mut self, session: RoutingSession) -> RoutingResult<()> {
        if !session.active {
            return Err(RoutingError::Cancelled);
        }
        
        if session.segments.is_empty() {
            return Err(RoutingError::NoPath("No segments to commit".to_string()));
        }
        
        // Add traces
        for segment in session.segments {
            self.layout.traces.push(Trace {
                net: session.net.clone(),
                layer: segment.layer,
                start: segment.start,
                end: segment.end,
                width: segment.width,
                unit: LengthUnit::Mm,
            });
        }
        
        // Add vias
        for via_pos in session.vias {
            self.layout.vias.push(Via {
                net: session.net.clone(),
                position: via_pos,
                via_type: ViaType::Through,
                drill: self.config.via_drill,
                pad: self.config.via_pad,
                start_layer: None,
                end_layer: None,
                unit: LengthUnit::Mm,
            });
        }
        
        Ok(())
    }
    
    /// Get next width preset (cycle through presets).
    pub fn next_width(&self, current: f64) -> f64 {
        let presets = &self.config.width_presets;
        if presets.is_empty() {
            return current;
        }
        
        // Find current index
        let idx = presets.iter()
            .position(|&w| (w - current).abs() < 0.001)
            .unwrap_or(0);
        
        presets[(idx + 1) % presets.len()]
    }
    
    /// Get previous width preset.
    pub fn prev_width(&self, current: f64) -> f64 {
        let presets = &self.config.width_presets;
        if presets.is_empty() {
            return current;
        }
        
        let idx = presets.iter()
            .position(|&w| (w - current).abs() < 0.001)
            .unwrap_or(0);
        
        if idx == 0 {
            presets[presets.len() - 1]
        } else {
            presets[idx - 1]
        }
    }
    
    /// Get available copper layers.
    pub fn copper_layers(&self) -> Vec<&str> {
        self.layout.layers.iter()
            .filter(|l| matches!(l.layer_type, crate::layout::LayerType::Copper))
            .map(|l| l.name.as_str())
            .collect()
    }
    
    /// Switch to next copper layer.
    pub fn next_layer(&self, current: &str) -> Option<String> {
        let layers = self.copper_layers();
        let idx = layers.iter().position(|&l| l == current)?;
        let next_idx = (idx + 1) % layers.len();
        Some(layers[next_idx].to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn make_position(x: f64, y: f64) -> Position {
        Position { x, y, z: None, unit: LengthUnit::Mm }
    }
    
    #[test]
    fn test_routing_session_creation() {
        let session = RoutingSession::new(
            "VCC".to_string(),
            make_position(10.0, 10.0),
            "F.Cu".to_string(),
            0.25,
        );
        
        assert_eq!(session.net, "VCC");
        assert_eq!(session.current_layer, "F.Cu");
        assert!(session.active);
        assert!(session.segments.is_empty());
    }
    
    #[test]
    fn test_routing_session_add_segment() {
        let mut session = RoutingSession::new(
            "NET1".to_string(),
            make_position(0.0, 0.0),
            "F.Cu".to_string(),
            0.25,
        );
        
        session.add_segment(make_position(10.0, 0.0));
        session.add_segment(make_position(10.0, 10.0));
        
        assert_eq!(session.segments.len(), 2);
        assert_eq!(session.current_point().x, 10.0);
        assert_eq!(session.current_point().y, 10.0);
    }
    
    #[test]
    fn test_routing_session_undo() {
        let mut session = RoutingSession::new(
            "NET1".to_string(),
            make_position(0.0, 0.0),
            "F.Cu".to_string(),
            0.25,
        );
        
        session.add_segment(make_position(10.0, 0.0));
        session.add_segment(make_position(10.0, 10.0));
        
        assert!(session.undo_segment());
        assert_eq!(session.segments.len(), 1);
        assert_eq!(session.cursor.x, 10.0);
        assert_eq!(session.cursor.y, 0.0);
        
        assert!(session.undo_segment());
        assert_eq!(session.segments.len(), 0);
        
        // Can't undo past start
        assert!(!session.undo_segment());
    }
    
    #[test]
    fn test_routing_session_via_insert() {
        let mut session = RoutingSession::new(
            "NET1".to_string(),
            make_position(5.0, 5.0),
            "F.Cu".to_string(),
            0.25,
        );
        
        session.insert_via("B.Cu".to_string());
        
        assert_eq!(session.vias.len(), 1);
        assert_eq!(session.current_layer, "B.Cu");
    }
    
    #[test]
    fn test_routing_session_total_length() {
        let mut session = RoutingSession::new(
            "NET1".to_string(),
            make_position(0.0, 0.0),
            "F.Cu".to_string(),
            0.25,
        );
        
        session.add_segment(make_position(10.0, 0.0));
        session.add_segment(make_position(10.0, 10.0));
        
        let length = session.total_length();
        assert!((length - 20.0).abs() < 0.001);
    }
    
    #[test]
    fn test_routing_mode_toggle() {
        let mode = RoutingMode::HorizontalFirst;
        assert_eq!(mode.toggle(), RoutingMode::VerticalFirst);
        assert_eq!(mode.toggle().toggle(), RoutingMode::HorizontalFirst);
    }
    
    #[test]
    fn test_router_start_route() {
        let mut layout = Layout::with_board_size(100.0, 80.0, LengthUnit::Mm);
        let config = RoutingConfig::default();
        let router = Router::new(&mut layout, config);
        
        let session = router.start_route(
            "VCC".to_string(),
            make_position(10.0, 10.0),
            "F.Cu".to_string(),
        );
        
        assert!(session.is_ok());
    }
    
    #[test]
    fn test_router_invalid_layer() {
        let mut layout = Layout::with_board_size(100.0, 80.0, LengthUnit::Mm);
        let config = RoutingConfig::default();
        let router = Router::new(&mut layout, config);
        
        let session = router.start_route(
            "VCC".to_string(),
            make_position(10.0, 10.0),
            "Invalid.Layer".to_string(),
        );
        
        assert!(session.is_err());
    }
    
    #[test]
    fn test_router_calculate_segments_horizontal_first() {
        let mut layout = Layout::with_board_size(100.0, 80.0, LengthUnit::Mm);
        let config = RoutingConfig {
            corner_style: CornerStyle::Sharp,
            snap_to_grid: false,
            ..Default::default()
        };
        let router = Router::new(&mut layout, config);
        
        let session = RoutingSession::new(
            "NET1".to_string(),
            make_position(0.0, 0.0),
            "F.Cu".to_string(),
            0.25,
        );
        
        let segments = router.calculate_segments(&session, make_position(10.0, 10.0));
        
        // Should have 2 segments for orthogonal routing
        assert_eq!(segments.len(), 2);
        // First segment should be horizontal
        assert_eq!(segments[0].start.x, 0.0);
        assert_eq!(segments[0].end.x, 10.0);
        assert_eq!(segments[0].end.y, 0.0);
    }
    
    #[test]
    fn test_router_commit_route() {
        let mut layout = Layout::with_board_size(100.0, 80.0, LengthUnit::Mm);
        let config = RoutingConfig::default();
        
        let mut session = RoutingSession::new(
            "VCC".to_string(),
            make_position(10.0, 10.0),
            "F.Cu".to_string(),
            0.25,
        );
        session.add_segment(make_position(50.0, 10.0));
        session.insert_via("B.Cu".to_string());
        session.add_segment(make_position(50.0, 50.0));
        
        {
            let mut router = Router::new(&mut layout, config);
            let result = router.commit_route(session);
            assert!(result.is_ok());
        }
        
        assert_eq!(layout.traces.len(), 2);
        assert_eq!(layout.vias.len(), 1);
    }
    
    #[test]
    fn test_router_width_presets() {
        let mut layout = Layout::with_board_size(100.0, 80.0, LengthUnit::Mm);
        let config = RoutingConfig {
            width_presets: vec![0.2, 0.25, 0.3],
            ..Default::default()
        };
        let router = Router::new(&mut layout, config);
        
        assert_eq!(router.next_width(0.2), 0.25);
        assert_eq!(router.next_width(0.25), 0.3);
        assert_eq!(router.next_width(0.3), 0.2); // Wraps around
        
        assert_eq!(router.prev_width(0.25), 0.2);
        assert_eq!(router.prev_width(0.2), 0.3); // Wraps around
    }
    
    #[test]
    fn test_router_copper_layers() {
        let mut layout = Layout::with_board_size(100.0, 80.0, LengthUnit::Mm);
        let config = RoutingConfig::default();
        let router = Router::new(&mut layout, config);
        
        let layers = router.copper_layers();
        assert!(layers.contains(&"F.Cu"));
        assert!(layers.contains(&"B.Cu"));
    }
}
