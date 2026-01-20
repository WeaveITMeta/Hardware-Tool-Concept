# 3D IC Viewer

## Overview

Hardware Tool provides real-time 3D visualization of integrated circuit layouts, including layer-by-layer views, via stacks, metal density, and parasitic extraction overlays for analog, digital, and mixed-signal designs.

> **Inherits from:** [Shared 3D Viewer Architecture](../../3d-visualization/shared-3d-viewer-architecture.md)
>
> This viewer extends the unified 3D visualization engine with IC-specific rendering modes and overlays. All standard navigation, cross-section, measurement, and export capabilities are inherited from the shared architecture.

---

## IC-Specific Extensions

## Viewer Features

```rust
IcViewer3D {
    // Rendering modes
    modes: vec![
        RenderMode::Physical,          // Realistic metal/dielectric
        RenderMode::LayerStack,        // Exploded layer view
        RenderMode::Density,           // Metal density heatmap
        RenderMode::Parasitic,         // RC extraction overlay
        RenderMode::CurrentDensity,    // EM/IR drop visualization
        RenderMode::XRay,              // Transparent dielectric
    ],
    
    // Layer visibility
    layers: LayerVisibility {
        substrate: true,
        diffusion: true,
        poly: true,
        metal1: true,
        metal2: true,
        // ... up to top metal
        via_stack: true,
        passivation: true,
    },
    
    // Parasitic overlay
    parasitic_display: ParasiticDisplay {
        show_resistance: true,
        show_capacitance: true,
        show_inductance: false,
        scale: ColorScale::Viridis,
        threshold: 0.1,  // Only show significant parasitics
    },
}
```

## Viewer UI

```
┌─────────────────────────────────────────────────────────────────┐
│ 3D IC Viewer: OpAmp_v2 (sky130)                                 │
├─────────────────────────────────────────────────────────────────┤
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │                                                             │ │
│ │    ════════════════════════════════════════════            │ │
│ │                    Metal 5 (Top)                           │ │
│ │    ════════════════════════════════════════════            │ │
│ │         │    │    │    │    │    │    │                    │ │
│ │         ●    ●    ●    ●    ●    ●    ●   Via4             │ │
│ │    ════════════════════════════════════════════            │ │
│ │                    Metal 4                                 │ │
│ │    ════════════════════════════════════════════            │ │
│ │         │    │    │    │    │    │    │                    │ │
│ │         ●    ●    ●    ●    ●    ●    ●   Via3             │ │
│ │    ════════════════════════════════════════════            │ │
│ │                    Metal 3                                 │ │
│ │    ════════════════════════════════════════════            │ │
│ │                        ...                                 │ │
│ │    ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓               │ │
│ │                    Substrate                               │ │
│ │                                                             │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ View: [Physical ▼]  Overlay: [Parasitic ▼]  PDK: sky130        │
│                                                                 │
│ Layers:                                                         │
│ ☑ Substrate   ☑ Diffusion   ☑ Poly   ☑ LI   ☑ M1-M5          │
│ ☑ Vias        ☐ Passivation ☐ Density overlay                 │
│                                                                 │
│ [Rotate] [Pan] [Zoom] [Section] [Export Image] [Export GDSII]   │
└─────────────────────────────────────────────────────────────────┘
```

---

## Parasitic Visualization

### RC Extraction Overlay

```rust
ParasiticVisualization {
    // Extraction source
    source: ExtractionSource::StarRC,  // or Calibre, QRC
    
    // Display options
    display: ParasiticDisplayOptions {
        // Resistance
        resistance: ResistanceDisplay {
            enabled: true,
            min_value: 0.1,        // Ohms
            max_value: 1000.0,
            color_scale: ColorScale::Hot,
            show_values: true,
        },
        
        // Capacitance
        capacitance: CapacitanceDisplay {
            enabled: true,
            min_value: 0.01,       // fF
            max_value: 100.0,
            color_scale: ColorScale::Cool,
            show_coupling: true,   // Show coupling caps
        },
        
        // Critical paths
        highlight_critical: true,
        critical_threshold: 10.0,  // ps delay contribution
    },
}
```

