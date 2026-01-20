# 3D RF Viewer

## Overview

Hardware Tool provides real-time 3D visualization of RF/microwave circuits and photonic devices, including layer-by-layer views, electromagnetic field distributions, S-parameter overlays, and antenna radiation patterns.

> **Inherits from:** [Shared 3D Viewer Architecture](../../3d-visualization/shared-3d-viewer-architecture.md)
>
> This viewer extends the unified 3D visualization engine with RF-specific rendering modes and overlays. All standard navigation, cross-section, measurement, and export capabilities are inherited from the shared architecture.

---

## RF-Specific Extensions

## Viewer Features

```rust
RfViewer3D {
    // Rendering modes
    modes: vec![
        RenderMode::Physical,          // Realistic materials
        RenderMode::LayerStack,        // Exploded substrate layers
        RenderMode::EMField,           // E/H field overlay
        RenderMode::CurrentDensity,    // Surface current
        RenderMode::SParameter,        // S-parameter visualization
        RenderMode::Radiation,         // Antenna pattern
    ],
    
    // Layer visibility
    layers: LayerVisibility {
        substrate: true,
        ground_plane: true,
        signal_traces: true,
        vias: true,
        components: true,
        ports: true,
    },
    
    // EM field display
    em_display: EmFieldDisplay {
        field_type: FieldType::Electric,
        frequency: 2.4e9,
        plane: Plane::XY { z: 0.0 },
        scale: ColorScale::Jet,
        animate: true,
    },
}
```

## Viewer UI

```
┌─────────────────────────────────────────────────────────────────┐
│ 3D RF Viewer: LNA_2.4GHz                                        │
├─────────────────────────────────────────────────────────────────┤
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │                                                             │ │
│ │    Port 1                                         Port 2   │ │
│ │    ●━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━●     │ │
│ │         │                                       │          │ │
│ │    ┌────┴────┐    ┌─────────┐    ┌────────┐    │          │ │
│ │    │ Matching│    │   LNA   │    │ Output │    │          │ │
│ │    │ Network │────│  Stage  │────│ Match  │────┘          │ │
│ │    └─────────┘    └─────────┘    └────────┘               │ │
│ │                                                             │ │
│ │    ════════════════════════════════════════════            │ │
│ │              Rogers 4350B (εr=3.48)                        │ │
│ │    ════════════════════════════════════════════            │ │
│ │                   Ground Plane                             │ │
│ │                                                             │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ View: [Physical ▼]  Overlay: [E-Field ▼]  Freq: [2.4 GHz]      │
│                                                                 │
│ Layers:                                                         │
│ ☑ Substrate   ☑ Ground   ☑ Traces   ☑ Vias   ☑ Components    │
│ ☐ E-field     ☐ H-field  ☐ Current density                    │
│                                                                 │
│ [Rotate] [Pan] [Zoom] [Section] [Animate] [Export]             │
└─────────────────────────────────────────────────────────────────┘
```

---

## Electromagnetic Field Visualization

### E-Field / H-Field Overlay

```rust
EmFieldVisualization {
    // Field configuration
    field: FieldConfig {
        field_type: FieldType::Electric,  // or Magnetic
        component: Component::Magnitude,   // or Ex, Ey, Ez
        frequency: 2.4e9,
    },
    
    // Visualization plane
    plane: VisualizationPlane {
        type_: PlaneType::XY,
        position: 0.0,                     // z = 0 (substrate surface)
        resolution: 200,                   // grid points
    },
    
    // Display settings
    display: FieldDisplaySettings {
        color_scale: ColorScale::Jet,
        min_value: 0.0,
        max_value: 1000.0,                 // V/m
        log_scale: true,
        dynamic_range: 40.0,               // dB
        opacity: 0.7,
    },
    
    // Animation
    animation: FieldAnimation {
        enabled: true,
        phase_sweep: true,
        frames_per_cycle: 30,
    },
}
```

### Field Visualization UI

