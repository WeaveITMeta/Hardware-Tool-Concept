# Integrated Circuit Design

## Overview

Hardware Tool provides a complete integrated circuit (IC) design flow, from transistor-level schematic capture through physical layout to GDSII tape-out. Supporting both digital and analog/mixed-signal design, Hardware Tool integrates with open-source PDKs and provides a unified environment for chip design alongside PCB and system-level work.

### Universal Hardware Vision

```
┌─────────────────────────────────────────────────────────────────┐
│                    Hardware Tool: Any Hardware                  │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐           │
│  │   IC    │  │   PCB   │  │ Quantum │  │  MEMS   │           │
│  │ Design  │  │ Design  │  │Hardware │  │ Sensors │           │
│  └────┬────┘  └────┬────┘  └────┬────┘  └────┬────┘           │
│       │            │            │            │                  │
│       └────────────┴────────────┴────────────┘                  │
│                          │                                      │
│                          ▼                                      │
│              ┌───────────────────────┐                         │
│              │   Unified Platform    │                         │
│              │   • Single UI         │                         │
│              │   • Shared libraries  │                         │
│              │   • Cross-domain sim  │                         │
│              │   • Integrated output │                         │
│              └───────────────────────┘                         │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### Supported IC Design Flows

| Flow | Description | Target |
|------|-------------|--------|
| **Digital ASIC** | RTL → Synthesis → P&R → GDSII | Standard cells, SoCs |
| **Analog IC** | Schematic → Layout → LVS → GDSII | Amplifiers, ADCs, PLLs |
| **Mixed-Signal** | Combined digital + analog | SoCs with analog blocks |
| **Custom Digital** | Full-custom layout | High-performance logic |
| **Memory** | SRAM, ROM compilers | Embedded memory |
| **I/O** | Pad cells, ESD | Chip interfaces |

## Process Design Kit (PDK) Support

### Supported PDKs

```rust
PDKSupport {
    // Open-source PDKs
    open_source: vec![
        PDK {
            name: "SkyWater SKY130",
            node: "130nm",
            type_: PDKType::Digital | PDKType::Analog,
            layers: 5,
            source: "https://github.com/google/skywater-pdk",
        },
        PDK {
            name: "GlobalFoundries GF180MCU",
            node: "180nm",
            type_: PDKType::Digital | PDKType::Analog | PDKType::HighVoltage,
            layers: 5,
            source: "https://github.com/google/gf180mcu-pdk",
        },
        PDK {
            name: "IHP SG13G2",
            node: "130nm SiGe BiCMOS",
            type_: PDKType::RF | PDKType::Analog,
            layers: 7,
            source: "https://github.com/IHP-GmbH/IHP-Open-PDK",
        },
        PDK {
            name: "ASAP7",
            node: "7nm (predictive)",
            type_: PDKType::Digital,
            layers: 9,
            source: "http://asap.asu.edu/asap/",
        },
    ],
    
    // Commercial PDK import
    commercial: CommercialPDKImport {
        tsmc: true,
        samsung: true,
        globalfoundries: true,
        intel: true,
        format: vec![PDKFormat::OpenAccess, PDKFormat::LEF_DEF],
    },
}
```

### PDK Browser

```
┌─────────────────────────────────────────────────────────────────┐
│ PDK Browser                                                     │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Installed PDKs:                                                 │
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ ✓ SkyWater SKY130                                  [Active] │ │
│ │   130nm CMOS, 5 metal layers                                │ │
│ │   Digital + Analog + I/O                                    │ │
│ │   Standard cells: sky130_fd_sc_hd (7,000+ cells)           │ │
│ │   [View Layers] [View Devices] [View Rules]                 │ │
│ ├─────────────────────────────────────────────────────────────┤ │
│ │ ✓ GlobalFoundries GF180MCU                                  │ │
│ │   180nm CMOS, 5 metal layers, high-voltage                  │ │
│ │   Digital + Analog + HV (up to 10.5V)                       │ │
│ │   [View Layers] [View Devices] [View Rules]                 │ │
│ ├─────────────────────────────────────────────────────────────┤ │
│ │ ○ IHP SG13G2                              [Install]         │ │
│ │   130nm SiGe BiCMOS, RF-optimized                          │ │
│ │   fT > 300 GHz, ideal for mmWave                           │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [+ Import PDK] [Download from Repository] [Manage]              │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## Transistor-Level Schematic Capture

### Device Primitives

