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

### 2. Design Rule Check (DRC)

| Current State | Consolidation Target |
|---------------|---------------------|
| `pcb-layout/drc.md` | Shared DRC engine architecture |
| `ic-design/.../physical-verification-drc-lvs.md` | IC-specific rules |
| `mems-sensors/.../mems-design-rule-check.md` | MEMS-specific rules |
| `quantum-hardware/.../quantum-design-rule-check.md` | Quantum-specific rules |
| `rf-photonics/.../rf-design-rule-check.md` | RF-specific rules |
| `advanced-packaging/.../packaging-design-rule-check.md` | Packaging-specific rules |

**Causal Keywords:** `DRC`, `design rule check`, `violation`, `clearance`, `rule engine`

**Shared Components:**
- Rule definition format
- Violation reporting UI
- Batch checking API
- Waiver management
- Report generation

**Domain-Specific:**
- Rule sets (PCB clearance vs IC metal spacing vs MEMS etch rules)
- Severity levels
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

### 5. Project Structure & Management

| Current State | Consolidation Target |
|---------------|---------------------|
| `core-architecture/project-structure-management.md` | Shared project architecture |
| `ic-design/core-architecture/ic-project-structure-and-management.md` | IC extensions |
| `quantum-hardware/core-architecture/quantum-project-structure-and-management.md` | Quantum extensions |
| `mems-sensors/core-architecture/mems-project-structure-and-management.md` | MEMS extensions |
| `rf-photonics/core-architecture/rf-project-structure-and-management.md` | RF extensions |
| `advanced-packaging/core-architecture/chiplet-project-structure-and-management.md` | Packaging extensions |

**Causal Keywords:** `project structure`, `project management`, `.hwt`, `directory`, `workspace`

**Shared Components:**
- `.hwt` file format
- Directory structure
- Git integration
- Multi-project workspaces
- Dependencies

**Domain-Specific:**
- Domain file extensions (`.hwt_ic`, `.hwt_quantum`, etc.)
- PDK/process references
- Domain-specific metadata

---

### 6. Libraries & Components

| Current State | Consolidation Target |
|---------------|---------------------|
| `schematic-editor/symbols-libraries.md` | Shared library architecture |
| `pcb-layout/footprints-libraries.md` | PCB footprints |
| `ic-design/.../cells-and-libraries.md` | IC cells |
| `quantum-hardware/.../gates-and-libraries.md` | Quantum gates |
| `mems-sensors/.../structures-and-libraries.md` | MEMS structures |
| `rf-photonics/.../rf-components-and-libraries.md` | RF components |
| `advanced-packaging/.../die-ip-and-libraries.md` | Die IP |

**Causal Keywords:** `library`, `libraries`, `component`, `symbol`, `footprint`, `cell`

**Shared Components:**
- Library file format
- Library browser UI
- Search and filtering
- Import/export
- Version management

**Domain-Specific:**
- Component types
- Properties and parameters
- Simulation models

---

### 7. Simulation Integration

| Current State | Consolidation Target |
|---------------|---------------------|
| `schematic-editor/spice-simulation.md` | Shared simulation architecture |
| `ic-design/.../rtl-simulation-integration.md` | RTL simulation |
| `quantum-hardware/.../quantum-simulation-integration.md` | Quantum simulation |
| `mems-sensors/.../mems-simulation-integration.md` | MEMS FEA |
| `rf-photonics/.../rf-simulation-integration.md` | RF/EM simulation |
| `advanced-packaging/.../packaging-simulation-integration.md` | Thermal/mechanical |

**Causal Keywords:** `simulation`, `simulate`, `solver`, `analysis`, `integration`

**Shared Components:**
- Simulation job management
- Results visualization
- Parameter sweeps
- Optimization loops
- Report generation

**Domain-Specific:**
- Solver types (SPICE, FEM, FDTD, etc.)
- Analysis types
- Model formats

---

### 8. Real-Time Sync/Preview

| Current State | Consolidation Target |
|---------------|---------------------|
| `advanced-features/realtime-preview.md` | Shared sync architecture |
| `ic-design/.../real-time-rtl-to-layout-sync.md` | IC sync |
| `quantum-hardware/.../real-time-circuit-to-layout-sync.md` | Quantum sync |
| `mems-sensors/.../real-time-mechanical-to-electrical-sync.md` | MEMS sync |
| `rf-photonics/.../real-time-schematic-to-em-sync.md` | RF sync |
| `advanced-packaging/.../real-time-die-to-package-sync.md` | Packaging sync |

**Causal Keywords:** `real-time`, `sync`, `live preview`, `bidirectional`

**Shared Components:**
- Change detection
- Incremental update
- Conflict resolution
- Performance optimization

**Domain-Specific:**
- What syncs (schematic↔layout, RTL↔gates, etc.)
- Sync rules

---

### 9. Export/Import & Interchange

| Current State | Consolidation Target |
|---------------|---------------------|
| Various domain-specific export docs | Shared export architecture |

**Causal Keywords:** `export`, `import`, `interchange`, `format`

**Shared Components:**
- Export job management
- Progress reporting
- Error handling
- Batch export
- Format detection

**Domain-Specific:**
- Format implementations (Gerber, GDSII, Touchstone, etc.)
- Format options

---

### 10. Thermal Simulation

| Current State | Consolidation Target |
|---------------|---------------------|
| `advanced-features/thermal-simulation.md` | Already shared |

**Causal Keywords:** `thermal`, `temperature`, `heat`, `Joule heating`

**Status:** Already consolidated. Domain docs can reference for thermal overlay data.

---

## Implementation Priority

| Priority | Module | Effort | Impact |
|----------|--------|--------|--------|
| 1 | 3D Visualization | ✓ Done | High |
| 2 | DRC | Medium | High |
| 3 | Libraries | Medium | High |
| 4 | Project Structure | Low | Medium |
| 5 | CLI | Low | Medium |
| 6 | Simulation | High | High |
| 7 | Real-Time Sync | Medium | Medium |
| 8 | Export/Import | Medium | Medium |

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
