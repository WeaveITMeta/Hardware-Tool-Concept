# Compatibility Strategy

## Overview

Hardware Tool provides comprehensive import, export, and conversion capabilities for all major EDA tools. This enables seamless migration from existing tools, collaboration with teams using different software, and integration into established workflows.

---

## Compatibility Matrix

### Legend

| Symbol | Meaning |
|:------:|---------|
| ✅ | Full support |
| ⚠️ | Partial/limited support |
| ❌ | Not supported |

---

### PCB Design Tools

| Tool                        | Import      | Export      | Live Edit   | Convert         |
|:----------------------------|:-----------:|:-----------:|:-----------:|:---------------:|
| KiCad 7/8                   | ✅ Full     | ✅ Full     | ✅ Yes      | ✅ Bidirectional |
| Altium Designer             | ✅ Full     | ✅ Full     | ⚠️ Limited  | ✅ Bidirectional |
| Eagle                       | ✅ Full     | ✅ Full     | ❌ No       | ✅ To HWT        |
| OrCAD / Allegro             | ✅ Full     | ⚠️ Partial  | ❌ No       | ✅ To HWT        |
| TSCircuit                   | ✅ Native   | ✅ Native   | ✅ Yes      | ✅ Bidirectional |
| EasyEDA                     | ✅ Full     | ⚠️ Partial  | ❌ No       | ✅ To HWT        |
| Fusion 360 Electronics      | ✅ Full     | ⚠️ Partial  | ❌ No       | ✅ To HWT        |

---

### IC Design Tools

| Tool                        | Import      | Export      | Live Edit   | Convert         |
|:----------------------------|:-----------:|:-----------:|:-----------:|:---------------:|
| Cadence Virtuoso            | ✅ OA/GDSII | ✅ GDSII    | ❌ No       | ✅ To HWT        |
| Synopsys Custom Compiler    | ✅ OA/GDSII | ✅ GDSII    | ❌ No       | ✅ To HWT        |
| Magic VLSI                  | ✅ Full     | ✅ Full     | ⚠️ Limited  | ✅ Bidirectional |
| Klayout                     | ✅ Full     | ✅ Full     | ⚠️ Limited  | ✅ Bidirectional |
| OpenROAD                    | ✅ LEF/DEF  | ✅ LEF/DEF  | ❌ No       | ✅ Bidirectional |

---

### RF / Microwave Tools

| Tool                        | Import      | Export      | Live Edit   | Convert         |
|:----------------------------|:-----------:|:-----------:|:-----------:|:---------------:|
| Keysight ADS                | ✅ Full     | ⚠️ Partial  | ❌ No       | ✅ To HWT        |
| AWR Microwave Office        | ✅ Full     | ⚠️ Partial  | ❌ No       | ✅ To HWT        |
| Ansys HFSS                  | ⚠️ Geometry | ⚠️ Geometry | ❌ No       | ⚠️ Partial       |
| CST Studio                  | ⚠️ Geometry | ⚠️ Geometry | ❌ No       | ⚠️ Partial       |
| Sonnet                      | ✅ Full     | ✅ Full     | ❌ No       | ✅ Bidirectional |

---

### Quantum Design Tools

| Tool                        | Import      | Export      | Live Edit   | Convert         |
|:----------------------------|:-----------:|:-----------:|:-----------:|:---------------:|
| Qiskit Metal                | ✅ Full     | ✅ Full     | ⚠️ Limited  | ✅ Bidirectional |
| KQCircuits                  | ✅ Full     | ✅ Full     | ⚠️ Limited  | ✅ Bidirectional |
| Ansys Q3D                   | ⚠️ Geometry | ⚠️ Geometry | ❌ No       | ⚠️ Partial       |

---

### MEMS Tools

| Tool                        | Import      | Export      | Live Edit   | Convert         |
|:----------------------------|:-----------:|:-----------:|:-----------:|:---------------:|
| Coventor MEMS+              | ⚠️ GDSII    | ✅ GDSII    | ❌ No       | ⚠️ Partial       |
| IntelliSense                | ⚠️ GDSII    | ✅ GDSII    | ❌ No       | ⚠️ Partial       |
| L-Edit                      | ✅ Full     | ✅ Full     | ❌ No       | ✅ Bidirectional |

---

## Import Capabilities

### KiCad Import