```rust
ICDevices {
    // MOSFET devices
    mosfet: MOSFETDevices {
        nmos: vec![
            Device { name: "nfet_01v8", vth: "standard", description: "1.8V NMOS" },
            Device { name: "nfet_01v8_lvt", vth: "low", description: "1.8V Low-Vt NMOS" },
            Device { name: "nfet_01v8_hvt", vth: "high", description: "1.8V High-Vt NMOS" },
            Device { name: "nfet_03v3_nvt", vth: "native", description: "3.3V Native NMOS" },
            Device { name: "nfet_05v0_nvt", vth: "native", description: "5V I/O NMOS" },
        ],
        pmos: vec![
            Device { name: "pfet_01v8", vth: "standard", description: "1.8V PMOS" },
            Device { name: "pfet_01v8_lvt", vth: "low", description: "1.8V Low-Vt PMOS" },
            Device { name: "pfet_01v8_hvt", vth: "high", description: "1.8V High-Vt PMOS" },
        ],
    },
    
    // Passive devices
    passives: PassiveDevices {
        resistors: vec![
            Device { name: "res_high_po", sheet_r: 1000.0, description: "High-R poly" },
            Device { name: "res_xhigh_po", sheet_r: 2000.0, description: "XHigh-R poly" },
            Device { name: "res_metal", sheet_r: 0.05, description: "Metal resistor" },
        ],
        capacitors: vec![
            Device { name: "cap_mim_m3m4", density: 2.0, description: "MIM cap M3-M4" },
            Device { name: "cap_var", type_: "varactor", description: "MOS varactor" },
        ],
        inductors: vec![
            Device { name: "ind_sym", type_: "symmetric", description: "Symmetric inductor" },
        ],
    },
    
    // Bipolar (for BiCMOS)
    bipolar: BipolarDevices {
        npn: vec![
            Device { name: "npn_05v5", ft: 50e9, description: "5.5V NPN" },
        ],
        pnp: vec![
            Device { name: "pnp_05v5", ft: 5e9, description: "5.5V PNP" },
        ],
    },
}
```

### Schematic Editor (IC Mode)

```
┌─────────────────────────────────────────────────────────────────┐
│ IC Schematic: Operational Amplifier                             │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│                         VDD                                     │
│                          │                                      │
│                    ┌─────┴─────┐                               │
│                    │           │                                │
│                   ─┴─         ─┴─                              │
│                  │M3 │       │M4 │  (PMOS current mirror)      │
│                   ─┬─         ─┬─                              │
│                    │           │                                │
│                    ├───────────┤                               │
│                    │           │                                │
│         VIN+ ──┤├──┤           ├──┤├── VIN-                    │
│               M1   │           │   M2  (NMOS diff pair)        │
│                    │           │                                │
│                    └─────┬─────┘                               │
│                          │                                      │
│                         ─┴─                                    │
│                        │M5 │  (Tail current source)            │
│                         ─┬─                                    │
│                          │                                      │
│                         VSS                                     │
│                                                                 │
│ Device Parameters:                                              │
│ M1, M2: nfet_01v8, W=10µ, L=1µ, fingers=2                      │
│ M3, M4: pfet_01v8, W=20µ, L=1µ, fingers=4                      │
│ M5: nfet_01v8, W=5µ, L=2µ                                      │
│                                                                 │
│ [Simulate] [Layout] [Check] [Parameters]                        │
└─────────────────────────────────────────────────────────────────┘
```

## SPICE Simulation

### Transistor-Level Simulation

```rust
ICSimulation {
    // Simulator engine
    engine: SimulatorEngine::Ngspice,  // or Xyce for parallel
    
    // Analysis types
    analyses: vec![
        Analysis::DC {
            sweep: Sweep::Linear { start: 0.0, stop: 1.8, step: 0.01 },
            source: "VIN",
        },
        Analysis::AC {
            frequency: FrequencySweep::Decade { start: 1.0, stop: 1e9, points: 100 },
        },
        Analysis::Transient {
            stop: 1e-6,
            step: 1e-12,
        },
        Analysis::Noise {
            output: "VOUT",
            input: "VIN",
            frequency: FrequencySweep::Decade { start: 1.0, stop: 1e9, points: 100 },
        },
        Analysis::MonteCarlo {
            iterations: 1000,
            vary: vec!["vth0", "tox", "u0"],
            distribution: Distribution::Gaussian,
        },
    ],
    
    // Corner analysis
    corners: vec![
        Corner::TT,  // Typical-Typical
        Corner::FF,  // Fast-Fast
        Corner::SS,  // Slow-Slow
        Corner::SF,  // Slow-Fast
        Corner::FS,  // Fast-Slow
    ],
    
    // Temperature sweep
    temperature: vec![-40.0, 25.0, 85.0, 125.0],
}
```

