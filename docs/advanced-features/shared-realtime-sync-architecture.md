# Shared Real-Time Sync Architecture

## Overview

Hardware Tool provides a **unified real-time synchronization engine** that works across all hardware domains. Whether you're syncing schematic-to-PCB, RTL-to-layout, circuit-to-qubit, mechanical-to-electrical, or die-to-package — the same bidirectional sync infrastructure keeps all views consistent.

> **"One Hardware Tool that does it all"** — The same cross-probing, live preview, and change propagation works for every hardware type.

---

## Shared Sync Components

All domain-specific sync implementations inherit these core capabilities:

| Component | Description |
|-----------|-------------|
| **Bidirectional Sync** | Changes propagate in both directions |
| **Cross-Probing** | Select in one view, highlight in others |
| **Conflict Resolution** | Detect and resolve sync conflicts |
| **Change Tracking** | Track pending changes with indicators |
| **Incremental Updates** | Only sync what changed |
| **Undo Integration** | Sync operations are undoable |

---

## Unified Data Model (All Domains)

```
┌─────────────────┐         ┌─────────────────┐
│   View A        │◄───────►│   View B        │
│   (Abstract)    │  Sync   │   (Physical)    │
└────────┬────────┘         └────────┬────────┘
         │                           │
         ▼                           ▼
┌─────────────────────────────────────────────┐
│              Unified Data Model             │
│         (Domain-Specific JSON IR)           │
└─────────────────────────────────────────────┘
```

All sync operations go through a unified intermediate representation, ensuring consistency across views.

---

## Sync Configuration (All Domains)

```rust
/// Base sync configuration inherited by all domain sync engines
SyncEngineBase {
    // Sync behavior
    enabled: bool,
    direction: SyncDirection,  // Forward, Back, Bidirectional
    
    // Timing
    mode: SyncMode::Immediate,  // or OnSave, Manual
    debounce_ms: 100,
    
    // Conflict handling
    conflict_resolution: ConflictResolution::Prompt,  // or AutoResolve, Reject
    
    // Performance
    incremental: true,
    batch_changes: true,
}

pub enum SyncDirection {
    Forward,       // Abstract → Physical
    Back,          // Physical → Abstract
    Bidirectional, // Both ways
}
```

---

## Sync Status Indicators (All Domains)

| Indicator | Meaning | Action |
|-----------|---------|--------|
| 🟢 Green | Fully synchronized | None needed |
| 🟡 Yellow | Pending changes | Will sync soon |
| 🔴 Red | Sync conflict | Requires resolution |
| ⚪ Gray | Sync disabled | Enable to sync |

### Status Bar

```
┌─────────────────────────────────────────────────────────────────┐
│ Sync: 🟢 Synchronized │ Last: 2s ago │ Pending: 0 │ [⟳ Sync Now]│
└─────────────────────────────────────────────────────────────────┘
```

---

## Cross-Probing (All Domains)

```rust
/// Cross-probing works identically across all domains
CrossProbe {
    enabled: bool,
    
    // Selection sync
    sync_selection: bool,
    multi_select: bool,
    
    // View behavior
    sync_zoom: bool,
    center_on_item: bool,
    
    // Visual feedback
    highlight_color: Color,
    highlight_duration: f64,  // seconds, 0 = persistent
    
    // Keyboard shortcut
    shortcut: Key::F4,
}
```

### Cross-Probe UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Schematic View                    │ Layout View                 │
├───────────────────────────────────┼─────────────────────────────┤
│                                   │                             │
│    ┌─────┐                        │      ╔═════════╗            │
│    │ U1  │ ← Selected             │      ║   U1    ║ ← Highlighted
│    │     │                        │      ║         ║            │
│    └─────┘                        │      ╚═════════╝            │
│                                   │                             │
│ Click component to cross-probe →  │  ← Automatically centered   │
└───────────────────────────────────┴─────────────────────────────┘
```

---

## Conflict Resolution (All Domains)

```rust
/// Conflict types detected by all sync engines
pub enum SyncConflict {
    // Both views modified same item
    ConcurrentEdit { item: String, view_a: Change, view_b: Change },
    
    // Item deleted in one view, modified in other
    DeleteModify { item: String, deleted_in: View, modified_in: View },
    
    // Incompatible changes
    Incompatible { item: String, reason: String },
}

