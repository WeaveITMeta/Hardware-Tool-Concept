# Shared Library Architecture

## Overview

Hardware Tool provides a **unified library management system** that works across all hardware domains. Whether you're managing PCB symbols, IC cells, quantum gates, MEMS structures, RF components, or die IP — the same library infrastructure handles storage, search, versioning, and import/export.

> **"One Hardware Tool that does it all"** — The same library browser, search, and management tools work for every hardware type.

---

## Shared Library Components

All domain-specific libraries inherit these core capabilities:

| Component | Description |
|-----------|-------------|
| **Library Browser** | Unified UI for browsing and searching components |
| **Search & Filter** | Full-text search, property filtering, fuzzy matching |
| **Version Control** | Library versioning with Git integration |
| **Import/Export** | Standard format interchange |
| **Property System** | Extensible component properties |
| **Validation** | Library quality checks |

---

## Library File Structure (All Domains)

```
my_library/
├── library.toml              # Library metadata
├── symbols/                  # Schematic symbols (PCB, IC, RF)
│   ├── resistor.hwt_sym
│   └── capacitor.hwt_sym
├── footprints/               # Physical footprints (PCB)
│   ├── 0603.hwt_fp
│   └── QFN-32.hwt_fp
├── cells/                    # Standard cells (IC)
│   ├── INV_X1.hwt_cell
│   └── NAND2_X1.hwt_cell
├── gates/                    # Quantum gates (Quantum)
│   ├── hadamard.hwt_gate
│   └── cnot.hwt_gate
├── structures/               # MEMS structures (MEMS)
│   ├── comb_drive.hwt_mems
│   └── spring.hwt_mems
├── dies/                     # Die IP (Packaging)
│   ├── compute_die.hwt_die
│   └── hbm_die.hwt_die
└── 3d_models/                # 3D models (all domains)
    ├── QFN-32.step
    └── BGA-256.step
```

---

## Library Metadata Format

```toml
# library.toml - works for all hardware domains
[library]
name = "My Components"
version = "1.0.0"
author = "Hardware Team"
license = "MIT"
description = "Custom component library"

# Supported domains
domains = ["pcb", "ic", "quantum", "mems", "rf", "packaging"]

# Dependencies
[dependencies]
"official-passives" = "^2.0"
"sky130-pdk" = "1.0.0"

# Quality settings
[quality]
require_description = true
require_datasheet = false
require_3d_model = false
```

---

## Library Browser UI (All Domains)

```
┌─────────────────────────────────────────────────────────────────┐
│ Library Browser                                          [✕]   │
├─────────────────────────────────────────────────────────────────┤
│ Search: [resistor 0603________________] [🔍] [Filters ▼]       │
├──────────────────────┬──────────────────────────────────────────┤
│ Libraries            │ Components                               │
│ ├─ 📁 Official       │ ┌────────────────────────────────────┐   │
│ │  ├─ Passives       │ │ ┌──────┐  Resistor 0603            │   │
│ │  ├─ Actives        │ │ │ ═══  │  0603 (1608 Metric)       │   │
│ │  └─ Connectors     │ │ └──────┘  Footprint: R_0603_1608   │   │
│ ├─ 📁 Manufacturer   │ │           Datasheet: ✓              │   │
│ │  ├─ TI             │ └────────────────────────────────────┘   │
│ │  ├─ STM            │ ┌────────────────────────────────────┐   │
│ │  └─ Analog         │ │ ┌──────┐  Resistor 0402            │   │
│ ├─ 📁 My Library     │ │ │ ═══  │  0402 (1005 Metric)       │   │
│ │  └─ Custom         │ │ └──────┘  Footprint: R_0402_1005   │   │
│ └─ 📁 Project        │ │           Datasheet: ✓              │   │
│                      │ └────────────────────────────────────┘   │
├──────────────────────┴──────────────────────────────────────────┤
│ Preview:                                                        │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │    ┌─────────────────────────────────────────────────────┐  │ │
│ │    │                                                     │  │ │
│ │    │  1 ●━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━● 2  │  │ │
│ │    │                                                     │  │ │
│ │    └─────────────────────────────────────────────────────┘  │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [Place] [Edit] [Copy to Library] [View Datasheet]               │
└─────────────────────────────────────────────────────────────────┘
```

---

## Search & Filter (All Domains)

```rust
/// Unified search works across all component types
LibrarySearch {
    // Text search
    query: String,
    
    // Filters (domain-agnostic)
    filters: SearchFilters {
        libraries: Vec<String>,      // Limit to specific libraries
        domains: Vec<Domain>,        // PCB, IC, Quantum, etc.
        has_3d_model: Option<bool>,
        has_datasheet: Option<bool>,
        has_simulation: Option<bool>,
    },
    
    // Property filters (domain-specific interpreted)
    property_filters: HashMap<String, PropertyFilter>,
    
    // Sorting
    sort_by: SortField,              // Name, Date, Popularity
    sort_order: SortOrder,
}
```

### Search Examples