```rust
KicadImport {
    // Supported versions
    versions: vec!["5.x", "6.x", "7.x", "8.x"],
    
    // File types
    files: KicadFiles {
        project: ".kicad_pro",
        schematic: ".kicad_sch",
        pcb: ".kicad_pcb",
        symbol_lib: ".kicad_sym",
        footprint_lib: ".kicad_mod",
        wrl_models: ".wrl",
        step_models: ".step",
    },
    
    // Import options
    options: ImportOptions {
        preserve_annotations: true,
        import_3d_models: true,
        import_design_rules: true,
        import_net_classes: true,
        convert_custom_shapes: true,
    },
}
```

#### CLI

```bash
# Import KiCad project
hwt import kicad ./my_project.kicad_pro

# Import with options
hwt import kicad ./my_project.kicad_pro --include-3d --preserve-rules

# Import library only
hwt import kicad-lib ./MyLibrary.kicad_sym
```

### Altium Import

```rust
AltiumImport {
    // Supported versions
    versions: vec!["AD19", "AD20", "AD21", "AD22", "AD23", "AD24"],
    
    // File types
    files: AltiumFiles {
        project: ".PrjPcb",
        schematic: ".SchDoc",
        pcb: ".PcbDoc",
        library: ".SchLib", ".PcbLib",
        integrated_lib: ".IntLib",
    },
    
    // Import options
    options: ImportOptions {
        preserve_variants: true,
        import_output_jobs: true,
        import_design_rules: true,
        import_room_definitions: true,
        convert_embedded_models: true,
    },
}
```

#### CLI

```bash
# Import Altium project
hwt import altium ./MyProject.PrjPcb

# Import with variant support
hwt import altium ./MyProject.PrjPcb --variants all
```

### Eagle Import

```rust
EagleImport {
    // Supported versions
    versions: vec!["6.x", "7.x", "8.x", "9.x"],
    
    // File types
    files: EagleFiles {
        schematic: ".sch",
        board: ".brd",
        library: ".lbr",
    },
    
    // Import options
    options: ImportOptions {
        convert_ulp_scripts: false,  // Not supported
        import_design_rules: true,
        import_cam_jobs: true,
    },
}
```

### OrCAD/Allegro Import

```rust
CadenceImport {
    // Supported formats
    formats: vec![
        "OrCAD Capture (.dsn)",
        "Allegro PCB (.brd)",
        "Allegro Library (.dra, .psm)",
        "OpenAccess (.oa)",
    ],
    
    // Import options
    options: ImportOptions {
        preserve_constraints: true,
        import_padstacks: true,
        import_skill_macros: false,  // Not supported
    },
}
```

### TSCircuit Import

```rust
TscircuitImport {
    // Native Circuit JSON
    circuit_json: true,
    
    // TypeScript source
    typescript: TypeScriptImport {
        parse_source: true,
        convert_to_rust: true,      // Optional
        preserve_logic: true,
    },
    
    // Import options
    options: ImportOptions {
        import_npm_packages: true,
        resolve_dependencies: true,
    },
}
```

#### CLI

```bash
# Import Circuit JSON
hwt import tscircuit ./circuit.json

# Import TypeScript source
hwt import tscircuit ./circuit.ts --convert-to-rust
```

---

## Export Capabilities

### KiCad Export

```rust
KicadExport {
    // Target version
    version: "8.0",
    
    // Export options
    options: ExportOptions {
        generate_project: true,
        generate_schematic: true,
        generate_pcb: true,
        generate_libraries: true,
        embed_3d_models: false,     // Reference external
        preserve_custom_fields: true,
    },
}
```

#### CLI

```bash
# Export to KiCad
hwt export kicad ./output/ --version 8

# Export schematic only
hwt export kicad ./output/ --schematic-only
```

### Altium Export

```rust
AltiumExport {
    // Target version
    version: "AD24",
    
    // Export options
    options: ExportOptions {
        generate_project: true,
        generate_variants: true,
        generate_output_jobs: true,
        embed_models: true,
    },
}
```

### Universal Formats

