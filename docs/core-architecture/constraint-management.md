# Constraint Management

## Overview

Hardware Tool provides a **unified constraint management system** that works across all hardware domains. Whether you're defining timing constraints for ICs, placement constraints for PCBs, routing constraints for RF, or coupling constraints for quantum — the same constraint infrastructure handles it all.

> **"One Hardware Tool That Does It All"** — The same constraint system works for every hardware type.

---

## Constraint Types

| Category | Domain | Examples |
|----------|--------|----------|
| **Timing** | IC, RF | Clock period, setup/hold, max delay |
| **Placement** | All | Keep-out zones, grouping, alignment |
| **Routing** | All | Length matching, differential pairs, shielding |
| **Electrical** | PCB, IC, RF | Impedance, current, voltage |
| **Physical** | All | Layer assignment, via limits, spacing |
| **Thermal** | All | Max temperature, power density |
| **Quantum** | Quantum | Coupling strength, crosstalk limits |

---

## Constraint File Format

### SDC-Compatible (Timing)

```tcl
# timing.sdc - Synopsys Design Constraints compatible
# Hardware Tool extends SDC for multi-domain support

# Clock definitions
create_clock -name clk_main -period 10.0 [get_ports clk]
create_clock -name clk_slow -period 100.0 [get_ports clk_div]

# Clock relationships
set_clock_groups -asynchronous -group {clk_main} -group {clk_slow}

# Input/output delays
set_input_delay -clock clk_main -max 2.0 [get_ports data_in*]
set_output_delay -clock clk_main -max 1.5 [get_ports data_out*]

# Timing exceptions
set_false_path -from [get_clocks clk_slow] -to [get_clocks clk_main]
set_multicycle_path 2 -setup -from [get_pins reg_a/Q] -to [get_pins reg_b/D]
set_max_delay 5.0 -from [get_ports async_in] -to [get_pins sync_reg/D]

# Clock uncertainty
set_clock_uncertainty -setup 0.1 [get_clocks clk_main]
set_clock_uncertainty -hold 0.05 [get_clocks clk_main]
```

### Hardware Tool Constraints (TOML)

```toml
# constraints.toml - Hardware Tool native format

[timing]
# Clock definitions
[[timing.clocks]]
name = "clk_main"
period = 10.0
port = "clk"

[[timing.clocks]]
name = "clk_slow"
period = 100.0
port = "clk_div"

# Max delays
[[timing.max_delay]]
from = "data_in*"
to = "reg_*/D"
delay = 5.0

[placement]
# Component groups
[[placement.groups]]
name = "power_section"
components = ["U1", "U2", "C1", "C2", "C3"]
keep_together = true
max_spread = 10.0  # mm

# Keep-out zones
[[placement.keepout]]
name = "antenna_clearance"
shape = "rectangle"
x = 50.0
y = 30.0
width = 20.0
height = 15.0
layers = ["F.Cu", "B.Cu"]

# Alignment
[[placement.align]]
components = ["R1", "R2", "R3", "R4"]
axis = "horizontal"
spacing = 2.0

[routing]
# Length matching
[[routing.length_match]]
name = "ddr_data"
nets = ["DDR_D0", "DDR_D1", "DDR_D2", "DDR_D3"]
tolerance = 0.5  # mm
reference = "DDR_CLK"

# Differential pairs
[[routing.differential]]
name = "usb_dp_dm"
positive = "USB_DP"
negative = "USB_DM"
impedance = 90.0
spacing = 0.15

# Shielding
[[routing.shield]]
net = "SENSITIVE_SIGNAL"
shield_net = "GND"
both_sides = true

[electrical]
# Impedance targets
[[electrical.impedance]]
net_class = "high_speed"
target = 50.0
tolerance = 10  # percent

# Current limits
[[electrical.current]]
net = "VBUS"
max_current = 3.0  # Amps
trace_width = "auto"  # Calculate automatically

[physical]
# Layer assignments
[[physical.layer_assign]]
net_class = "power"
layers = ["In1.Cu", "In2.Cu"]
prefer = "In1.Cu"

# Via limits
[[physical.via_limit]]
net_class = "signal"
max_vias = 4
```