### Simulation Results

```
┌─────────────────────────────────────────────────────────────────┐
│ Simulation Results: Operational Amplifier                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ AC Analysis (Bode Plot):                                        │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Gain (dB)                                                   │ │
│ │   80├─────────────────────────────────────────────────────  │ │
│ │     │────────────────────────╲                              │ │
│ │   60├─────────────────────────╲─────────────────────────── │ │
│ │     │                          ╲                            │ │
│ │   40├───────────────────────────╲──────────────────────── │ │
│ │     │                            ╲                          │ │
│ │   20├─────────────────────────────╲────────────────────── │ │
│ │     │                              ╲                        │ │
│ │    0├───────────────────────────────╲──────────────────── │ │
│ │     └──┬────┬────┬────┬────┬────┬────┬────┬────┬────┬──── │ │
│ │       1    10   100   1k   10k  100k  1M   10M  100M  1G   │ │
│ │                     Frequency (Hz)                          │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Performance Summary (TT, 25°C):                                │
│   DC Gain:        72.3 dB                                      │
│   Unity-Gain BW:  45.2 MHz                                     │
│   Phase Margin:   62.4°                                        │
│   Slew Rate:      25.3 V/µs                                    │
│   Input Offset:   1.2 mV                                       │
│   CMRR:           85.6 dB                                      │
│   PSRR:           78.2 dB                                      │
│   Power:          1.2 mW                                       │
│                                                                 │
│ Corner Analysis:                                                │
│   Corner │ Gain  │ GBW    │ PM    │ Power                      │
│   ───────┼───────┼────────┼───────┼───────                     │
│   TT     │ 72 dB │ 45 MHz │ 62°   │ 1.2 mW                    │
│   FF     │ 68 dB │ 58 MHz │ 55°   │ 1.8 mW                    │
│   SS     │ 75 dB │ 32 MHz │ 68°   │ 0.8 mW                    │
│                                                                 │
│ [Monte Carlo] [Temperature] [Export] [Optimize]                 │
└─────────────────────────────────────────────────────────────────┘
```

## Physical Layout

### Polygon-Based Layout Editor

```rust
ICLayout {
    // Layer stack
    layers: LayerStack {
        diffusion: vec!["diff", "tap"],
        poly: vec!["poly"],
        contact: vec!["licon", "mcon"],
        metal: vec!["li1", "met1", "met2", "met3", "met4", "met5"],
        via: vec!["via", "via2", "via3", "via4"],
        implant: vec!["nwell", "pwell", "nsdm", "psdm"],
    },
    
    // Drawing tools
    tools: LayoutTools {
        rectangle: true,
        polygon: true,
        path: true,
        via_array: true,
        instance: true,
        label: true,
        ruler: true,
    },
    
    // Parameterized cells (PCells)
    pcells: PcellSupport {
        mosfet: true,
        resistor: true,
        capacitor: true,
        inductor: true,
        via_array: true,
        guard_ring: true,
    },
    
    // Layout assistance
    assistance: LayoutAssistance {
        drc_as_you_draw: true,
        snap_to_grid: true,
        align_tools: true,
        symmetry_tools: true,
    },
}
```

### Layout Editor UI

