# Photonic Waveguide Design

## Overview

Hardware Tool provides comprehensive photonic waveguide design capabilities for silicon photonics, silicon nitride, and III-V platforms. Design single-mode waveguides, bends, couplers, and complex photonic circuits with accurate mode analysis and loss estimation.

> **Inherits from:** [Shared 3D Viewer Architecture](../../3d-visualization/shared-3d-viewer-architecture.md)
>
> This documentation covers photonic-specific waveguide design. All standard visualization, export, and measurement capabilities are inherited from the shared architecture.

---

## Waveguide Types

| Type | Platform | Wavelength | Loss |
|------|----------|------------|------|
| **Strip** | Silicon | 1550 nm | 2-3 dB/cm |
| **Rib** | Silicon | 1550 nm | 0.5-1 dB/cm |
| **Slot** | Silicon | 1550 nm | 5-10 dB/cm |
| **Channel** | SiN | 1550 nm | 0.1-0.5 dB/cm |
| **Ridge** | InP | 1550 nm | 1-2 dB/cm |

---

## Waveguide Configuration

```rust
WaveguideDesign {
    // Platform
    platform: PhotonicPlatform::SiliconPhotonics,
    
    // Waveguide geometry
    geometry: WaveguideGeometry::Strip {
        width: 500e-9,             // 500 nm
        height: 220e-9,            // 220 nm
        slab_height: 0.0,          // No slab (strip)
    },
    
    // Materials
    materials: WaveguideMaterials {
        core: Material::Silicon,
        cladding: Material::SiO2,
        substrate: Material::SiO2,
    },
    
    // Operating wavelength
    wavelength: 1550e-9,           // 1550 nm (C-band)
    
    // Mode
    mode: WaveguideMode::TE0,      // Fundamental TE mode
}
```

---

