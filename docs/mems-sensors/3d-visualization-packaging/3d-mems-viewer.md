# 3D MEMS Viewer

## Overview

Hardware Tool provides real-time 3D visualization of MEMS devices, including layer-by-layer fabrication views, release etch simulation, mechanical deformation overlays, and packaging integration previews.

> **Inherits from:** [Shared 3D Viewer Architecture](../../3d-visualization/shared-3d-viewer-architecture.md)
>
> This viewer extends the unified 3D visualization engine with MEMS-specific rendering modes and overlays. All standard navigation, cross-section, measurement, and export capabilities are inherited from the shared architecture.

---

## MEMS-Specific Extensions

## Viewer Features

```rust
MemsViewer3D {
    // Rendering modes
    modes: vec![
        RenderMode::Physical,          // Realistic materials
        RenderMode::LayerStack,        // Exploded process layers
        RenderMode::Released,          // Post-release structure
        RenderMode::Deformation,       // Modal/stress overlay
        RenderMode::Fabrication,       // Step-by-step process
        RenderMode::Packaged,          // With package/cap
    ],
    
    // Layer visibility
    layers: LayerVisibility {
        substrate: true,
        oxide_1: true,
        poly_0: true,
        oxide_2: true,
        poly_1: true,
        poly_2: true,
        metal: true,
        passivation: true,
    },
    
    // Deformation overlay
    deformation_display: DeformationDisplay {
        mode: DeformationMode::Modal,  // or Static, Dynamic
        scale_factor: 100.0,           // Exaggerate for visibility
        color_scale: ColorScale::Jet,
        animate: true,
    },
}
```

## Viewer UI

```
┌─────────────────────────────────────────────────────────────────┐
│ 3D MEMS Viewer: Accelerometer_v3                                │
├─────────────────────────────────────────────────────────────────┤
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │                                                             │ │
│ │              ┌─────────────────────────┐                    │ │
│ │              │     Proof Mass          │                    │ │
│ │    ┌────────┤                         ├────────┐           │ │
│ │    │ Spring │    ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓     │ Spring │           │ │
│ │    │ Beam   │    ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓     │ Beam   │           │ │
│ │    └────────┤                         ├────────┘           │ │
│ │              │   Sense Fingers        │                    │ │
│ │              │   ||||||||||||||||     │                    │ │
│ │              └─────────────────────────┘                    │ │
│ │                                                             │ │
│ │    ════════════════════════════════════════════            │ │
│ │                    Substrate                               │ │
│ │                                                             │ │
│ │    [Process: PolyMUMPs]  [Released: Yes]  [Gap: 2μm]       │ │
│ │                                                             │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ View: [Released ▼]  Overlay: [Deformation ▼]  Mode: [1st ▼]    │
│                                                                 │
│ Layers:                                                         │
│ ☑ Substrate   ☑ Poly0   ☑ Poly1   ☑ Poly2   ☑ Metal          │
│ ☐ Oxide (sacrificial)   ☑ Anchors  ☑ Etch holes              │
│                                                                 │
│ [Rotate] [Pan] [Zoom] [Section] [Animate] [Export STEP]        │
└─────────────────────────────────────────────────────────────────┘
```

---

## Fabrication Process Visualization

### Step-by-Step View

```rust
FabricationVisualization {
    // Process definition
    process: "PolyMUMPs",
    
    // Steps
    steps: vec![
        FabStep { name: "Substrate", layer: "substrate", action: Action::Deposit },
        FabStep { name: "Nitride", layer: "nitride", action: Action::Deposit },
        FabStep { name: "Poly0", layer: "poly0", action: Action::PatternDeposit },
        FabStep { name: "Oxide1", layer: "oxide1", action: Action::Deposit },
        FabStep { name: "Oxide1 Pattern", layer: "oxide1", action: Action::Etch },
        FabStep { name: "Poly1", layer: "poly1", action: Action::PatternDeposit },
        FabStep { name: "Oxide2", layer: "oxide2", action: Action::Deposit },
        FabStep { name: "Poly2", layer: "poly2", action: Action::PatternDeposit },
        FabStep { name: "Metal", layer: "metal", action: Action::PatternDeposit },
        FabStep { name: "Release", layer: "oxide*", action: Action::Release },
    ],
    
    // Animation
    animation: FabAnimation {
        enabled: true,
        step_duration: 1.0,  // seconds
        auto_play: false,
    },
}
```

### Fabrication UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Fabrication Process View                                 [✕]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Process: PolyMUMPs                                             │
│                                                                 │
│ Step 6 of 10: Poly1 Deposition                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │                                                             │ │
│ │    ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓  ← Poly1 (2μm)          │ │
│ │    ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  ← Oxide1 (sacrificial)  │ │
│ │    ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓  ← Poly0 (0.5μm)        │ │
│ │    ════════════════════════════════  ← Nitride              │ │
│ │    ████████████████████████████████  ← Substrate            │ │
│ │                                                             │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [|◀] [◀] Step: [6____] [▶] [▶|] [▶ Play]                      │
│                                                                 │
│ Layer Properties:                                               │
│ • Poly1: 2.0 μm thick, phosphorus-doped polysilicon           │
│ • Deposition: LPCVD @ 610°C                                    │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Release Etch Simulation

