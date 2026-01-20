# Shared Project Architecture

## Overview

Hardware Tool provides a **unified project management system** that works across all hardware domains. Whether you're managing a PCB project, IC design, quantum processor, MEMS device, RF circuit, or advanced package — the same project structure, file formats, version control, and build system handles it all.

> **"One Hardware Tool That Does It All"** — The same project format, directory structure, and management tools work for every hardware type.

---

## Shared Project Components

All domain-specific projects inherit these core capabilities:

| Component | Description |
|-----------|-------------|
| **Project File** | `.hwt` TOML-based configuration |
| **Directory Structure** | Consistent folder organization |
| **Version Control** | Git-friendly, diff-able formats |
| **Build System** | Declarative targets, incremental builds |
| **Dependencies** | Library references, external sources |
| **Search** | Project-wide component/net search |

---

## Unified Project File Format

### Base `.hwt` Format (All Domains)

```toml
# project.hwt - works for any hardware domain
[project]
name = "my-design"
version = "1.0.0"
format_version = "1"
author = "Hardware Team"
license = "MIT"

# Hardware domain (determines available features)
domain = "pcb"  # pcb | ic | quantum | mems | rf | packaging

# Description
description = "Project description"
created = "2026-01-19"
modified = "2026-01-19"

[settings]
# Domain-agnostic settings
units = "mm"              # mm | mil | um | nm
grid = 0.1
snap_to_grid = true

[libraries]
# Shared library references
local = ["./libs"]
remote = ["https://hardware-tool.dev/libs/standard"]

[output]
# Output directory
dir = "./output"

[version_control]
# Git integration
enabled = true
auto_commit = false
```

---

## Directory Structure (All Domains)

```
my-project/
├── project.hwt              # Main project file (shared format)
├── .hwtignore               # Git-style ignore file
├── .hwt/                    # Project cache/state
│   ├── cache/
│   └── history/
├── design/                  # Design files (domain-specific)
│   ├── [domain-specific files]
│   └── ...
├── libraries/               # Local libraries
│   └── [domain-specific libs]
├── simulation/              # Simulation files
│   ├── configs/
│   └── results/
├── output/                  # Generated outputs
│   ├── [domain-specific outputs]
│   └── reports/
└── docs/                    # Project documentation
    └── README.md
```

---

## Domain-Specific Extensions

Each domain extends the base project format:

### PCB Domain

```toml
[project]
domain = "pcb"

[schematics]
main = "main.hwt_sch"
sheets = ["power.hwt_sch", "io.hwt_sch"]

[pcb]
layout = "board.hwt_pcb"
stackup = "4-layer-standard"

[output.pcb]
gerber = true
ipc2581 = true
odbpp = false
bom = true
```

**Directory:**
```
design/
├── schematics/
│   ├── main.hwt_sch
│   └── sheets/
├── pcb/
│   └── board.hwt_pcb
└── 3d-models/
```

### IC Domain

```toml
[project]
domain = "ic"

[pdk]
name = "sky130"
version = "1.0.0"
path = "/pdk/sky130"

[rtl]
top = "chip_top"
sources = ["src/*.v"]

[layout]
top = "chip_top.hwt_ic"

[output.ic]
gdsii = true
lef = true
liberty = true
```

**Directory:**
```
design/
├── rtl/
│   └── *.v
├── synthesis/
├── layout/
│   └── *.hwt_ic
└── verification/
```

### Quantum Domain

```toml
[project]
domain = "quantum"

[processor]
qubits = 5
topology = "linear"
qubit_type = "transmon"

[circuit]
main = "algorithm.hwt_qc"

[layout]
chip = "processor.hwt_quantum"

[output.quantum]
gdsii = true
qiskit_pulse = true
openqasm = true
```

**Directory:**
```
design/
├── circuits/
│   └── *.hwt_qc
├── layout/
│   └── *.hwt_quantum
└── calibration/
```

### MEMS Domain

```toml
[project]
domain = "mems"

[process]
foundry = "MEMSCAP"
process = "PolyMUMPs"

[device]
main = "accelerometer.hwt_mems"

[output.mems]
gdsii = true
dxf = true
step = true
```

**Directory:**
```
design/
├── devices/
│   └── *.hwt_mems
├── mechanical/
└── electrical/
```

### RF Domain

```toml
[project]
domain = "rf"

[substrate]
material = "Rogers4350B"
thickness = 0.508
er = 3.48

[schematic]
main = "lna.hwt_rf_sch"

[layout]
main = "lna.hwt_rf"

[output.rf]
gerber = true
touchstone = true
```

**Directory:**
```
design/
├── schematics/
│   └── *.hwt_rf_sch
├── layout/
│   └── *.hwt_rf
└── s-parameters/
```

### Packaging Domain

```toml
[project]
domain = "packaging"

[package]
type = "2.5D"
interposer = true

[dies]
compute = { file = "compute.hwt_die", count = 1 }
hbm = { file = "hbm.hwt_die", count = 4 }

[output.packaging]
gdsii = true
odbpp = true
apd = true
```

