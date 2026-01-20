# Shared Module Consolidation Guide

## Overview

This document identifies functionality that is **shared across all hardware domains** and should be consolidated into the unified platform architecture. Domain-specific documentation should reference these shared modules rather than duplicating content.

---

## Consolidation Pattern

Each shared module follows this pattern:

1. **Shared Architecture Doc** - Defines common infrastructure, APIs, and behavior
2. **Domain Extension Docs** - Reference shared doc, document only domain-specific extensions

Example:
```
3d-visualization/
├── shared-3d-viewer-architecture.md    ← Common infrastructure
├── 3d-pcb-viewer.md                    ← PCB-specific extensions
quantum-hardware/3d-visualization-cryogenics/
├── 3d-qubit-viewer.md                  ← Quantum-specific extensions (references shared)
```

---

## Modules Requiring Consolidation

### 1. 3D Visualization ✓ CONSOLIDATED

| Shared Doc | Domain Extensions |
|------------|-------------------|
| `3d-visualization/shared-3d-viewer-architecture.md` | PCB, IC, Quantum, MEMS, RF, Packaging viewers |

**Causal Keywords:** `3D viewer`, `visualization`, `Bevy`, `render`, `cross-section`, `export STEP`

---

### 2. Design Rule Check (DRC) ✓ CONSOLIDATED

| Shared Doc | Domain Extensions |
|------------|-------------------|
| `advanced-features/shared-drc-architecture.md` | PCB, IC, Quantum, MEMS, RF, Packaging DRC |

**Causal Keywords:** `DRC`, `design rule check`, `violation`, `clearance`, `rule engine`

**Shared Components:**
- Rule engine architecture
- Severity levels (Error, Warning, Info, Ignore)
- Violation reporting UI
- Exclusion/waiver management
- Report generation (JSON, CSV, HTML)
- Real-time checking
- CLI commands

**Domain-Specific:**
- Rule categories (PCB clearance, IC spacing, MEMS etch holes, etc.)
- Rule values (foundry/manufacturer specific)
- Fix suggestions

---

### 3. Undo/Redo & Versioning

| Current State | Consolidation Target |
|---------------|---------------------|
| `advanced-features/undo-redo-versioning.md` | Already shared |

**Causal Keywords:** `undo`, `redo`, `history`, `versioning`, `command pattern`

**Status:** Already consolidated. Domain docs reference this shared module.

---

### 4. Command-Line Interface (CLI)

| Current State | Consolidation Target |
|---------------|---------------------|
| `advanced-features/cli.md` | Shared CLI architecture |

**Causal Keywords:** `CLI`, `command-line`, `batch`, `automation`, `hwt` command

**Shared Components:**
- Command structure (`hwt <domain> <action>`)
- Output formats (JSON, table, plain)
- Configuration files
- Exit codes
- Logging

**Domain-Specific:**
- Domain commands (`hwt pcb export`, `hwt ic synthesize`, `hwt quantum transpile`)
- Domain options

---

### 5. Project Structure & Management ✓ CONSOLIDATED

| Shared Doc | Domain Extensions |
|------------|-------------------|
| `core-architecture/shared-project-architecture.md` | PCB, IC, Quantum, MEMS, RF, Packaging projects |

**Causal Keywords:** `project structure`, `project management`, `.hwt`, `directory`, `workspace`

**Shared Components:**
- `.hwt` TOML-based project format
- Directory structure conventions
- Git integration & `.hwtignore`
- Build system (validate, simulate, export)
- Project-wide search
- CLI commands

**Domain-Specific:**
- Domain file extensions (`.hwt_ic`, `.hwt_quantum`, etc.)
- PDK/process references
- Domain-specific sections in project file

---

### 6. Libraries & Components ✓ CONSOLIDATED

| Shared Doc | Domain Extensions |
|------------|-------------------|
| `core-architecture/shared-library-architecture.md` | Symbols, Footprints, Cells, Gates, Structures, Dies |

**Causal Keywords:** `library`, `libraries`, `component`, `symbol`, `footprint`, `cell`

