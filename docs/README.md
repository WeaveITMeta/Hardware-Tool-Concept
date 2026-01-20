# Hardware Tool Documentation

**The Universal Hardware Design Platform**

*Design ANY hardware: PCBs, Integrated Circuits, Quantum Processors, MEMS, RF/Photonics, and Advanced Packaging — all in one unified Rust-native environment.*

Rust + Slint + Bevy 2D/3D + Multi-Physics Simulation  
Gerber RS-274X + IPC-2581 + ODB++ + GDSII + OASIS

A revolutionary, pure-Rust EDA suite that transcends traditional boundaries. From PCB layout to transistor-level IC design, from superconducting qubits to MEMS accelerometers, from RF amplifiers to photonic circuits — Hardware Tool provides a unified platform for designing any kind of hardware. Built with Slint + Bevy for fluid editing, real-time 3D visualization, physics-based simulation, and AI-powered design assistance.

*The next-generation open-source EDA experience — design any hardware, at any scale, with one tool.*

---

## Documentation Index

### Core Architecture & Fundamentals

- [Project Structure & Management](./core-architecture/project-structure-management.md) - Unified project file format, directory structure
- [Unified Project File Format](./core-architecture/unified-project-file-format.md) - .hwt format specification, KiCAD compatibility
- [Circuit JSON as Intermediate Representation](./core-architecture/circuit-json-ir.md) - Universal data model for schematics, layouts, nets
- [Schematic Capture Workflow](./core-architecture/schematic-capture-workflow.md) - From abstract circuit to netlist generation
- [PCB Layout Workflow](./core-architecture/pcb-layout-workflow.md) - From netlist to physical board realization
- [Programmatic / Code-First Design](./core-architecture/programmatic-design.md) - Defining circuits via Rust code/macros

### Schematic Editor Concepts

- [Symbols & Libraries](./schematic-editor/symbols-libraries.md) - Custom + official symbols with pins, properties
- [Hierarchical & Multi-Sheet Schematics](./schematic-editor/hierarchical-schematics.md) - Sub-sheets, sheet pins, labels
- [Wiring & Connectivity](./schematic-editor/wiring-connectivity.md) - Wires, buses, labels, power symbols, net classes
- [Electrical Rules Check (ERC)](./schematic-editor/erc.md) - Pin connection validation, unconnected pins
- [Annotation & Reference Designators](./schematic-editor/annotation-reference-designators.md) - Automatic numbering, back-annotation
- [SPICE Simulation Integration](./schematic-editor/spice-simulation.md) - Netlist export and ngspice simulation

### PCB Layout Concepts

- [Footprints & Libraries](./pcb-layout/footprints-libraries.md) - Footprint assignment, pad definitions, 3D models
- [Component Placement](./pcb-layout/component-placement.md) - Manual drag/drop, auto-placement hints
- [Interactive Routing](./pcb-layout/interactive-routing.md) - Manual + push-and-shove router, differential pairs
- [Automatic Routing & Layout Engines](./pcb-layout/auto-routing.md) - pcbFlex / pcbGrid / pcbPack auto-layout
- [Copper Zones / Pours](./pcb-layout/copper-zones.md) - Filled regions, priority, thermals, clearance
- [Design Rule Check (DRC)](./pcb-layout/drc.md) - Clearance, connection, size, via violations
- [Multi-Layer Support](./pcb-layout/multi-layer.md) - Inner layers, blind/buried vias, stackup
- [Via & Via Stitching](./pcb-layout/via-stitching.md) - Plated, tented, microvias, ground stitching
- [Net Inspection & Highlighting](./pcb-layout/net-inspection.md) - Real-time net tracing, ratsnest display

### 3D Visualization & Mechanical Integration

- [3D PCB Viewer](./3d-visualization/3d-pcb-viewer.md) - Real-time 3D rendering with raytracing
- [STEP & 3D Model Export/Import](./3d-visualization/step-3d-models.md) - Mechanical fit checks, enclosure integration

### Manufacturing & Output Formats

