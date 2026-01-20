# GDSII Quantum Export

## Overview

Hardware Tool exports quantum processor layouts in GDSII format optimized for superconducting circuit fabrication. Supports single and multi-layer processes with Josephson junction definitions.

## Export Configuration

```rust
QuantumGDSIIExport {
    // Format settings
    format: GDSIIFormat {
        version: 6,
        units: GDSUnits {
            user_unit: 1e-6,           // 1 µm
            database_unit: 1e-9,       // 1 nm
        },
    },
    
    // Layer mapping for superconducting process
    layer_map: QuantumLayerMap {
        base_metal: (1, 0),            // Nb ground plane
        wiring: (2, 0),                // Nb wiring layer
        junction_base: (3, 0),         // Junction bottom electrode
        junction_top: (4, 0),          // Junction top electrode
        bandage: (5, 0),               // Bandage layer
        resist: (10, 0),               // E-beam resist
    },
    
    // Junction handling
    junctions: JunctionExport {
        include_dose_layers: true,
        dose_modulation: vec![
            DoseLayer { layer: (20, 0), dose_factor: 1.0 },
            DoseLayer { layer: (21, 0), dose_factor: 0.8 },
        ],
    },
}
```

## Export UI

```
┌─────────────────────────────────────────────────────────────────┐
│ GDSII Export: Quantum Processor                                 │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Output File: [5_qubit_processor.gds                    ] [...]  │
│                                                                 │
│ Top Cell: [processor_top                         ▼]             │
│                                                                 │
│ Process: [Single-layer Al ▼]                                   │
│                                                                 │
│ Layer Mapping:                                                  │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Internal Layer    │ GDS Layer │ Description                 │ │
│ │ ──────────────────┼───────────┼──────────────────────────── │ │
│ │ base_metal        │   1:0     │ Ground plane (Al/Nb)        │ │
│ │ wiring            │   2:0     │ CPW traces                  │ │
│ │ junction_base     │   3:0     │ JJ bottom electrode         │ │
│ │ junction_top      │   4:0     │ JJ top electrode            │ │
│ │ bandage           │   5:0     │ Bandage connections         │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Options:                                                        │
│ ☑ Include junction dose layers                                 │
│ ☑ Fracture for e-beam                                          │
│ ☑ Include alignment marks                                      │
│ ☐ Flatten hierarchy                                            │
│                                                                 │
│ Statistics:                                                     │
│   Qubits: 5    Resonators: 5    Junctions: 10                  │
│   Total polygons: 12,345                                       │
│                                                                 │
│ [Cancel]                                    [Export GDSII]      │
└─────────────────────────────────────────────────────────────────┘
```

## Rust API

```rust
// Export quantum layout to GDSII
let processor = project.get_processor("5_qubit")?;

processor.export_gdsii(QuantumGDSIIConfig {
    path: "output/5_qubit_processor.gds",
    top_cell: "processor_top",
    process: Process::SingleLayerAl,
    include_junctions: true,
    fracture_for_ebeam: true,
})?;

// Verify export
let stats = processor.gdsii_stats()?;
println!("Junctions: {}", stats.junction_count);
println!("Polygons: {}", stats.polygon_count);
```

## Related Topics

- [CIF Export](./cif-export.md)
- [Fabrication Drawings](./fabrication-drawings-for-foundries.md)
