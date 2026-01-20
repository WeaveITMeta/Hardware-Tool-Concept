# Timing Analysis

## Overview

Hardware Tool provides comprehensive static timing analysis (STA) for IC designs, supporting setup/hold checks, clock domain crossing analysis, and timing closure optimization.

> **Inherits from:** [Shared Simulation Architecture](../../advanced-features/shared-simulation-architecture.md)
>
> This documentation covers IC-specific timing analysis. All standard job management, results visualization, and export capabilities are inherited from the shared architecture.

---

## Timing Analysis Modes

| Mode | Description | Use Case |
|------|-------------|----------|
| **Setup** | Data arrives before clock edge | Max delay paths |
| **Hold** | Data stable after clock edge | Min delay paths |
| **Recovery** | Async reset to clock | Reset timing |
| **Removal** | Clock to async reset | Reset release |

---

## Timing Configuration

```rust
TimingAnalysis {
    // Clock definitions
    clocks: vec![
        Clock {
            name: "clk_main",
            period: 10.0,          // ns
            waveform: (0.0, 5.0),  // rise, fall
            source: "clk_port",
        },
        Clock {
            name: "clk_slow",
            period: 100.0,
            source: "clk_div/Q",
        },
    ],
    
    // Analysis corners
    corners: vec![
        Corner::Slow { temp: 125.0, voltage: 0.9 },
        Corner::Fast { temp: -40.0, voltage: 1.1 },
        Corner::Typical { temp: 25.0, voltage: 1.0 },
    ],
    
    // Constraints
    constraints: ConstraintFile::load("timing.sdc")?,
    
    // Options
    options: TimingOptions {
        propagate_clocks: true,
        check_async_paths: true,
        report_unconstrained: true,
    },
}
```

---

## Timing Report