/// Resolution strategies
pub enum ConflictResolution {
    Prompt,           // Ask user
    PreferAbstract,   // Abstract view wins
    PreferPhysical,   // Physical view wins
    Merge,            // Attempt automatic merge
    Reject,           // Reject conflicting change
}
```

### Conflict Resolution UI

```
┌─────────────────────────────────────────────────────────────────┐
│ ⚠ Sync Conflict Detected                                 [✕]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Item: U1 (Microcontroller)                                     │
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Schematic Change:          │ Layout Change:                 │ │
│ │ Value: STM32F4 → STM32F7   │ Position: (10,20) → (15,25)   │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [Keep Schematic] [Keep Layout] [Keep Both] [Cancel]            │
└─────────────────────────────────────────────────────────────────┘
```

---

## Change Tracking (All Domains)

```rust
/// Pending changes tracked by all sync engines
PendingChanges {
    // Change queue
    changes: Vec<SyncChange>,
    
    // Grouping
    group_by_item: bool,
    group_by_type: bool,
    
    // Display
    show_in_status_bar: bool,
    show_count: true,
}

pub struct SyncChange {
    pub item: String,
    pub change_type: ChangeType,
    pub source_view: View,
    pub timestamp: DateTime,
    pub applied: bool,
}
```

---

## Keyboard Shortcuts (All Domains)

| Shortcut | Action |
|----------|--------|
| `F4` | Cross-probe selected item |
| `Ctrl+Shift+S` | Force sync now |
| `Ctrl+Shift+R` | Resolve conflicts |
| `Ctrl+Shift+U` | Undo last sync |

---

## CLI Commands (All Domains)

```bash
# Check sync status
hwt sync status my_design.hwt

# Force sync
hwt sync now my_design.hwt

# Show pending changes
hwt sync pending my_design.hwt

# Resolve conflicts
hwt sync resolve my_design.hwt --strategy prefer-abstract
```

---

## Domain-Specific Sync Pairs

Each hardware domain extends the shared sync engine with specialized view pairs:

| Domain | View A (Abstract) | View B (Physical) | Sync Items |
|--------|-------------------|-------------------|------------|
| **PCB** | Schematic | PCB Layout | Components, Nets, Values |
| **IC** | RTL/Netlist | Layout | Cells, Nets, Timing |
| **Quantum** | Circuit | Qubit Layout | Gates, Qubits, Coupling |
| **MEMS** | Mechanical | Electrical | Structures, Electrodes |
| **RF** | Schematic | EM Layout | Components, Transmission lines |
| **Packaging** | Die Map | Package Layout | Dies, Bumps, Routing |

See domain-specific documentation:

- [PCB Real-Time Preview](./realtime-preview.md) - Schematic ↔ PCB sync
- [IC RTL-to-Layout Sync](../ic-design/advanced-features/real-time-rtl-to-layout-sync.md)
- [Quantum Circuit-to-Layout Sync](../quantum-hardware/advanced-features/real-time-circuit-to-layout-sync.md)
- [MEMS Mechanical-to-Electrical Sync](../mems-sensors/advanced-features/real-time-mechanical-to-electrical-sync.md)
- [RF Schematic-to-EM Sync](../rf-photonics/advanced-features/real-time-schematic-to-em-sync.md)
- [Packaging Die-to-Package Sync](../advanced-packaging/advanced-features/real-time-die-to-package-sync.md)

---

## Rust API (Shared Base)

```rust
use hardware_tool::sync::*;

/// All domain sync engines implement this trait
trait SyncEngine {
    // Sync operations
    fn sync_now(&mut self) -> Result<SyncReport>;
    fn sync_item(&mut self, item: &str) -> Result<()>;
    
    // Status
    fn get_status(&self) -> SyncStatus;
    fn get_pending_changes(&self) -> Vec<SyncChange>;
    
    // Conflicts
    fn get_conflicts(&self) -> Vec<SyncConflict>;
    fn resolve_conflict(&mut self, conflict: &SyncConflict, resolution: ConflictResolution) -> Result<()>;
    
    // Cross-probing
    fn cross_probe(&self, item: &str) -> Result<()>;
    fn highlight_in_view(&self, view: View, items: &[&str]) -> Result<()>;
    
    // Configuration
    fn set_config(&mut self, config: SyncConfig) -> Result<()>;
    fn get_config(&self) -> SyncConfig;
}
```

---

## Related Topics

- [Shared Module Consolidation](../core-architecture/shared-module-consolidation.md)
- [Undo/Redo & Versioning](./undo-redo-versioning.md)
- [Real-Time Collaboration](./realtime-collaboration.md)
