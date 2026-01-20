# Qubit Calibration

## Overview

Hardware Tool provides comprehensive qubit calibration workflows for superconducting, photonic, and trapped ion quantum processors. Calibrate frequencies, gate pulses, readout parameters, and track calibration drift over time.

> **Inherits from:** [Shared Simulation Architecture](../../advanced-features/shared-simulation-architecture.md)
>
> This documentation covers quantum-specific calibration. All standard job management, results visualization, and export capabilities are inherited from the shared architecture.

---

## Calibration Types

| Calibration | Description | Frequency |
|-------------|-------------|-----------|
| **Qubit Frequency** | Resonance frequency (f01) | Daily |
| **Readout** | Resonator frequency, amplitude | Daily |
| **Single-Qubit Gates** | X, Y, Z pulse calibration | Daily |
| **Two-Qubit Gates** | CNOT, CZ, iSWAP | Daily |
| **T1/T2** | Coherence times | Hourly |
| **Crosstalk** | Inter-qubit coupling | Weekly |

---

## Calibration Configuration

```rust
CalibrationConfig {
    // Target qubits
    qubits: QubitSelection::All,  // or Specific(vec!["Q0", "Q1"])
    
    // Calibration sequence
    sequence: vec![
        Calibration::QubitFrequency,
        Calibration::ReadoutFrequency,
        Calibration::ReadoutAmplitude,
        Calibration::PiPulse,
        Calibration::DragCoefficient,
        Calibration::T1,
        Calibration::T2Ramsey,
        Calibration::T2Echo,
    ],
    
    // Options
    options: CalibrationOptions {
        auto_update: true,           // Update calibration data
        verify_after: true,          // Run verification
        abort_on_failure: false,     // Continue on single failure
    },
}
```

---

