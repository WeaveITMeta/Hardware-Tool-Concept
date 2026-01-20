# PDK & Process Integration

## Overview

Hardware Tool provides a **unified Process Design Kit (PDK) integration system** that works across IC, MEMS, Quantum, and RF domains. Whether you're targeting sky130, GF180MCU, TSMC nodes, MEMS foundries, or quantum fabrication facilities — the same PDK browser, installer, and design rule integration handles it all.

> **"One Hardware Tool That Does It All"** — The same PDK management works for any fabrication process.

---

## Supported Domains

| Domain | PDK Type | Examples |
|--------|----------|----------|
| **IC** | Semiconductor foundry | sky130, GF180MCU, TSMC, Intel |
| **MEMS** | MEMS foundry | PolyMUMPs, SOIMUMPs, PiezoMUMPs |
| **Quantum** | Quantum fab | IBM Quantum, Google Sycamore |
| **RF** | RF/MMIC foundry | WIN Semi, Qorvo, UMS |

---

## PDK File Structure

```
pdk_name/
├── pdk.toml                    # PDK metadata
├── tech/
│   ├── layers.toml             # Layer definitions
│   ├── rules.toml              # Design rules
│   └── stack.toml              # Layer stack
├── cells/
│   ├── standard/               # Standard cells (IC)
│   ├── io/                     # I/O cells
│   └── primitives/             # Transistors, resistors, caps
├── models/
│   ├── spice/                  # SPICE models
│   ├── liberty/                # Timing models (.lib)
│   └── verilog/                # Behavioral models
├── symbols/
│   └── *.hwt_sym               # Schematic symbols
├── drc/
│   ├── rules.drc               # DRC rule deck
│   └── antenna.drc             # Antenna rules
├── lvs/
│   └── rules.lvs               # LVS rule deck
└── docs/
    ├── design_manual.pdf
    └── release_notes.md
```

---

## PDK Metadata Format

```toml
# pdk.toml
[pdk]
name = "sky130"
version = "1.0.0"
foundry = "SkyWater"
process = "130nm CMOS"
description = "SkyWater SKY130 open-source PDK"

[pdk.license]
type = "Apache-2.0"
url = "https://github.com/google/skywater-pdk"

[pdk.compatibility]
hardware_tool_version = ">=1.0.0"
domains = ["ic"]

[technology]
node = "130nm"
metal_layers = 5
poly_layers = 1
mim_cap = true
high_res_resistor = true

[variants]
default = "sky130_fd_sc_hd"
available = [
    "sky130_fd_sc_hd",   # High density
    "sky130_fd_sc_hs",   # High speed
    "sky130_fd_sc_lp",   # Low power
    "sky130_fd_sc_hdll", # High density, low leakage
]

[contacts]
foundry_url = "https://www.skywatertechnology.com"
support_email = "pdk-support@skywater.com"
```

---

## Layer Definitions

```toml
# tech/layers.toml
[layers]
# Format: name = { gds_layer, gds_datatype, description, color }

[layers.drawing]
diff    = { layer = 65, datatype = 20, desc = "Diffusion", color = "#FFE4B5" }
poly    = { layer = 66, datatype = 20, desc = "Polysilicon", color = "#FF6B6B" }
li1     = { layer = 67, datatype = 20, desc = "Local interconnect", color = "#4ECDC4" }
met1    = { layer = 68, datatype = 20, desc = "Metal 1", color = "#45B7D1" }
met2    = { layer = 69, datatype = 20, desc = "Metal 2", color = "#96CEB4" }
met3    = { layer = 70, datatype = 20, desc = "Metal 3", color = "#FFEAA7" }
met4    = { layer = 71, datatype = 20, desc = "Metal 4", color = "#DDA0DD" }
met5    = { layer = 72, datatype = 20, desc = "Metal 5", color = "#98D8C8" }

[layers.via]
licon   = { layer = 66, datatype = 44, desc = "LI to poly/diff contact" }
mcon    = { layer = 67, datatype = 44, desc = "Metal 1 contact" }
via1    = { layer = 68, datatype = 44, desc = "Via 1" }
via2    = { layer = 69, datatype = 44, desc = "Via 2" }
via3    = { layer = 70, datatype = 44, desc = "Via 3" }
via4    = { layer = 71, datatype = 44, desc = "Via 4" }

[layers.implant]
nwell   = { layer = 64, datatype = 20, desc = "N-well" }
pwell   = { layer = 64, datatype = 44, desc = "P-well" }
nsdm    = { layer = 93, datatype = 44, desc = "N+ source/drain" }
psdm    = { layer = 94, datatype = 20, desc = "P+ source/drain" }
```