## Waveguide Design UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Photonic Waveguide Design                                [✕]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Platform: [Silicon Photonics ▼]  Wavelength: [1550 nm___]      │
│                                                                 │
│ Geometry:                                                       │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Type: [Strip ▼]                                            │ │
│ │                                                             │ │
│ │              ┌─────────────┐                               │ │
│ │              │   Core      │ ← Width: [500 nm]             │ │
│ │              │   (Si)      │ ← Height: [220 nm]            │ │
│ │ ─────────────┴─────────────┴─────────────                  │ │
│ │           Buried Oxide (SiO2)                              │ │
│ │ ═══════════════════════════════════════                    │ │
│ │              Substrate (Si)                                │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Mode Analysis:                                                  │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Mode    │ neff    │ ng      │ Loss (dB/cm) │ Confinement  │ │
│ │ ────────┼─────────┼─────────┼──────────────┼───────────── │ │
│ │ TE0     │ 2.4521  │ 4.2156  │ 2.5          │ 85.2%        │ │
│ │ TM0     │ 1.8234  │ 3.8921  │ 3.2          │ 72.1%        │ │
│ │ TE1     │ cutoff  │ --      │ --           │ --           │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [Calculate Modes] [Optimize Width] [View Mode Profile]         │
└─────────────────────────────────────────────────────────────────┘
```

---

## Mode Analysis

### Mode Solver

```rust
ModeSolver {
    // Solver type
    solver: SolverType::FDE,       // Finite Difference Eigenmode
    
    // Mesh
    mesh: ModeMesh {
        x_resolution: 10e-9,       // 10 nm
        y_resolution: 10e-9,
        boundary_distance: 2e-6,   // 2 μm from core
    },
    
    // Search
    search: ModeSearch {
        n_modes: 4,
        polarization: Polarization::Both,
        n_eff_range: (1.5, 3.5),
    },
    
    // Output
    output: ModeOutput {
        field_profiles: true,
        effective_index: true,
        group_index: true,
        dispersion: true,
        loss: true,
    },
}
```

### Mode Profile Visualization

```
┌─────────────────────────────────────────────────────────────────┐
│ Mode Profile: TE0                                        [✕]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │                                                             │ │
│ │                    ░░░░░░░░░░░                             │ │
│ │                  ░░░░░░░░░░░░░░░                           │ │
│ │                ░░░░░▓▓▓▓▓▓▓░░░░░                          │ │
│ │               ░░░░▓▓▓▓▓▓▓▓▓▓░░░░                          │ │
│ │              ░░░░▓▓▓▓████▓▓▓░░░░                          │ │
│ │              ░░░▓▓▓▓██████▓▓▓░░░                          │ │
│ │              ░░░▓▓▓▓██████▓▓▓░░░  ← Core                  │ │
│ │ ─────────────░░░▓▓▓▓██████▓▓▓░░░─────────────             │ │
│ │              ░░░░░░░░░░░░░░░░░░░                          │ │
│ │                                                             │ │
│ │ █ = Max field    ▓ = Medium    ░ = Low                    │ │
│ │                                                             │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ neff = 2.4521    ng = 4.2156    Aeff = 0.12 μm²               │
│ Confinement = 85.2%    Loss = 2.5 dB/cm                        │
│                                                                 │
│ [Ex] [Ey] [Ez] [|E|] [Hx] [Hy] [Hz] [Poynting]                │
└─────────────────────────────────────────────────────────────────┘
```

---

## Waveguide Components

### Bends

```rust
WaveguideBend {
    // Bend type
    bend_type: BendType::Euler,    // or Circular, Bezier
    
    // Parameters
    radius: 5e-6,                  // 5 μm minimum radius
    angle: 90.0,                   // degrees
    
    // Optimization
    optimize: BendOptimization {
        target: OptTarget::MinLoss,
        min_radius: 3e-6,
        max_length: 20e-6,
    },
}
```

### Couplers

```rust
DirectionalCoupler {
    // Geometry
    gap: 200e-9,                   // 200 nm gap
    coupling_length: 10e-6,        // 10 μm
    
    // Target
    target_coupling: 0.5,          // 50% power transfer (3dB)
    
    // Wavelength
    wavelength: 1550e-9,
    bandwidth: 40e-9,              // C-band
}
```

### Ring Resonators

```rust
RingResonator {
    // Ring geometry
    radius: 10e-6,                 // 10 μm
    
    // Coupling
    bus_coupling: CouplingConfig {
        gap: 150e-9,
        coupling_length: 5e-6,
    },
    
    // Target
    target_fsr: 10e9,              // 10 GHz FSR
    target_q: 50000,               // Quality factor
    
    // Type
    resonator_type: ResonatorType::AllPass,  // or AddDrop
}
```

---

## Component Library

```
┌─────────────────────────────────────────────────────────────────┐
│ Photonic Component Library                               [✕]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Platform: [Silicon Photonics ▼]  PDK: [SiEPIC ▼]              │
│                                                                 │
│ Waveguides:                                                     │
│ ├─ Strip Waveguide (500nm × 220nm)                             │
│ ├─ Rib Waveguide (500nm × 220nm, 90nm slab)                    │
│ └─ Slot Waveguide (200nm slots)                                │
│                                                                 │
│ Bends:                                                          │
│ ├─ Euler Bend (5μm radius, 0.02 dB loss)                       │
│ ├─ Circular Bend (10μm radius, 0.01 dB loss)                   │
│ └─ S-Bend (various offsets)                                    │
│                                                                 │
│ Couplers:                                                       │
│ ├─ Directional Coupler (3dB, 10dB, 20dB)                       │
│ ├─ MMI 1×2 (3dB splitter)                                      │
│ ├─ MMI 2×2 (3dB coupler)                                       │
│ ├─ Y-Junction                                                   │
│ └─ Grating Coupler (TE, TM)                                    │
│                                                                 │
│ Resonators:                                                     │
│ ├─ Ring Resonator (various radii)                              │
│ ├─ Racetrack Resonator                                         │
│ └─ Disk Resonator                                              │
│                                                                 │
│ Active:                                                         │
│ ├─ Phase Shifter (thermo-optic)                                │
│ ├─ Modulator (carrier depletion)                               │
│ └─ Photodetector (Ge-on-Si)                                    │
│                                                                 │
│ [Insert Component] [View Details] [Create Custom]              │
└─────────────────────────────────────────────────────────────────┘
```

---

## Loss Budget

```rust
LossBudget {
    // Components
    components: vec![
        LossComponent::GratingCoupler { loss: 3.0 },      // dB
        LossComponent::Waveguide { length: 1e-3, loss_per_cm: 2.5 },
        LossComponent::Bend { count: 4, loss_each: 0.02 },
        LossComponent::Coupler { loss: 0.1 },
        LossComponent::Ring { loss: 0.5 },
        LossComponent::GratingCoupler { loss: 3.0 },
    ],
    
    // Calculation
    total_loss: 7.33,              // dB
    
    // Budget
    budget: 10.0,                  // dB max allowed
    margin: 2.67,                  // dB remaining
}
```

### Loss Budget UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Optical Loss Budget                                      [✕]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Path: Input → Ring Filter → Output                             │
│                                                                 │
│ Component Losses:                                               │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Component              │ Loss (dB) │ Cumulative            │ │
│ │ ───────────────────────┼───────────┼────────────────────── │ │
│ │ Input Grating Coupler  │ 3.00      │ ████████████░░ 3.00  │ │
│ │ Waveguide (500 μm)     │ 0.13      │ ████████████░░ 3.13  │ │
│ │ Euler Bend ×2          │ 0.04      │ ████████████░░ 3.17  │ │
│ │ Directional Coupler    │ 0.10      │ ████████████░░ 3.27  │ │
│ │ Ring Resonator         │ 0.50      │ █████████████░ 3.77  │ │
│ │ Directional Coupler    │ 0.10      │ █████████████░ 3.87  │ │
│ │ Euler Bend ×2          │ 0.04      │ █████████████░ 3.91  │ │
│ │ Waveguide (500 μm)     │ 0.13      │ █████████████░ 4.04  │ │
│ │ Output Grating Coupler │ 3.00      │ ██████████████ 7.04  │ │
│ │ ───────────────────────┼───────────┼────────────────────── │ │
│ │ Total                  │ 7.04 dB   │ Budget: 10 dB ✓      │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Margin: 2.96 dB                                                │
│                                                                 │
│ [Optimize Path] [Export Report]                                │
└─────────────────────────────────────────────────────────────────┘
```

