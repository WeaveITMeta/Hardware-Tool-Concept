# Coherence Time Analysis

## Overview

Hardware Tool provides comprehensive coherence time analysis for quantum processors, estimating T1 (energy relaxation), T2 (dephasing), and T2* (inhomogeneous dephasing) based on layout geometry, materials, and environmental factors.

> **Inherits from:** [Shared Simulation Architecture](../../advanced-features/shared-simulation-architecture.md)
>
> This documentation covers quantum-specific coherence analysis. All standard job management, results visualization, and export capabilities are inherited from the shared architecture.

---

## Coherence Metrics

| Metric | Description | Typical Values |
|--------|-------------|----------------|
| **T1** | Energy relaxation time | 50-200 μs (transmon) |
| **T2** | Dephasing time (echo) | 30-150 μs |
| **T2*** | Free induction decay | 10-50 μs |
| **Tφ** | Pure dephasing | 50-500 μs |

---

## Analysis Configuration

```rust
CoherenceAnalysis {
    // Target qubits
    qubits: QubitSelection::All,
    
    // Decoherence mechanisms
    mechanisms: vec![
        Mechanism::DielectricLoss,
        Mechanism::QuasiparticleTunneling,
        Mechanism::FluxNoise,
        Mechanism::ChargeNoise,
        Mechanism::PhotonShot,
        Mechanism::ThermalPhotons,
    ],
    
    // Environment
    environment: EnvironmentConfig {
        temperature: 20e-3,        // 20 mK
        magnetic_field: 0.0,
        ir_filtering: true,
    },
    
    // Materials
    materials: MaterialConfig {
        substrate: Material::Silicon,
        metal: Material::Aluminum,
        junction: Material::AlOx,
    },
}
```

---

## Coherence Analysis UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Coherence Time Analysis                                  [✕]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Processor: 5-Qubit Transmon  |  T = 20 mK                      │
│                                                                 │
│ Estimated Coherence Times:                                     │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Qubit │ T1 (μs)  │ T2 (μs)  │ T2* (μs) │ Limiting Factor   │ │
│ │ ──────┼──────────┼──────────┼──────────┼────────────────── │ │
│ │ Q0    │ 95 ± 12  │ 48 ± 8   │ 22 ± 5   │ Dielectric loss   │ │
│ │ Q1    │ 88 ± 10  │ 42 ± 6   │ 18 ± 4   │ Dielectric loss   │ │
│ │ Q2    │ 72 ± 15  │ 35 ± 7   │ 15 ± 3   │ Flux noise        │ │
│ │ Q3    │ 45 ± 8   │ 22 ± 5   │ 10 ± 2   │ Quasiparticles    │ │
│ │ Q4    │ 92 ± 11  │ 45 ± 7   │ 20 ± 4   │ Dielectric loss   │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Decoherence Budget (Q0):                                       │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Mechanism              │ 1/T1 (kHz) │ Contribution         │ │
│ │ ───────────────────────┼────────────┼───────────────────── │ │
│ │ Dielectric loss        │ 6.2        │ ████████████░░ 59%  │ │
│ │ Quasiparticle          │ 2.1        │ ████░░░░░░░░░░ 20%  │ │
│ │ Thermal photons        │ 1.5        │ ███░░░░░░░░░░░ 14%  │ │
│ │ Purcell (readout)      │ 0.7        │ █░░░░░░░░░░░░░ 7%   │ │
│ │ ───────────────────────┼────────────┼───────────────────── │ │
│ │ Total                  │ 10.5       │ T1 = 95 μs          │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [Optimize Layout] [Compare Materials] [Export Report]          │
└─────────────────────────────────────────────────────────────────┘
```

---

## Decoherence Mechanisms

### Dielectric Loss

```rust
DielectricLossModel {
    // Substrate
    substrate: SubstrateLoss {
        material: Material::Silicon,
        tan_delta: 1e-6,           // Loss tangent
        participation: 0.001,      // Electric field participation
    },
    
    // Surface oxide
    surface_oxide: SurfaceLoss {
        thickness: 3e-9,           // 3 nm native oxide
        tan_delta: 0.002,
        participation: 0.01,
    },
    
    // Metal-substrate interface
    interface: InterfaceLoss {
        tan_delta: 0.003,
        participation: 0.005,
    },
}
```

### Quasiparticle Tunneling

```rust
QuasiparticleModel {
    // Junction parameters
    junction: JunctionParams {
        critical_current: 20e-9,   // 20 nA
        capacitance: 50e-15,       // 50 fF
        gap: 180e-6,               // Al gap in eV
    },
    
    // Quasiparticle density
    xqp: 1e-7,                     // Normalized density
    
    // Temperature
    temperature: 20e-3,            // 20 mK
}
```

### Flux Noise

```rust
FluxNoiseModel {
    // Noise amplitude
    amplitude: 1e-6,               // Φ0/√Hz at 1 Hz
    
    // Spectrum
    spectrum: NoiseSpectrum::OneOverF { exponent: 0.9 },
    
    // Qubit sensitivity
    sensitivity: FluxSensitivity::from_sweet_spot(0.5),  // Φ0
}
```

---

## Layout Optimization

```rust
CoherenceOptimization {
    // Target
    target: OptimizationTarget::MaximizeT1,
    
    // Constraints
    constraints: vec![
        Constraint::MinFrequency(4.5e9),
        Constraint::MaxFrequency(6.0e9),
        Constraint::MinAnharmonicity(200e6),
    ],
    
    // Variables
    variables: vec![
        Variable::JunctionArea,
        Variable::PadSize,
        Variable::GapWidth,
        Variable::SubstrateThickness,
    ],
    
    // Method
    method: OptimizationMethod::GradientDescent,
}
```

### Optimization Results

```
┌─────────────────────────────────────────────────────────────────┐
│ Layout Optimization Results                              [✕]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Optimization: Maximize T1 for Q0                               │
│                                                                 │
│ Parameter Changes:                                              │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Parameter           │ Before    │ After     │ Change       │ │
│ │ ────────────────────┼───────────┼───────────┼───────────── │ │
│ │ Pad size            │ 300 μm    │ 250 μm    │ -17%         │ │
│ │ Gap width           │ 20 μm     │ 30 μm     │ +50%         │ │
│ │ Junction area       │ 0.04 μm²  │ 0.035 μm² │ -12%         │ │
│ │ Substrate thickness │ 500 μm    │ 500 μm    │ 0%           │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Coherence Improvement:                                          │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Metric    │ Before   │ After    │ Improvement              │ │
│ │ ──────────┼──────────┼──────────┼───────────────────────── │ │
│ │ T1        │ 95 μs    │ 142 μs   │ +49% ████████████████   │ │
│ │ T2        │ 48 μs    │ 68 μs    │ +42% ██████████████     │ │
│ │ Frequency │ 5.00 GHz │ 5.12 GHz │ +2.4%                    │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [Apply Changes] [Compare Layouts] [Export]                     │
└─────────────────────────────────────────────────────────────────┘
```

---

## Material Comparison

```
┌─────────────────────────────────────────────────────────────────┐
│ Material Comparison                                      [✕]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Substrate Comparison (same geometry):                          │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Substrate        │ T1 (μs) │ T2 (μs) │ Cost   │ Avail.     │ │
│ │ ─────────────────┼─────────┼─────────┼────────┼─────────── │ │
│ │ Silicon (HR)     │ 95      │ 48      │ $      │ ✓ Common   │ │
│ │ Sapphire         │ 180     │ 85      │ $$     │ ✓ Common   │ │
│ │ Silicon (isotope)│ 250     │ 120     │ $$$$   │ ○ Limited  │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Metal Comparison:                                               │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Metal     │ T1 (μs) │ Tc (K) │ Gap (μeV) │ Process        │ │
│ │ ──────────┼─────────┼────────┼───────────┼─────────────── │ │
│ │ Aluminum  │ 95      │ 1.2    │ 180       │ Standard       │ │
│ │ Niobium   │ 120     │ 9.2    │ 1500      │ Standard       │ │
│ │ Tantalum  │ 300     │ 4.5    │ 700       │ Advanced       │ │
│ │ TiN       │ 150     │ 4.5    │ 700       │ Advanced       │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [Apply Material] [Run Full Analysis]                           │
└─────────────────────────────────────────────────────────────────┘
```

---

## CLI Commands

```bash
# Run coherence analysis
hwt quantum coherence processor.hwt_quantum

