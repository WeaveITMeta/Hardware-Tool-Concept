# Shared Export/Import Architecture

## Overview

Hardware Tool provides a **unified export/import infrastructure** that works across all hardware domains. Whether you're exporting Gerber for PCBs, GDSII for ICs, Touchstone for RF, or ODB++ for packaging — the same job management, progress reporting, validation, and batch processing handles it all.

> **"One Hardware Tool That Does It All"** — The same export workflow, validation, and batch processing works for every hardware type and output format.

---

## Shared Export/Import Components

All domain-specific exporters/importers inherit these core capabilities:

| Component | Description |
|-----------|-------------|
| **Job Manager** | Queue, progress, cancellation |
| **Validation** | Pre-export design checks |
| **Batch Processing** | Multiple formats in one operation |
| **Progress Reporting** | Real-time status updates |
| **Error Handling** | Consistent error reporting |
| **History** | Export log and re-export |

---

## Export Job Manager (All Domains)

```rust
/// Unified export job management
ExportJobManager {
    // Job queue
    queue: JobQueue {
        max_concurrent: 4,
        priority_scheduling: true,
    },
    
    // Progress tracking
    progress: ProgressConfig {
        show_percentage: true,
        show_current_step: true,
        show_time_remaining: true,
    },
    
    // Validation
    validation: ValidationConfig {
        run_drc_before_export: true,
        block_on_errors: true,
        warn_on_warnings: true,
    },
}
```

---

## Export UI (All Domains)

```
┌─────────────────────────────────────────────────────────────────┐
│ Export Design                                            [✕]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Output Directory: [./output/_______________] [Browse]          │
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Format              │ Status    │ Options                   │ │
│ │ ────────────────────┼───────────┼───────────────────────── │ │
│ │ ☑ Gerber RS-274X    │ Ready     │ [Configure...]           │ │
│ │ ☑ Excellon Drill    │ Ready     │ [Configure...]           │ │
│ │ ☐ IPC-2581          │ --        │ [Configure...]           │ │
│ │ ☐ ODB++             │ --        │ [Configure...]           │ │
│ │ ☑ BOM (CSV)         │ Ready     │ [Configure...]           │ │
│ │ ☑ Pick & Place      │ Ready     │ [Configure...]           │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Pre-Export Validation:                                          │
│ ☑ Run DRC before export                                        │
│ ☑ Block export on errors                                       │
│ ☐ Include design review report                                 │
│                                                                 │
│ [Export Selected] [Export All] [Cancel]                        │
└─────────────────────────────────────────────────────────────────┘
```

---

## Export Progress (All Domains)

```
┌─────────────────────────────────────────────────────────────────┐
│ Exporting...                                             [✕]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Overall Progress: ████████████░░░░░░░░ 60%                     │
│                                                                 │
│ Current: Gerber - Top Copper (F.Cu)                            │
│ Step: 3 of 5 formats                                           │
│ Time Remaining: ~12 seconds                                    │
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ ✓ Gerber RS-274X      Complete    2.3 MB                   │ │
│ │ ✓ Excellon Drill      Complete    45 KB                    │ │
│ │ ● BOM (CSV)           In Progress...                       │ │
│ │ ○ Pick & Place        Pending                              │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [Pause] [Cancel]                                               │
└─────────────────────────────────────────────────────────────────┘
```

---

## Import UI (All Domains)

```
┌─────────────────────────────────────────────────────────────────┐
│ Import Design                                            [✕]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Source: [./external/design.kicad_pro____] [Browse]             │
│                                                                 │
│ Detected Format: KiCad 7.0 Project                             │
│                                                                 │
│ Import Options:                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ ☑ Schematics (3 sheets)                                    │ │
│ │ ☑ PCB Layout                                               │ │
│ │ ☑ Libraries (symbols + footprints)                         │ │
│ │ ☑ 3D Models                                                │ │
│ │ ☐ Design Rules (use Hardware Tool defaults)                │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Conflict Resolution:                                            │
│ ○ Overwrite existing    ● Rename duplicates    ○ Skip         │
│                                                                 │
│ [Preview Import] [Import] [Cancel]                             │
└─────────────────────────────────────────────────────────────────┘
```

---

## Pre-Export Validation (All Domains)

```rust
/// Validation before export
PreExportValidation {
    // Design rule checks
    run_drc: bool,
    drc_severity_threshold: Severity::Error,
    
    // Domain-specific checks
    domain_checks: vec![
        Check::UnconnectedPins,
        Check::MissingFootprints,
        Check::OverlappingComponents,
    ],
    
    // Blocking behavior
    block_on_errors: bool,
    block_on_warnings: bool,
    
    // Report generation
    generate_report: bool,
    report_format: ReportFormat::HTML,
}
```

### Validation Results

```
┌─────────────────────────────────────────────────────────────────┐
│ Pre-Export Validation                                    [✕]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ ✓ Design Rule Check: PASSED (0 errors, 2 warnings)            │
│ ✓ Connectivity Check: PASSED                                   │
│ ✓ Footprint Check: PASSED                                      │
│ ⚠ 3D Model Check: 2 components missing models                  │
│                                                                 │
│ Warnings:                                                       │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ ⚠ Silkscreen over pad at R5                                │ │
│ │ ⚠ Track near board edge at (12.5, 45.0)                    │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [Continue Export] [Fix Issues] [Cancel]                        │
└─────────────────────────────────────────────────────────────────┘
```

---

## Batch Export (All Domains)