---

## CLI Commands

```bash
# Mode analysis
hwt photonic mode waveguide.hwt_rf --width 500nm --height 220nm

# Optimize waveguide
hwt photonic optimize waveguide.hwt_rf --target single-mode

# Loss budget
hwt photonic loss-budget circuit.hwt_rf

# Component library
hwt photonic library list --platform silicon

# Export
hwt photonic export circuit.hwt_rf --format gdsii
```

---

## Rust API

```rust
use hardware_tool::photonics::*;

// Create waveguide
let wg = Waveguide::strip(500e-9, 220e-9)?;

// Solve modes
let modes = wg.solve_modes(ModeSolverConfig {
    wavelength: 1550e-9,
    n_modes: 4,
})?;

// Get TE0 mode
let te0 = modes.get(Mode::TE0)?;
println!("neff = {:.4}, ng = {:.4}", te0.n_eff, te0.n_group);

// Create ring resonator
let ring = RingResonator::new(10e-6, 150e-9)?;
let spectrum = ring.transmission_spectrum(1500e-9, 1600e-9, 1001)?;

// Calculate loss budget
let budget = circuit.loss_budget()?;
println!("Total loss: {:.2} dB", budget.total_loss);

// Export
circuit.export_gdsii("photonic_circuit.gds")?;
```

---

## Related Topics

- [3D RF Viewer](../3d-visualization-em/3d-rf-viewer.md)
- [EM Field Simulation](../3d-visualization-em/em-field-simulation.md)
- [Microstrip/Coplanar Routing](./microstrip-coplanar-routing.md)
- [S-Parameter Data Generation](../manufacturing-output/s-parameter-data-generation.md)
