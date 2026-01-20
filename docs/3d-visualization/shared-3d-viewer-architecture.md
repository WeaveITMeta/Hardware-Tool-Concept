# Shared 3D Viewer Architecture

## Overview

Hardware Tool provides a **unified 3D visualization engine** powered by Bevy that works across all hardware domains. Each domain extends this shared architecture with domain-specific rendering modes, overlays, and export options.

This document describes the **common infrastructure** inherited by all domain-specific 3D viewers.

---

## Shared Viewer Components

All domain-specific 3D viewers (PCB, IC, Quantum, MEMS, RF, Packaging) inherit these core capabilities:

| Component | Description |
|-----------|-------------|
| **Render Engine** | Bevy-based real-time rendering with raytracing support |
| **Navigation** | Rotate, pan, zoom, preset views |
| **Layer Visibility** | Toggle individual layers on/off |
| **Cross-Section** | Cut-away views along any axis |
| **Measurement** | Distance, clearance, height tools |
| **Export** | Screenshot, animation, STEP, STL |
| **UI Integration** | Floating panel, maximize/restore, keyboard shortcuts |

---

## Core Viewer Configuration

```rust
/// Base configuration inherited by all domain viewers
Viewer3DBase {
    // Rendering engine (shared)
    engine: RenderEngine::Bevy,
    
    // Quality presets (shared)
    quality: QualityPreset::High,  // Low, Medium, High, Ultra
    
    // Core features (shared)
    shadows: bool,
    reflections: bool,
    ambient_occlusion: bool,
    
    // Raytracing (shared)
    raytracing: RaytracingConfig {
        enabled: bool,
        samples_per_pixel: u32,
        max_bounces: u32,
        progressive: bool,
        denoise: bool,
    },
}
```

---

## Navigation Controls (All Domains)

### Mouse Controls

| Action | Control |
|--------|---------|
| Rotate | Left-click + drag |
| Pan | Middle-click + drag |
| Zoom | Scroll wheel |
| Select | Left-click |
| Context menu | Right-click |

### Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `Home` | Reset view |
| `T` | Top view |
| `B` | Bottom view |
| `F` | Front view |
| `R` | Right view |
| `1-9` | Preset views |
| `F8` | Toggle 3D panel |
| `Shift+F8` | Maximize/restore |
| `F11` | Fullscreen |

### View Presets

```rust
/// Standard view presets available in all domains
ViewPreset {
    top: Camera::orthographic(0.0, 90.0, 0.0),
    bottom: Camera::orthographic(0.0, -90.0, 0.0),
    front: Camera::orthographic(0.0, 0.0, 0.0),
    back: Camera::orthographic(180.0, 0.0, 0.0),
    left: Camera::orthographic(-90.0, 0.0, 0.0),
    right: Camera::orthographic(90.0, 0.0, 0.0),
    isometric: Camera::perspective(45.0, 35.264, 45.0),
}
```

---

## Floating Panel UI (All Domains)

The 3D viewer appears as a floating panel that can be maximized to full viewport.

```
┌─────────────────────────────────────────┐
│ 3D Preview                    [⬜][✕]  │  ← Maximize | Close
├─────────────────────────────────────────┤
│                                         │
│   ┌─────────────────────────────────┐   │
│   │                                 │   │
│   │   [Real-time 3D render]         │   │
│   │                                 │   │
│   └─────────────────────────────────┘   │
│                                         │
└─────────────────────────────────────────┘
```

### Panel Configuration

```rust
/// Floating panel config (shared across all domains)
Floating3DPanel {
    visible: bool,
    position: PanelPosition::TopRight,
    width: 320,
    height: 240,
    resizable: true,
    min_size: (200, 150),
    max_size: (600, 450),
    remember_size: true,
    remember_visibility: true,
}
```

---

## Cross-Section View (All Domains)

```rust
/// Cross-section configuration (shared)
SectionPlane {
    enabled: bool,
    axis: Axis,              // X, Y, Z
    position: f64,           // Distance from origin
    direction: Direction,    // Positive, Negative
    
    // Display options
    show_plane: bool,
    plane_color: Color,
    cap_cut: bool,
    cap_color: Color,
    
    // Animation
    animated: bool,
    sweep_range: (f64, f64),
    speed: f64,
}
```

---

## Export Capabilities (All Domains)

### Screenshot

```rust
viewer.screenshot(ScreenshotConfig {
    path: "render.png",
    width: 1920,
    height: 1080,
    samples: 4,                          // Anti-aliasing
    background: Background::Transparent, // or Solid, Gradient, HDRI
});
```