```rust
ReleaseEtchSimulation {
    // Etchant
    etchant: Etchant::HF49,
    
    // Etch parameters
    parameters: EtchParameters {
        etch_rate: 1.5,            // μm/min for oxide
        selectivity_poly: 1000.0,  // Poly etch rate = oxide/1000
        selectivity_nitride: 50.0,
        temperature: 25.0,
    },
    
    // Simulation
    simulation: EtchSimulation {
        time_steps: 100,
        max_time: 30.0,            // minutes
        show_undercut: true,
        show_stiction_risk: true,
    },
}
```

---

## Deformation Visualization

### Modal Analysis Overlay

```rust
ModalVisualization {
    // Mode selection
    mode_number: 1,                // 1st mode (fundamental)
    
    // Display
    display: ModalDisplay {
        deformation_scale: 100.0,  // Exaggerate for visibility
        color_by: ColorBy::Displacement,
        color_scale: ColorScale::Jet,
        show_undeformed: true,     // Ghost outline
        animate: true,
        animation_speed: 1.0,
    },
    
    // Mode info
    mode_info: ModeInfo {
        frequency: 12500.0,        // Hz
        effective_mass: 1.2e-9,    // kg
        quality_factor: 5000.0,
    },
}
```

### Static Deformation

```rust
StaticDeformationVisualization {
    // Load case
    load: LoadCase::Gravity { direction: Axis::Z, magnitude: 9.81 },
    
    // Display
    display: StaticDisplay {
        deformation_scale: 1000.0,
        color_by: ColorBy::VonMisesStress,
        color_scale: ColorScale::Hot,
        max_stress: 100e6,         // Pa
        show_stress_concentrations: true,
    },
}
```

---

## Packaging Integration

```rust
PackagingVisualization {
    // Package type
    package: PackageType::LCC,
    
    // Components
    components: PackageComponents {
        die: true,
        wire_bonds: true,
        cap: true,
        cavity: true,
        getter: true,
    },
    
    // Display options
    display: PackageDisplay {
        show_die_attach: true,
        show_wire_bonds: true,
        show_hermetic_seal: true,
        transparency: 0.3,         // For cap
    },
}
```

### Packaged View UI

```
┌─────────────────────────────────────────────────────────────────┐
│ 3D Packaged View                                         [✕]   │
├─────────────────────────────────────────────────────────────────┤
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │                                                             │ │
│ │         ┌─────────────────────────────────┐                │ │
│ │         │          Glass Cap              │ (transparent)  │ │
│ │         │    ┌───────────────────┐        │                │ │
│ │         │    │   MEMS Device     │        │                │ │
│ │         │    │   ╔═══════════╗   │        │                │ │
│ │         │    │   ║ Proof Mass║   │        │                │ │
│ │         │    │   ╚═══════════╝   │        │                │ │
│ │         │    └───────────────────┘        │                │ │
│ │         │  ∿∿∿  Wire Bonds  ∿∿∿          │                │ │
│ │         └─────────────────────────────────┘                │ │
│ │         ════════════════════════════════════               │ │
│ │                   LCC Package                              │ │
│ │         ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ●                   │ │
│ │                   Solder Pads                              │ │
│ │                                                             │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Package: [LCC-24 ▼]  Cap: [Glass ▼]  Cavity: [Vacuum ▼]        │
│                                                                 │
│ ☑ Die   ☑ Wire bonds   ☑ Cap (30% transparent)   ☑ Getter    │
│                                                                 │
│ [Explode View] [Cross-Section] [Export STEP]                   │
└─────────────────────────────────────────────────────────────────┘
```

---

## Rust API

```rust
use hardware_tool::mems::viewer::*;

// Create 3D viewer
let viewer = MemsViewer3D::new(&device)?;

// Set view mode
viewer.set_mode(RenderMode::Released)?;

// Show deformation overlay
viewer.show_deformation(DeformationConfig {
    mode: DeformationMode::Modal { mode_number: 1 },
    scale_factor: 100.0,
    animate: true,
})?;

// Show fabrication process
viewer.show_fabrication_process(FabricationConfig {
    process: "PolyMUMPs",
    step: 6,
})?;

// Show packaged view
viewer.show_packaged(PackageConfig {
    package_type: PackageType::LCC,
    cap_transparency: 0.3,
})?;

// Export
viewer.export_image("mems_3d.png", Resolution::UHD)?;
viewer.export_step("mems_device.step")?;
```

---

## CLI Commands

```bash
# Open 3D viewer
hwt mems view device.hwt_mems --3d

# Show released structure
hwt mems view device.hwt_mems --3d --mode released

# Show deformation
hwt mems view device.hwt_mems --3d --overlay deformation --mode 1

# Show fabrication
hwt mems view device.hwt_mems --3d --fabrication --step 6

# Export
hwt mems view device.hwt_mems --3d --export mems_3d.step
```

---

## Related Topics

- [Shared 3D Viewer Architecture](../../3d-visualization/shared-3d-viewer-architecture.md)
- [MEMS Packaging Integration](./mems-packaging-integration.md)
- [MEMS Design Rule Check](../layout-concepts/mems-design-rule-check.md)
- [Resonance & Sensitivity Calculators](../advanced-features/resonance-and-sensitivity-calculators.md)
