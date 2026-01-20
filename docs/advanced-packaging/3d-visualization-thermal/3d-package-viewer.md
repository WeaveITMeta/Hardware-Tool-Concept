# 3D Package Viewer

## Overview

Hardware Tool provides real-time 3D visualization of advanced packages, including exploded views, cross-sections, thermal overlays, and stress distribution for multi-die assemblies.

> **Inherits from:** [Shared 3D Viewer Architecture](../../3d-visualization/shared-3d-viewer-architecture.md)
>
> This viewer extends the unified 3D visualization engine with packaging-specific rendering modes and overlays. All standard navigation, cross-section, measurement, and export capabilities are inherited from the shared architecture.

---

## Packaging-Specific Extensions

## Viewer Features

```rust
PackageViewer3D {
    // Rendering modes
    modes: vec![
        RenderMode::Physical,          // Realistic materials
        RenderMode::Exploded,          // Separated layers
        RenderMode::CrossSection,      // Cut-away view
        RenderMode::Thermal,           // Temperature overlay
        RenderMode::Stress,            // Mechanical stress
        RenderMode::XRay,              // Internal structure
    ],
    
    // Component visibility
    visibility: ComponentVisibility {
        dies: true,
        interposer: true,
        substrate: true,
        bumps: true,
        underfill: true,
        heat_spreader: true,
        package_body: true,
    },
    
    // Overlay options
    overlays: OverlayOptions {
        thermal_map: true,
        stress_map: false,
        current_density: false,
        signal_paths: false,
    },
}
```

## Viewer UI

```
┌─────────────────────────────────────────────────────────────────┐
│ 3D Package Viewer: HPC Accelerator                             │
├─────────────────────────────────────────────────────────────────┤
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │                                                             │ │
│ │           ┌─────────────────────────────────┐              │ │
│ │           │      Heat Spreader (Cu)         │              │ │
│ │           └─────────────────────────────────┘              │ │
│ │                        │                                    │ │
│ │    ┌──────┐    ┌──────┴──────┐    ┌──────┐                │ │
│ │    │ HBM3 │    │   Compute   │    │ HBM3 │                │ │
│ │    │ Die  │    │    Die      │    │ Die  │                │ │
│ │    └──┬───┘    └──────┬──────┘    └──┬───┘                │ │
│ │       │               │               │                     │ │
│ │    ═══╧═══════════════╧═══════════════╧═══                 │ │
│ │              Silicon Interposer                            │ │
│ │    ═══════════════════════════════════════                 │ │
│ │                        │                                    │ │
│ │    ════════════════════╧════════════════════               │ │
│ │              Package Substrate                             │ │
│ │    ═════════════════════════════════════════               │ │
│ │         ● ● ● ● ● ● ● ● ● ● ● ● ● ● ●                     │ │
│ │                   BGA Balls                                │ │
│ │                                                             │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ View: [Exploded ▼]  Overlay: [Thermal ▼]  Scale: [1:1]         │
│                                                                 │
│ Components:                                                     │
│ ☑ Dies    ☑ Interposer    ☑ Substrate    ☑ Heat spreader      │
│ ☑ Bumps   ☑ Underfill     ☑ BGA balls    ☐ Package body       │
│                                                                 │
│ [Rotate] [Pan] [Zoom] [Section] [Export Image] [Export STEP]    │
└─────────────────────────────────────────────────────────────────┘
```

## Cross-Section View

```rust
CrossSectionView {
    // Section plane
    plane: SectionPlane {
        axis: Axis::X,
        position: 0.0,                 // Center
        direction: Direction::Positive,
    },
    
    // Display options
    display: CrossSectionDisplay {
        show_dimensions: true,
        show_materials: true,
        show_layer_names: true,
        highlight_tsv: true,
    },
    
    // Animation
    animation: SectionAnimation {
        enabled: true,
        sweep_range: (-25e-3, 25e-3),
        speed: 1.0,
    },
}
```

## Rust API

```rust
// Create 3D viewer
let viewer = PackageViewer3D::new(&system)?;

// Set view mode
viewer.set_mode(RenderMode::Exploded)?;

// Show thermal overlay
viewer.show_thermal_overlay(ThermalConfig {
    min_temp: 25.0,
    max_temp: 105.0,
    color_scale: ColorScale::Jet,
})?;

// Create cross-section
viewer.set_cross_section(CrossSection {
    axis: Axis::X,
    position: 0.0,
})?;

// Export
viewer.export_image("package_3d.png", Resolution::UHD)?;
viewer.export_step("package.step")?;
```

## Related Topics

- [Thermal-Mechanical Export/Import](./thermal-mechanical-export-import.md)
- [Die Placement & Stacking](../layout-concepts/die-placement-and-stacking.md)