```
┌─────────────────────────────────────────────────────────────────┐
│ EM Field Visualization                                   [✕]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Field Type: ● Electric  ○ Magnetic  ○ Poynting                │
│                                                                 │
│ Component: [Magnitude ▼]  Frequency: [2.4 GHz___]              │
│                                                                 │
│ Visualization Plane:                                            │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Plane: [XY ▼]  Position Z: [0.0 mm___]                     │ │
│ │ Resolution: [200 × 200]                                     │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Display:                                                        │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Scale: [Jet ▼]  ☑ Log scale  Range: [40 dB]                │ │
│ │ Opacity: [70%____]                                          │ │
│ │                                                             │ │
│ │ ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓                  │ │
│ │ 0 dB                                          -40 dB       │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Animation: [▶ Play]  Speed: [1.0x ▼]                           │
│                                                                 │
│ [Apply] [Reset] [Export Animation]                             │
└─────────────────────────────────────────────────────────────────┘
```

---

## Surface Current Visualization

```rust
SurfaceCurrentVisualization {
    // Current configuration
    current: CurrentConfig {
        frequency: 2.4e9,
        port_excitation: vec![
            PortExcitation { port: 1, magnitude: 1.0, phase: 0.0 },
        ],
    },
    
    // Display
    display: CurrentDisplay {
        show_magnitude: true,
        show_vectors: true,
        vector_density: 20,
        color_scale: ColorScale::Hot,
        log_scale: true,
    },
    
    // Highlighting
    highlight: CurrentHighlight {
        show_hotspots: true,
        threshold: 0.8,                    // 80% of max
        show_current_paths: true,
    },
}
```

---

## S-Parameter Visualization

```rust
SParameterVisualization {
    // S-parameter data
    data: SParameterData {
        source: "simulation.s2p",
        frequency_range: (1e9, 6e9),
    },
    
    // 3D overlay
    overlay: SParamOverlay {
        parameter: SParam::S21,            // Transmission
        frequency: 2.4e9,
        show_magnitude: true,
        show_phase: false,
        
        // Color mapping
        color_by: ColorBy::Magnitude,
        good_threshold: -1.0,              // dB
        bad_threshold: -3.0,
    },
    
    // Port visualization
    ports: PortVisualization {
        show_port_labels: true,
        show_reference_planes: true,
        show_impedance: true,
    },
}
```

### S-Parameter Overlay UI

```
┌─────────────────────────────────────────────────────────────────┐
│ S-Parameter Overlay                                      [✕]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Parameter: [S21 ▼]  Frequency: [2.4 GHz___]                    │
│                                                                 │
│ At 2.4 GHz:                                                    │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ S11: -15.2 dB (Return Loss)                    ✓ Good      │ │
│ │ S21: -0.8 dB (Insertion Loss)                  ✓ Good      │ │
│ │ S12: -25.3 dB (Isolation)                      ✓ Good      │ │
│ │ S22: -18.1 dB (Output Match)                   ✓ Good      │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Overlay Options:                                                │
│ ☑ Show magnitude on traces                                     │
│ ☐ Show phase                                                   │
│ ☑ Highlight problem areas (>-10 dB return loss)               │
│                                                                 │
│ [Apply] [Show Smith Chart] [Export Touchstone]                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Antenna Radiation Pattern

```rust
RadiationPatternVisualization {
    // Pattern configuration
    pattern: PatternConfig {
        frequency: 2.4e9,
        pattern_type: PatternType::Gain,   // or Directivity, EIRP
        polarization: Polarization::Total, // or Theta, Phi, RHCP, LHCP
    },
    
    // 3D display
    display: PatternDisplay {
        style: PatternStyle::Surface3D,    // or Polar2D, Cartesian
        color_scale: ColorScale::Jet,
        min_gain: -30.0,                   // dBi
        max_gain: 10.0,
        show_main_lobe: true,
        show_side_lobes: true,
        show_back_lobe: true,
    },
    
    // Annotations
    annotations: PatternAnnotations {
        show_peak_gain: true,
        show_beamwidth: true,
        show_front_to_back: true,
        show_null_directions: true,
    },
}
```

### Radiation Pattern UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Antenna Radiation Pattern                                [✕]   │
├─────────────────────────────────────────────────────────────────┤
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │                                                             │ │
│ │                         ╱╲                                  │ │
│ │                        ╱  ╲                                 │ │
│ │                       ╱    ╲  Main Lobe                    │ │
│ │                      ╱      ╲                               │ │
│ │                     ╱   ●    ╲                              │ │
│ │            ────────╱──Antenna──╲────────                    │ │
│ │                   ╲            ╱                            │ │
│ │                    ╲   Side   ╱                             │ │
│ │                     ╲  Lobes ╱                              │ │
│ │                      ╲      ╱                               │ │
│ │                       ╲    ╱                                │ │
│ │                        Back                                 │ │
│ │                                                             │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Frequency: [2.4 GHz]  Pattern: [Gain ▼]  Pol: [Total ▼]        │
│                                                                 │
│ Results:                                                        │
│ • Peak Gain: 6.2 dBi @ θ=0°, φ=0°                              │
│ • 3dB Beamwidth: 65° (E-plane), 72° (H-plane)                  │
│ • Front-to-Back: 18.5 dB                                       │
│ • Efficiency: 92%                                              │
│                                                                 │
│ [2D Polar] [3D Surface] [Export Pattern]                       │
└─────────────────────────────────────────────────────────────────┘
```