```
┌─────────────────────────────────────────────────────────────────┐
│ IC Layout: Operational Amplifier                    [DRC: 0 ✓] │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Layers:        Canvas:                                          │
│ ┌────────┐    ┌───────────────────────────────────────────────┐ │
│ │☑ nwell │    │                                               │ │
│ │☑ diff  │    │  ┌─────────────────────────────────────────┐ │ │
│ │☑ poly  │    │  │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│ │ │
│ │☑ li1   │    │  │░░┌─────┐░░░░░░░░░░░░░░░░┌─────┐░░░░░░░│ │ │
│ │☑ met1  │    │  │░░│ M3  │░░░░░░░░░░░░░░░░│ M4  │░░░░░░░│ │ │
│ │☑ met2  │    │  │░░│PMOS │░░░░░░░░░░░░░░░░│PMOS │░░░░░░░│ │ │
│ │☐ met3  │    │  │░░└──┬──┘░░░░░░░░░░░░░░░░└──┬──┘░░░░░░░│ │ │
│ │☐ met4  │    │  │░░░░░│░░░░░░░░░░░░░░░░░░░░░░│░░░░░░░░░░│ │ │
│ └────────┘    │  │░░░░░├──────────────────────┤░░░░░░░░░░│ │ │
│               │  │░░░░░│░░░░░░░░░░░░░░░░░░░░░░│░░░░░░░░░░│ │ │
│ Tools:        │  │░░┌──┴──┐░░░░░░░░░░░░░░┌──┴──┐░░░░░░░░│ │ │
│ ┌────────┐    │  │░░│ M1  │░░░░░░░░░░░░░░│ M2  │░░░░░░░░│ │ │
│ │▢ Rect  │    │  │░░│NMOS │░░░░░░░░░░░░░░│NMOS │░░░░░░░░│ │ │
│ │◇ Poly  │    │  │░░└──┬──┘░░░░░░░░░░░░░░└──┬──┘░░░░░░░░│ │ │
│ │═ Path  │    │  │░░░░░└──────────┬──────────┘░░░░░░░░░░│ │ │
│ │⊞ Via   │    │  │░░░░░░░░░░░░░░░░│░░░░░░░░░░░░░░░░░░░░│ │ │
│ │⊡ Inst  │    │  │░░░░░░░░░░░░┌───┴───┐░░░░░░░░░░░░░░░░│ │ │
│ └────────┘    │  │░░░░░░░░░░░░│  M5   │░░░░░░░░░░░░░░░░│ │ │
│               │  │░░░░░░░░░░░░│ NMOS  │░░░░░░░░░░░░░░░░│ │ │
│               │  │░░░░░░░░░░░░└───────┘░░░░░░░░░░░░░░░░│ │ │
│               │  └─────────────────────────────────────────┘ │ │
│               └───────────────────────────────────────────────┘ │
│                                                                 │
│ Grid: 0.005µm │ Cursor: (12.345, 67.890) │ Area: 45×32 µm²     │
│                                                                 │
│ [DRC] [LVS] [Extract] [Generate GDS] [3D View]                  │
└─────────────────────────────────────────────────────────────────┘
```

## Design Rule Check (DRC)

### IC-Specific DRC Rules

```rust
ICDesignRules {
    // Minimum dimensions
    minimum: MinimumRules {
        poly_width: 0.15,             // µm
        poly_spacing: 0.21,
        metal1_width: 0.14,
        metal1_spacing: 0.14,
        via_size: 0.15,
        via_spacing: 0.17,
        diff_width: 0.15,
        diff_spacing: 0.27,
    },
    
    // Enclosure rules
    enclosure: EnclosureRules {
        poly_over_diff: 0.13,
        metal_over_via: 0.06,
        contact_in_diff: 0.04,
    },
    
    // Density rules
    density: DensityRules {
        metal_min_density: 0.28,      // 28%
        metal_max_density: 0.78,      // 78%
        poly_min_density: 0.15,
    },
    
    // Antenna rules
    antenna: AntennaRules {
        max_gate_area_ratio: 400.0,
        max_gate_perimeter_ratio: 200.0,
    },
    
    // Well rules
    well: WellRules {
        nwell_spacing: 1.27,
        nwell_to_diff: 0.34,
        tap_spacing_max: 15.0,        // Latch-up prevention
    },
}
```

### DRC Results

```
┌─────────────────────────────────────────────────────────────────┐
│ DRC Results: Operational Amplifier                              │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Summary: 3 errors, 2 warnings                                   │
│                                                                 │
│ Errors:                                                         │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ ✗ met1.5: Metal1 spacing < 0.14µm                          │ │
│ │   Location: (23.45, 12.67) - (23.58, 12.67)                │ │
│ │   Actual: 0.13µm, Required: 0.14µm                         │ │
│ │   [Zoom To] [Fix Automatically]                             │ │
│ ├─────────────────────────────────────────────────────────────┤ │
│ │ ✗ poly.2: Poly width < 0.15µm                              │ │
│ │   Location: (45.12, 8.90)                                   │ │
│ │   Actual: 0.14µm, Required: 0.15µm                         │ │
│ │   [Zoom To] [Fix Automatically]                             │ │
│ ├─────────────────────────────────────────────────────────────┤ │
│ │ ✗ antenna.1: Gate area ratio exceeded                      │ │
│ │   Device: M1, Ratio: 450 (max: 400)                        │ │
│ │   [Zoom To] [Add Diode] [Route to Higher Metal]            │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Warnings:                                                       │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ ⚠ density.1: Metal1 density 26% (min: 28%)                 │ │
│ │   Region: (0,0) to (50,50)                                  │ │
│ │   [Add Fill] [Ignore]                                       │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [Fix All] [Export Report] [Re-run DRC] [Close]                  │
└─────────────────────────────────────────────────────────────────┘
```

