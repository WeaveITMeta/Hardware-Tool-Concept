# MEMS & Sensor Design

## Overview

Hardware Tool provides comprehensive MEMS (Micro-Electro-Mechanical Systems) and sensor design capabilities, from concept through fabrication. Design accelerometers, gyroscopes, pressure sensors, microphones, and custom MEMS devices with integrated multi-physics simulation.

### MEMS Device Categories

```
┌─────────────────────────────────────────────────────────────────┐
│                    MEMS & Sensor Types                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Inertial Sensors        Pressure Sensors      Acoustic         │
│  ┌─────────────────┐    ┌─────────────────┐   ┌─────────────┐  │
│  │ • Accelerometer │    │ • Absolute      │   │ • MEMS Mic  │  │
│  │ • Gyroscope     │    │ • Differential  │   │ • Ultrasonic│  │
│  │ • IMU           │    │ • Barometer     │   │ • Piezo     │  │
│  └─────────────────┘    └─────────────────┘   └─────────────┘  │
│                                                                 │
│  Optical MEMS           RF MEMS               Bio-MEMS          │
│  ┌─────────────────┐    ┌─────────────────┐   ┌─────────────┐  │
│  │ • Micromirrors  │    │ • Switches      │   │ • Lab-on-chip│ │
│  │ • Gratings      │    │ • Resonators    │   │ • Microfluidic│ │
│  │ • Modulators    │    │ • Filters       │   │ • Biosensors │  │
│  └─────────────────┘    └─────────────────┘   └─────────────┘  │
│                                                                 │
│  Actuators              Thermal Sensors       Chemical          │
│  ┌─────────────────┐    ┌─────────────────┐   ┌─────────────┐  │
│  │ • Comb drives   │    │ • Bolometers    │   │ • Gas sensors│  │
│  │ • Piezo         │    │ • Thermopiles   │   │ • pH sensors │  │
│  │ • Electrostatic │    │ • RTD arrays    │   │ • Humidity   │  │
│  └─────────────────┘    └─────────────────┘   └─────────────┘  │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## Accelerometer Design

### Capacitive MEMS Accelerometer

```rust
Accelerometer {
    // Sensing configuration
    sensing: SensingConfig {
        type_: SensingType::Capacitive,
        axes: 3,                      // X, Y, Z
        range: 16.0,                  // ±16 g
        sensitivity: 1000.0,          // fF/g
        noise_density: 100e-6,        // 100 µg/√Hz
        bandwidth: 1000.0,            // Hz
    },
    
    // Proof mass
    proof_mass: ProofMass {
        material: "Silicon",
        dimensions: (500e-6, 500e-6, 50e-6),  // 500×500×50 µm
        mass: 29e-9,                  // 29 µg
        perforation: Perforation {
            enabled: true,
            hole_size: 5e-6,
            pitch: 10e-6,
        },
    },
    
    // Springs
    springs: SpringConfig {
        type_: SpringType::FoldedBeam,
        beams_per_side: 4,
        beam_length: 200e-6,          // 200 µm
        beam_width: 3e-6,             // 3 µm
        beam_thickness: 50e-6,        // 50 µm (device layer)
        stiffness: 10.0,              // N/m
    },
    
    // Sense electrodes
    electrodes: ElectrodeConfig {
        type_: ElectrodeType::CombFinger,
        fingers_per_side: 50,
        finger_length: 100e-6,
        finger_width: 3e-6,
        finger_gap: 2e-6,
        overlap: 80e-6,
    },
    
    // Readout
    readout: ReadoutConfig {
        type_: ReadoutType::DifferentialCapacitive,
        nominal_capacitance: 1e-12,   // 1 pF
        parasitic_capacitance: 0.5e-12,
    },
}
```

### Accelerometer Layout

```
┌─────────────────────────────────────────────────────────────────┐
│ MEMS Layout: 3-Axis Accelerometer                               │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │                                                             ││
│  │   Anchor    Spring      Proof Mass       Spring    Anchor   ││
│  │   ┌───┐    ╔═══════╗   ┌───────────────┐ ╔═══════╗ ┌───┐   ││
│  │   │   │════╣       ╠═══│               │═╣       ╠═│   │   ││
│  │   │   │    ╚═══════╝   │   ┌───────┐   │ ╚═══════╝ │   │   ││
│  │   └───┘                │   │       │   │           └───┘   ││
│  │                        │   │ Holes │   │                    ││
│  │   Fixed    ║║║║║║║║║   │   │       │   │   ║║║║║║║║║ Fixed ││
│  │   Fingers  ║║║║║║║║║   │   └───────┘   │   ║║║║║║║║║ Fingers││
│  │            ║║║║║║║║║   │               │   ║║║║║║║║║        ││
│  │   ┌───┐    ╔═══════╗   │               │ ╔═══════╗ ┌───┐   ││
│  │   │   │════╣       ╠═══│               │═╣       ╠═│   │   ││
│  │   │   │    ╚═══════╝   └───────────────┘ ╚═══════╝ │   │   ││
│  │   └───┘                                            └───┘   ││
│  │                                                             ││
│  │   Moving fingers interdigitated with fixed fingers          ││
│  │   (shown simplified - actual design has 50+ fingers)        ││
│  │                                                             ││
│  └─────────────────────────────────────────────────────────────┘│
│                                                                 │
│ Device Parameters:                                              │
│   Resonant frequency: 3.2 kHz                                  │
│   Sensitivity: 1.2 pF/g                                        │
│   Brownian noise: 45 µg/√Hz                                    │
│   Full-scale range: ±16 g                                      │
│                                                                 │
│ [Simulate] [FEA Analysis] [Export GDS] [3D View]                │
└─────────────────────────────────────────────────────────────────┘
```

## Gyroscope Design

### Vibratory MEMS Gyroscope

```rust
Gyroscope {
    // Configuration
    config: GyroConfig {
        type_: GyroType::TuningFork,
        axes: 1,                      // Single axis (Z)
        range: 2000.0,                // ±2000 °/s
        sensitivity: 20e-3,           // 20 mV/(°/s)
        noise_density: 0.01,          // 0.01 °/s/√Hz
        bandwidth: 100.0,             // Hz
    },
    
    // Drive mode
    drive: DriveMode {
        frequency: 15000.0,           // 15 kHz
        amplitude: 10e-6,             // 10 µm
        quality_factor: 10000,
        actuation: ActuationType::Electrostatic,
    },
    
    // Sense mode
    sense: SenseMode {
        frequency: 15100.0,           // 15.1 kHz (mode-split)
        quality_factor: 5000,
        detection: DetectionType::Capacitive,
    },
    
    // Proof masses (tuning fork)
    masses: vec![
        ProofMass {
            dimensions: (400e-6, 200e-6, 40e-6),
            mass: 7.5e-9,             // 7.5 µg
        },
        ProofMass {
            dimensions: (400e-6, 200e-6, 40e-6),
            mass: 7.5e-9,
        },
    ],
    
    // Coupling
    coupling: CouplingConfig {
        type_: CouplingType::Lever,
        stiffness_ratio: 0.1,
    },
}
```

## Pressure Sensor Design

### Piezoresistive Pressure Sensor

```rust
PressureSensor {
    // Configuration
    config: PressureConfig {
        type_: PressureType::Absolute,
        range: (0.0, 1000.0),         // 0-1000 kPa
        sensitivity: 0.1,             // mV/kPa
        accuracy: 0.01,               // 1% FS
        temperature_range: (-40.0, 125.0),
    },
    
    // Diaphragm
    diaphragm: Diaphragm {
        shape: DiaphragmShape::Square,
        size: 1000e-6,                // 1 mm
        thickness: 20e-6,             // 20 µm
        material: "Silicon",
        stress_concentration: true,   // Boss structure
    },
    
    // Piezoresistors
    piezoresistors: PiezoresistorConfig {
        type_: PiezoresistorType::PDoped,
        count: 4,                     // Wheatstone bridge
        resistance: 5000.0,           // 5 kΩ
        placement: Placement::MaxStress,
        tcr: 1500e-6,                 // ppm/°C
    },
    
    // Cavity
    cavity: CavityConfig {
        type_: CavityType::Vacuum,    // For absolute sensor
        pressure: 0.001,              // 1 mbar reference
        getter: true,                 // Maintain vacuum
    },
}
```

### Pressure Sensor Layout

```
┌─────────────────────────────────────────────────────────────────┐
│ MEMS Layout: Piezoresistive Pressure Sensor                     │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Cross-Section:                                                 │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │                     Pressure                                ││
│  │                        ↓↓↓                                  ││
│  │   ┌─────────────────────────────────────────────────────┐  ││
│  │   │  R1        Diaphragm (20µm Si)           R2        │  ││
│  │   │  ▓▓                                      ▓▓        │  ││
│  │   └──╱──────────────────────────────────────────╲──────┘  ││
│  │     ╱                                            ╲         ││
│  │    ╱              Vacuum Cavity                   ╲        ││
│  │   ╱                                                ╲       ││
│  │  ┌──────────────────────────────────────────────────┐     ││
│  │  │                 Silicon Substrate                 │     ││
│  │  └──────────────────────────────────────────────────┘     ││
│  │                                                             ││
│  └─────────────────────────────────────────────────────────────┘│
│                                                                 │
│  Top View (Wheatstone Bridge):                                  │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │                                                             ││
│  │              VDD                                            ││
│  │               │                                             ││
│  │          ┌────┴────┐                                       ││
│  │          │         │                                        ││
│  │      ┌───┤ R1      ├───┐                                   ││
│  │      │   │ (edge)  │   │                                    ││
│  │      │   └─────────┘   │                                    ││
│  │   V+ ●                 ● V-                                 ││
│  │      │   ┌─────────┐   │                                    ││
│  │      │   │ R3      │   │                                    ││
│  │      └───┤ (center)├───┘                                   ││
│  │          │         │                                        ││
│  │          └────┬────┘                                       ││
│  │               │                                             ││
│  │              GND                                            ││
│  │                                                             ││
│  └─────────────────────────────────────────────────────────────┘│
│                                                                 │
│ [FEA Stress] [Simulate] [Export GDS] [3D View]                  │
└─────────────────────────────────────────────────────────────────┘
```

## MEMS Microphone

### Capacitive MEMS Microphone

```rust
MEMSMicrophone {
    // Acoustic parameters
    acoustic: AcousticConfig {
        sensitivity: -38.0,           // dBV/Pa
        frequency_range: (20.0, 20000.0),  // Hz
        snr: 65.0,                    // dB
        thd: 0.1,                     // % at 1 kHz, 94 dB SPL
        max_spl: 120.0,               // dB
    },
    
    // Diaphragm
    diaphragm: MicDiaphragm {
        material: "Polysilicon",
        diameter: 800e-6,             // 800 µm
        thickness: 1e-6,              // 1 µm
        tension: 100.0,               // N/m
        perforation: None,            // Solid diaphragm
    },
    
    // Backplate
    backplate: Backplate {
        material: "Polysilicon",
        thickness: 2e-6,
        hole_diameter: 10e-6,
        hole_pitch: 20e-6,
        acoustic_resistance: 1e9,     // Pa·s/m³
    },
    
    // Air gap
    air_gap: AirGap {
        nominal: 3e-6,                // 3 µm
        collapse_voltage: 10.0,       // V
    },
    
    // Back cavity
    back_cavity: BackCavity {
        volume: 1e-9,                 // 1 mm³
        vent: VentConfig {
            type_: VentType::Slot,
            acoustic_resistance: 1e8,
        },
    },
}
```

## Multi-Physics Simulation

### FEA Structural Analysis

```rust
MEMSStructuralFEA {
    // Mesh
    mesh: FEAMesh {
        element_type: ElementType::Tetrahedral,
        min_size: 1e-6,               // 1 µm
        max_size: 50e-6,              // 50 µm
        refinement: MeshRefinement {
            at_corners: true,
            at_thin_features: true,
            growth_rate: 1.2,
        },
    },
    
    // Material properties
    materials: vec![
        Material {
            name: "Silicon <100>",
            youngs_modulus: 169e9,    // Pa
            poissons_ratio: 0.22,
            density: 2330.0,          // kg/m³
            thermal_expansion: 2.6e-6,
            anisotropic: true,
            stiffness_matrix: silicon_stiffness_matrix(),
        },
    ],
    
    // Analysis types
    analyses: vec![
        Analysis::Static {
            loads: vec![
                Load::Pressure { surface: "top", value: 100e3 },
                Load::Acceleration { direction: "z", value: 9.81 * 16.0 },
            ],
        },
        Analysis::Modal {
            num_modes: 10,
            frequency_range: (0.0, 100e3),
        },
        Analysis::Harmonic {
            frequency_range: (1.0, 10e3),
            damping: 0.001,
        },
    ],
}
```

### Electrostatic Simulation

```rust
ElectrostaticSimulation {
    // Solver
    solver: ElectrostaticSolver::BEM,  // Boundary Element Method
    
    // Electrodes
    electrodes: vec![
        Electrode { name: "proof_mass", voltage: Variable },
        Electrode { name: "fixed_left", voltage: 0.0 },
        Electrode { name: "fixed_right", voltage: 0.0 },
    ],
    
    // Analysis
    analysis: ElectrostaticAnalysis {
        capacitance_matrix: true,
        electrostatic_force: true,
        pull_in_voltage: true,
    },
    
    // Coupled simulation
    coupling: ElectromechanicalCoupling {
        enabled: true,
        iterations: 10,
        convergence: 1e-6,
    },
}
```

### Simulation Results

```
┌─────────────────────────────────────────────────────────────────┐
│ MEMS Simulation Results: Accelerometer                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Modal Analysis:                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Mode │ Frequency │ Description          │ Mass Part.        │ │
│ │ ─────┼───────────┼──────────────────────┼─────────────      │ │
│ │  1   │  3.21 kHz │ X-axis translation   │ 0.95 (sense)     │ │
│ │  2   │  3.25 kHz │ Y-axis translation   │ 0.94 (sense)     │ │
│ │  3   │  8.45 kHz │ Z-axis translation   │ 0.92 (sense)     │ │
│ │  4   │ 15.2 kHz  │ Rotation (torsion)   │ 0.02 (spurious)  │ │
│ │  5   │ 22.1 kHz  │ In-plane rotation    │ 0.01 (spurious)  │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Stress Analysis (at 16g):                                       │
│   Max stress: 45 MPa (at spring anchor)                        │
│   Yield strength: 7 GPa (silicon)                              │
│   Safety factor: 155× ✓                                        │
│                                                                 │
│ Electrostatic Analysis:                                         │
│   Nominal capacitance: 1.05 pF                                 │
│   Sensitivity: 1.18 pF/g                                       │
│   Pull-in voltage: 8.5 V                                       │
│   Operating voltage: 1.8 V (safe margin) ✓                    │
│                                                                 │
│ Noise Analysis:                                                 │
│   Brownian noise: 42 µg/√Hz                                    │
│   Electronic noise: 85 µg/√Hz                                  │
│   Total noise: 95 µg/√Hz                                       │
│                                                                 │
│ [View Mode Shapes] [Stress Map] [Export] [Optimize]             │
└─────────────────────────────────────────────────────────────────┘
```

## MEMS Fabrication

### Process Flow Definition

```rust
MEMSFabrication {
    // Process type
    process: ProcessType::SurfaceMicromachining,
    
    // Layer stack
    layers: vec![
        Layer { name: "Substrate", material: "Silicon", thickness: 500e-6 },
        Layer { name: "Oxide1", material: "SiO2", thickness: 2e-6 },
        Layer { name: "Poly1", material: "Polysilicon", thickness: 2e-6 },
        Layer { name: "Oxide2", material: "SiO2", thickness: 0.75e-6 },
        Layer { name: "Poly2", material: "Polysilicon", thickness: 1.5e-6 },
        Layer { name: "Metal", material: "Gold", thickness: 0.5e-6 },
    ],
    
    // Process steps
    steps: vec![
        Step::Deposit { layer: "Oxide1", method: "LPCVD" },
        Step::Pattern { layer: "Oxide1", mask: "ANCHOR" },
        Step::Deposit { layer: "Poly1", method: "LPCVD" },
        Step::Dope { layer: "Poly1", type_: "Phosphorus", dose: 1e15 },
        Step::Pattern { layer: "Poly1", mask: "POLY1" },
        Step::Deposit { layer: "Oxide2", method: "LPCVD" },
        Step::Pattern { layer: "Oxide2", mask: "DIMPLE" },
        Step::Deposit { layer: "Poly2", method: "LPCVD" },
        Step::Pattern { layer: "Poly2", mask: "POLY2" },
        Step::Deposit { layer: "Metal", method: "Evaporation" },
        Step::Pattern { layer: "Metal", mask: "METAL" },
        Step::Release { etchant: "HF vapor", time: 30.0 },
    ],
    
    // Design rules
    design_rules: MEMSDesignRules {
        min_feature: 2e-6,
        min_spacing: 2e-6,
        min_anchor: 4e-6,
        min_etch_hole: 3e-6,
        max_etch_hole_spacing: 30e-6,
    },
}
```

### Foundry Support

```rust
FoundrySupport {
    // Supported foundries
    foundries: vec![
        Foundry {
            name: "MEMSCAP PolyMUMPs",
            process: "Surface micromachining",
            layers: 3,
            min_feature: 2e-6,
        },
        Foundry {
            name: "MEMSCAP SOIMUMPs",
            process: "SOI bulk micromachining",
            device_layer: 25e-6,
            min_feature: 2e-6,
        },
        Foundry {
            name: "STMicroelectronics ThELMA",
            process: "Epitaxial polysilicon",
            device_layer: 22e-6,
            min_feature: 1e-6,
        },
        Foundry {
            name: "X-FAB XMB10",
            process: "Bulk micromachining",
            wafer_thickness: 400e-6,
            min_feature: 3e-6,
        },
    ],
    
    // DRC per foundry
    drc: FoundryDRC {
        auto_select: true,
        check_on_save: true,
    },
}
```

## Packaging Integration

### MEMS Packaging

```rust
MEMSPackaging {
    // Package type
    package: PackageType::LGA,
    
    // Cavity
    cavity: CavityPackage {
        type_: CavityType::Hermetic,
        atmosphere: Atmosphere::Nitrogen,
        pressure: 100.0,              // mbar
        getter: true,
    },
    
    // Die attach
    die_attach: DieAttach {
        method: AttachMethod::AuSn,
        stress_relief: true,
    },
    
    // Wire bonding
    wire_bond: WireBond {
        material: "Gold",
        diameter: 25e-6,
        loop_height: 150e-6,
    },
    
    // Lid
    lid: LidConfig {
        material: "Kovar",
        seal: SealType::SeamWeld,
        port: Some(Port { diameter: 0.5e-3, for_pressure_sensor: true }),
    },
}
```

## API Usage

```rust
// Create MEMS accelerometer
let accel = Accelerometer::new("3axis_accel")?;