---

## Photonic Mode Visualization

```rust
PhotonicModeVisualization {
    // Waveguide mode
    mode: WaveguideMode {
        mode_type: ModeType::TE,
        mode_order: (0, 0),                // TE00
        wavelength: 1550e-9,               // 1550 nm
    },
    
    // Display
    display: ModeDisplay {
        field_component: Component::Ey,
        cross_section: CrossSection::XY,
        color_scale: ColorScale::RdBu,     // Diverging for +/-
        show_effective_index: true,
        show_mode_area: true,
    },
    
    // Propagation
    propagation: PropagationDisplay {
        show_propagation: true,
        length: 100e-6,                    // 100 μm
        animate: true,
    },
}
```

---

## Rust API

```rust
use hardware_tool::rf::viewer::*;

// Create 3D viewer
let viewer = RfViewer3D::new(&circuit)?;

// Set view mode
viewer.set_mode(RenderMode::Physical)?;

// Show E-field overlay
viewer.show_em_field(EmFieldConfig {
    field_type: FieldType::Electric,
    frequency: 2.4e9,
    plane: Plane::XY { z: 0.0 },
    animate: true,
})?;

// Show surface current
viewer.show_surface_current(CurrentConfig {
    frequency: 2.4e9,
    port: 1,
})?;

// Show radiation pattern
viewer.show_radiation_pattern(PatternConfig {
    frequency: 2.4e9,
    style: PatternStyle::Surface3D,
})?;

// Export
viewer.export_image("rf_3d.png", Resolution::UHD)?;
viewer.export_step("rf_circuit.step")?;
```

---

## CLI Commands

```bash
# Open 3D viewer
hwt rf view circuit.hwt_rf --3d

# Show E-field
hwt rf view circuit.hwt_rf --3d --overlay efield --freq 2.4GHz

# Show surface current
hwt rf view circuit.hwt_rf --3d --overlay current --freq 2.4GHz

# Show radiation pattern
hwt rf view antenna.hwt_rf --3d --pattern --freq 2.4GHz

# Export
hwt rf view circuit.hwt_rf --3d --export rf_3d.png
```

---

## Related Topics

- [Shared 3D Viewer Architecture](../../3d-visualization/shared-3d-viewer-architecture.md)
- [EM Field Simulation](./em-field-simulation.md)
- [S-Parameter Analysis](../manufacturing-output/s-parameter-data-generation.md)
- [Electromagnetic Simulation](../../advanced-features/electromagnetic-simulation.md)