```
┌─────────────────────────────────────────────────────────────────┐
│ Static Timing Analysis Report                            [✕]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Design: my_chip  |  Corner: slow_125C_0.9V  |  Mode: func      │
│                                                                 │
│ Summary:                                                        │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Check      │ Endpoints │ Violated │ WNS (ns) │ TNS (ns)    │ │
│ │ ───────────┼───────────┼──────────┼──────────┼──────────── │ │
│ │ Setup      │ 12,456    │ 3        │ -0.125   │ -0.312      │ │
│ │ Hold       │ 12,456    │ 0        │ 0.045    │ 0.000       │ │
│ │ Recovery   │ 234       │ 0        │ 0.892    │ 0.000       │ │
│ │ Removal    │ 234       │ 0        │ 0.156    │ 0.000       │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Critical Path (Setup):                                          │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Startpoint: reg_a/Q (clk_main)                             │ │
│ │ Endpoint:   reg_b/D (clk_main)                             │ │
│ │ Path Group: clk_main                                        │ │
│ │ Slack:      -0.125 ns (VIOLATED)                           │ │
│ │                                                             │ │
│ │ Point                    Incr    Path                       │ │
│ │ ─────────────────────────────────────────────────────────  │ │
│ │ clock clk_main (rise)    0.000   0.000                     │ │
│ │ reg_a/CLK                0.150   0.150                     │ │
│ │ reg_a/Q                  0.320   0.470                     │ │
│ │ U1/A → U1/Y              0.085   0.555                     │ │
│ │ U2/A → U2/Y              0.092   0.647                     │ │
│ │ ...                                                         │ │
│ │ reg_b/D                  0.045   9.875                     │ │
│ │ data arrival time                9.875                     │ │
│ │                                                             │ │
│ │ clock clk_main (rise)    10.000                            │ │
│ │ clock uncertainty        -0.100                            │ │
│ │ reg_b/CLK                0.150   10.050                    │ │
│ │ library setup            -0.200  9.750                     │ │
│ │ data required time               9.750                     │ │
│ │ ─────────────────────────────────────────────────────────  │ │
│ │ slack (VIOLATED)                 -0.125                    │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [View Path] [Fix Suggestions] [Export Report]                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## Clock Domain Crossing (CDC)

```rust
CdcAnalysis {
    // Clock domains
    domains: vec![
        ClockDomain { name: "clk_main", clock: "clk_main" },
        ClockDomain { name: "clk_slow", clock: "clk_slow" },
        ClockDomain { name: "clk_async", clock: "clk_ext" },
    ],
    
    // CDC checks
    checks: vec![
        CdcCheck::Synchronizer,        // Missing synchronizer
        CdcCheck::ReconvergentPaths,   // Reconvergent CDC
        CdcCheck::DataStability,       // Multi-bit crossing
        CdcCheck::ResetDomain,         // Async reset crossing
    ],
    
    // Synchronizer requirements
    synchronizer: SynchronizerConfig {
        min_stages: 2,
        max_mtbf: 1e6,                 // hours
    },
}
```

---

## Power Analysis

```rust
PowerAnalysis {
    // Activity
    activity: ActivitySource::VCD("simulation.vcd"),
    
    // Analysis types
    analyses: vec![
        PowerType::Dynamic,            // Switching power
        PowerType::Leakage,            // Static power
        PowerType::Internal,           // Short-circuit
    ],
    
    // Corners
    corners: vec![
        PowerCorner::Typical,
        PowerCorner::Worst,
    ],
    
    // Report
    report: PowerReport {
        hierarchy_depth: 3,
        group_by: GroupBy::Module,
        include_clock_network: true,
    },
}
```

### Power Report

```
┌─────────────────────────────────────────────────────────────────┐
│ Power Analysis Report                                    [✕]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Design: my_chip  |  Corner: typical  |  Activity: 50%          │
│                                                                 │
│ Summary:                                                        │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Category        │ Power (mW) │ Percentage                   │ │
│ │ ────────────────┼────────────┼───────────────────────────── │ │
│ │ Dynamic         │ 45.2       │ ████████████████░░░░ 72%    │ │
│ │ Leakage         │ 12.8       │ █████░░░░░░░░░░░░░░░ 20%    │ │
│ │ Internal        │ 5.0        │ ██░░░░░░░░░░░░░░░░░░ 8%     │ │
│ │ ────────────────┼────────────┼───────────────────────────── │ │
│ │ Total           │ 63.0       │ 100%                         │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ By Hierarchy:                                                   │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Module          │ Power (mW) │ Percentage                   │ │
│ │ ────────────────┼────────────┼───────────────────────────── │ │
│ │ cpu_core        │ 28.5       │ ████████████░░░░░░░░ 45%    │ │
│ │ memory_ctrl     │ 15.2       │ ██████░░░░░░░░░░░░░░ 24%    │ │
│ │ clock_network   │ 10.8       │ █████░░░░░░░░░░░░░░░ 17%    │ │
│ │ io_pads         │ 5.5        │ ███░░░░░░░░░░░░░░░░░ 9%     │ │
│ │ other           │ 3.0        │ █░░░░░░░░░░░░░░░░░░░ 5%     │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [Drill Down] [Optimize] [Export Report]                        │
└─────────────────────────────────────────────────────────────────┘
```

---

## CLI Commands

```bash
# Run timing analysis
hwt ic timing design.hwt_ic --corner slow

# Report paths
hwt ic timing design.hwt_ic --report setup --paths 10

# CDC analysis
hwt ic timing design.hwt_ic --cdc

# Power analysis
hwt ic power design.hwt_ic --vcd simulation.vcd

# Export reports
hwt ic timing design.hwt_ic --export timing_report.html
hwt ic power design.hwt_ic --export power_report.html
```

---

## Rust API

```rust
use hardware_tool::ic::timing::*;

// Run timing analysis
let timing = design.analyze_timing(TimingConfig {
    corners: vec![Corner::Slow],
    constraints: "timing.sdc",
})?;

// Check results
println!("WNS: {:.3} ns", timing.worst_negative_slack());
println!("TNS: {:.3} ns", timing.total_negative_slack());

// Get critical paths
for path in timing.critical_paths(10) {
    println!("{} -> {}: {:.3} ns slack",
        path.startpoint, path.endpoint, path.slack);
}

// Run power analysis
let power = design.analyze_power(PowerConfig {
    activity: ActivitySource::VCD("sim.vcd"),
})?;

println!("Total power: {:.1} mW", power.total() * 1000.0);
```

---

## Related Topics

- [Parasitic Extraction](../3d-visualization-parasitics/parasitic-extraction.md)
- [Constraint Management](../../core-architecture/constraint-management.md)
- [Timing & Power Calculators](../advanced-features/timing-and-power-calculators.md)
