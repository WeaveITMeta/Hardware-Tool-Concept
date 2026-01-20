# Qiskit Pulse Export

## Overview

Hardware Tool exports quantum control pulse sequences in Qiskit Pulse format, enabling direct integration with quantum control systems and cloud quantum computers.

## Pulse Configuration

```rust
QiskitPulseExport {
    // Backend configuration
    backend: BackendConfig {
        dt: 0.222e-9,                  // Sample time (s)
        channels: vec![
            Channel::Drive { qubit: 0, frequency: 5.0e9 },
            Channel::Drive { qubit: 1, frequency: 5.15e9 },
            Channel::Control { qubits: (0, 1), frequency: 5.075e9 },
            Channel::Measure { qubit: 0, frequency: 7.0e9 },
        ],
    },
    
    // Pulse library
    pulses: PulseLibrary {
        x90: GaussianSquare { duration: 160, sigma: 40, width: 80 },
        x180: GaussianSquare { duration: 160, sigma: 40, width: 80 },
        cr: GaussianSquare { duration: 640, sigma: 64, width: 512 },
    },
    
    // Calibration data
    calibration: CalibrationData {
        include_frequencies: true,
        include_amplitudes: true,
        include_drag: true,
    },
}
```

## Export UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Qiskit Pulse Export                                             │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Output: [pulse_schedule.py                             ] [...]  │
│                                                                 │
│ Gate Set:                                                       │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Gate    │ Duration │ Fidelity │ Include                     │ │
│ │ ────────┼──────────┼──────────┼──────────────────────────── │ │
│ │ X90     │  35 ns   │  99.95%  │ ☑                          │ │
│ │ X180    │  35 ns   │  99.94%  │ ☑                          │ │
│ │ Y90     │  35 ns   │  99.95%  │ ☑                          │ │
│ │ Y180    │  35 ns   │  99.94%  │ ☑                          │ │
│ │ CZ      │ 200 ns   │  99.20%  │ ☑                          │ │
│ │ CR      │ 300 ns   │  99.15%  │ ☑                          │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Options:                                                        │
│ ☑ Include calibration data                                     │
│ ☑ Include DRAG coefficients                                    │
│ ☑ Generate Qiskit schedule                                     │
│ ☐ Export raw waveforms (CSV)                                   │
│                                                                 │
│ [Cancel]                                    [Export Pulses]     │
└─────────────────────────────────────────────────────────────────┘
```

## Generated Python Code

```python
from qiskit import pulse
from qiskit.pulse import library

# Backend configuration
dt = 0.222e-9  # seconds

# Drive channels
d0 = pulse.DriveChannel(0)
d1 = pulse.DriveChannel(1)

# Control channels
u0 = pulse.ControlChannel(0)

# Measure channels
m0 = pulse.MeasureChannel(0)
m1 = pulse.MeasureChannel(1)

# Pulse definitions
x90_pulse = library.GaussianSquare(
    duration=160, amp=0.2, sigma=40, width=80
)
x180_pulse = library.GaussianSquare(
    duration=160, amp=0.4, sigma=40, width=80
)

# Gate schedules
with pulse.build(name='x90_q0') as x90_q0:
    pulse.play(x90_pulse, d0)

with pulse.build(name='cz_q0_q1') as cz_q0_q1:
    pulse.play(cz_pulse, u0)
```

## Rust API

```rust
// Export pulse schedules
let processor = project.get_processor("5_qubit")?;

processor.export_qiskit_pulse(PulseExportConfig {
    path: "output/pulse_schedule.py",
    gates: vec![Gate::X90, Gate::X180, Gate::CZ],
    include_calibration: true,
})?;
```

## Related Topics

- [Qubit Calibration Data](./qubit-calibration-data-generation.md)
- [Quantum Simulation Integration](../circuit-editor/quantum-simulation-integration.md)