---

## Design Rules

```toml
# tech/rules.toml
[rules]
version = "1.0.0"
units = "um"

[rules.metal1]
min_width = 0.14
min_spacing = 0.14
min_area = 0.083
max_width = 4.0
enclosure_via = 0.06

[rules.metal2]
min_width = 0.14
min_spacing = 0.14
min_area = 0.0676
max_width = 4.0
enclosure_via = 0.055

[rules.via1]
size = 0.15
min_spacing = 0.17
enclosure_top = 0.055
enclosure_bottom = 0.06

[rules.poly]
min_width = 0.15
min_spacing = 0.21
min_area = 0.1
gate_extension = 0.13

[rules.diff]
min_width = 0.15
min_spacing = 0.27
min_area = 0.265
```

---

## PDK Browser UI

```
┌─────────────────────────────────────────────────────────────────┐
│ PDK Browser                                              [✕]   │
├─────────────────────────────────────────────────────────────────┤
│ Search: [sky130___________________] [🔍] [Filters ▼]           │
├──────────────────────┬──────────────────────────────────────────┤
│ Installed PDKs       │ PDK Details                              │
│ ├─ ✓ sky130 v1.0.0   │ ┌────────────────────────────────────┐   │
│ ├─ ✓ gf180mcu v0.9   │ │ SkyWater SKY130                    │   │
│ └─ ✓ asap7 v1.0      │ │ ─────────────────────────────────  │   │
│                      │ │ Foundry: SkyWater Technology       │   │
│ Available PDKs       │ │ Process: 130nm CMOS                │   │
│ ├─ ○ tsmc28 v2.1     │ │ Metal Layers: 5                    │   │
│ ├─ ○ intel16 v1.0    │ │ License: Apache-2.0                │   │
│ ├─ ○ polymumps v3.0  │ │                                    │   │
│ └─ ○ ibm_quantum     │ │ Variants:                          │   │
│                      │ │ ● sky130_fd_sc_hd (High Density)   │   │
│                      │ │ ○ sky130_fd_sc_hs (High Speed)     │   │
│                      │ │ ○ sky130_fd_sc_lp (Low Power)      │   │
│                      │ └────────────────────────────────────┘   │
├──────────────────────┴──────────────────────────────────────────┤
│ [Install Selected] [Update All] [Import from File] [Settings]  │
└─────────────────────────────────────────────────────────────────┘
```

---

## PDK Installation

### From Registry

```bash
# Install from official registry
hwt pdk install sky130
hwt pdk install gf180mcu
hwt pdk install polymumps

# Install specific version
hwt pdk install sky130@1.0.0

# Install with variant
hwt pdk install sky130 --variant sky130_fd_sc_hs
```

### From Local File

```bash
# Install from local archive
hwt pdk install ./my_pdk.zip

# Install from directory
hwt pdk install ./pdk_directory/
```

### From Git Repository

```bash
# Install from Git
hwt pdk install https://github.com/google/skywater-pdk.git
```

---

## Project PDK Configuration