---

## Constraint Editor UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Constraint Editor                                        [✕]   │
├─────────────────────────────────────────────────────────────────┤
│ [Timing] [Placement] [Routing] [Electrical] [Physical]         │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Timing Constraints                                              │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Clocks                                          [+ Add]    │ │
│ │ ├─ clk_main: 10.0 ns (100 MHz)                  [Edit][✕] │ │
│ │ └─ clk_slow: 100.0 ns (10 MHz)                  [Edit][✕] │ │
│ ├─────────────────────────────────────────────────────────────┤ │
│ │ Input Delays                                    [+ Add]    │ │
│ │ ├─ data_in* → clk_main: max 2.0 ns              [Edit][✕] │ │
│ │ └─ addr* → clk_main: max 1.5 ns                 [Edit][✕] │ │
│ ├─────────────────────────────────────────────────────────────┤ │
│ │ Exceptions                                      [+ Add]    │ │
│ │ ├─ False path: clk_slow → clk_main              [Edit][✕] │ │
│ │ └─ Multicycle: reg_a/Q → reg_b/D (2 cycles)     [Edit][✕] │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Constraint Status: ✓ 12 defined, 0 violations                  │
│                                                                 │
│ [Import SDC] [Export SDC] [Validate] [Apply]                   │
└─────────────────────────────────────────────────────────────────┘
```

---

## Placement Constraints

### Visual Constraint Editor

```
┌─────────────────────────────────────────────────────────────────┐
│ Placement Constraints                                    [✕]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │                     PCB Layout View                         │ │
│ │  ┌─────────────────────────────────────────────────────┐    │ │
│ │  │                                                     │    │ │
│ │  │    ┌─────────┐  Group: power_section               │    │ │
│ │  │    │ U1  U2  │  ════════════════════               │    │ │
│ │  │    │ C1 C2 C3│                                     │    │ │
│ │  │    └─────────┘                                     │    │ │
│ │  │                                                     │    │ │
│ │  │    ░░░░░░░░░░░  Keep-out: antenna_clearance        │    │ │
│ │  │    ░░░░░░░░░░░                                     │    │ │
│ │  │                                                     │    │ │
│ │  │    R1 ─ R2 ─ R3 ─ R4  Aligned (horizontal)         │    │ │
│ │  │                                                     │    │ │
│ │  └─────────────────────────────────────────────────────┘    │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Tools: [Select] [Group] [Keep-out] [Align] [Region]            │
└─────────────────────────────────────────────────────────────────┘
```

---

## Routing Constraints

### Length Matching

```rust
LengthMatchConstraint {
    name: "ddr_data_bus",
    nets: vec!["DDR_D0", "DDR_D1", "DDR_D2", "DDR_D3", "DDR_D4", "DDR_D5", "DDR_D6", "DDR_D7"],
    
    // Matching mode
    mode: MatchMode::ToReference,
    reference_net: Some("DDR_CLK"),
    
    // Tolerance
    tolerance: Length::mm(0.5),
    
    // Tuning options
    tuning: TuningOptions {
        pattern: TuningPattern::Serpentine,
        amplitude: Length::mm(0.3),
        spacing: Length::mm(0.2),
    },
}
```

### Differential Pairs

```rust
DifferentialConstraint {
    name: "usb_pair",
    positive: "USB_DP",
    negative: "USB_DM",
    
    // Electrical
    impedance: Impedance::ohms(90.0),
    impedance_tolerance: 10.0,  // percent
    
    // Physical
    spacing: Length::mm(0.15),
    max_skew: Length::mm(0.1),
    
    // Routing
    via_pattern: ViaPattern::Symmetric,
    layer_changes: LayerChangeRule::Together,
}
```

---

## Quantum Constraints

```toml
# quantum_constraints.toml

[coupling]
# Qubit coupling constraints
[[coupling.pairs]]
qubit_a = "Q0"
qubit_b = "Q1"
target_coupling = 30.0  # MHz
tolerance = 5.0

[[coupling.pairs]]
qubit_a = "Q1"
qubit_b = "Q2"
target_coupling = 30.0
tolerance = 5.0