## Calibration UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Qubit Calibration                                        [✕]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Processor: 5-Qubit Transmon  |  Last Cal: 2h ago              │
│                                                                 │
│ Calibration Status:                                            │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Qubit │ Freq (GHz) │ T1 (μs) │ T2 (μs) │ Gate Fid │ Status │ │
│ │ ──────┼────────────┼─────────┼─────────┼──────────┼─────── │ │
│ │ Q0    │ 5.0012     │ 85.2    │ 42.1    │ 99.92%   │ ✓ OK   │ │
│ │ Q1    │ 5.1523     │ 92.1    │ 38.5    │ 99.89%   │ ✓ OK   │ │
│ │ Q2    │ 5.3001     │ 78.4    │ 35.2    │ 99.85%   │ ✓ OK   │ │
│ │ Q3    │ 5.4512     │ 45.2    │ 18.1    │ 98.92%   │ ⚠ Drift│ │
│ │ Q4    │ 5.6008     │ 88.5    │ 40.2    │ 99.91%   │ ✓ OK   │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Two-Qubit Gates:                                               │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Gate     │ Qubits  │ Fidelity │ Duration │ Status          │ │
│ │ ─────────┼─────────┼──────────┼──────────┼──────────────── │ │
│ │ CNOT     │ Q0-Q1   │ 99.21%   │ 180 ns   │ ✓ OK            │ │
│ │ CNOT     │ Q1-Q2   │ 99.15%   │ 185 ns   │ ✓ OK            │ │
│ │ CNOT     │ Q2-Q3   │ 98.45%   │ 195 ns   │ ⚠ Recal needed  │ │
│ │ CNOT     │ Q3-Q4   │ 99.08%   │ 182 ns   │ ✓ OK            │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [Run Full Calibration] [Calibrate Selected] [View History]     │
└─────────────────────────────────────────────────────────────────┘
```

---

## Frequency Calibration

### Qubit Spectroscopy

```rust
QubitSpectroscopy {
    // Frequency range
    frequency_range: (4.8e9, 5.8e9),
    frequency_step: 1e6,
    
    // Pulse parameters
    pulse: SpectroscopyPulse {
        duration: 10e-6,           // 10 μs
        amplitude: 0.1,
    },
    
    // Fitting
    fit: FitConfig {
        model: FitModel::Lorentzian,
        extract: vec![
            FitParameter::CenterFrequency,
            FitParameter::Linewidth,
        ],
    },
}
```

### Ramsey Experiment

```rust
RamseyExperiment {
    // Delay range
    delay_range: (0.0, 50e-6),     // 0 to 50 μs
    delay_points: 100,
    
    // Detuning
    detuning: 1e6,                 // 1 MHz artificial detuning
    
    // Fitting
    fit: FitConfig {
        model: FitModel::DampedOscillation,
        extract: vec![
            FitParameter::Frequency,   // Actual detuning
            FitParameter::T2Star,      // Dephasing time
        ],
    },
}
```

---

## Gate Calibration

### Pi Pulse Calibration

```rust
PiPulseCalibration {
    // Amplitude range
    amplitude_range: (0.0, 1.0),
    amplitude_points: 51,
    
    // Rabi experiment
    rabi: RabiConfig {
        pulse_type: PulseType::Gaussian,
        duration: 20e-9,           // 20 ns
        sigma: 5e-9,
    },
    
    // Fitting
    fit: FitConfig {
        model: FitModel::Sinusoidal,
        extract: vec![
            FitParameter::PiAmplitude,
            FitParameter::RabiFrequency,
        ],
    },
}
```

### DRAG Calibration

```rust
DragCalibration {
    // DRAG coefficient range
    drag_range: (-2.0, 2.0),
    drag_points: 41,
    
    // Sequence
    sequence: DragSequence::AllXY,  // or ErrorAmplification
    
    // Target
    target: DragTarget::MinimizeLeakage,
}
```

---

## Two-Qubit Gate Calibration

```rust
TwoQubitCalibration {
    // Gate type
    gate: TwoQubitGate::CZ,
    
    // Qubits
    control: "Q0",
    target: "Q1",
    
    // Calibration steps
    steps: vec![
        TwoQubitStep::FluxPulseAmplitude,
        TwoQubitStep::FluxPulseDuration,
        TwoQubitStep::ConditionalPhase,
        TwoQubitStep::SingleQubitPhases,
    ],
    
    // Verification
    verification: TwoQubitVerification {
        method: VerificationMethod::ProcessTomography,
        shots: 10000,
    },
}
```

---

## Calibration History

```
┌─────────────────────────────────────────────────────────────────┐
│ Calibration History: Q0                                  [✕]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Frequency Drift (24h):                                         │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ 5.002 │                    ●                               │ │
│ │       │              ●  ●     ●  ●                         │ │
│ │ 5.001 │        ●  ●              ●  ●  ●                   │ │
│ │       │     ●                          ●  ●                │ │
│ │ 5.000 │──●──────────────────────────────────●──            │ │
│ │       └────────────────────────────────────────────        │ │
│ │         0h    4h    8h    12h   16h   20h   24h           │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ T1 History (7 days):                                           │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ 100 │     ●                                                │ │
│ │     │  ●     ●  ●                                          │ │
│ │  80 │           ●  ●  ●  ●  ●  ●  ●                       │ │
│ │     │                             ●  ●  ●                  │ │
│ │  60 │──────────────────────────────────────                │ │
│ │     └────────────────────────────────────────────          │ │
│ │       Mon  Tue  Wed  Thu  Fri  Sat  Sun                   │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [Export Data] [Set Alerts] [Compare Qubits]                    │
└─────────────────────────────────────────────────────────────────┘
```

---

## Calibration Data Export

```rust
CalibrationExport {
    // Format
    format: CalibrationFormat::JSON,  // or Qiskit, Cirq
    
    // Content
    content: CalibrationContent {
        qubit_frequencies: true,
        gate_parameters: true,
        readout_parameters: true,
        coherence_times: true,
        crosstalk_matrix: true,
    },
    
    // Metadata
    metadata: CalibrationMetadata {
        timestamp: true,
        temperature: true,
        fridge_status: true,
    },
}
```

### JSON Output

```json
{
  "calibration_version": "1.0",
  "timestamp": "2026-01-20T15:30:00Z",
  "processor": "5_qubit_transmon",
  "qubits": {
    "Q0": {
      "frequency": 5.0012e9,
      "anharmonicity": -320e6,
      "t1": 85.2e-6,
      "t2": 42.1e-6,
      "readout_frequency": 7.012e9,
      "readout_amplitude": 0.15,
      "pi_pulse": {
        "amplitude": 0.234,
        "duration": 20e-9,
        "drag_coefficient": 0.85
      }
    }
  },
  "two_qubit_gates": {
    "CZ_Q0_Q1": {
      "flux_amplitude": 0.45,
      "duration": 40e-9,
      "conditional_phase": 3.14159,
      "fidelity": 0.9921
    }
  }
}
```

---

## CLI Commands

```bash
# Run calibration
hwt quantum calibrate processor.hwt_quantum --full
hwt quantum calibrate processor.hwt_quantum --qubits Q0,Q1

# Specific calibrations
hwt quantum calibrate processor.hwt_quantum --frequency
hwt quantum calibrate processor.hwt_quantum --gates
hwt quantum calibrate processor.hwt_quantum --t1t2

# View history
hwt quantum calibration history --qubit Q0 --days 7

# Export
hwt quantum calibration export --format json --output calibration.json
hwt quantum calibration export --format qiskit --output backend.json
```

---

## Rust API

```rust
use hardware_tool::quantum::calibration::*;

// Run full calibration
let result = processor.calibrate(CalibrationConfig::full())?;

// Check results
for qubit in result.qubits() {
    println!("{}: f={:.4} GHz, T1={:.1} μs, fidelity={:.2}%",
        qubit.name,
        qubit.frequency / 1e9,
        qubit.t1 * 1e6,
        qubit.gate_fidelity * 100.0);
}

// Export calibration
result.export_json("calibration.json")?;
result.export_qiskit("backend.json")?;

// Track drift
let history = processor.calibration_history(Duration::days(7))?;
for entry in history.frequency_drift("Q0") {
    println!("{}: {:.6} GHz", entry.timestamp, entry.frequency / 1e9);
}
```

---

## Related Topics

- [Decoherence & Fidelity Calculators](./decoherence-and-fidelity-calculators.md)
- [Qiskit Pulse Export](../manufacturing-output/qiskit-pulse-export.md)
- [Testing & Debug](../../advanced-features/testing-debug.md)
- [3D Qubit Viewer](../3d-visualization-cryogenics/3d-qubit-viewer.md)
