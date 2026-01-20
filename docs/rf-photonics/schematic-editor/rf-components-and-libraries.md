# RF Components & Libraries

## Overview

Hardware Tool provides comprehensive RF component libraries including transistors, passive components, transmission line models, and vendor-specific device models for RF/microwave design.

## Component Categories

```rust
RFComponentLibrary {
    // Active devices
    active: ActiveDevices {
        transistors: vec![
            TransistorModel::BJT,      // BFU730F, BFP640, etc.
            TransistorModel::FET,      // ATF-34143, etc.
            TransistorModel::HEMT,     // pHEMT, mHEMT
            TransistorModel::HBT,      // InGaP HBT
        ],
        diodes: vec![
            DiodeModel::Schottky,
            DiodeModel::PIN,
            DiodeModel::Varactor,
        ],
    },
    
    // Passive components
    passive: PassiveComponents {
        resistors: vec![ResistorModel::ThinFilm, ResistorModel::ThickFilm],
        capacitors: vec![CapacitorModel::MIM, CapacitorModel::Interdigital],
        inductors: vec![InductorModel::Spiral, InductorModel::BondWire],
    },
    
    // Transmission lines
    transmission_lines: vec![
        TLineModel::Microstrip,
        TLineModel::CPW,
        TLineModel::Stripline,
        TLineModel::Coax,
    ],
}
```

## Component Browser

```
┌─────────────────────────────────────────────────────────────────┐
│ RF Component Browser                                            │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Search: [lna transistor                    ] [🔍]               │
│                                                                 │
│ Category: [Transistors ▼]  Type: [BJT ▼]  Vendor: [All ▼]      │
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Part Number    │ Type │ Freq    │ NF    │ Gain  │ Vendor   │ │
│ │ ───────────────┼──────┼─────────┼───────┼───────┼───────── │ │
│ │ BFU730F        │ BJT  │ 12 GHz  │ 0.9dB │ 21dB  │ Infineon │ │
│ │ BFP640         │ BJT  │ 10 GHz  │ 1.0dB │ 19dB  │ Infineon │ │
│ │ BFP740         │ BJT  │ 15 GHz  │ 0.8dB │ 20dB  │ Infineon │ │
│ │ ATF-34143      │ pHEMT│ 18 GHz  │ 0.5dB │ 16dB  │ Broadcom │ │
│ │ ATF-54143      │ eHEMT│ 12 GHz  │ 0.4dB │ 17dB  │ Broadcom │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Selected: BFU730F                                               │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Symbol:    C ──┤├── B                                       │ │
│ │                 │                                           │ │
│ │                 E                                           │ │
│ │                                                             │ │
│ │ Key Specs @ 2.4 GHz, Vce=2V, Ic=10mA:                      │ │
│ │   NF_min = 0.9 dB    Gmax = 21 dB                          │ │
│ │   S21 = 18 dB        S11 = -8 dB                           │ │
│ │                                                             │ │
│ │ Models: S-parameters, SPICE, ADS                           │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [View Datasheet] [View S-params] [Insert] [Close]               │
└─────────────────────────────────────────────────────────────────┘
```

## S-Parameter Models

```rust
SParameterModel {
    // Model metadata
    metadata: ModelMetadata {
        part_number: "BFU730F",
        vendor: "Infineon",
        bias: BiasCondition { vce: 2.0, ic: 10e-3 },
    },
    
    // Frequency data
    data: SParamData {
        format: Format::Touchstone,
        frequencies: vec![/* 0.1 - 12 GHz */],
        s11: vec![/* complex values */],
        s21: vec![/* complex values */],
        s12: vec![/* complex values */],
        s22: vec![/* complex values */],
    },
    
    // Noise parameters
    noise: NoiseParams {
        nf_min: vec![/* vs frequency */],
        gamma_opt: vec![/* optimal source reflection */],
        rn: vec![/* noise resistance */],
    },
}
```

## Rust API

```rust
// Load RF component library
let lib = RFComponentLibrary::load("rf_transistors")?;

// Find components
let lna_transistors = lib.find(ComponentQuery {
    category: Category::Transistor,
    frequency_min: 2.4e9,
    nf_max: 1.5,
})?;

// Get S-parameters
let bfu730f = lib.get_component("BFU730F")?;
let s_params = bfu730f.get_s_parameters(BiasCondition {
    vce: 2.0,
    ic: 10e-3,
})?;

// Use in schematic
schematic.add_component(&bfu730f, Position::new(100, 100))?;
```

## Related Topics

- [Hierarchical RF Blocks](./hierarchical-rf-blocks.md)
- [Transmission Lines & Connectivity](./transmission-lines-and-connectivity.md)