**Shared Components:**
- Library file structure
- Library browser UI
- Search and filtering
- Import/export formats
- Version control (Git)
- Validation system
- CLI commands

**Domain-Specific:**
- Component types (symbols, footprints, cells, gates, structures, dies)
- Domain properties (pins, pads, timing, fidelity, resonance)
- Simulation models

---

### 7. Simulation Integration ✓ CONSOLIDATED

| Shared Doc | Domain Extensions |
|------------|-------------------|
| `advanced-features/shared-simulation-architecture.md` | SPICE, RTL, Quantum, MEMS FEA, RF/EM, Thermal |

**Causal Keywords:** `simulation`, `simulate`, `solver`, `analysis`, `integration`

**Shared Components:**
- Job manager (queue, schedule, monitor)
- Results visualization (waveforms, plots)
- Parameter sweeps
- Optimization engine
- Caching & incremental simulation
- CLI commands

**Domain-Specific:**
- Solver types (SPICE, FEM, FDTD, STA, etc.)
- Analysis types (DC, AC, modal, S-parameters)
- Model formats

---

### 8. Real-Time Sync/Preview ✓ CONSOLIDATED

| Shared Doc | Domain Extensions |
|------------|-------------------|
| `advanced-features/shared-realtime-sync-architecture.md` | PCB, IC, Quantum, MEMS, RF, Packaging sync |

**Causal Keywords:** `real-time`, `sync`, `live preview`, `bidirectional`, `cross-probe`

**Shared Components:**
- Bidirectional sync engine
- Cross-probing
- Conflict resolution UI
- Change tracking & indicators
- Incremental updates
- CLI commands

**Domain-Specific:**
- View pairs (schematic↔layout, RTL↔gates, circuit↔qubits)
- Sync item types
- Domain-specific conflict rules

---

### 9. Export/Import & Interchange ✓ CONSOLIDATED

| Shared Doc | Domain Extensions |
|------------|-------------------|
| `advanced-features/shared-export-import-architecture.md` | Gerber, GDSII, Touchstone, ODB++, etc. |

**Causal Keywords:** `export`, `import`, `interchange`, `format`

**Shared Components:**
- Export job manager (queue, progress, cancellation)
- Pre-export validation
- Batch processing
- Export history & re-export
- Import preview & conflict resolution
- CLI commands

**Domain-Specific:**
- Format implementations (Gerber, GDSII, Touchstone, etc.)
- Format-specific options

---

### 10. Thermal Simulation

| Current State | Consolidation Target |
|---------------|---------------------|
| `advanced-features/thermal-simulation.md` | Already shared |

**Causal Keywords:** `thermal`, `temperature`, `heat`, `Joule heating`

**Status:** Already consolidated. Domain docs can reference for thermal overlay data.

---

## Implementation Priority

| Priority | Module | Status | Impact |
|----------|--------|--------|--------|
| 1 | 3D Visualization | ✓ Done | High |
| 2 | DRC | ✓ Done | High |
| 3 | Libraries | ✓ Done | High |
| 4 | Simulation | ✓ Done | High |
| 5 | Real-Time Sync | ✓ Done | Medium |
| 6 | Project Structure | ✓ Done | Medium |
| 7 | Export/Import | ✓ Done | Medium |
| 8 | CLI | Already shared | Medium |
| 9 | Undo/Redo | Already shared | Medium |
| 10 | Thermal | Already shared | Medium |

---

## How to Consolidate a Module

1. **Identify shared components** - What's common across all domains?
2. **Create shared architecture doc** - Document common infrastructure
3. **Update domain docs** - Add "Inherits from" reference, remove duplicated content
4. **Update main README** - Ensure shared module is in "Unified Platform Foundation" section
5. **Update domain READMEs** - Ensure domain-specific extensions are clear

---

## Related Topics

- [Main README](../README.md) - Unified platform documentation structure
- [Shared 3D Viewer Architecture](../3d-visualization/shared-3d-viewer-architecture.md) - Example of consolidated module