[crosstalk]
# Crosstalk limits
[[crosstalk.limits]]
source = "Q0"
target = "Q2"
max_coupling = 0.5  # MHz (should be isolated)

[frequency]
# Qubit frequency constraints
[[frequency.targets]]
qubit = "Q0"
frequency = 5.0  # GHz
tolerance = 0.1

[[frequency.separation]]
qubits = ["Q0", "Q1"]
min_separation = 0.3  # GHz (avoid frequency collision)
```

---

## Cross-Domain Constraint Propagation

```rust
/// Constraints can propagate across domain boundaries
ConstraintPropagation {
    // IC timing → Package routing
    ic_to_package: vec![
        PropagationRule {
            source: "IC.timing.max_delay",
            target: "Package.routing.max_length",
            transform: |delay| delay * PROPAGATION_VELOCITY,
        },
    ],
    
    // PCB impedance → RF matching
    pcb_to_rf: vec![
        PropagationRule {
            source: "PCB.electrical.impedance",
            target: "RF.matching.source_impedance",
            transform: |z| z,
        },
    ],
}
```

---

## Constraint Validation

```
┌─────────────────────────────────────────────────────────────────┐
│ Constraint Validation Report                             [✕]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Summary: 2 errors, 3 warnings, 45 passed                       │
│                                                                 │
│ ✗ Errors                                                       │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ ✗ Length match "ddr_data": DDR_D3 exceeds tolerance        │ │
│ │   Expected: 45.2mm ±0.5mm, Actual: 46.1mm (Δ0.9mm)         │ │
│ │   [Go to Net] [Auto-Tune]                                  │ │
│ │                                                             │ │
│ │ ✗ Impedance "high_speed": NET_CLK out of spec              │ │
│ │   Expected: 50Ω ±10%, Actual: 58Ω (+16%)                   │ │
│ │   [Go to Net] [Adjust Trace Width]                         │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ ⚠ Warnings                                                     │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ ⚠ Group "power_section": C3 is 12mm from group center      │ │
│ │ ⚠ Via limit: NET_DATA has 5 vias (limit: 4)                │ │
│ │ ⚠ Differential skew: USB pair skew is 0.08mm (limit: 0.1)  │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [Re-Validate] [Export Report] [Auto-Fix All]                   │
└─────────────────────────────────────────────────────────────────┘
```

---

## CLI Commands

```bash
# Constraint management
hwt constraint list                      # List all constraints
hwt constraint validate                  # Validate constraints
hwt constraint validate --fix            # Auto-fix violations
hwt constraint export --format sdc       # Export as SDC
hwt constraint import timing.sdc         # Import SDC file

# Specific constraint types
hwt constraint timing report             # Timing report
hwt constraint length-match report       # Length matching report
hwt constraint impedance report          # Impedance report

# Constraint editing
hwt constraint add timing-clock clk 10ns
hwt constraint add length-match ddr_bus --nets "DDR_D*" --tolerance 0.5mm
hwt constraint remove <constraint_id>
```

---

## Rust API

```rust
use hardware_tool::constraints::*;

// Load constraints
let constraints = Constraints::load("constraints.toml")?;

// Add timing constraint
constraints.timing.add_clock(Clock {
    name: "clk_main".into(),
    period: Duration::ns(10.0),
    port: "clk".into(),
})?;

// Add placement constraint
constraints.placement.add_group(PlacementGroup {
    name: "power_section".into(),
    components: vec!["U1", "U2", "C1", "C2"],
    keep_together: true,
    max_spread: Length::mm(10.0),
})?;

// Validate
let report = design.validate_constraints(&constraints)?;
for violation in report.violations() {
    println!("{}: {}", violation.severity, violation.message);
}

// Auto-fix
design.fix_constraint_violations(&constraints, FixOptions::default())?;
```

---

## Related Topics

- [Shared DRC Architecture](../advanced-features/shared-drc-architecture.md)
- [Interactive Routing](../pcb-layout/interactive-routing.md)
- [Signal & Power Integrity](../advanced-features/signal-power-integrity.md)
- [PDK Integration](./pdk-process-integration.md)
