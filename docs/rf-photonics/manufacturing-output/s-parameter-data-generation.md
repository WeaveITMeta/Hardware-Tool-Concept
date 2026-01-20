# S-Parameter Data Generation

## Overview

Hardware Tool generates S-parameter data in Touchstone format from EM simulations and circuit analysis, enabling integration with external RF simulators and measurement equipment.

## Touchstone Export

```rust
TouchstoneExport {
    // Format settings
    format: TouchstoneFormat {
        version: TouchstoneVersion::V2,
        parameter_type: ParameterType::S,
        format: DataFormat::MagnitudeAngle,
        frequency_unit: FrequencyUnit::GHz,
        impedance: 50.0,
    },
    
    // Frequency sweep
    frequency: FrequencySweep {
        start: 0.1e9,
        stop: 20e9,
        points: 201,
        scale: Scale::Logarithmic,
    },
    
    // Port configuration
    ports: vec![
        Port { number: 1, name: "RF_IN", impedance: 50.0 },
        Port { number: 2, name: "RF_OUT", impedance: 50.0 },
    ],
}
```

## Export UI

```
┌─────────────────────────────────────────────────────────────────┐
│ S-Parameter Export                                              │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Output File: [lna_sparams.s2p                          ] [...]  │
│                                                                 │
│ Format: [Touchstone v2 ▼]  Data: [Mag/Angle ▼]                 │
│                                                                 │
│ Frequency Range:                                                │
│   Start: [0.1    ] GHz    Stop: [20     ] GHz                  │
│   Points: [201   ]        Scale: [Log ▼]                       │
│                                                                 │
│ Port Configuration:                                             │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Port │ Name    │ Impedance │ Type                           │ │
│ │ ─────┼─────────┼───────────┼─────────────────────────────── │ │
│ │  1   │ RF_IN   │   50 Ω    │ Single-ended                   │ │
│ │  2   │ RF_OUT  │   50 Ω    │ Single-ended                   │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Data Source: [EM Simulation ▼]                                 │
│                                                                 │
│ Preview @ 2.4 GHz:                                              │
│   S11 = -15.2 dB ∠-45°    S21 = 18.5 dB ∠125°                 │
│   S12 = -25.3 dB ∠-80°    S22 = -12.8 dB ∠-30°                │
│                                                                 │
│ Options:                                                        │
│ ☑ Include noise parameters                                     │
│ ☑ Include comments with design info                            │
│ ☐ Export mixed-mode parameters                                 │
│                                                                 │
│ [Cancel]                                    [Export S-params]   │
└─────────────────────────────────────────────────────────────────┘
```

## Generated Touchstone File

```
! Hardware Tool S-Parameter Export
! Design: WiFi LNA
! Date: 2026-01-19
! Substrate: Rogers RO4003C
!
# GHz S MA R 50
! freq       S11_mag   S11_ang   S21_mag   S21_ang   S12_mag   S12_ang   S22_mag   S22_ang
0.1          0.892     -12.5     7.943     165.2     0.056     -85.3     0.794     -8.2
0.5          0.631     -35.8     8.912     145.8     0.050     -92.1     0.562     -22.5
1.0          0.398     -52.3     8.511     132.4     0.045     -98.7     0.355     -38.9
2.0          0.224     -78.5     8.128     118.6     0.038     -112.3    0.200     -62.4
2.4          0.173     -85.2     8.414     125.1     0.035     -118.5    0.178     -68.7
2.5          0.168     -87.1     8.318     123.8     0.034     -120.2    0.172     -70.3
5.0          0.251     -125.6    6.310     95.2      0.028     -145.8    0.224     -98.5
10.0         0.398     -168.2    3.162     52.8      0.020     -172.3    0.355     -142.1
20.0         0.562     -178.5    1.000     12.5      0.010     -175.8    0.501     -168.9
```

## Rust API

```rust
// Generate S-parameters from simulation
let circuit = project.get_circuit("lna")?;

let s_params = circuit.simulate_s_parameters(SParamConfig {
    frequency: FrequencySweep::log(0.1e9, 20e9, 201),
    ports: vec![
        Port::new(1, "RF_IN", 50.0),
        Port::new(2, "RF_OUT", 50.0),
    ],
})?;

// Export to Touchstone
s_params.export_touchstone("lna_sparams.s2p", TouchstoneConfig {
    version: TouchstoneVersion::V2,
    format: DataFormat::MagnitudeAngle,
    include_noise: true,
})?;

// Query specific values
let s21_at_2_4ghz = s_params.get(2.4e9, SParam::S21)?;
println!("S21 @ 2.4 GHz: {} dB", s21_at_2_4ghz.magnitude_db());
```

## Related Topics

- [Gerber RF Export](./gerber-rf-export.md)
- [RF Simulation Integration](../schematic-editor/rf-simulation-integration.md)