```toml
# project.hwt
[project]
name = "my_chip"
domain = "ic"

[pdk]
name = "sky130"
version = "1.0.0"
variant = "sky130_fd_sc_hd"

# PDK overrides (optional)
[pdk.overrides]
# Override specific design rules for this project
"rules.metal1.min_width" = 0.16  # Tighter than default

# Custom layers
[pdk.custom_layers]
my_marker = { layer = 200, datatype = 0, desc = "Custom marker" }
```

---

## Layer Mapping

```rust
/// Map between PDK layers and design layers
LayerMapping {
    // Automatic mapping from PDK
    auto_map: true,
    
    // Manual overrides
    overrides: HashMap<String, LayerRef> {
        "top_metal" => "met5",
        "power_rail" => "met4",
    },
    
    // Alias support
    aliases: HashMap<String, String> {
        "M1" => "met1",
        "M2" => "met2",
    },
}
```

---

## DRC Rule Import

```rust
/// Import DRC rules from PDK
DrcRuleImport {
    // Source
    pdk: "sky130",
    rule_deck: "drc/rules.drc",
    
    // Import options
    options: ImportOptions {
        // Severity mapping
        severity_map: HashMap {
            "error" => Severity::Error,
            "warning" => Severity::Warning,
            "recommended" => Severity::Info,
        },
        
        // Rule filtering
        include_categories: vec!["metal", "via", "spacing"],
        exclude_rules: vec!["antenna"],  // Handle separately
        
        // Value adjustments
        scale_factor: 1.0,
    },
}
```

---

## Supported Open-Source PDKs

| PDK | Domain | Node | Status |
|-----|--------|------|--------|
| **sky130** | IC | 130nm | ✓ Supported |
| **gf180mcu** | IC | 180nm | ✓ Supported |
| **asap7** | IC | 7nm (predictive) | ✓ Supported |
| **FreePDK45** | IC | 45nm (academic) | ✓ Supported |
| **PolyMUMPs** | MEMS | Surface | ✓ Supported |
| **SOIMUMPs** | MEMS | SOI | ✓ Supported |
| **PiezoMUMPs** | MEMS | Piezo | ✓ Supported |

---

## Commercial PDK Support

Hardware Tool supports commercial PDKs through:

1. **NDA-Protected Import** — Import PDK files under NDA
2. **Encrypted Storage** — Secure storage of proprietary data
3. **License Validation** — Integration with foundry license servers

```toml
# Commercial PDK configuration
[pdk]
name = "tsmc28"
license_server = "license.foundry.com:5280"
encrypted = true
```

---

## CLI Commands

```bash
# PDK management
hwt pdk list                    # List installed PDKs
hwt pdk install <name>          # Install PDK
hwt pdk update <name>           # Update PDK
hwt pdk remove <name>           # Remove PDK
hwt pdk info <name>             # Show PDK details

# PDK validation
hwt pdk validate <name>         # Validate PDK integrity
hwt pdk check-rules <name>      # Check DRC rules

# Layer operations
hwt pdk layers <name>           # List layers
hwt pdk layers <name> --export  # Export layer map

# Cell library
hwt pdk cells <name>            # List cells
hwt pdk cells <name> --search "INV"
```

---

## Rust API

```rust
use hardware_tool::pdk::*;

// Load PDK
let pdk = Pdk::load("sky130")?;

// Access layers
let met1 = pdk.layer("met1")?;
println!("Metal 1: GDS {}/{}", met1.layer, met1.datatype);

// Access design rules
let rules = pdk.rules();
println!("M1 min width: {} um", rules.metal1.min_width);

// Access cells
let inv = pdk.cell("sky130_fd_sc_hd__inv_1")?;
println!("Inverter: {} pins", inv.pins.len());

// Run DRC with PDK rules
let violations = design.run_drc(&pdk.drc_rules())?;
```

---

## Related Topics

- [Shared DRC Architecture](../advanced-features/shared-drc-architecture.md)
- [Shared Library Architecture](./shared-library-architecture.md)
- [IC Design Module](../ic-design/README.md)
- [MEMS Design Module](../mems-sensors/README.md)
