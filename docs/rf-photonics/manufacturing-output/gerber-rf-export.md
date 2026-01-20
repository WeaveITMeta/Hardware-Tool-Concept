# Gerber RF Export

## Overview

Hardware Tool exports RF PCB designs in Gerber RS-274X format with RF-specific optimizations including controlled impedance annotations, via fence patterns, and high-frequency design notes.

## RF Export Configuration

```rust
RFGerberExport {
    // Standard Gerber settings
    format: GerberFormat {
        version: RS274X,
        coordinates: Coordinates::Absolute,
        units: Units::Metric,
        precision: (4, 6),             // 4.6 format
    },
    
    // RF-specific options
    rf_options: RFExportOptions {
        include_impedance_notes: true,
        include_stackup_info: true,
        controlled_impedance_layers: vec!["Top", "L2", "L3", "Bottom"],
        via_fence_annotation: true,
    },
    
    // Layer mapping
    layers: RFLayerMapping {
        signal_layers: vec!["Top", "L2", "L3", "Bottom"],
        ground_planes: vec!["L1_GND", "L4_GND"],
        rf_keepout: "RF_KEEPOUT",
    },
}
```

## Export UI

```
┌─────────────────────────────────────────────────────────────────┐
│ RF Gerber Export                                                │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Output Directory: [./gerber_rf/                        ] [...]  │
│                                                                 │
│ Layer Selection:                                                │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Layer        │ Type      │ Impedance │ Export               │ │
│ │ ─────────────┼───────────┼───────────┼───────────────────── │ │
│ │ Top          │ Signal    │ 50Ω µstrip│ ☑                   │ │
│ │ L1_GND       │ Ground    │ -         │ ☑                   │ │
│ │ L2           │ Signal    │ 50Ω strip │ ☑                   │ │
│ │ L3           │ Signal    │ 50Ω strip │ ☑                   │ │
│ │ L4_GND       │ Ground    │ -         │ ☑                   │ │
│ │ Bottom       │ Signal    │ 50Ω µstrip│ ☑                   │ │
│ │ Solder Mask  │ Mask      │ -         │ ☑                   │ │
│ │ Silkscreen   │ Silk      │ -         │ ☑                   │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ RF Options:                                                     │
│ ☑ Include impedance control notes                              │
│ ☑ Include stackup specification                                │
│ ☑ Annotate via fences                                          │
│ ☑ Include RF design rules                                      │
│                                                                 │
│ Stackup Info:                                                   │
│   Substrate: Rogers RO4003C (εr=3.55, tan δ=0.0027)           │
│   50Ω microstrip: 1.12mm width on 0.508mm substrate           │
│   50Ω stripline: 0.45mm width between ground planes           │
│                                                                 │
│ [Cancel]                                    [Export Gerbers]    │
└─────────────────────────────────────────────────────────────────┘
```

## Fabrication Notes

```rust
RFFabricationNotes {
    // Impedance control
    impedance: ImpedanceNotes {
        controlled_impedance: true,
        target_impedance: 50.0,
        tolerance: 10.0,               // ±10%
        test_coupons: true,
    },
    
    // Material specification
    material: MaterialSpec {
        substrate: "Rogers RO4003C",
        dielectric_constant: 3.55,
        loss_tangent: 0.0027,
        copper_weight: "1 oz",
    },
    
    // Special instructions
    instructions: vec![
        "Controlled impedance: 50Ω ±10% on layers Top, Bottom",
        "Use TDR measurement for impedance verification",
        "Via fence spacing: 0.5mm max for RF isolation",
    ],
}
```

## Rust API

```rust
// Export RF Gerbers
let layout = project.get_layout("rf_frontend")?;

layout.export_gerber_rf(RFGerberConfig {
    output_dir: "gerber_rf/",
    include_impedance_notes: true,
    include_stackup: true,
    substrate: Substrate::Rogers_RO4003C,
})?;

// Generate fabrication notes
layout.generate_rf_fab_notes("gerber_rf/fab_notes.txt")?;
```

## Related Topics

- [S-Parameter Data Generation](./s-parameter-data-generation.md)
- [Fabrication Drawings with Tolerances](./fabrication-drawings-with-tolerances.md)
