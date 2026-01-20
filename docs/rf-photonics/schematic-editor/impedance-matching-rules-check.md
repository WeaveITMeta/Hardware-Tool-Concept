# Impedance Matching Rules Check

## Overview

Hardware Tool provides automated impedance matching validation, checking VSWR, return loss, and matching network quality across the design frequency range.

## Matching Rules

```rust
ImpedanceMatchingRules {
    // Global rules
    global: GlobalMatchingRules {
        target_impedance: 50.0,        // Ω
        max_vswr: 2.0,                 // :1
        min_return_loss: 10.0,         // dB
        frequency_range: (2.4e9, 2.5e9),
    },
    
    // Port-specific rules
    port_rules: vec![
        PortRule {
            port: "RF_IN",
            max_vswr: 1.5,
            min_return_loss: 14.0,
        },
        PortRule {
            port: "RF_OUT",
            max_vswr: 2.0,
            min_return_loss: 10.0,
        },
    ],
    
    // Matching network rules
    network_rules: MatchingNetworkRules {
        max_component_q: 100.0,
        min_bandwidth: 100e6,
        max_insertion_loss: 0.5,       // dB
    },
}
```

## Rules Check UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Impedance Matching Rules Check: LNA                            │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Frequency Range: 2.4 - 2.5 GHz                                 │
│                                                                 │
│ Results:                                                        │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Port     │ VSWR   │ Return Loss │ Status                    │ │
│ │ ─────────┼────────┼─────────────┼────────────────────────── │ │
│ │ RF_IN    │ 1.35:1 │   16.3 dB   │ ✓ Pass (< 1.5:1)         │ │
│ │ RF_OUT   │ 1.82:1 │   11.2 dB   │ ✓ Pass (< 2.0:1)         │ │
│ │ LO_IN    │ 2.15:1 │    8.9 dB   │ ✗ Fail (> 2.0:1)         │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Matching Network Analysis:                                      │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Network      │ IL (dB) │ BW (MHz) │ Q_eff │ Status         │ │
│ │ ─────────────┼─────────┼──────────┼───────┼─────────────── │ │
│ │ Input match  │  0.25   │   150    │  45   │ ✓ Pass         │ │
│ │ Output match │  0.35   │   120    │  62   │ ✓ Pass         │ │
│ │ Interstage   │  0.55   │    80    │  95   │ ⚠ Warning      │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Violations:                                                     │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ ✗ LO_IN: VSWR 2.15:1 exceeds limit 2.0:1                   │ │
│ │   Recommendation: Add series inductor 2.2nH                 │ │
│ │   [Auto-Fix] [View on Smith Chart] [Ignore]                 │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [Re-check] [Export Report] [Close]                              │
└─────────────────────────────────────────────────────────────────┘
```

## Smith Chart Integration

```rust
SmithChartAnalysis {
    // Display options
    display: SmithChartDisplay {
        show_constant_vswr: vec![1.5, 2.0, 3.0],
        show_stability_circles: true,
        show_noise_circles: true,
        show_gain_circles: true,
    },
    
    // Matching synthesis
    synthesis: MatchingSynthesis {
        source_z: Complex::new(50.0, 0.0),
        load_z: Complex::new(35.0, -15.0),
        topology: Topology::LNetwork,
        q_constraint: Some(50.0),
    },
}
```

## Rust API

```rust
// Run impedance matching check
let schematic = project.get_schematic("lna")?;

let result = schematic.check_impedance_matching(MatchingConfig {
    target_z: 50.0,
    max_vswr: 2.0,
    frequency_range: (2.4e9, 2.5e9),
})?;

// Check results
for violation in result.violations() {
    println!("{}: VSWR {} at {} GHz", 
        violation.port, violation.vswr, violation.frequency / 1e9);
}

// Auto-fix matching
if let Some(fix) = result.suggest_fix("LO_IN")? {
    println!("Suggested fix: {:?}", fix);
    schematic.apply_matching_fix(&fix)?;
}
```

## Related Topics

- [RF Components & Libraries](./rf-components-and-libraries.md)
- [Insertion Loss & VSWR Calculators](../advanced-features/insertion-loss-and-vswr-calculators.md)