## Layout vs Schematic (LVS)

### LVS Verification

```rust
LVSCheck {
    // Extraction settings
    extraction: ExtractionSettings {
        extract_parasitic_r: true,
        extract_parasitic_c: true,
        extract_coupling_c: true,
        merge_parallel_devices: true,
        merge_series_resistors: true,
    },
    
    // Comparison settings
    comparison: ComparisonSettings {
        match_by: MatchBy::Name,      // or Topology
        tolerance: DeviceTolerance {
            width: 0.01,              // 1%
            length: 0.01,
            fingers: 0,               // Exact match
        },
        ignore_bulk: false,
        ignore_substrate: false,
    },
    
    // Output
    output: LVSOutput {
        netlist: true,
        comparison_report: true,
        extracted_spice: true,
    },
}
```

### LVS Results

```
┌─────────────────────────────────────────────────────────────────┐
│ LVS Results: Operational Amplifier                              │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ ✓ LVS CLEAN                                                    │
│                                                                 │
│ Device Comparison:                                              │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Device Type  │ Schematic │ Layout │ Match                   │ │
│ │ ─────────────┼───────────┼────────┼───────                  │ │
│ │ NMOS         │     3     │    3   │  ✓                     │ │
│ │ PMOS         │     2     │    2   │  ✓                     │ │
│ │ Resistor     │     0     │    0   │  ✓                     │ │
│ │ Capacitor    │     0     │    0   │  ✓                     │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Net Comparison:                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Net Type     │ Schematic │ Layout │ Match                   │ │
│ │ ─────────────┼───────────┼────────┼───────                  │ │
│ │ Total Nets   │     8     │    8   │  ✓                     │ │
│ │ Power Nets   │     2     │    2   │  ✓                     │ │
│ │ Signal Nets  │     6     │    6   │  ✓                     │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Device Parameter Comparison:                                    │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Device │ Param  │ Schematic │ Layout  │ Match               │ │
│ │ ───────┼────────┼───────────┼─────────┼───────              │ │
│ │ M1     │ W      │ 10.0µm    │ 10.0µm  │  ✓                 │ │
│ │ M1     │ L      │ 1.0µm     │ 1.0µm   │  ✓                 │ │
│ │ M1     │ nf     │ 2         │ 2       │  ✓                 │ │
│ │ ...    │ ...    │ ...       │ ...     │  ✓                 │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [View Extracted Netlist] [Cross-probe] [Export] [Close]         │
└─────────────────────────────────────────────────────────────────┘
```

## Parasitic Extraction

### RC Extraction

```rust
ParasiticExtraction {
    // Extraction type
    type_: ExtractionType::RCc,       // R, C, and coupling C
    
    // Accuracy
    accuracy: ExtractionAccuracy {
        mode: AccuracyMode::Signoff,  // or Turbo for quick
        coupling_threshold: 0.01,     // fF
        resistance_threshold: 0.1,    // Ω
    },
    
    // Reduction
    reduction: ReductionSettings {
        enabled: true,
        method: ReductionMethod::TICER,
        target_nodes: 1000,
    },
    
    // Output format
    output: ExtractionOutput {
        format: NetlistFormat::DSPF,  // or SPEF, SPICE
        include_device_parasitics: true,
    },
}
```

### Post-Layout Simulation