### Parasitic Overlay UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Parasitic Overlay                                        [✕]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Extraction: StarRC (post-layout)                               │
│                                                                 │
│ Display:                                                        │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ ☑ Resistance                                               │ │
│ │   Range: [0.1 Ω] to [1000 Ω]                               │ │
│ │   ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓                │ │
│ │   Low                                              High    │ │
│ │                                                             │ │
│ │ ☑ Capacitance                                              │ │
│ │   Range: [0.01 fF] to [100 fF]                             │ │
│ │   ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒                │ │
│ │   Low                                              High    │ │
│ │                                                             │ │
│ │ ☑ Show coupling capacitance                                │ │
│ │ ☑ Highlight critical nets (>10ps delay)                    │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Critical Nets: CLK (45ps), DATA[0] (32ps), VREF (28ps)        │
│                                                                 │
│ [Apply] [Reset] [Export Report]                                │
└─────────────────────────────────────────────────────────────────┘
```

---

## Metal Density Visualization

```rust
DensityVisualization {
    // Density calculation
    calculation: DensityCalculation {
        window_size: (10.0, 10.0),  // um
        step_size: (5.0, 5.0),
        layers: vec!["M1", "M2", "M3", "M4", "M5"],
    },
    
    // Display
    display: DensityDisplay {
        color_scale: ColorScale::RdYlGn,  // Red=low, Green=good
        min_density: 0.0,
        max_density: 1.0,
        target_range: (0.3, 0.7),  // PDK requirements
        show_violations: true,
    },
}
```

---

## Current Density / IR Drop

```rust
CurrentDensityVisualization {
    // Analysis source
    source: AnalysisSource::Voltus,  // or RedHawk, Redhawk-SC
    
    // Display options
    display: CurrentDensityDisplay {
        // IR drop
        ir_drop: IrDropDisplay {
            enabled: true,
            max_drop: 0.05,        // 50mV
            color_scale: ColorScale::Jet,
        },
        
        // EM (electromigration)
        em_analysis: EmDisplay {
            enabled: true,
            show_violations: true,
            current_limit: 1.0,    // mA/um
        },
        
        // Power grid
        power_grid: PowerGridDisplay {
            show_straps: true,
            show_vias: true,
            highlight_weak_spots: true,
        },
    },
}
```

---

## Cross-Section View

```rust
IcCrossSection {
    // Section plane
    plane: SectionPlane {
        axis: Axis::X,
        position: 50.0,            // um from origin
    },
    
    // Display options
    display: CrossSectionDisplay {
        show_layer_names: true,
        show_dimensions: true,
        show_materials: true,
        show_via_connections: true,
        
        // Layer colors (from PDK)
        use_pdk_colors: true,
    },
    
    // Annotations
    annotations: vec![
        Annotation::LayerThickness,
        Annotation::ViaStack,
        Annotation::DielectricConstant,
    ],
}
```

---

## Rust API

```rust
use hardware_tool::ic::viewer::*;

// Create 3D viewer
let viewer = IcViewer3D::new(&layout)?;

// Set view mode
viewer.set_mode(RenderMode::Physical)?;

// Show parasitic overlay
viewer.show_parasitic_overlay(ParasiticConfig {
    source: "extraction.spef",
    show_resistance: true,
    show_capacitance: true,
    highlight_critical: true,
})?;

// Show density map
viewer.show_density_map(DensityConfig {
    layers: vec!["M1", "M2", "M3"],
    window_size: 10.0,
})?;

// Create cross-section
viewer.set_cross_section(CrossSection {
    axis: Axis::Y,
    position: 100.0,
})?;

// Export
viewer.export_image("ic_3d.png", Resolution::UHD)?;
viewer.export_gdsii("layout.gds")?;
```

---

## CLI Commands

```bash
# Open 3D viewer
hwt ic view layout.hwt_ic --3d

# Show parasitic overlay
hwt ic view layout.hwt_ic --3d --overlay parasitic --spef extraction.spef

# Show density map
hwt ic view layout.hwt_ic --3d --overlay density --layers M1,M2,M3

# Export 3D image
hwt ic view layout.hwt_ic --3d --export ic_3d.png

# Cross-section
hwt ic view layout.hwt_ic --3d --section Y:100um
```

---

## Related Topics

- [Shared 3D Viewer Architecture](../../3d-visualization/shared-3d-viewer-architecture.md)
- [Parasitic Extraction](./parasitic-extraction.md)
- [Physical Verification (DRC/LVS)](../analog-mixed-signal/physical-verification-drc-lvs.md)
- [IR Drop Analysis](../advanced-features/ir-drop-analysis.md)