- [Gerber RS-274X Export](./manufacturing-output/gerber-export.md) - Copper, mask, silkscreen, drill layers
- [IPC-2581 Export](./manufacturing-output/ipc2581-export.md) - Single-file intelligent format
- [ODB++ Export](./manufacturing-output/odbpp-export.md) - Folder-based format for professional fabs
- [BOM & Pick-and-Place Generation](./manufacturing-output/bom-pick-place.md) - Centroid files, component lists
- [Fabrication & Assembly Drawings](./manufacturing-output/fabrication-drawings.md) - Drawing sheets, title blocks

### Advanced & Productivity Features

- [Library Conventions & Quality Control](./advanced-features/library-conventions.md) - KLC-style guidelines
- [Design for Manufacturability (DFM) Checks](./advanced-features/dfm-checks.md) - Yield prediction, acute angles
- [Undo/Redo & Versioning](./advanced-features/undo-redo-versioning.md) - Command-based history, crash recovery
- [Command-Line Interface (CLI)](./advanced-features/cli.md) - Batch processing, export, automation
- [Calculator Tools](./advanced-features/calculator-tools.md) - Track width, via current, impedance, thermal overlay, test points
- [Thermal Simulation](./advanced-features/thermal-simulation.md) - Physics-based Joule heating, FEM solver, heat maps, radiation
- [Signal & Power Integrity](./advanced-features/signal-power-integrity.md) - S-parameters, eye diagrams, PDN impedance, crosstalk
- [Electromagnetic Simulation](./advanced-features/electromagnetic-simulation.md) - EMC/EMI, near-field, radiated emissions, shielding
- [Layout Properties & Constraints](./advanced-features/layout-properties-constraints.md) - Manual nudges, keep-out zones
- [Real-Time Preview & Iteration](./advanced-features/realtime-preview.md) - Live schematic-to-PCB sync
- [Real-Time Collaboration](./advanced-features/realtime-collaboration.md) - Multi-user editing, CRDT sync, design review
- [Variant Manager](./advanced-features/variant-manager.md) - What-if design exploration, cost comparison, variant BOMs
- [Digital Twin & AR Debug](./advanced-features/digital-twin-ar.md) - Live simulation, current flow, AR overlay, fault injection

### UX/UI Design

- [Main Window Layout](./ux-ui-design/main-window-layout.md) - Adaptive interface, canvas-first experience
- [Innovative Interaction Patterns](./ux-ui-design/innovative-interaction-patterns.md) - Magnet cursor, gesture router, shadow nudge
- [Visual Style Guidelines](./ux-ui-design/visual-style-guidelines.md) - Color palette, typography, motion design
- [Accessibility & Theming](./ux-ui-design/accessibility-and-theming.md) - Dark/light modes, color-blind support
- [Keyboard Shortcuts Reference](./ux-ui-design/keyboard-shortcuts-reference.md) - Complete shortcut catalog
- [Onboarding & First Experience](./ux-ui-design/onboarding-and-first-experience.md) - Tutorials, migration guides

### AI Integration

- [API Keys Configuration](./ai-integration/api-keys-configuration.md) - Secure AI provider setup, multi-provider support
- [Native AI Design Assistant](./ai-integration/native-ai-assistant.md) - First-party AI with direct tool access
- [AI-Powered Routing & Optimization](./ai-integration/ai-routing-optimization.md) - First-principles optimization, 5-10x efficiency
- [Generative AI Design](./advanced-features/generative-ai-design.md) - Spec-to-design, component selection, layout generation
- [Benchmarking Simulator](./ai-integration/benchmarking-simulator.md) - Real-time physics-based validation

### Integrated Circuit Design

- [IC Design Overview](./ic-design/integrated-circuit-design.md) - Transistor-level schematic, layout, DRC, LVS, GDSII
- PDK Support - SkyWater 130nm, GF180MCU, IHP SG13G2, commercial PDKs
- Digital ASIC Flow - RTL synthesis, place & route, timing closure
- Analog IC Design - Op-amps, ADCs, PLLs, custom layout

### Quantum Hardware Design