```
┌─────────────────────────────────────────────────────────────────┐
│ Post-Layout Simulation Comparison                               │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Parameter          │ Pre-Layout │ Post-Layout │ Degradation    │
│ ───────────────────┼────────────┼─────────────┼───────────     │
│ DC Gain            │ 72.3 dB    │ 71.8 dB     │ -0.7%         │
│ Unity-Gain BW      │ 45.2 MHz   │ 42.1 MHz    │ -6.9%         │
│ Phase Margin       │ 62.4°      │ 58.2°       │ -6.7%         │
│ Slew Rate          │ 25.3 V/µs  │ 24.1 V/µs   │ -4.7%         │
│ Input Offset       │ 1.2 mV     │ 1.4 mV      │ +16.7%        │
│ Power              │ 1.2 mW     │ 1.2 mW      │ 0%            │
│                                                                 │
│ Parasitic Summary:                                              │
│   Total extracted R: 245 Ω                                     │
│   Total extracted C: 89 fF                                     │
│   Coupling C: 12 fF                                            │
│                                                                 │
│ Critical Parasitics:                                            │
│   ⚠ Net "out": 45fF load (was 20fF) - reduces BW              │
│   ⚠ Net "tail": 8Ω series R - slight gain loss                │
│                                                                 │
│ [Optimize Layout] [Accept] [Export Report]                      │
└─────────────────────────────────────────────────────────────────┘
```

## Digital ASIC Flow

### RTL-to-GDSII Flow

```rust
DigitalASICFlow {
    // Synthesis
    synthesis: SynthesisConfig {
        tool: SynthesisTool::Yosys,
        target_library: "sky130_fd_sc_hd",
        optimization: OptimizationGoal::Area,  // or Speed, Power
        constraints: TimingConstraints {
            clock_period: 10.0,       // ns (100 MHz)
            input_delay: 1.0,
            output_delay: 1.0,
        },
    },
    
    // Floorplanning
    floorplan: FloorplanConfig {
        die_area: (0.0, 0.0, 500.0, 500.0),  // µm
        core_area: (10.0, 10.0, 490.0, 490.0),
        utilization: 0.5,             // 50%
        aspect_ratio: 1.0,
    },
    
    // Placement
    placement: PlacementConfig {
        tool: PlacementTool::OpenROAD,
        global_placement: true,
        detailed_placement: true,
        timing_driven: true,
    },
    
    // Clock Tree Synthesis
    cts: CTSConfig {
        clock_nets: vec!["clk"],
        target_skew: 0.1,             // ns
        buffer_cell: "sky130_fd_sc_hd__clkbuf_4",
    },
    
    // Routing
    routing: RoutingConfig {
        tool: RoutingTool::OpenROAD,
        global_routing: true,
        detailed_routing: true,
        antenna_fix: true,
    },
    
    // Signoff
    signoff: SignoffConfig {
        sta: true,                    // Static Timing Analysis
        power: true,
        drc: true,
        lvs: true,
    },
}
```

### Digital Flow UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Digital ASIC Flow: RISC-V Core                                  │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Flow Progress:                                                  │
│                                                                 │
│ ✓ RTL Import              [Complete]                           │
│ ✓ Synthesis               [Complete] - 12,456 cells            │
│ ✓ Floorplanning           [Complete] - 500×500 µm              │
│ ✓ Placement               [Complete] - 48% utilization         │
│ ✓ Clock Tree Synthesis    [Complete] - 0.08ns skew             │
│ ● Routing                 [In Progress] ████████░░ 82%         │
│ ○ Signoff                 [Pending]                            │
│ ○ GDSII Export            [Pending]                            │
│                                                                 │
│ Current Metrics:                                                │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Timing:                                                     │ │
│ │   WNS (Worst Negative Slack): +0.23 ns ✓                   │ │
│ │   TNS (Total Negative Slack): 0 ns ✓                       │ │
│ │   Clock period: 10 ns (100 MHz)                            │ │
│ │                                                             │ │
│ │ Power:                                                      │ │
│ │   Dynamic: 12.3 mW                                         │ │
│ │   Leakage: 0.8 mW                                          │ │
│ │   Total: 13.1 mW                                           │ │
│ │                                                             │ │
│ │ Area:                                                       │ │
│ │   Die: 0.25 mm²                                            │ │
│ │   Core utilization: 48%                                    │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [Pause] [View Layout] [Timing Report] [Power Report] [Settings] │
└─────────────────────────────────────────────────────────────────┘
```

## GDSII Export

### Tape-Out Package

```rust
TapeOutPackage {
    // GDSII generation
    gdsii: GDSIIConfig {
        units: GDSUnits { user: 1e-6, database: 1e-9 },
        flatten: false,
        include_fill: true,
        layer_map: LayerMap::PDK,
    },
    
    // Additional files
    additional_files: vec![
        OutputFile::LEF,              // Library Exchange Format
        OutputFile::DEF,              // Design Exchange Format
        OutputFile::SPEF,             // Parasitic data
        OutputFile::SDF,              // Timing data
        OutputFile::Verilog,          // Gate-level netlist
    ],
    
    // Verification
    verification: TapeOutVerification {
        final_drc: true,
        final_lvs: true,
        antenna_check: true,
        density_check: true,
        erc: true,
    },
    
    // Documentation
    documentation: TapeOutDocs {
        design_summary: true,
        pin_list: true,
        timing_summary: true,
        power_summary: true,
    },
}
```

## Rust Implementation

### IC Design Core

```rust
use ndarray::Array2;
use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════
// PDK Layer Definition
// ═══════════════════════════════════════════════════════════════

