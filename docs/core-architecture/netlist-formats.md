# Netlist Formats

## Overview

Hardware Tool supports a comprehensive set of **netlist interchange formats** for design data exchange between tools, simulators, and manufacturing flows. Whether you're exporting SPICE for simulation, Verilog for synthesis, or structural netlists for verification — the same unified export infrastructure handles it all.

> **"One Hardware Tool That Does It All"** — Import and export netlists in any industry-standard format.

---

## Supported Formats

| Format | Domain | Direction | Description |
|--------|--------|-----------|-------------|
| **SPICE** | PCB, IC, RF | Import/Export | Circuit simulation |
| **Verilog** | IC | Import/Export | RTL and structural |
| **SystemVerilog** | IC | Import/Export | Advanced RTL |
| **VHDL** | IC | Import/Export | RTL and structural |
| **EDIF** | PCB, IC | Import/Export | Electronic Design Interchange |
| **Structural JSON** | All | Export | Hardware Tool native |
| **KiCad Netlist** | PCB | Import/Export | KiCad compatibility |
| **OrCAD Netlist** | PCB | Import | OrCAD compatibility |
| **OpenQASM** | Quantum | Import/Export | Quantum circuits |
| **Qiskit** | Quantum | Export | Quantum Python |

---

## SPICE Netlist

### Export Format

```spice
* Hardware Tool SPICE Netlist
* Project: amplifier_v2
* Date: 2026-01-20
* Tool: Hardware Tool v1.0.0

.TITLE Amplifier Stage

* Include model libraries
.INCLUDE "models/transistors.lib"
.INCLUDE "models/passives.lib"

* Power supplies
VCC VCC 0 DC 5.0
VEE VEE 0 DC -5.0

* Input signal
VIN INPUT 0 AC 1.0 SIN(0 0.1 1K)

* Components
R1 INPUT BASE 10K
R2 VCC BASE 47K
R3 VCC COLLECTOR 4.7K
R4 EMITTER VEE 1K
C1 INPUT BASE 10U
C2 COLLECTOR OUTPUT 10U
Q1 COLLECTOR BASE EMITTER 2N2222

* Analysis
.DC VIN -1 1 0.01
.AC DEC 100 1 1MEG
.TRAN 1U 10M

.END
```

### Export Options

```rust
SpiceExportOptions {
    // Format options
    format: SpiceFormat::Standard,  // Standard, HSpice, LTSpice, Ngspice
    
    // Include options
    include_models: true,
    include_subcircuits: true,
    include_comments: true,
    
    // Analysis inclusion
    include_analysis: true,
    analysis_types: vec![Analysis::DC, Analysis::AC, Analysis::Transient],
    
    // Naming
    net_naming: NetNaming::Hierarchical,  // Flat, Hierarchical
    case_sensitive: false,
}
```

---

## Verilog Netlist

### Structural Netlist

```verilog
// Hardware Tool Verilog Netlist
// Project: my_chip
// Date: 2026-01-20

module top (
    input  wire clk,
    input  wire rst_n,
    input  wire [7:0] data_in,
    output wire [7:0] data_out
);

    // Internal wires
    wire [7:0] reg_out;
    wire [7:0] alu_out;

    // Instances
    register_8bit u_reg (
        .clk(clk),
        .rst_n(rst_n),
        .d(data_in),
        .q(reg_out)
    );

    alu_8bit u_alu (
        .a(reg_out),
        .b(data_in),
        .op(2'b00),
        .result(alu_out)
    );

    assign data_out = alu_out;

endmodule
```

### Gate-Level Netlist

```verilog
// Gate-level netlist (post-synthesis)
module top (clk, rst_n, data_in, data_out);
    input clk, rst_n;
    input [7:0] data_in;
    output [7:0] data_out;

    wire n1, n2, n3, n4;

    sky130_fd_sc_hd__inv_1 U1 (.A(rst_n), .Y(n1));
    sky130_fd_sc_hd__nand2_1 U2 (.A(data_in[0]), .B(n1), .Y(n2));
    sky130_fd_sc_hd__dff_1 U3 (.CLK(clk), .D(n2), .Q(data_out[0]));
    // ... more gates
endmodule
```

### Export Options

```rust
VerilogExportOptions {
    // Netlist type
    netlist_type: VerilogType::Structural,  // RTL, Structural, GateLevel
    
    // Hierarchy
    flatten: false,
    max_hierarchy_depth: None,
    
    // Naming
    escape_identifiers: true,
    net_naming: NetNaming::Hierarchical,
    
    // Cell library
    cell_library: Some("sky130_fd_sc_hd"),
    
    // Timing
    include_delays: false,
    delay_format: DelayFormat::SDF,
}
```

---

## EDIF Netlist

```edif
(edif my_design
  (edifVersion 2 0 0)
  (edifLevel 0)
  (keywordMap (keywordLevel 0))
  (status
    (written
      (timestamp 2026 1 20 12 0 0)
      (program "Hardware Tool" (version "1.0.0"))))
  
  (library my_lib
    (edifLevel 0)
    (technology (numberDefinition))
    
    (cell RESISTOR
      (cellType GENERIC)
      (view netlist
        (viewType NETLIST)
        (interface
          (port P1 (direction INPUT))
          (port P2 (direction OUTPUT)))))
    
    (cell TOP
      (cellType GENERIC)
      (view netlist
        (viewType NETLIST)
        (interface
          (port VCC (direction INPUT))
          (port GND (direction INPUT))
          (port OUT (direction OUTPUT)))
        (contents
          (instance R1 (viewRef netlist (cellRef RESISTOR)))
          (net VCC (joined (portRef VCC) (portRef P1 (instanceRef R1))))
          (net N1 (joined (portRef P2 (instanceRef R1)) (portRef OUT)))))))
  
  (design my_design (cellRef TOP (libraryRef my_lib))))
```

