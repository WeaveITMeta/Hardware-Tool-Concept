# Shared Simulation Architecture

## Overview

Hardware Tool provides a **unified simulation infrastructure** that works across all hardware domains. Whether you're running SPICE for PCBs, RTL simulation for ICs, quantum state evolution, MEMS FEA, RF electromagnetic analysis, or thermal simulation for packages — the same job management, results visualization, and optimization framework handles it all.

> **"One Hardware Tool that does it all"** — The same simulation workflow, results viewer, and optimization engine works for every hardware type.

---

## Shared Simulation Components

All domain-specific simulators inherit these core capabilities:

| Component | Description |
|-----------|-------------|
| **Job Manager** | Queue, schedule, and monitor simulation jobs |
| **Results Viewer** | Unified waveform, plot, and data visualization |
| **Parameter Sweeps** | Automated multi-parameter exploration |
| **Optimization** | Goal-driven parameter optimization |
| **Caching** | Intelligent result caching for incremental runs |
| **Export** | Standard data export formats |

---

## Simulation Job Manager (All Domains)

```rust
/// Unified job management for all simulation types
SimulationJobManager {
    // Job queue
    queue: JobQueue {
        max_concurrent: 4,
        priority_scheduling: true,
    },
    
    // Resource management
    resources: ResourceConfig {
        max_memory_gb: 16,
        max_threads: 8,
        gpu_enabled: true,
    },
    
    // Caching
    cache: CacheConfig {
        enabled: true,
        max_size_gb: 10,
        invalidate_on_change: true,
    },
}
```

---

## Job Status UI (All Domains)

```
┌─────────────────────────────────────────────────────────────────┐
│ Simulation Jobs                                          [✕]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Job          │ Type    │ Status    │ Progress │ Time       │ │
│ │ ─────────────┼─────────┼───────────┼──────────┼─────────── │ │
│ │ amplifier_ac │ SPICE   │ Running   │ ████░░ 67% │ 0:23     │ │
│ │ chip_timing  │ STA     │ Queued    │ ░░░░░░  0% │ --       │ │
│ │ qubit_em     │ FDTD    │ Complete  │ ██████ 100%│ 5:42     │ │
│ │ sensor_modal │ FEA     │ Complete  │ ██████ 100%│ 2:15     │ │
│ │ rf_sparams   │ MoM     │ Failed    │ ████░░ 45% │ 1:30     │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [Pause All] [Cancel] [View Results] [Retry Failed]             │
└─────────────────────────────────────────────────────────────────┘
```

---

## Results Visualization (All Domains)

### Waveform Viewer

```
┌─────────────────────────────────────────────────────────────────┐
│ Waveform Viewer: amplifier_ac                            [✕]   │
├─────────────────────────────────────────────────────────────────┤
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │     │                    ╭──────────────────╮               │ │
│ │  20 │                   ╱                    ╲              │ │
│ │     │                  ╱                      ╲             │ │
│ │   0 │─────────────────╱────────────────────────╲────────────│ │
│ │     │                                           ╲           │ │
│ │ -20 │                                            ╲          │ │
│ │     │                                             ╲         │ │
│ │     └───────────────────────────────────────────────────────│ │
│ │       1Hz        100Hz       10kHz       1MHz       100MHz  │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Signals: [✓] V(out) [✓] V(in) [ ] I(R1)                        │
│ Scale: [Lin ▼] [dB ▼]  Cursors: [A: 10kHz] [B: 100kHz]         │
│                                                                 │
│ [Zoom] [Pan] [Measure] [Export] [Add Signal]                   │
└─────────────────────────────────────────────────────────────────┘
```

### Data Table

```rust
/// Results data access (works for any simulation type)
let results = simulation.get_results()?;

// Access by signal name
let vout = results.get_signal("V(out)")?;

// Iterate data points
for (x, y) in vout.iter() {
    println!("{}: {}", x, y);
}

// Statistical analysis
println!("Max: {}", vout.max());
println!("Min: {}", vout.min());
println!("RMS: {}", vout.rms());
```

---

## Parameter Sweeps (All Domains)

```rust
/// Unified parameter sweep works for any simulation type
ParameterSweep {
    // Parameters to sweep
    parameters: vec![
        SweepParameter {
            name: "R1",
            values: SweepValues::Linear { start: 1e3, stop: 10e3, steps: 10 },
        },
        SweepParameter {
            name: "C1",
            values: SweepValues::Logarithmic { start: 1e-12, stop: 1e-9, steps: 5 },
        },
    ],
    
    // Sweep type
    sweep_type: SweepType::Full,  // or Diagonal, MonteCarlo
    
    // Parallelization
    parallel: true,
    max_jobs: 8,
}
```

### Sweep Results

