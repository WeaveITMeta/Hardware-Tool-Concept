# Assembly Data Generation

## Overview

Hardware Tool generates comprehensive assembly data for advanced packaging, including die placement, bump bonding, wire bonding, and underfill specifications.

## Assembly Data Types

```rust
AssemblyDataGeneration {
    // Die placement
    die_placement: DiePlacement {
        format: PlacementFormat::CSV,
        coordinates: CoordinateSystem::PackageCenter,
        include: vec![
            PlacementData::Position,
            PlacementData::Rotation,
            PlacementData::DieName,
            PlacementData::Technology,
        ],
    },
    
    // Bump bonding
    bump_bonding: BumpBonding {
        format: BondingFormat::CSV,
        include: vec![
            BondingData::BumpID,
            BondingData::Position,
            BondingData::NetName,
            BondingData::BumpType,
        ],
    },
    
    // Underfill
    underfill: UnderfillSpec {
        regions: true,
        material: true,
        dispense_pattern: true,
    },
}
```

## Assembly Data UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Assembly Data Generation                                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Output Directory: [assembly_data/                      ] [...]  │
│                                                                 │
│ Data Types:                                                     │
│ ☑ Die placement file (die_placement.csv)                       │
│ ☑ Bump bonding map (bump_map.csv)                              │
│ ☑ Underfill specification (underfill_spec.csv)                 │
│ ☑ Assembly drawing (assembly_drawing.pdf)                      │
│ ☐ Wire bond diagram (N/A)                                      │
│                                                                 │
│ Die Placement Preview:                                          │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Die       │ X (mm)  │ Y (mm)  │ Rot (°) │ Tech             │ │
│ │ ──────────┼─────────┼─────────┼─────────┼───────────────── │ │
│ │ Compute   │   0.000 │   0.000 │    0    │ TSMC N5          │ │
│ │ HBM3_1    │ -12.000 │   0.000 │    0    │ SK Hynix HBM3    │ │
│ │ HBM3_2    │  12.000 │   0.000 │    0    │ SK Hynix HBM3    │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Bump Statistics:                                                │
│   Total bumps: 8,192                                           │
│   Signal: 6,144    Power: 1,024    Ground: 1,024              │
│                                                                 │
│ Underfill Regions:                                              │
│   Compute die: Capillary flow, Material: UF-3808              │
│   HBM stacks: Pre-applied, Material: NCF-200                  │
│                                                                 │
│ [Preview Drawing] [Generate All] [Close]                        │
└─────────────────────────────────────────────────────────────────┘
```

## Generated Files

### Die Placement (CSV)

```csv
# Die Placement Data
# Package: HPC_Accelerator
# Date: 2026-01-19
Die_Name,X_mm,Y_mm,Rotation_deg,Technology,Size_X_mm,Size_Y_mm,Bump_Count
Compute,0.000,0.000,0,TSMC_N5,10.0,10.0,4096
HBM3_1,-12.000,0.000,0,SK_HBM3,8.0,10.0,2048
HBM3_2,12.000,0.000,0,SK_HBM3,8.0,10.0,2048
```

### Bump Map (CSV)

```csv
# Bump Bonding Map
# Die: Compute
Bump_ID,X_um,Y_um,Net_Name,Type,Diameter_um,Pitch_um
B0001,27.5,27.5,VDD_CORE,Power,25,55
B0002,82.5,27.5,VDD_CORE,Power,25,55
B0003,137.5,27.5,VSS,Ground,25,55
B0004,192.5,27.5,UCIe_TX_0,Signal,25,55
...
```

## Rust API

```rust
// Generate assembly data
let system = project.get_system("hpc_accelerator")?;

// Die placement
system.export_die_placement(DiePlacementConfig {
    output: "assembly_data/die_placement.csv",
    coordinate_system: CoordinateSystem::PackageCenter,
})?;

// Bump map
system.export_bump_map(BumpMapConfig {
    output: "assembly_data/bump_map.csv",
    include_net_names: true,
})?;

// Underfill spec
system.export_underfill_spec(UnderfillConfig {
    output: "assembly_data/underfill_spec.csv",
})?;

// Assembly drawing
system.export_assembly_drawing(DrawingConfig {
    output: "assembly_data/assembly_drawing.pdf",
    include_dimensions: true,
    include_cross_section: true,
})?;
```

## Related Topics

- [ODB++ Packaging Export](./odb-packaging-export.md)
- [Fabrication Drawings with Stackups](./fabrication-drawings-with-stackups.md)
