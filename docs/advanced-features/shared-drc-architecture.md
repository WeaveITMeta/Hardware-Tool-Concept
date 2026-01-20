# Shared Design Rule Check (DRC) Architecture

## Overview

Hardware Tool provides a **unified Design Rule Check engine** that works across all hardware domains. Each domain extends this shared architecture with domain-specific rule categories while inheriting common infrastructure for rule management, violation reporting, and interactive checking.

> **"One Hardware Tool that does it all"** — The same DRC engine validates PCBs, ICs, quantum circuits, MEMS devices, RF layouts, and advanced packages.

---

## Shared DRC Components

All domain-specific DRC implementations inherit these core capabilities:

| Component | Description |
|-----------|-------------|
| **Rule Engine** | Extensible rule definition and evaluation |
| **Violation Reporting** | Consistent error/warning format across domains |
| **Interactive Checking** | Real-time validation during editing |
| **Batch Checking** | Full design verification with parallel execution |
| **Exclusion Management** | Waiver system for intentional violations |
| **Report Generation** | JSON, CSV, HTML export |

---

## Core DRC Configuration

```rust
/// Base DRC configuration inherited by all domain checkers
DRCEngineBase {
    // Execution mode (shared)
    mode: DRCMode {
        incremental: bool,        // Check only modified regions
        hierarchical: bool,       // Preserve design hierarchy
        parallel: bool,           // Multi-threaded execution
        threads: usize,           // Thread count
    },
    
    // Reporting (shared)
    reporting: DRCReporting {
        max_errors: usize,        // Limit reported errors
        group_by_rule: bool,      // Group violations by rule
        include_coordinates: bool, // Include location data
        generate_markers: bool,   // Create visual markers
    },
    
    // Performance (shared)
    performance: DRCPerformance {
        debounce_ms: u32,         // Delay for real-time checking
        cache_results: bool,      // Cache unchanged regions
    },
}
```

---

## Severity Levels (All Domains)

```rust
/// Consistent severity levels across all hardware domains
pub enum DRCSeverity {
    Error,      // Must fix before manufacturing
    Warning,    // Should review, may cause issues
    Info,       // Informational, best practice
    Ignore,     // Suppressed by exclusion
}
```

### Severity Configuration

```rust
/// Configure severity per rule (works in any domain)
DRCSeverityConfig {
    rule_severities: HashMap<String, DRCSeverity>,
    default_severity: DRCSeverity::Warning,
}
```

---

## Violation Report Format (All Domains)

```
═══════════════════════════════════════════════════════════
DRC Report - [design_name].[extension]
Generated: [timestamp]
Domain: [PCB | IC | Quantum | MEMS | RF | Packaging]
═══════════════════════════════════════════════════════════

Summary:
  Errors:   [count]
  Warnings: [count]
  Excluded: [count]

───────────────────────────────────────────────────────────
ERRORS ([count])
───────────────────────────────────────────────────────────

[E001] [Rule Name]: [Description]
  Location: ([x], [y]) on [layer]
  Actual: [value] [unit]
  Required: [value] [unit]
  [Domain-specific details]

───────────────────────────────────────────────────────────
WARNINGS ([count])
───────────────────────────────────────────────────────────

[W001] [Rule Name]: [Description]
  ...
```

---

## DRC Results UI (All Domains)

```
┌─────────────────────────────────────────────────────────────────┐
│ DRC Results: [design_name]                                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Summary: [N] errors, [M] warnings                               │
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Rule          │ Count │ Severity │ Description              │ │
│ │ ──────────────┼───────┼──────────┼───────────────────────── │ │
│ │ [rule.id]     │  [N]  │ [level]  │ [description]            │ │
│ │ ...           │  ...  │ ...      │ ...                      │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Error Details:                                                  │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ ✗ [rule.id]: [description]                                  │ │
│ │   Location: [coordinates]                                   │ │
│ │   Actual: [value]                                           │ │
│ │   Required: [value]                                         │ │
│ │   [Zoom To] [Fix] [Waive]                                   │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [Fix All] [Export Report] [Re-run DRC] [Close]                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## Marker Navigation (All Domains)

| Action | Shortcut |
|--------|----------|
| Next error | `N` |
| Previous error | `P` |
| Jump to error | Click in report |
| Clear markers | `Ctrl+Shift+C` |
| Toggle markers | `Ctrl+M` |

---

## Exclusion Management (All Domains)

```rust
/// Waiver system works identically across all domains
DRCExclusion {
    rule: String,              // Rule ID to exclude
    location: Option<Point>,   // Specific location (optional)
    layer: Option<String>,     // Specific layer (optional)
    scope: ExclusionScope,     // Single, Region, Global
    reason: String,            // Required justification
    created: DateTime,         // Audit trail
    created_by: String,        // User who created
}