| Format | Export | Import | Notes |
|--------|--------|--------|-------|
| **Gerber RS-274X** | ✅ | ✅ | Manufacturing |
| **Gerber X2** | ✅ | ✅ | Extended attributes |
| **ODB++** | ✅ | ✅ | Full design data |
| **IPC-2581** | ✅ | ✅ | Industry standard |
| **STEP** | ✅ | ✅ | 3D mechanical |
| **GDSII** | ✅ | ✅ | IC masks |
| **OASIS** | ✅ | ✅ | Compressed GDSII |
| **LEF/DEF** | ✅ | ✅ | IC P&R |
| **Touchstone** | ✅ | ✅ | S-parameters |
| **SPICE** | ✅ | ✅ | Simulation |
| **Circuit JSON** | ✅ | ✅ | Native IR |

---

## Live Edit Mode

### Concept

Live Edit allows opening files from other EDA tools directly, editing in Hardware Tool, and saving back to the original format without conversion.

```
┌─────────────────────────────────────────────────────────────────┐
│ Live Edit Mode                                                  │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────┐      ┌─────────────┐      ┌─────────────┐     │
│  │   KiCad     │      │  Hardware   │      │   KiCad     │     │
│  │   .kicad_*  │ ───► │    Tool     │ ───► │   .kicad_*  │     │
│  │   (source)  │      │   (edit)    │      │   (saved)   │     │
│  └─────────────┘      └─────────────┘      └─────────────┘     │
│                                                                 │
│  • No conversion to .hwt format                                │
│  • Changes saved directly to original format                   │
│  • Full Hardware Tool features available                       │
│  • Seamless round-trip                                         │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### Supported Tools

| Tool | Live Edit Support | Limitations |
|------|-------------------|-------------|
| **KiCad** | ✅ Full | None |
| **Altium** | ⚠️ Limited | Some advanced features |
| **TSCircuit** | ✅ Full | None |
| **Eagle** | ❌ No | Convert only |
| **OrCAD** | ❌ No | Convert only |

### CLI

```bash
# Open KiCad file in live edit mode
hwt open ./project.kicad_pro --live

# Open and auto-save on changes
hwt open ./project.kicad_pro --live --auto-save
```

---

## Conversion Workflows

### Full Project Conversion

```rust
ProjectConversion {
    // Source
    source: ConversionSource {
        tool: "kicad",
        path: "./old_project.kicad_pro",
    },
    
    // Target
    target: ConversionTarget {
        format: "hwt",
        path: "./new_project.hwt",
    },
    
    // Options
    options: ConversionOptions {
        // Content
        convert_schematics: true,
        convert_pcb: true,
        convert_libraries: true,
        convert_3d_models: true,
        
        // Metadata
        preserve_history: false,     // Git history not transferable
        preserve_annotations: true,
        preserve_design_rules: true,
        
        // Validation
        validate_after: true,
        generate_report: true,
    },
}
```

### Batch Conversion

```bash
# Convert all KiCad projects in directory
hwt convert batch ./kicad_projects/ --from kicad --to hwt

# Convert with report
hwt convert batch ./projects/ --from altium --to hwt --report conversion.html
```

### Library Conversion

```bash
# Convert KiCad symbol library
hwt convert library ./MySymbols.kicad_sym --to hwt

# Convert Altium integrated library
hwt convert library ./MyLib.IntLib --to hwt

# Convert with footprint mapping
hwt convert library ./symbols.lbr --to hwt --footprint-map ./mapping.json
```

---

## Conversion UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Project Conversion                                       [✕]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Source: [KiCad ▼]  File: [my_project.kicad_pro    ] [Browse]   │
│ Target: [Hardware Tool ▼]                                      │
│                                                                 │
│ Detected Content:                                               │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ ☑ Schematic (3 sheets, 245 components)                     │ │
│ │ ☑ PCB Layout (4 layers, 1,234 traces)                      │ │
│ │ ☑ Symbol Library (45 symbols)                              │ │
│ │ ☑ Footprint Library (52 footprints)                        │ │
│ │ ☑ 3D Models (38 STEP files)                                │ │
│ │ ☑ Design Rules (custom DRC rules)                          │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Options:                                                        │
│ ☑ Preserve annotations and comments                            │
│ ☑ Convert custom fields to properties                          │
│ ☑ Import net classes                                           │
│ ☑ Validate after conversion                                    │
│                                                                 │
│ Warnings:                                                       │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ ⚠ 3 symbols use KiCad-specific features (will be adapted)  │ │
│ │ ⚠ 1 footprint has unsupported pad shape (will approximate) │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [Preview Changes] [Convert] [Cancel]                           │
└─────────────────────────────────────────────────────────────────┘
```

