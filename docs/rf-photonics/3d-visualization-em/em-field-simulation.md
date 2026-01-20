# EM Field Simulation

## Overview

Hardware Tool provides integrated electromagnetic field simulation for RF/microwave circuits and photonic devices, supporting FDTD, FEM, and MoM solvers for accurate S-parameter extraction, field distribution analysis, and radiation pattern computation.

> **Inherits from:** [Shared Simulation Architecture](../../advanced-features/shared-simulation-architecture.md)
>
> This documentation covers RF-specific EM simulation. All standard job management, results visualization, and export capabilities are inherited from the shared architecture.

---

## Solver Types

| Solver | Method | Best For |
|--------|--------|----------|
| **FDTD** | Finite-Difference Time-Domain | Broadband, transient, antennas |
| **FEM** | Finite Element Method | Complex geometries, resonators |
| **MoM** | Method of Moments | Planar structures, antennas |
| **Hybrid** | Combined methods | Multi-scale problems |

---

## Simulation Configuration

```rust
EmSimulation {
    // Solver selection
    solver: Solver::FDTD,
    
    // Frequency range
    frequency: FrequencyConfig {
        start: 1e9,
        stop: 6e9,
        points: 501,
        adaptive: true,
    },
    
    // Mesh settings
    mesh: MeshConfig {
        type_: MeshType::Adaptive,
        min_size: 0.01,            // mm
        max_size: 1.0,
        cells_per_wavelength: 20,
        refinement_regions: vec![
            RefinementRegion::NearPorts { factor: 2.0 },
            RefinementRegion::NearVias { factor: 1.5 },
        ],
    },
    
    // Boundary conditions
    boundaries: BoundaryConditions {
        type_: BoundaryType::PML,  // Perfectly Matched Layer
        layers: 8,
        distance: 0.25,            // wavelengths
    },
    
    // Ports
    ports: vec![
        Port { number: 1, type_: PortType::Lumped, impedance: 50.0 },
        Port { number: 2, type_: PortType::Lumped, impedance: 50.0 },
    ],
}
```

---

## Simulation UI

```
┌─────────────────────────────────────────────────────────────────┐
│ EM Simulation Setup                                      [✕]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Circuit: LNA_2.4GHz.hwt_rf                                     │
│                                                                 │
│ Solver: ● FDTD  ○ FEM  ○ MoM  ○ Hybrid                        │
│                                                                 │
│ Frequency:                                                      │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Start: [1.0 GHz___]  Stop: [6.0 GHz___]  Points: [501__]   │ │
│ │ ☑ Adaptive frequency sampling                              │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Mesh:                                                           │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Type: [Adaptive ▼]  Cells/λ: [20___]                       │ │
│ │ Min size: [0.01 mm]  Max size: [1.0 mm]                    │ │
│ │ ☑ Refine near ports  ☑ Refine near vias                   │ │
│ │ Estimated cells: 1.2M                                       │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Boundaries:                                                     │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Type: [PML ▼]  Layers: [8___]  Distance: [0.25 λ]          │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Ports: 2 defined  [Configure Ports...]                         │
│                                                                 │
│ Estimated Time: ~15 minutes (GPU accelerated)                  │
│                                                                 │
│ [Validate] [Run Simulation] [Cancel]                           │
└─────────────────────────────────────────────────────────────────┘
```

---

## Port Configuration

```rust
PortConfig {
    // Port definition
    ports: vec![
        Port {
            number: 1,
            name: "Input",
            type_: PortType::Lumped,
            impedance: Complex::new(50.0, 0.0),
            position: (0.0, 0.0, 0.0),
            orientation: Axis::X,
            width: 0.5,            // mm
            
            // De-embedding
            deembed: Some(DeembedConfig {
                length: 2.0,       // mm
                reference_plane: ReferencePlane::PortEdge,
            }),
        },
        Port {
            number: 2,
            name: "Output",
            type_: PortType::WavePort,
            impedance: Complex::new(50.0, 0.0),
            modes: 1,
        },
    ],
}
```

---

## Simulation Results

### S-Parameters

```
┌─────────────────────────────────────────────────────────────────┐
│ EM Simulation Results                                    [✕]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Simulation completed in 12m 34s                                │
│ Mesh cells: 1,245,678  |  Memory: 4.2 GB                       │
│                                                                 │
│ S-Parameters:                                                   │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │     │                                                       │ │
│ │   0 │────────────────────────────────── S21                │ │
│ │     │                                                       │ │
│ │ -10 │                                                       │ │
│ │     │                                                       │ │
│ │ -20 │──────────────────────────────────────────── S11      │ │
│ │     │                                                       │ │
│ │ -30 │                                                       │ │
│ │     └───────────────────────────────────────────────────    │ │
│ │       1        2        3        4        5        6 GHz   │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ At 2.4 GHz:                                                    │
│ • S11: -18.5 dB  (VSWR: 1.27)                                  │
│ • S21: -0.45 dB                                                │
│ • S12: -0.45 dB                                                │
│ • S22: -22.1 dB  (VSWR: 1.17)                                  │
│                                                                 │
│ [View Fields] [Smith Chart] [Export S2P] [3D View]             │
└─────────────────────────────────────────────────────────────────┘
```