```
┌─────────────────────────────────────────────────────────────────┐
│ Parameter Sweep Results                                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Sweep: R1 × C1 (50 combinations)                               │
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ R1 (kΩ) │ C1 (pF) │ Gain (dB) │ BW (MHz) │ Phase (°)       │ │
│ │ ────────┼─────────┼───────────┼──────────┼──────────        │ │
│ │   1.0   │   1.0   │   20.1    │   15.2   │   -45.2         │ │
│ │   1.0   │   3.2   │   20.0    │   12.1   │   -52.3         │ │
│ │   1.0   │  10.0   │   19.8    │    8.5   │   -67.1         │ │
│ │   ...   │   ...   │   ...     │   ...    │   ...           │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [Plot] [Export CSV] [Find Optimal] [Compare]                   │
└─────────────────────────────────────────────────────────────────┘
```

---

## Optimization Engine (All Domains)

```rust
/// Goal-driven optimization works for any simulation type
Optimization {
    // Goals
    goals: vec![
        OptimizationGoal {
            metric: "gain",
            target: Target::Maximize,
            weight: 1.0,
        },
        OptimizationGoal {
            metric: "power",
            target: Target::Minimize,
            weight: 0.5,
        },
        OptimizationGoal {
            metric: "bandwidth",
            target: Target::GreaterThan(10e6),
            weight: 0.8,
        },
    ],
    
    // Variables
    variables: vec![
        OptVariable { name: "R1", min: 1e3, max: 100e3 },
        OptVariable { name: "C1", min: 1e-12, max: 1e-9 },
    ],
    
    // Algorithm
    algorithm: OptAlgorithm::GradientDescent,  // or GeneticAlgorithm, ParticleSwarm
    
    // Constraints
    max_iterations: 100,
    convergence_threshold: 0.01,
}
```

---

## Caching & Incremental Simulation (All Domains)

```rust
/// Intelligent caching reduces simulation time
SimulationCache {
    // Cache behavior
    enabled: true,
    
    // Invalidation
    invalidate_on: vec![
        InvalidationTrigger::SchematicChange,
        InvalidationTrigger::ParameterChange,
        InvalidationTrigger::ModelChange,
    ],
    
    // Incremental simulation
    incremental: IncrementalConfig {
        enabled: true,
        reuse_dc_solution: true,
        reuse_mesh: true,  // For FEA/EM
    },
}
```

---

## Export Formats (All Domains)

```rust
/// Export results in standard formats
results.export(ExportConfig {
    format: ExportFormat::CSV,
    path: "results.csv",
    signals: vec!["V(out)", "I(R1)"],
})?;

// Supported formats
enum ExportFormat {
    CSV,           // Comma-separated values
    JSON,          // Structured data
    HDF5,          // Large datasets
    Touchstone,    // S-parameters (RF)
    PSDF,          // Power spectral density
    VCD,           // Value change dump (digital)
}
```

---

## CLI Commands (All Domains)

```bash
# Run simulation (type auto-detected)
hwt simulate my_design.hwt
hwt simulate my_design.hwt --analysis ac
hwt simulate my_design.hwt --analysis transient --stop 1ms

# Parameter sweep
hwt simulate my_design.hwt --sweep R1=1k:10k:10

# Optimization
hwt optimize my_design.hwt --goal "maximize:gain" --goal "minimize:power"

# Job management
hwt jobs list
hwt jobs status <job_id>
hwt jobs cancel <job_id>
hwt jobs results <job_id> --export csv
```

---

## Domain-Specific Simulators

Each hardware domain extends the shared simulation infrastructure with specialized solvers:

| Domain | Simulator Types | Analysis Types |
|--------|-----------------|----------------|
| **PCB** | SPICE (ngspice) | DC, AC, Transient, Noise |
| **IC** | SPICE, STA, Power | Timing, Power, IR drop |
| **Quantum** | State evolution, Lindblad | Fidelity, Decoherence |
| **MEMS** | FEA (modal, stress) | Modal, Stress, Electrostatic |
| **RF** | FDTD, MoM, FEM | S-parameters, Far-field |
| **Packaging** | Thermal FEA, SI | Temperature, Signal integrity |

See domain-specific documentation:

- [SPICE Simulation](../schematic-editor/spice-simulation.md) - PCB circuit simulation
- [RTL Simulation](../ic-design/rtl-logic-design/rtl-simulation-integration.md) - IC digital simulation
- [Quantum Simulation](../quantum-hardware/circuit-editor/quantum-simulation-integration.md) - Quantum state evolution
- [MEMS Simulation](../mems-sensors/device-editor/mems-simulation-integration.md) - Mechanical FEA
- [RF Simulation](../rf-photonics/schematic-editor/rf-simulation-integration.md) - Electromagnetic analysis
- [Thermal Simulation](./thermal-simulation.md) - Heat analysis

---

## Related Topics

- [Shared Module Consolidation](../core-architecture/shared-module-consolidation.md)
- [AI-Powered Optimization](../ai-integration/ai-routing-optimization.md)
- [Benchmarking Simulator](../ai-integration/benchmarking-simulator.md)