struct Layer {
    name: String,
    gds_layer: u16,
    gds_datatype: u16,
    layer_type: LayerType,
    color: Color,
}

enum LayerType {
    Diffusion,
    Poly,
    Metal,
    Via,
    Implant,
    Marker,
}

// ═══════════════════════════════════════════════════════════════
// Polygon Geometry
// ═══════════════════════════════════════════════════════════════

struct Polygon {
    layer: LayerId,
    points: Vec<Point>,
}

impl Polygon {
    fn area(&self) -> f64 {
        // Shoelace formula
        let n = self.points.len();
        let mut area = 0.0;
        for i in 0..n {
            let j = (i + 1) % n;
            area += self.points[i].x * self.points[j].y;
            area -= self.points[j].x * self.points[i].y;
        }
        (area / 2.0).abs()
    }
    
    fn perimeter(&self) -> f64 {
        let n = self.points.len();
        let mut perim = 0.0;
        for i in 0..n {
            let j = (i + 1) % n;
            let dx = self.points[j].x - self.points[i].x;
            let dy = self.points[j].y - self.points[i].y;
            perim += (dx * dx + dy * dy).sqrt();
        }
        perim
    }
}

// ═══════════════════════════════════════════════════════════════
// IC Cell
// ═══════════════════════════════════════════════════════════════

struct ICCell {
    name: String,
    polygons: Vec<Polygon>,
    instances: Vec<Instance>,
    pins: Vec<Pin>,
    labels: Vec<Label>,
}

impl ICCell {
    fn flatten(&self) -> ICCell {
        let mut flat = ICCell::new(&self.name);
        
        // Add own polygons
        flat.polygons.extend(self.polygons.clone());
        
        // Recursively flatten instances
        for inst in &self.instances {
            let child = inst.cell.flatten();
            for poly in child.polygons {
                flat.polygons.push(poly.transform(&inst.transform));
            }
        }
        
        flat
    }
    
    fn to_gdsii(&self) -> GDSIIStructure {
        GDSIIStructure {
            name: self.name.clone(),
            elements: self.polygons.iter()
                .map(|p| GDSIIElement::Boundary {
                    layer: p.layer.gds_layer,
                    datatype: p.layer.gds_datatype,
                    points: p.points.clone(),
                })
                .collect(),
        }
    }
}

// ═══════════════════════════════════════════════════════════════
// DRC Engine
// ═══════════════════════════════════════════════════════════════

struct DRCEngine {
    rules: Vec<DRCRule>,
    pdk: PDK,
}

impl DRCEngine {
    fn check(&self, cell: &ICCell) -> Vec<DRCViolation> {
        let mut violations = Vec::new();
        
        for rule in &self.rules {
            match rule {
                DRCRule::MinWidth { layer, width } => {
                    for poly in cell.polygons.iter().filter(|p| p.layer == *layer) {
                        if let Some(min_width) = poly.min_width() {
                            if min_width < *width {
                                violations.push(DRCViolation {
                                    rule: rule.clone(),
                                    location: poly.center(),
                                    actual: min_width,
                                    required: *width,
                                });
                            }
                        }
                    }
                }
                DRCRule::MinSpacing { layer, spacing } => {
                    let polys: Vec<_> = cell.polygons.iter()
                        .filter(|p| p.layer == *layer)
                        .collect();
                    
                    for i in 0..polys.len() {
                        for j in (i + 1)..polys.len() {
                            if let Some(dist) = polys[i].distance_to(polys[j]) {
                                if dist < *spacing {
                                    violations.push(DRCViolation {
                                        rule: rule.clone(),
                                        location: polys[i].center(),
                                        actual: dist,
                                        required: *spacing,
                                    });
                                }
                            }
                        }
                    }
                }
                // ... more rules
                _ => {}
            }
        }
        
        violations
    }
}