### Field Distribution

```rust
FieldResults {
    // Available field data
    fields: vec![
        FieldData::Electric { component: Component::All },
        FieldData::Magnetic { component: Component::All },
        FieldData::SurfaceCurrent,
        FieldData::PowerFlow,
    ],
    
    // Frequency points
    frequencies: vec![1e9, 2e9, 2.4e9, 3e9, 4e9, 5e9, 6e9],
    
    // Export options
    export: FieldExport {
        formats: vec![
            ExportFormat::VTK,     // ParaView
            ExportFormat::HDF5,    // Large datasets
            ExportFormat::CSV,     // Simple analysis
        ],
    },
}
```

---

## Antenna Analysis

```rust
AntennaAnalysis {
    // Far-field computation
    far_field: FarFieldConfig {
        theta_range: (-180.0, 180.0),
        phi_range: (0.0, 360.0),
        resolution: 1.0,           // degrees
    },
    
    // Pattern parameters
    pattern: PatternAnalysis {
        compute_gain: true,
        compute_directivity: true,
        compute_efficiency: true,
        compute_axial_ratio: true,
    },
    
    // Polarization
    polarization: PolarizationAnalysis {
        compute_co_pol: true,
        compute_cross_pol: true,
        reference: PolarizationRef::Ludwig3,
    },
}
```

---

## GPU Acceleration

```rust
GpuAcceleration {
    // GPU configuration
    enabled: true,
    device: GpuDevice::Auto,       // or Specific(0)
    
    // Memory management
    memory: GpuMemory {
        max_usage: 0.8,            // 80% of VRAM
        out_of_core: true,         // Swap to system RAM if needed
    },
    
    // Performance
    performance: GpuPerformance {
        precision: Precision::Single,  // or Double
        async_compute: true,
    },
}
```

---

## Convergence Settings

```rust
ConvergenceSettings {
    // FDTD convergence
    fdtd: FdtdConvergence {
        energy_decay: -30.0,       // dB
        max_timesteps: 100000,
        steady_state_check: true,
    },
    
    // FEM convergence
    fem: FemConvergence {
        delta_s: 0.01,             // S-parameter change
        max_passes: 10,
        min_passes: 2,
    },
    
    // Adaptive meshing
    adaptive: AdaptiveConvergence {
        enabled: true,
        target_delta_s: 0.02,
        max_refinements: 5,
    },
}
```

---

## Export Formats

| Format | Extension | Description |
|--------|-----------|-------------|
| **Touchstone** | `.s2p`, `.s4p` | S-parameters |
| **MDIF** | `.mdf` | Measured data |
| **CITI** | `.cti` | Common format |
| **VTK** | `.vtk` | Field visualization |
| **HDF5** | `.h5` | Large field data |

---

## CLI Commands

```bash
# Run EM simulation
hwt rf simulate circuit.hwt_rf --solver fdtd --freq 1G:6G:501

# Configure mesh
hwt rf simulate circuit.hwt_rf --mesh adaptive --cells-per-lambda 20

# GPU acceleration
hwt rf simulate circuit.hwt_rf --gpu

# Export results
hwt rf simulate circuit.hwt_rf --export results.s2p

# View fields
hwt rf simulate circuit.hwt_rf --export-fields fields.vtk
```

---

## Rust API

```rust
use hardware_tool::rf::simulation::*;

// Configure simulation
let config = EmSimulationConfig {
    solver: Solver::FDTD,
    frequency: FrequencyRange::new(1e9, 6e9, 501),
    mesh: MeshConfig::adaptive(20),  // 20 cells/wavelength
    gpu: true,
};

// Run simulation
let results = circuit.simulate_em(config)?;

// Get S-parameters
let s21 = results.s_parameter(2, 1, 2.4e9)?;
println!("S21 at 2.4 GHz: {:.2} dB", s21.magnitude_db());

// Export Touchstone
results.export_touchstone("circuit.s2p")?;

// Get field data
let e_field = results.electric_field(2.4e9)?;
e_field.export_vtk("e_field_2.4GHz.vtk")?;
```

---

## Related Topics

- [3D RF Viewer](./3d-rf-viewer.md)
- [Shared Simulation Architecture](../../advanced-features/shared-simulation-architecture.md)
- [Electromagnetic Simulation](../../advanced-features/electromagnetic-simulation.md)
- [S-Parameter Data Generation](../manufacturing-output/s-parameter-data-generation.md)
