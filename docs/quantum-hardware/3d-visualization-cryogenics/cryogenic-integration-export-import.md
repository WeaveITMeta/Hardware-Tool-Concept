# Cryogenic Integration Export/Import

## Overview

Hardware Tool supports integration with cryogenic systems, including dilution refrigerator mounting, thermal anchoring, and RF/DC wiring specifications.

## Cryogenic Configuration

```rust
CryogenicIntegration {
    // Refrigerator stages
    stages: vec![
        CryoStage { name: "Room temp", temperature: 300.0, power_budget: 1000.0 },
        CryoStage { name: "50K", temperature: 50.0, power_budget: 40.0 },
        CryoStage { name: "4K", temperature: 4.0, power_budget: 1.5 },
        CryoStage { name: "Still", temperature: 0.8, power_budget: 0.025 },
        CryoStage { name: "CP", temperature: 0.1, power_budget: 0.0004 },
        CryoStage { name: "MXC", temperature: 0.02, power_budget: 0.00001 },
    ],
    
    // Chip mounting
    mounting: ChipMounting {
        stage: "MXC",
        holder_type: HolderType::PCB,
        thermal_contact: ThermalContact::GoldWireBond,
    },
    
    // Wiring
    wiring: CryoWiring {
        rf_lines: vec![
            RFLine { name: "XY_Q0", attenuation: vec![20, 10, 6, 3, 0] },
            RFLine { name: "Readout", attenuation: vec![0, 0, 0, 0, 0] },
        ],
        dc_lines: vec![
            DCLine { name: "Flux_Q0", filtering: Filtering::RCPi },
        ],
    },
}
```

## Integration UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Cryogenic Integration: 5-Qubit Processor                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Dilution Refrigerator: [Bluefors LD400 ▼]                      │
│                                                                 │
│ Thermal Stages:                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Stage    │ Temp   │ Power Budget │ Heat Load │ Status       │ │
│ │ ─────────┼────────┼──────────────┼───────────┼───────────── │ │
│ │ 300K     │ 300 K  │    1000 W    │   50 W    │ ✓ OK        │ │
│ │ 50K      │  50 K  │      40 W    │   15 W    │ ✓ OK        │ │
│ │ 4K       │   4 K  │     1.5 W    │  0.8 W    │ ✓ OK        │ │
│ │ Still    │ 800 mK │      25 mW   │   12 mW   │ ✓ OK        │ │
│ │ CP       │ 100 mK │     400 µW   │  150 µW   │ ✓ OK        │ │
│ │ MXC      │  20 mK │      10 µW   │    5 µW   │ ✓ OK        │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ RF Wiring:                                                      │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Line      │ Type    │ Attenuation (dB per stage)            │ │
│ │ ──────────┼─────────┼────────────────────────────────────── │ │
│ │ XY_Q0     │ Drive   │ 20 → 10 → 6 → 3 → 0 (Total: 39 dB)   │ │
│ │ XY_Q1     │ Drive   │ 20 → 10 → 6 → 3 → 0 (Total: 39 dB)   │ │
│ │ Readout   │ Measure │  0 →  0 → 0 → 0 → 0 (Total:  0 dB)   │ │
│ │ Flux_Q0   │ DC      │ RC-π filter at MXC                    │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Chip Mounting:                                                  │
│   Stage: MXC (20 mK)                                           │
│   Holder: PCB with gold wire bonds                             │
│   Thermal resistance: 0.5 K/W                                  │
│                                                                 │
│ [Calculate Heat Load] [Export Wiring Diagram] [Export BOM]      │
└─────────────────────────────────────────────────────────────────┘
```

## Export Formats

```rust
CryoExportFormats {
    // Wiring diagram
    wiring_diagram: WiringDiagram {
        format: DiagramFormat::PDF,
        include_attenuation: true,
        include_filters: true,
    },
    
    // Bill of materials
    bom: CryoBOM {
        include_attenuators: true,
        include_cables: true,
        include_filters: true,
        include_connectors: true,
    },
    
    // Thermal model
    thermal_model: ThermalModel {
        format: ModelFormat::COMSOL,
        include_wiring_heat_load: true,
    },
}
```

## Rust API

```rust
// Configure cryogenic integration
let cryo = CryogenicIntegration::new(Refrigerator::BlueforsLD400)?;

// Add chip mounting
cryo.set_chip_mounting(ChipMounting {
    stage: Stage::MXC,
    holder: HolderType::PCB,
})?;

// Configure wiring
cryo.add_rf_line("XY_Q0", RFLineConfig {
    attenuation: vec![20, 10, 6, 3, 0],
    cable_type: CableType::NbTi,
})?;

// Calculate heat load
let heat_load = cryo.calculate_heat_load()?;
println!("MXC heat load: {} µW", heat_load.mxc * 1e6);

// Export
cryo.export_wiring_diagram("wiring.pdf")?;
cryo.export_bom("cryo_bom.csv")?;
```

## Related Topics

- [3D Qubit Viewer](./3d-qubit-viewer.md)
- [Qubit Calibration Data](../manufacturing-output/qubit-calibration-data-generation.md)
