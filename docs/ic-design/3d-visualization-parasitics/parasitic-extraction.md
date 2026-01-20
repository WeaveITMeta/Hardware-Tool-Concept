# Parasitic Extraction

## Overview

Hardware Tool provides integrated parasitic extraction capabilities for IC designs, extracting resistance, capacitance, and inductance from physical layouts for accurate post-layout simulation and timing analysis.

> **Inherits from:** [Shared Simulation Architecture](../../advanced-features/shared-simulation-architecture.md)
>
> This documentation covers IC-specific parasitic extraction. All standard job management, results visualization, and export capabilities are inherited from the shared architecture.

---

## Extraction Modes

| Mode | Speed | Accuracy | Use Case |
|------|-------|----------|----------|
| **RC** | Fast | Good | Digital timing |
| **RCC** | Medium | Better | Mixed-signal |
| **RLC** | Slow | Best | High-frequency, inductance-sensitive |

---

## Extraction Configuration

```rust
ParasiticExtraction {
    // Extraction mode
    mode: ExtractionMode::RCC,
    
    // Scope
    scope: ExtractionScope {
        nets: NetSelection::All,      // or Specific(vec!["CLK", "DATA*"])
        cells: CellSelection::All,
        hierarchical: true,
    },
    
    // Accuracy settings
    accuracy: AccuracySettings {
        coupling_threshold: 0.01,     // fF - ignore smaller caps
        resistance_threshold: 0.1,    // Ohms
        max_coupling_distance: 5.0,   // um
    },
    
    // Technology
    technology: TechConfig {
        pdk: "sky130",
        corner: "tt",                 // typical-typical
        temperature: 25.0,
    },
}
```

---

## Extraction UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Parasitic Extraction                                     [✕]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Layout: opamp_v2.hwt_ic                                        │
│ PDK: sky130 (tt corner, 25°C)                                  │
│                                                                 │
│ Extraction Mode:                                                │
│ ○ RC (fast)    ● RCC (balanced)    ○ RLC (accurate)           │
│                                                                 │
│ Scope:                                                          │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Nets:  ● All    ○ Selected    ○ Critical only              │ │
│ │ Cells: ● All    ○ Top-level   ○ Selected                   │ │
│ │ ☑ Hierarchical extraction                                  │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Accuracy:                                                       │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Coupling threshold:    [0.01 fF____]                       │ │
│ │ Resistance threshold:  [0.1 Ω______]                       │ │
│ │ Max coupling distance: [5.0 μm_____]                       │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Output Format: [SPEF ▼]  ☑ DSPF  ☐ Detailed report            │
│                                                                 │
│ [Extract] [Cancel]                                             │
└─────────────────────────────────────────────────────────────────┘
```

---

## Output Formats

### SPEF (Standard Parasitic Exchange Format)

```spef
*SPEF "IEEE 1481-2009"
*DESIGN "opamp_v2"
*DATE "2026-01-20"
*VENDOR "Hardware Tool"
*PROGRAM "HWT Parasitic Extractor"
*VERSION "1.0"
*DESIGN_FLOW "NETLIST_TYPE_VERILOG"
*DIVIDER /
*DELIMITER :
*BUS_DELIMITER [ ]
*T_UNIT 1 PS
*C_UNIT 1 FF
*R_UNIT 1 OHM
*L_UNIT 1 HENRY

*NAME_MAP
*1 VDD
*2 VSS
*3 OUT
*4 INP
*5 INM

*D_NET *3 1.234
*CONN
*P *3 O
*I *1:Q O *D sky130_fd_sc_hd__inv_1
*CAP
1 *3:1 0.045
2 *3:2 0.032
3 *3:1 *2 0.012
*RES
1 *3:1 *3:2 2.5
*END
```

### DSPF (Detailed Standard Parasitic Format)

```dspf
* DSPF file for opamp_v2
* Extracted by Hardware Tool

.SUBCKT opamp_v2 VDD VSS OUT INP INM

* Net: OUT
ROUT_1 OUT OUT_1 2.5
COUT_1 OUT_1 VSS 0.045fF
COUT_2 OUT_1 VDD 0.012fF
ROUT_2 OUT_1 OUT_2 1.8
COUT_3 OUT_2 VSS 0.032fF

.ENDS
```

---

## Extraction Results

```
┌─────────────────────────────────────────────────────────────────┐
│ Extraction Results                                       [✕]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Extraction completed in 12.5 seconds                           │
│                                                                 │
│ Summary:                                                        │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Nets extracted:        1,245                               │ │
│ │ Total resistance:      45.2 kΩ                             │ │
│ │ Total capacitance:     12.8 pF                             │ │
│ │ Coupling capacitance:  3.2 pF                              │ │
│ │ Parasitic elements:    28,456                              │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Critical Nets (by delay impact):                               │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Net      │ R (Ω)  │ C (fF) │ Delay (ps) │ Status           │ │
│ │ ─────────┼────────┼────────┼────────────┼───────────────── │ │
│ │ CLK      │ 125.3  │ 85.2   │ 45.2       │ ⚠ High           │ │
│ │ DATA[0]  │ 89.5   │ 62.1   │ 32.1       │ ⚠ Medium         │ │
│ │ VREF     │ 234.1  │ 28.5   │ 28.0       │ ● OK             │ │
│ │ OUT      │ 45.2   │ 42.3   │ 18.5       │ ● OK             │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [View in 3D] [Export SPEF] [Run Timing] [Close]                │
└─────────────────────────────────────────────────────────────────┘
```

---

## Back-Annotation

```rust
BackAnnotation {
    // Source
    parasitic_file: "extraction.spef",
    
    // Target
    netlist: "design.v",
    
    // Options
    options: BackAnnotationOptions {
        annotate_delays: true,
        annotate_slews: true,
        create_sdf: true,
        
        // Reduction
        reduce_parasitics: true,
        reduction_method: ReductionMethod::AWE,
        poles: 3,
    },
}
```

---

## CLI Commands

```bash
# Run extraction
hwt ic extract layout.hwt_ic --mode rcc --output extraction.spef

# Extract specific nets
hwt ic extract layout.hwt_ic --nets "CLK,DATA*" --output critical.spef

# Extract with options
hwt ic extract layout.hwt_ic --mode rlc --coupling-threshold 0.001

# Back-annotate
hwt ic backannotate design.v --spef extraction.spef --output design_annotated.v

# Generate SDF
hwt ic extract layout.hwt_ic --sdf timing.sdf
```

---

## Rust API

```rust
use hardware_tool::ic::extraction::*;

// Configure extraction
let config = ExtractionConfig {
    mode: ExtractionMode::RCC,
    scope: ExtractionScope::all(),
    accuracy: AccuracySettings::default(),
};

// Run extraction
let result = layout.extract_parasitics(config)?;

// Get summary
println!("Total R: {} kΩ", result.total_resistance() / 1000.0);
println!("Total C: {} pF", result.total_capacitance() * 1e12);

// Export SPEF
result.export_spef("extraction.spef")?;

// Get critical nets
for net in result.critical_nets(10) {
    println!("{}: R={:.1}Ω, C={:.1}fF, delay={:.1}ps",
        net.name, net.resistance, net.capacitance * 1e15, net.delay * 1e12);
}
```

---

## Related Topics

- [3D IC Viewer](./3d-ic-viewer.md)
- [Physical Verification (DRC/LVS)](../analog-mixed-signal/physical-verification-drc-lvs.md)
- [Timing Analysis](../rtl-logic-design/timing-analysis.md)
- [Shared Simulation Architecture](../../advanced-features/shared-simulation-architecture.md)