**Directory:**
```
design/
├── dies/
│   └── *.hwt_die
├── interposer/
├── substrate/
└── assembly/
```

---

## Version Control Integration (All Domains)

```rust
/// Git-friendly project management
VersionControl {
    // File formats
    formats: FileFormats {
        text_based: true,           // Human-readable
        diff_friendly: true,        // Clean diffs
        merge_friendly: true,       // Minimal conflicts
    },
    
    // Ignore patterns
    ignore: vec![
        ".hwt/cache/",
        "output/",
        "*.bak",
    ],
    
    // Lock files
    lock_file: "project.hwt.lock",
    
    // Hooks
    pre_commit: Some("hwt validate"),
    post_merge: Some("hwt sync"),
}
```

### `.hwtignore` (All Domains)

```gitignore
# Hardware Tool ignore file (works like .gitignore)

# Cache and temporary
.hwt/cache/
*.bak
*.tmp

# Generated outputs
output/

# Simulation results (regenerable)
simulation/results/

# OS files
.DS_Store
Thumbs.db
```

---

## Build System (All Domains)

```rust
/// Unified build system
BuildSystem {
    // Targets
    targets: vec![
        BuildTarget::Validate,      // Check design rules
        BuildTarget::Simulate,      // Run simulations
        BuildTarget::Export,        // Generate outputs
        BuildTarget::Package,       // Create release package
    ],
    
    // Incremental builds
    incremental: true,
    
    // Parallelization
    parallel: true,
    max_jobs: 8,
    
    // Caching
    cache_enabled: true,
    cache_dir: ".hwt/cache",
}
```

### Build Commands (All Domains)

```bash
# Validate design
hwt build validate

# Run all simulations
hwt build simulate

# Generate outputs
hwt build export

# Full build
hwt build all

# Clean
hwt clean
```

---

## Project Search (All Domains)

```rust
/// Project-wide search
ProjectSearch {
    // Search scope
    scope: SearchScope {
        components: true,
        nets: true,
        libraries: true,
        files: true,
        properties: true,
    },
    
    // Search features
    features: SearchFeatures {
        fuzzy_match: true,
        regex: true,
        case_sensitive: false,
        whole_word: false,
    },
}
```

### Search UI

```
┌─────────────────────────────────────────────────────────────────┐
│ 🔍 Project Search                                        [✕]   │
├─────────────────────────────────────────────────────────────────┤
│ Search: [STM32_______________________] [Components ▼]          │
├─────────────────────────────────────────────────────────────────┤
│ Results (3 found):                                              │
│                                                                 │
│ 📦 U1 - STM32F407VGT6                                          │
│    main.hwt_sch:45  Value: STM32F407VGT6                       │
│                                                                 │
│ 📦 U3 - STM32F103C8T6                                          │
│    power.hwt_sch:12  Value: STM32F103C8T6                      │
│                                                                 │
│ 📚 STM32F4_LQFP100                                             │
│    libs/mcu/stm32.hwt_lib  Footprint                           │
│                                                                 │
│ [Go to] [Select All] [Export List]                             │
└─────────────────────────────────────────────────────────────────┘
```

---

## CLI Commands (All Domains)

```bash
# Create new project (specify domain)
hwt new my_project --domain pcb
hwt new my_project --domain ic
hwt new my_project --domain quantum
hwt new my_project --domain mems
hwt new my_project --domain rf
hwt new my_project --domain packaging

# Open project
hwt open project.hwt

# Project info
hwt info

# Validate project
hwt validate

# Search project
hwt search "STM32"
hwt search --nets "VCC"
hwt search --files "*.hwt_sch"

# Build
hwt build [target]

# Clean
hwt clean
```

---

## Rust API (Shared Base)

```rust
use hardware_tool::project::*;

/// All domain projects implement this trait
trait Project {
    // Metadata
    fn name(&self) -> &str;
    fn domain(&self) -> HardwareDomain;
    fn version(&self) -> &Version;
    
    // Files
    fn files(&self) -> Vec<ProjectFile>;
    fn add_file(&mut self, file: ProjectFile) -> Result<()>;
    fn remove_file(&mut self, path: &str) -> Result<()>;
    
    // Libraries
    fn libraries(&self) -> Vec<LibraryRef>;
    fn add_library(&mut self, lib: LibraryRef) -> Result<()>;
    
    // Build
    fn build(&self, target: BuildTarget) -> Result<BuildReport>;
    fn clean(&self) -> Result<()>;
    
    // Search
    fn search(&self, query: &SearchQuery) -> Vec<SearchResult>;
    
    // Version control
    fn git_status(&self) -> Result<GitStatus>;
    fn commit(&self, message: &str) -> Result<()>;
}
```

---

## Related Topics

- [Shared Module Consolidation](./shared-module-consolidation.md)
- [Unified Project File Format](./unified-project-file-format.md)
- [Shared Library Architecture](./shared-library-architecture.md)
- [Command-Line Interface](../advanced-features/cli.md)