pub enum ExclusionScope {
    Single,    // This specific violation
    Region,    // All violations in area
    Rule,      // All violations of this rule
    Global,    // Entire design
}
```

---

## Real-Time DRC (All Domains)

```rust
/// Interactive checking during editing (shared behavior)
RealtimeDRC {
    enabled: bool,
    
    // Check triggers
    check_on_move: bool,
    check_on_route: bool,
    check_on_place: bool,
    
    // Visual feedback
    highlight_violations: bool,
    block_violations: bool,     // Prevent invalid operations
    
    // Performance
    debounce_ms: u32,
    incremental: bool,
}
```

---

## Report Export (All Domains)

```rust
/// Export formats available for all domains
report.export_json("drc_report.json")?;
report.export_csv("drc_report.csv")?;
report.export_html("drc_report.html")?;
report.export_pdf("drc_report.pdf")?;
```

---

## Rust API (Shared Base)

```rust
use hardware_tool::drc::*;

/// All domain DRC checkers implement this trait
trait DRCChecker {
    // Run full check
    fn run_drc(&self, config: DRCConfig) -> Result<DRCReport>;
    
    // Run incremental check
    fn run_incremental_drc(&self, modified: &[Item]) -> Result<DRCReport>;
    
    // Real-time checking
    fn check_operation(&self, op: &Operation) -> Vec<DRCViolation>;
    
    // Exclusions
    fn add_exclusion(&mut self, exclusion: DRCExclusion) -> Result<()>;
    fn remove_exclusion(&mut self, id: &str) -> Result<()>;
    fn list_exclusions(&self) -> Vec<DRCExclusion>;
    
    // Report
    fn export_report(&self, report: &DRCReport, format: ReportFormat) -> Result<()>;
}

/// Shared violation structure
pub struct DRCViolation {
    pub id: String,
    pub rule: String,
    pub severity: DRCSeverity,
    pub message: String,
    pub location: Point,
    pub layer: Option<String>,
    pub actual_value: Option<f64>,
    pub required_value: Option<f64>,
    pub unit: Option<String>,
    pub fix_suggestion: Option<String>,
}
```

---

## Domain-Specific Extensions

Each hardware domain extends the shared DRC engine with specialized rule categories:

| Domain | Rule Categories | Example Rules |
|--------|-----------------|---------------|
| **PCB** | Clearance, Size, Connectivity, Courtyard | Track spacing, via drill, unconnected pads |
| **IC** | Width, Spacing, Enclosure, Density, Antenna | Metal spacing, poly width, gate antenna ratio |
| **Quantum** | Crosstalk, Isolation, Coupling | Qubit-qubit spacing, resonator coupling |
| **MEMS** | Structural, Etch Holes, Release | Beam width, hole spacing, undercut |
| **RF** | Impedance, Coupling, Grounding | Trace impedance, via fence spacing |
| **Packaging** | Bump, TSV, Warpage | Bump pitch, TSV spacing, die alignment |

See domain-specific documentation:

- [PCB DRC](../pcb-layout/drc.md)
- [IC Physical Verification](../ic-design/analog-mixed-signal/physical-verification-drc-lvs.md)
- [Quantum DRC](../quantum-hardware/layout-concepts/quantum-design-rule-check.md)
- [MEMS DRC](../mems-sensors/layout-concepts/mems-design-rule-check.md)
- [RF DRC](../rf-photonics/layout-concepts/rf-design-rule-check.md)
- [Packaging DRC](../advanced-packaging/layout-concepts/packaging-design-rule-check.md)

---

## CLI Commands (All Domains)

```bash
# Run DRC (domain auto-detected from file extension)
hwt drc my_design.hwt_pcb
hwt drc my_chip.hwt_ic
hwt drc my_processor.hwt_quantum
hwt drc my_sensor.hwt_mems
hwt drc my_amplifier.hwt_rf
hwt drc my_package.hwt_pkg

# Common options (work across all domains)
hwt drc <file> --output report.json
hwt drc <file> --severity error      # Only errors
hwt drc <file> --incremental         # Check modified only
hwt drc <file> --parallel --threads 8
```

---

## Related Topics

- [Shared Module Consolidation](../core-architecture/shared-module-consolidation.md)
- [Command-Line Interface](./cli.md)
- [Real-Time Preview](./realtime-preview.md)