---

## Structural JSON (Native)

```json
{
  "format": "hwt_netlist",
  "version": "1.0.0",
  "project": "my_design",
  "timestamp": "2026-01-20T12:00:00Z",
  
  "modules": [
    {
      "name": "top",
      "ports": [
        {"name": "VCC", "direction": "input", "width": 1},
        {"name": "GND", "direction": "input", "width": 1},
        {"name": "OUT", "direction": "output", "width": 1}
      ],
      "instances": [
        {
          "name": "R1",
          "module": "RESISTOR",
          "properties": {"value": "10k"},
          "connections": {
            "P1": "VCC",
            "P2": "N1"
          }
        },
        {
          "name": "C1",
          "module": "CAPACITOR",
          "properties": {"value": "100n"},
          "connections": {
            "P1": "N1",
            "P2": "GND"
          }
        }
      ],
      "nets": [
        {"name": "VCC", "type": "power"},
        {"name": "GND", "type": "ground"},
        {"name": "N1", "type": "signal"},
        {"name": "OUT", "type": "signal"}
      ]
    }
  ]
}
```

---

## OpenQASM (Quantum)

```qasm
// Hardware Tool OpenQASM Export
// Project: grover_search
// Qubits: 3

OPENQASM 3.0;
include "stdgates.inc";

qubit[3] q;
bit[3] c;

// Initialize
reset q;

// Hadamard on all qubits
h q[0];
h q[1];
h q[2];

// Oracle
x q[2];
ccx q[0], q[1], q[2];
x q[2];

// Diffusion operator
h q[0];
h q[1];
h q[2];
x q[0];
x q[1];
x q[2];
ccx q[0], q[1], q[2];
x q[0];
x q[1];
x q[2];
h q[0];
h q[1];
h q[2];

// Measure
c = measure q;
```

---

## Import/Export UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Export Netlist                                           [✕]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Format: [SPICE ▼]                                              │
│                                                                 │
│ Output: [./output/netlist.sp________] [Browse]                 │
│                                                                 │
│ Options:                                                        │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ ☑ Include model libraries                                  │ │
│ │ ☑ Include subcircuits                                      │ │
│ │ ☑ Include analysis commands                                │ │
│ │ ☐ Flatten hierarchy                                        │ │
│ │                                                             │ │
│ │ SPICE Dialect: [Standard ▼]                                │ │
│ │ Net Naming:    [Hierarchical ▼]                            │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [Export] [Cancel]                                              │
└─────────────────────────────────────────────────────────────────┘
```

---

## CLI Commands

```bash
# Export netlists
hwt netlist export design.hwt --format spice --output netlist.sp
hwt netlist export design.hwt --format verilog --output netlist.v
hwt netlist export design.hwt --format edif --output netlist.edif
hwt netlist export design.hwt --format json --output netlist.json

# Export options
hwt netlist export design.hwt --format spice --flatten
hwt netlist export design.hwt --format verilog --gate-level
hwt netlist export design.hwt --format spice --dialect ngspice

# Import netlists
hwt netlist import external.sp --format spice
hwt netlist import external.v --format verilog
hwt netlist import external.edif --format edif

# Validate netlist
hwt netlist validate netlist.sp

# Compare netlists (LVS-style)
hwt netlist compare schematic.sp layout.sp
```

---

## Rust API

```rust
use hardware_tool::netlist::*;

// Export SPICE
let spice = design.export_netlist(NetlistFormat::Spice(SpiceOptions {
    dialect: SpiceDialect::Ngspice,
    include_models: true,
    ..Default::default()
}))?;
spice.write_to_file("output.sp")?;

// Export Verilog
let verilog = design.export_netlist(NetlistFormat::Verilog(VerilogOptions {
    netlist_type: VerilogType::Structural,
    cell_library: Some("sky130_fd_sc_hd".into()),
    ..Default::default()
}))?;

// Import netlist
let imported = Netlist::import("external.sp", NetlistFormat::Spice(Default::default()))?;
design.merge_netlist(&imported)?;

// Compare netlists
let comparison = Netlist::compare(&schematic_netlist, &layout_netlist)?;
println!("Match: {}", comparison.is_match());
for diff in comparison.differences() {
    println!("  {}: {}", diff.location, diff.description);
}
```

---

## Format Conversion

```rust
// Convert between formats
let spice = Netlist::from_file("input.sp", NetlistFormat::Spice)?;
spice.export("output.v", NetlistFormat::Verilog)?;

// Batch conversion
hwt netlist convert ./netlists/*.sp --to verilog --output ./converted/
```

---

## Related Topics

- [Shared Export/Import Architecture](../advanced-features/shared-export-import-architecture.md)
- [SPICE Simulation](../schematic-editor/spice-simulation.md)
- [Circuit JSON IR](./circuit-json-ir.md)
- [PDK Integration](./pdk-process-integration.md)