// ═══════════════════════════════════════════════════════════════
// LVS Engine
// ═══════════════════════════════════════════════════════════════

struct LVSEngine {
    pdk: PDK,
}

impl LVSEngine {
    fn extract_netlist(&self, cell: &ICCell) -> ExtractedNetlist {
        let mut netlist = ExtractedNetlist::new();
        
        // Find transistors (poly over diffusion)
        let poly_polys = cell.get_layer_polygons("poly");
        let diff_polys = cell.get_layer_polygons("diff");
        
        for poly in &poly_polys {
            for diff in &diff_polys {
                if let Some(intersection) = poly.intersection(diff) {
                    // This is a transistor gate
                    let width = intersection.width();
                    let length = intersection.length();
                    
                    // Determine type from implant layers
                    let is_nmos = cell.has_layer_at("nsdm", intersection.center());
                    
                    netlist.add_device(Device::MOSFET {
                        type_: if is_nmos { MOSType::NMOS } else { MOSType::PMOS },
                        width,
                        length,
                        gate: self.find_net_at("poly", intersection.center()),
                        drain: self.find_net_at("diff", intersection.drain_center()),
                        source: self.find_net_at("diff", intersection.source_center()),
                    });
                }
            }
        }
        
        netlist
    }
    
    fn compare(&self, schematic: &Netlist, layout: &ExtractedNetlist) -> LVSResult {
        // Compare device counts
        let sch_devices = schematic.device_counts();
        let lay_devices = layout.device_counts();
        
        if sch_devices != lay_devices {
            return LVSResult::Mismatch {
                reason: "Device count mismatch".to_string(),
                schematic: sch_devices,
                layout: lay_devices,
            };
        }
        
        // Compare connectivity
        // ... topology matching algorithm
        
        LVSResult::Clean
    }
}
```

## API Usage

```rust
// Create IC project with PDK
let project = ICProject::new("my_opamp", PDK::SkyWater130)?;

// Create schematic
let mut schematic = project.new_schematic("opamp")?;
schematic.add_device(NMOS::new("M1", 10.0, 1.0, 2))?;  // W=10µ, L=1µ, nf=2
schematic.add_device(NMOS::new("M2", 10.0, 1.0, 2))?;
schematic.add_device(PMOS::new("M3", 20.0, 1.0, 4))?;
schematic.add_device(PMOS::new("M4", 20.0, 1.0, 4))?;
schematic.add_device(NMOS::new("M5", 5.0, 2.0, 1))?;
schematic.connect("M1.D", "M3.D")?;
schematic.connect("M2.D", "M4.D")?;
// ... more connections

// Simulate
let sim = schematic.simulate(SimConfig::AC {
    start: 1.0,
    stop: 1e9,
    points_per_decade: 100,
})?;
println!("DC Gain: {} dB", sim.dc_gain_db());
println!("GBW: {} MHz", sim.unity_gain_bw() / 1e6);

// Create layout
let mut layout = project.new_layout("opamp")?;
layout.place_device("M1", Position::new(10.0, 10.0))?;
layout.place_device("M2", Position::new(30.0, 10.0))?;
// ... more placement and routing

// Run DRC
let drc_result = layout.run_drc()?;
if !drc_result.is_clean() {
    println!("DRC errors: {:?}", drc_result.errors);
}

// Run LVS
let lvs_result = layout.run_lvs(&schematic)?;
assert!(lvs_result.is_clean());

// Extract parasitics and re-simulate
let extracted = layout.extract_parasitics()?;
let post_sim = extracted.simulate(SimConfig::AC { ... })?;
println!("Post-layout GBW: {} MHz", post_sim.unity_gain_bw() / 1e6);

// Export GDSII
layout.export_gdsii("opamp.gds")?;
```

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `L` | Toggle layer visibility |
| `Shift+L` | Layer properties |
| `R` | Rectangle tool |
| `P` | Polygon tool |
| `V` | Via tool |
| `I` | Instance tool |
| `D` | Run DRC |
| `Shift+D` | Run LVS |
| `E` | Extract parasitics |
| `G` | Export GDSII |

## Related Topics

- [Schematic Capture](../core-architecture/schematic-capture-workflow.md) - Schematic design
- [SPICE Simulation](../schematic-editor/spice-simulation.md) - Circuit simulation
- [Quantum Hardware](../quantum-hardware/quantum-circuit-design.md) - Quantum IC design
- [Advanced Packaging](../advanced-packaging/chiplet-integration.md) - Multi-die integration