// Configure sensing
accel.set_range(16.0)?;  // ±16 g
accel.set_bandwidth(1000.0)?;  // 1 kHz

// Design proof mass and springs
accel.design_proof_mass(ProofMassConfig {
    target_mass: 30e-9,  // 30 µg
    aspect_ratio: 1.0,
})?;

accel.design_springs(SpringConfig {
    target_frequency: 3000.0,  // 3 kHz
    type_: SpringType::FoldedBeam,
})?;

// Design sense electrodes
accel.design_electrodes(ElectrodeConfig {
    target_sensitivity: 1.0,  // pF/g
    type_: ElectrodeType::CombFinger,
})?;

// Run simulations
let modal = accel.simulate_modal(10)?;
println!("Resonant frequency: {} Hz", modal.modes[0].frequency);

let stress = accel.simulate_stress(16.0)?;  // At 16g
println!("Max stress: {} MPa", stress.max / 1e6);

let noise = accel.calculate_noise()?;
println!("Noise floor: {} µg/√Hz", noise.total * 1e6);

// Export for fabrication
accel.export_gdsii("accelerometer.gds", Foundry::MEMSCAP_PolyMUMPs)?;
```

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `M` | Add MEMS component |
| `S` | Add spring |
| `E` | Add electrode |
| `Shift+F` | Run FEA simulation |
| `Shift+M` | Run modal analysis |
| `G` | Export GDSII |

## Related Topics

- [IC Design](../ic-design/integrated-circuit-design.md) - ASIC integration
- [Advanced Packaging](../advanced-packaging/chiplet-integration.md) - MEMS packaging
- [Thermal Simulation](../advanced-features/thermal-simulation.md) - Thermal analysis
- [RF Design](../rf-photonics/rf-microwave-design.md) - RF MEMS