---

## Conversion Report

```
┌─────────────────────────────────────────────────────────────────┐
│ Conversion Report                                        [✕]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Source: my_project.kicad_pro (KiCad 8.0)                       │
│ Target: my_project.hwt (Hardware Tool)                         │
│ Duration: 2.3 seconds                                          │
│                                                                 │
│ Summary:                                                        │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Category          │ Items │ Converted │ Warnings │ Errors  │ │
│ │ ──────────────────┼───────┼───────────┼──────────┼──────── │ │
│ │ Schematics        │ 3     │ 3         │ 0        │ 0       │ │
│ │ Components        │ 245   │ 245       │ 2        │ 0       │ │
│ │ Nets              │ 312   │ 312       │ 0        │ 0       │ │
│ │ PCB Traces        │ 1,234 │ 1,234     │ 0        │ 0       │ │
│ │ Vias              │ 89    │ 89        │ 0        │ 0       │ │
│ │ Zones             │ 4     │ 4         │ 0        │ 0       │ │
│ │ Symbols           │ 45    │ 45        │ 1        │ 0       │ │
│ │ Footprints        │ 52    │ 52        │ 1        │ 0       │ │
│ │ 3D Models         │ 38    │ 38        │ 0        │ 0       │ │
│ │ ──────────────────┼───────┼───────────┼──────────┼──────── │ │
│ │ Total             │ 2,022 │ 2,022     │ 4        │ 0       │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Warnings:                                                       │
│ • R_Custom: Custom symbol shape approximated                   │
│ • U_FPGA: 256-pin BGA footprint simplified                     │
│ • C_Variable: Variable capacitor model not available           │
│ • QFN-48: Thermal pad shape adjusted                           │
│                                                                 │
│ ✓ Conversion successful - no errors                            │
│                                                                 │
│ [Open Project] [View Details] [Export Report] [Close]          │
└─────────────────────────────────────────────────────────────────┘
```

---

## Rust API

```rust
use hardware_tool::compatibility::*;

// Import KiCad project
let project = import_kicad("./project.kicad_pro", ImportOptions::default())?;

// Import Altium project
let project = import_altium("./project.PrjPcb", ImportOptions {
    preserve_variants: true,
    ..Default::default()
})?;

// Export to KiCad
project.export_kicad("./output/", ExportOptions {
    version: KicadVersion::V8,
    ..Default::default()
})?;

// Live edit mode
let live_project = open_live("./project.kicad_pro")?;
live_project.modify(|design| {
    design.add_component(...)?;
    Ok(())
})?;
live_project.save()?;  // Saves back to .kicad_pro

// Batch conversion
let results = convert_batch(
    "./kicad_projects/",
    ConversionConfig {
        source_format: Format::KiCad,
        target_format: Format::Hwt,
        ..Default::default()
    }
)?;

for result in results {
    println!("{}: {} warnings, {} errors",
        result.file, result.warnings.len(), result.errors.len());
}
```

---

## CLI Reference

```bash
# Import commands
hwt import kicad <file>           # Import KiCad project
hwt import altium <file>          # Import Altium project
hwt import eagle <file>           # Import Eagle project
hwt import orcad <file>           # Import OrCAD project
hwt import tscircuit <file>       # Import TSCircuit project
hwt import gdsii <file>           # Import GDSII file
hwt import gerber <dir>           # Import Gerber files

# Export commands
hwt export kicad <output>         # Export to KiCad
hwt export altium <output>        # Export to Altium
hwt export gerber <output>        # Export Gerber files
hwt export gdsii <output>         # Export GDSII
hwt export step <output>          # Export 3D STEP
hwt export circuit-json <output>  # Export Circuit JSON

# Live edit
hwt open <file> --live            # Open in live edit mode

# Conversion
hwt convert <file> --to <format>  # Convert single file
hwt convert batch <dir> --from <format> --to <format>  # Batch convert

# Library conversion
hwt convert library <file> --to <format>
```

---

## Related Topics

- [Comparison with KiCad and TSCircuit](../appendices/comparison-with-kicad-and-tscircuit.md)
- [Circuit JSON IR](./circuit-json-ir.md)
- [Shared Export/Import Architecture](../advanced-features/shared-export-import-architecture.md)
- [Project Structure & Management](./project-structure-management.md)