# Analyze specific qubits
hwt quantum coherence processor.hwt_quantum --qubits Q0,Q1

# Optimize for T1
hwt quantum coherence processor.hwt_quantum --optimize t1

# Compare materials
hwt quantum coherence processor.hwt_quantum --compare-materials

# Export report
hwt quantum coherence processor.hwt_quantum --export coherence_report.html
```

---

## Rust API

```rust
use hardware_tool::quantum::coherence::*;

// Run coherence analysis
let analysis = processor.analyze_coherence(CoherenceConfig::default())?;

// Get results
for qubit in analysis.qubits() {
    println!("{}: T1={:.1} μs, T2={:.1} μs, limiting: {}",
        qubit.name,
        qubit.t1 * 1e6,
        qubit.t2 * 1e6,
        qubit.limiting_mechanism);
}

// Get decoherence budget
let budget = analysis.decoherence_budget("Q0")?;
for mechanism in budget.mechanisms() {
    println!("{}: {:.1}% of 1/T1", mechanism.name, mechanism.contribution * 100.0);
}

// Optimize layout
let optimized = processor.optimize_coherence(OptimizationConfig {
    target: OptimizationTarget::MaximizeT1,
    ..Default::default()
})?;
println!("T1 improved: {:.1} μs -> {:.1} μs",
    analysis.t1("Q0") * 1e6,
    optimized.t1("Q0") * 1e6);
```

---

## Related Topics

- [Decoherence & Fidelity Calculators](../advanced-features/decoherence-and-fidelity-calculators.md)
- [Qubit Calibration](../advanced-features/qubit-calibration.md)
- [3D Qubit Viewer](../3d-visualization-cryogenics/3d-qubit-viewer.md)
- [Qubit Placement](./qubit-placement.md)
