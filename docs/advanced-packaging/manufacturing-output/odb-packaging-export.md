# ODB++ Packaging Export

## Overview

Hardware Tool exports advanced packaging designs in ODB++ format, supporting substrate layouts, RDL layers, and assembly data for OSAT (Outsourced Semiconductor Assembly and Test) facilities.

## Export Configuration

```rust
ODBPackagingExport {
    // Format settings
    format: ODBFormat {
        version: "8.1",
        compression: Compression::Gzip,
    },
    
    // Layer mapping
    layers: PackagingLayerMapping {
        rdl_layers: vec!["RDL_M1", "RDL_M2", "RDL_M3", "RDL_M4"],
        substrate_layers: vec!["SUB_L1", "SUB_L2", "SUB_L3", "SUB_L4"],
        solder_mask: vec!["SM_TOP", "SM_BOT"],
        bump_layers: vec!["UBUMP", "C4_BUMP", "BGA"],
    },
    
    // Assembly data
    assembly: AssemblyData {
        die_placement: true,
        bump_map: true,
        wire_bond: true,
        underfill: true,
    },
    
    // Metadata
    metadata: PackageMetadata {
        package_type: "FCBGA",
        body_size: (55e-3, 55e-3),
        ball_count: 2500,
        ball_pitch: 1.0e-3,
    },
}
```

## Export UI

```
┌─────────────────────────────────────────────────────────────────┐
│ ODB++ Packaging Export                                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Output: [hpc_accelerator_odb/                          ] [...]  │
│                                                                 │
│ Package Type: [FCBGA ▼]  Body: 55mm × 55mm                     │
│                                                                 │
│ Layer Selection:                                                │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Layer        │ Type       │ Thickness │ Export              │ │
│ │ ─────────────┼────────────┼───────────┼──────────────────── │ │
│ │ RDL_M4       │ RDL        │ 2 µm      │ ☑                  │ │
│ │ RDL_M3       │ RDL        │ 2 µm      │ ☑                  │ │
│ │ RDL_M2       │ RDL        │ 2 µm      │ ☑                  │ │
│ │ RDL_M1       │ RDL        │ 2 µm      │ ☑                  │ │
│ │ SUB_L1       │ Substrate  │ 15 µm     │ ☑                  │ │
│ │ SUB_L2       │ Substrate  │ 15 µm     │ ☑                  │ │
│ │ UBUMP        │ Bump       │ 25 µm     │ ☑                  │ │
│ │ BGA          │ Ball       │ 400 µm    │ ☑                  │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Assembly Data:                                                  │
│ ☑ Die placement coordinates                                    │
│ ☑ Bump map and net assignment                                  │
│ ☑ Underfill regions                                            │
│ ☐ Wire bond diagram (N/A for flip-chip)                        │
│                                                                 │
│ Die Information:                                                │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Die          │ Position    │ Size      │ Bumps              │ │
│ │ ─────────────┼─────────────┼───────────┼─────────────────── │ │
│ │ Compute      │ (0, 0)      │ 10×10 mm  │ 4,096              │ │
│ │ HBM3_1       │ (-12, 0)    │ 8×10 mm   │ 2,048              │ │
│ │ HBM3_2       │ (12, 0)     │ 8×10 mm   │ 2,048              │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [Cancel]                                    [Export ODB++]      │
└─────────────────────────────────────────────────────────────────┘
```

## Rust API

```rust
// Export to ODB++
let system = project.get_system("hpc_accelerator")?;

system.export_odb(ODBPackagingConfig {
    output_dir: "hpc_accelerator_odb/",
    include_rdl: true,
    include_substrate: true,
    include_assembly: true,
})?;

// Verify export
let stats = system.odb_stats()?;
println!("Layers: {}", stats.layer_count);
println!("Nets: {}", stats.net_count);
println!("Components: {}", stats.component_count);
```

## Related Topics

- [APD Export](./apd-export.md)
- [Assembly Data Generation](./assembly-data-generation.md)