- [Quantum Circuit Design](./quantum-hardware/quantum-circuit-design.md) - Superconducting qubits, photonics, trapped ions
- Transmon & Resonator Design - CPW geometry, coupling, EM simulation
- Quantum Control - Pulse design, gate optimization, Hamiltonian simulation
- Fabrication Integration - Process flow, design rules, GDSII export

### MEMS & Sensor Design

- [MEMS Sensor Design](./mems-sensors/mems-sensor-design.md) - Accelerometers, gyroscopes, pressure sensors
- Multi-Physics Simulation - FEA structural, electrostatic, thermal
- Foundry Support - PolyMUMPs, SOIMUMPs, ThELMA, XMB10
- Packaging Integration - Hermetic cavity, wire bond, die attach

### RF, Microwave & Photonics

- [RF/Microwave Design](./rf-photonics/rf-microwave-design.md) - LNA, PA, filters, antennas
- Smith Chart & Matching - Impedance matching, S-parameters
- Photonic Integrated Circuits - Silicon photonics, waveguides, modulators
- EM Simulation - 3D FDTD, mode solver, radiation patterns

### Advanced Packaging & Chiplets

- [Chiplet Integration](./advanced-packaging/chiplet-integration.md) - 2.5D/3D packaging, interposers, UCIe
- Silicon Interposer - TSV, RDL, micro-bumps, C4 bumps
- Fan-Out Packaging - FOWLP, RDL routing, ball grid
- Thermal & Mechanical - Warpage, stress, thermal resistance

### Appendices

- [Performance Targets](./appendices/performance-targets.md) - Response times, memory usage, benchmarks
- [Roadmap & Priorities](./appendices/roadmap-priorities.md) - Development phases, milestones
- [Comparison with KiCAD and TSCircuit](./appendices/comparison-with-kicad-and-tscircuit.md) - Feature matrix, philosophy

---

## Quick Start

```bash
# Install Hardware Tool
cargo install hardware-tool

# Create new project
hwt new my_project

# Open GUI
hwt open my_project.hwt

# Export Gerbers (CLI)
hwt export gerber my_board.hwt_pcb --output ./gerber/

# AI-assisted routing
hwt ai route --optimize signal-integrity
```

## Technology Stack

| Component | Technology |
|-----------|------------|
| Language | Rust |
| UI Framework | Slint |
| 3D Rendering | Bevy |
| Gerber Export | gerber-rs |
| PCB Formats | Gerber RS-274X, IPC-2581, ODB++ |
| IC Formats | GDSII, OASIS, LEF/DEF |
| Simulation | SPICE (ngspice), FEM, FDTD, BEM |
| AI Integration | OpenAI, Anthropic, Google, Local (Ollama) |

## Hardware Domains Supported

| Domain | Capabilities |
|--------|--------------|
| **PCB Design** | Schematic capture, layout, routing, DRC, manufacturing output |
| **Integrated Circuits** | Transistor-level design, PDK support, DRC/LVS, GDSII tape-out |
| **Quantum Hardware** | Superconducting qubits, photonics, trapped ions, control systems |
| **MEMS & Sensors** | Accelerometers, gyroscopes, pressure sensors, multi-physics FEA |
| **RF/Microwave** | LNAs, PAs, filters, antennas, Smith chart, S-parameters |
| **Photonics** | Silicon photonics, waveguides, ring resonators, modulators |
| **Advanced Packaging** | Chiplets, 2.5D/3D, interposers, TSV, UCIe, thermal analysis |

## Core UX Philosophy

1. **Fluid Canvas-First Experience** — One continuous, zoomable, pannable workspace
2. **Context-Aware Adaptive Interface** — UI elements appear when and where needed
3. **Dual-Paradigm Mastery** — GUI and code-first workflows, deeply synchronized
4. **Visual Language of Feedback** — Rich, non-intrusive visual feedback everywhere
5. **Performance as a Feature** — Instant response even on very large designs
6. **Accessibility by Default** — Dark/light, high-contrast, color-blind friendly

## License

[License information here]