```rust
// Search works the same way for any domain
let results = library.search(SearchQuery {
    text: "resistor 10k",
    domain: Some(Domain::PCB),
})?;

let results = library.search(SearchQuery {
    text: "NAND",
    domain: Some(Domain::IC),
})?;

let results = library.search(SearchQuery {
    text: "transmon",
    domain: Some(Domain::Quantum),
})?;
```

---

## Component Properties (All Domains)

```rust
/// Base properties shared by all component types
ComponentBase {
    // Identity
    name: String,
    description: String,
    keywords: Vec<String>,
    
    // Metadata
    author: String,
    version: String,
    created: DateTime,
    modified: DateTime,
    
    // References
    datasheet: Option<Url>,
    documentation: Option<Url>,
    
    // 3D model (if applicable)
    model_3d: Option<Model3DRef>,
    
    // Custom properties
    properties: HashMap<String, PropertyValue>,
}
```

---

## Library Validation (All Domains)

```rust
/// Quality checks work for any library type
LibraryValidator {
    // Required fields
    require_description: bool,
    require_keywords: bool,
    require_datasheet: bool,
    require_3d_model: bool,
    
    // Naming conventions
    naming_pattern: Option<Regex>,
    
    // Domain-specific validation
    domain_validators: HashMap<Domain, Box<dyn DomainValidator>>,
}

// Run validation
let report = library.validate(ValidationConfig::default())?;
for issue in report.issues {
    println!("{}: {}", issue.severity, issue.message);
}
```

---

## Import/Export (All Domains)

```rust
/// Import from external formats
library.import(ImportConfig {
    source: "external_lib.kicad_sym",
    format: ImportFormat::KiCad,
    target_library: "my_library",
})?;

/// Export to external formats
library.export(ExportConfig {
    components: &selected,
    format: ExportFormat::KiCad,
    output: "exported.kicad_sym",
})?;
```

### Supported Import/Export Formats

| Domain | Import From | Export To |
|--------|-------------|-----------|
| **PCB** | KiCad, Altium, Eagle | KiCad, Altium |
| **IC** | Liberty, LEF, OpenAccess | Liberty, LEF |
| **Quantum** | Qiskit, Cirq | Qiskit, Cirq |
| **MEMS** | GDS, DXF | GDS, DXF |
| **RF** | ADS, AWR | Touchstone |
| **Packaging** | APD, ODB++ | APD |

---

## Version Control (All Domains)

```rust
/// Git-based library versioning
LibraryVersionControl {
    // Track changes
    track_changes: true,
    
    // Commit on save
    auto_commit: false,
    
    // Branch management
    default_branch: "main",
    
    // Remote sync
    remote: Option<GitRemote>,
}

// View history
let history = library.get_history("resistor.hwt_sym")?;
for commit in history {
    println!("{}: {} by {}", commit.date, commit.message, commit.author);
}

// Restore previous version
library.restore("resistor.hwt_sym", "abc123")?;
```

---

## CLI Commands (All Domains)

```bash
# Library management (works for any domain)
hwt library list
hwt library search "resistor"
hwt library add my_lib ./path/to/library
hwt library remove my_lib
hwt library update my_lib

# Import/export
hwt library import kicad_lib.kicad_sym --to my_lib
hwt library export my_lib --format kicad --output exported/

# Validation
hwt library validate my_lib
hwt library validate my_lib --fix

# Version control
hwt library history my_lib/resistor.hwt_sym
hwt library restore my_lib/resistor.hwt_sym abc123
```

---

## Domain-Specific Extensions

Each hardware domain extends the shared library system with specialized component types:

| Domain | Component Types | Specialized Properties |
|--------|-----------------|----------------------|
| **PCB** | Symbols, Footprints | Pins, Pads, Courtyard |
| **IC** | Cells, Devices | Timing, Power, PDK |
| **Quantum** | Gates, Qubits | Fidelity, Coherence |
| **MEMS** | Structures, Sensors | Resonance, Sensitivity |
| **RF** | Components, Models | S-parameters, Impedance |
| **Packaging** | Dies, Bumps | TSV, Thermal |

See domain-specific documentation:

- [PCB Symbols & Libraries](../schematic-editor/symbols-libraries.md)
- [PCB Footprints & Libraries](../pcb-layout/footprints-libraries.md)
- [IC Cells & Libraries](../ic-design/rtl-logic-design/cells-and-libraries.md)
- [Quantum Gates & Libraries](../quantum-hardware/circuit-editor/gates-and-libraries.md)
- [MEMS Structures & Libraries](../mems-sensors/device-editor/structures-and-libraries.md)
- [RF Components & Libraries](../rf-photonics/schematic-editor/rf-components-and-libraries.md)
- [Die IP & Libraries](../advanced-packaging/die-editor/die-ip-and-libraries.md)

---

## Related Topics

- [Shared Module Consolidation](./shared-module-consolidation.md)
- [Project Structure & Management](./project-structure-management.md)
- [Library Conventions & Quality Control](../advanced-features/library-conventions.md)