### Animation

```rust
viewer.export_animation(AnimationConfig {
    path: "rotation.mp4",
    type_: AnimationType::Turntable,
    duration: 10.0,
    fps: 30,
    resolution: (1920, 1080),
    codec: VideoCodec::H264,
});
```

### 3D Model Export

```rust
// STEP export (mechanical interchange)
viewer.export_step("model.step")?;

// STL export (3D printing)
viewer.export_stl("model.stl")?;
```

---

## Performance Settings (All Domains)

### Quality Presets

| Preset | Description | Use Case |
|--------|-------------|----------|
| `Low` | Fast preview, basic shading | Large designs, low-end hardware |
| `Medium` | Balanced quality/performance | General use |
| `High` | Full detail, shadows, AO | Presentation |
| `Ultra` | Raytracing, max quality | Final renders |

### GPU Configuration

```rust
GpuConfig {
    prefer_discrete: true,
    max_vram_mb: 4096,
    use_compute: true,
    async_compute: true,
}
```

---

## Domain-Specific Extensions

Each hardware domain extends the shared viewer with specialized capabilities:

| Domain | Specialized Modes | Specialized Overlays |
|--------|-------------------|---------------------|
| **PCB** | Copper, Solder Mask, X-Ray | Net highlighting |
| **IC** | Layer Stack, Metal Fill | Current density |
| **Quantum** | EM Field, CPW | Field distribution |
| **MEMS** | Deformation, Modal | Stress/strain |
| **RF** | Field Pattern, Radiation | S-parameter |
| **Packaging** | Exploded, Thermal | Temperature map |

See domain-specific documentation:

- [3D PCB Viewer](./3d-pcb-viewer.md)
- [3D IC Viewer](../ic-design/3d-visualization-parasitics/3d-ic-viewer.md)
- [3D Qubit Viewer](../quantum-hardware/3d-visualization-cryogenics/3d-qubit-viewer.md)
- [3D MEMS Viewer](../mems-sensors/3d-visualization-packaging/3d-mems-viewer.md)
- [3D RF Viewer](../rf-photonics/3d-visualization-em/3d-rf-viewer.md)
- [3D Package Viewer](../advanced-packaging/3d-visualization-thermal/3d-package-viewer.md)

---

## Rust API (Shared Base)

```rust
use hardware_tool::viewer::*;

// All domain viewers implement this trait
trait Viewer3D {
    // Navigation
    fn set_view(&mut self, preset: ViewPreset) -> Result<()>;
    fn rotate(&mut self, yaw: f64, pitch: f64) -> Result<()>;
    fn pan(&mut self, dx: f64, dy: f64) -> Result<()>;
    fn zoom(&mut self, factor: f64) -> Result<()>;
    
    // Visibility
    fn set_layer_visibility(&mut self, layer: &str, visible: bool) -> Result<()>;
    fn set_transparency(&mut self, layer: &str, alpha: f64) -> Result<()>;
    
    // Cross-section
    fn set_cross_section(&mut self, section: SectionPlane) -> Result<()>;
    fn clear_cross_section(&mut self) -> Result<()>;
    
    // Measurement
    fn measure_distance(&self, a: Point3D, b: Point3D) -> f64;
    fn measure_clearance(&self, item_a: &str, item_b: &str) -> f64;
    
    // Export
    fn screenshot(&self, config: ScreenshotConfig) -> Result<()>;
    fn export_animation(&self, config: AnimationConfig) -> Result<()>;
    fn export_step(&self, path: &str) -> Result<()>;
    fn export_stl(&self, path: &str) -> Result<()>;
    
    // Highlighting
    fn highlight(&mut self, items: &[&str]) -> Result<()>;
    fn clear_highlights(&mut self) -> Result<()>;
}
```

---

## Bevy Integration (Shared)

```rust
use bevy::prelude::*;
use hardware_tool::viewer::*;

/// Shared Bevy plugin for 3D visualization
pub struct Viewer3DPlugin;

impl Plugin for Viewer3DPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_viewer)
            .add_systems(Update, (
                handle_navigation,
                update_visibility,
                render_overlays,
            ));
    }
}

fn setup_viewer(mut commands: Commands) {
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 100.0, 100.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    
    // Lighting
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
}
```

---

## Related Topics

- [STEP & 3D Model Export/Import](./step-3d-models.md)
- [Thermal Simulation](../advanced-features/thermal-simulation.md) - Thermal overlay data
- [Electromagnetic Simulation](../advanced-features/electromagnetic-simulation.md) - EM field overlay data
