# Die IP & Libraries

## Overview

Hardware Tool provides comprehensive die and chiplet IP libraries for advanced packaging design, including compute dies, memory stacks, I/O chiplets, and analog/mixed-signal dies.

## Die Library Structure

```rust
DieLibrary {
    // Die categories
    categories: vec![
        DieCategory::Compute,          // CPU, GPU, accelerators
        DieCategory::Memory,           // HBM, DDR, SRAM
        DieCategory::IO,               // SerDes, PHY, interfaces
        DieCategory::Analog,           // ADC, DAC, PLL
        DieCategory::Power,            // PMIC, voltage regulators
    ],
    
    // Die properties
    properties: DieProperties {
        technology_node: true,
        die_size: true,
        bump_map: true,
        power_profile: true,
        thermal_profile: true,
    },
    
    // Interface standards
    interfaces: vec![
        Interface::UCIe,
        Interface::HBM3,
        Interface::BoW,
        Interface::AIB,
    ],
}
```

## Die Browser

```
┌─────────────────────────────────────────────────────────────────┐
│ Die IP Browser                                                  │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Search: [compute accelerator                   ] [🔍]           │
│                                                                 │
│ Category: [Compute ▼]  Node: [All ▼]  Interface: [UCIe ▼]      │
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Die Name       │ Node  │ Size (mm²) │ Power │ Interface    │ │
│ │ ───────────────┼───────┼────────────┼───────┼───────────── │ │
│ │ GPU_Compute_A  │ 5nm   │   100      │ 150W  │ UCIe 64-lane │ │
│ │ GPU_Compute_B  │ 5nm   │   150      │ 200W  │ UCIe 128-lane│ │
│ │ AI_Accelerator │ 3nm   │    80      │ 100W  │ UCIe 64-lane │ │
│ │ CPU_Core_Die   │ 5nm   │    50      │  75W  │ UCIe 32-lane │ │
│ │ NPU_Inference  │ 7nm   │    40      │  50W  │ UCIe 32-lane │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Selected: GPU_Compute_A                                         │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ ┌─────────────────────────────────────┐                     │ │
│ │ │ ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● │ UCIe bumps           │ │
│ │ │                                     │                     │ │
│ │ │         GPU_Compute_A               │ 10mm × 10mm        │ │
│ │ │           5nm TSMC                  │                     │ │
│ │ │           150W TDP                  │                     │ │
│ │ │                                     │                     │ │
│ │ │ ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● │ Power/GND bumps      │ │
│ │ └─────────────────────────────────────┘                     │ │
│ │                                                             │ │
│ │ Bump count: 4,096    Pitch: 55 µm    Interface: UCIe Std   │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [View Bump Map] [View Datasheet] [Insert] [Close]               │
└─────────────────────────────────────────────────────────────────┘
```

## Die Definition

```rust
DieDefinition {
    // Basic info
    info: DieInfo {
        name: "GPU_Compute_A",
        vendor: "Custom",
        technology: "TSMC N5",
        die_size: (10e-3, 10e-3),      // 10mm × 10mm
    },
    
    // Bump map
    bump_map: BumpMap {
        pitch: 55e-6,                   // 55 µm
        array: BumpArray::Grid { rows: 64, cols: 64 },
        bump_type: BumpType::MicroBump,
        zones: vec![
            BumpZone { name: "UCIe", rows: (0, 16), function: Function::Signal },
            BumpZone { name: "Power", rows: (16, 48), function: Function::Power },
            BumpZone { name: "UCIe", rows: (48, 64), function: Function::Signal },
        ],
    },
    
    // Power profile
    power: PowerProfile {
        tdp: 150.0,                     // W
        peak: 200.0,                    // W
        idle: 10.0,                     // W
        power_map: PowerMap::Uniform,
    },
    
    // Thermal profile
    thermal: ThermalProfile {
        max_junction: 105.0,            // °C
        theta_jc: 0.1,                  // °C/W
    },
}
```

## Rust API

```rust
// Load die library
let lib = DieLibrary::load("chiplet_ip")?;

// Find dies
let compute_dies = lib.find(DieQuery {
    category: Category::Compute,
    interface: Some(Interface::UCIe),
    max_power: Some(200.0),
})?;

// Get die details
let gpu_die = lib.get_die("GPU_Compute_A")?;
println!("Size: {}mm × {}mm", gpu_die.size.0 * 1000.0, gpu_die.size.1 * 1000.0);
println!("Bump count: {}", gpu_die.bump_map.count());

// Add to system
system.add_die(&gpu_die, Position::center())?;
```

## Related Topics

- [Hierarchical Multi-Die Assemblies](./hierarchical-multi-die-assemblies.md)
- [TSV & Microbump Connectivity](./tsv-and-microbump-connectivity.md)