```rust
/// Export multiple formats in one operation
BatchExport {
    // Formats to export
    formats: vec![
        ExportFormat::Gerber,
        ExportFormat::Excellon,
        ExportFormat::BOM,
        ExportFormat::PickPlace,
    ],
    
    // Output organization
    output_dir: "./output",
    create_subdirs: true,      // gerber/, drill/, etc.
    timestamp_dirs: false,     // output_2026-01-19/
    
    // Parallelization
    parallel: true,
    max_concurrent: 4,
}
```

---

## Export History (All Domains)

```rust
/// Track export operations
ExportHistory {
    // History storage
    max_entries: 100,
    
    // Entry data
    entries: vec![
        ExportEntry {
            timestamp: DateTime,
            formats: vec![...],
            output_dir: String,
            success: bool,
            duration: Duration,
            file_sizes: HashMap<String, u64>,
        },
    ],
    
    // Re-export
    allow_reexport: true,
}
```

### History UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Export History                                           [✕]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Date/Time           │ Formats      │ Status │ Size         │ │
│ │ ────────────────────┼──────────────┼────────┼───────────── │ │
│ │ 2026-01-19 20:30    │ Gerber, BOM  │ ✓      │ 2.5 MB       │ │
│ │ 2026-01-18 15:45    │ Gerber       │ ✓      │ 2.3 MB       │ │
│ │ 2026-01-17 10:20    │ IPC-2581     │ ✗      │ --           │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [Re-Export] [Open Folder] [Delete] [Clear History]             │
└─────────────────────────────────────────────────────────────────┘
```

---

## Domain-Specific Formats

### PCB Domain

| Format | Extension | Description |
|--------|-----------|-------------|
| Gerber RS-274X | `.gbr` | Manufacturing layers |
| Excellon | `.drl` | Drill files |
| IPC-2581 | `.xml` | Intelligent PCB format |
| ODB++ | folder | Open database |
| BOM | `.csv` | Bill of materials |
| Pick & Place | `.csv` | Assembly data |

### IC Domain

| Format | Extension | Description |
|--------|-----------|-------------|
| GDSII | `.gds` | Mask layout |
| OASIS | `.oas` | Compressed layout |
| LEF | `.lef` | Library exchange |
| DEF | `.def` | Design exchange |
| Liberty | `.lib` | Timing library |
| Verilog | `.v` | Netlist |

### Quantum Domain

| Format | Extension | Description |
|--------|-----------|-------------|
| GDSII | `.gds` | Mask layout |
| CIF | `.cif` | Caltech format |
| Qiskit Pulse | `.py` | Control sequences |
| OpenQASM | `.qasm` | Circuit description |
| JSON | `.json` | Calibration data |

### MEMS Domain

| Format | Extension | Description |
|--------|-----------|-------------|
| GDSII | `.gds` | Mask layout |
| CIF | `.cif` | Caltech format |
| DXF | `.dxf` | CAD interchange |
| STEP | `.step` | 3D mechanical |
| STL | `.stl` | 3D printing |

### RF Domain

| Format | Extension | Description |
|--------|-----------|-------------|
| Gerber | `.gbr` | PCB layers |
| GDSII | `.gds` | MMIC layout |
| Touchstone | `.s2p` | S-parameters |
| MDIF | `.mdf` | Measured data |

### Packaging Domain

| Format | Extension | Description |
|--------|-----------|-------------|
| GDSII | `.gds` | Interposer/RDL |
| ODB++ | folder | Substrate |
| IPC-2581 | `.xml` | Single-file |
| APD | `.apd` | Assembly spec |
| STEP | `.step` | 3D mechanical |

---

## CLI Commands (All Domains)

```bash
# Export (format auto-detected or specified)
hwt export my_design.hwt
hwt export my_design.hwt --format gerber
hwt export my_design.hwt --format gdsii
hwt export my_design.hwt --all

# Batch export
hwt export my_design.hwt --formats gerber,bom,pickplace

# Export with validation
hwt export my_design.hwt --validate
hwt export my_design.hwt --no-validate

# Import
hwt import external.kicad_pro
hwt import external.gds --domain ic
hwt import external.s2p --domain rf

# Export history
hwt export history
hwt export reexport <entry_id>
```

---

## Rust API (Shared Base)

```rust
use hardware_tool::export::*;

/// All domain exporters implement this trait
trait Exporter {
    // Export operations
    fn export(&self, config: ExportConfig) -> Result<ExportReport>;
    fn export_batch(&self, configs: Vec<ExportConfig>) -> Result<BatchReport>;
    
    // Validation
    fn validate(&self) -> Result<ValidationReport>;
    
    // Progress
    fn on_progress(&self, callback: impl Fn(Progress));
    
    // Cancellation
    fn cancel(&self) -> Result<()>;
}

/// All domain importers implement this trait
trait Importer {
    // Import operations
    fn import(&self, source: &Path, config: ImportConfig) -> Result<ImportReport>;
    
    // Preview
    fn preview(&self, source: &Path) -> Result<ImportPreview>;
    
    // Format detection
    fn detect_format(&self, source: &Path) -> Result<ImportFormat>;
}

/// Export configuration
pub struct ExportConfig {
    pub format: ExportFormat,
    pub output_dir: PathBuf,
    pub options: HashMap<String, Value>,
    pub validate: bool,
}

/// Export report
pub struct ExportReport {
    pub success: bool,
    pub files: Vec<ExportedFile>,
    pub duration: Duration,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}
```

---

## Related Topics

- [Shared Module Consolidation](../core-architecture/shared-module-consolidation.md)
- [Manufacturing Output](../manufacturing-output/gerber-export.md) - PCB exports
- [GDSII Export](../ic-design/manufacturing-output/gdsii-export.md) - IC exports
- [Command-Line Interface](./cli.md)
