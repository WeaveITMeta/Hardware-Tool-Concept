# Testing & Debug

## Overview

Hardware Tool provides comprehensive **Design for Test (DFT)** and **debug capabilities** across all hardware domains. Whether you're adding test points to PCBs, inserting scan chains in ICs, or debugging quantum gate fidelity — the same testing infrastructure handles it all.

> **"One Hardware Tool That Does It All"** — The same test and debug workflow works for every hardware type.

---

## Testing Capabilities by Domain

| Domain | Test Features |
|--------|---------------|
| **PCB** | Test points, boundary scan, bed-of-nails, flying probe |
| **IC** | Scan chain, BIST, JTAG, memory test |
| **Quantum** | Gate fidelity, T1/T2 measurement, crosstalk |
| **MEMS** | Functional test, calibration, parametric |
| **RF** | S-parameter verification, spectrum analysis |
| **Packaging** | Continuity, thermal, stress |

---

## Test Point Management (PCB)

### Automatic Test Point Insertion

```rust
TestPointInsertion {
    // Target nets
    target_nets: NetSelection::All,  // or Specific(vec!["VCC", "GND", "CLK"])
    
    // Placement rules
    placement: PlacementRules {
        min_spacing: Length::mm(2.0),
        prefer_layer: Layer::Top,
        avoid_components: Length::mm(1.0),
        grid_snap: Length::mm(0.5),
    },
    
    // Test point type
    test_point: TestPointType {
        pad_size: Length::mm(1.0),
        drill_size: None,  // SMD test point
        net_label: true,
    },
    
    // Coverage target
    coverage: CoverageTarget {
        power_nets: 100,      // percent
        signal_nets: 80,
        ground_nets: 100,
    },
}
```

### Test Point UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Test Point Manager                                       [✕]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Coverage: 85% (127/150 nets accessible)                        │
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Net          │ Test Point │ Location    │ Access │ Status  │ │
│ │ ─────────────┼────────────┼─────────────┼────────┼──────── │ │
│ │ VCC          │ TP1        │ (10.2, 5.5) │ Top    │ ✓       │ │
│ │ GND          │ TP2        │ (12.0, 5.5) │ Top    │ ✓       │ │
│ │ CLK          │ TP3        │ (15.3, 8.2) │ Top    │ ✓       │ │
│ │ DATA[0]      │ TP4        │ (20.1, 12.0)│ Top    │ ✓       │ │
│ │ DATA[1]      │ --         │ --          │ --     │ ✗ None  │ │
│ │ RESET_N      │ TP5        │ (8.5, 15.2) │ Bottom │ ⚠ Bot   │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [Auto-Insert] [Add Manual] [Remove] [Generate Report]          │
└─────────────────────────────────────────────────────────────────┘
```

---

## Boundary Scan (JTAG)

### BSDL Integration

```rust
BoundaryScan {
    // JTAG chain
    chain: JtagChain {
        tck: "JTAG_TCK",
        tms: "JTAG_TMS",
        tdi: "JTAG_TDI",
        tdo: "JTAG_TDO",
        trst: Some("JTAG_TRST"),
    },
    
    // Devices in chain
    devices: vec![
        JtagDevice {
            reference: "U1",
            bsdl_file: "stm32f4.bsdl",
            ir_length: 5,
        },
        JtagDevice {
            reference: "U2",
            bsdl_file: "fpga.bsdl",
            ir_length: 6,
        },
    ],
    
    // Test generation
    tests: vec![
        JtagTest::Connectivity,
        JtagTest::Shorts,
        JtagTest::Opens,
        JtagTest::StuckAt,
    ],
}
```

### JTAG Chain Viewer

```
┌─────────────────────────────────────────────────────────────────┐
│ JTAG Chain Viewer                                        [✕]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Chain: JTAG_TDI → [U1: STM32F4] → [U2: FPGA] → JTAG_TDO       │
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Device │ IDCODE     │ IR Len │ Cells │ Status              │ │
│ │ ───────┼────────────┼────────┼───────┼──────────────────── │ │
│ │ U1     │ 0x06413041 │ 5      │ 156   │ ✓ BSDL loaded       │ │
│ │ U2     │ 0x0362D093 │ 6      │ 284   │ ✓ BSDL loaded       │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Tests:                                                          │
│ ☑ Connectivity test                                            │
│ ☑ Shorts detection                                             │
│ ☑ Opens detection                                              │
│ ☐ Memory test (requires BIST)                                  │
│                                                                 │
│ [Validate Chain] [Generate Tests] [Export SVF]                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## IC Design for Test (DFT)

### Scan Chain Insertion

```rust
ScanChainInsertion {
    // Scan configuration
    scan_style: ScanStyle::MuxD,
    
    // Chain configuration
    chains: vec![
        ScanChain {
            name: "scan_chain_1",
            max_length: 500,
            clock_domain: "clk_main",
        },
    ],
    
    // Scan ports
    ports: ScanPorts {
        scan_enable: "SE",
        scan_in: vec!["SI"],
        scan_out: vec!["SO"],
    },
    
    // Exclusions
    exclude: vec![
        "async_reg_*",
        "clock_gate_*",
    ],
    
    // Compression
    compression: Some(CompressionConfig {
        ratio: 10,
        type_: CompressionType::EDT,
    }),
}
```

### Built-In Self-Test (BIST)

```rust
MemoryBist {
    // Target memories
    memories: vec!["sram_4k", "rom_2k"],
    
    // Test algorithms
    algorithms: vec![
        BistAlgorithm::MarchC,
        BistAlgorithm::Checkerboard,
    ],
    
    // BIST controller
    controller: BistController {
        clock: "bist_clk",
        start: "bist_start",
        done: "bist_done",
        fail: "bist_fail",
    },
}
```

---

## Debug Probe Integration

### Supported Probes

| Probe | Interface | Domains |
|-------|-----------|---------|
| **J-Link** | JTAG, SWD | PCB, IC |
| **ST-Link** | SWD | PCB (STM32) |
| **Black Magic** | JTAG, SWD | PCB |
| **OpenOCD** | JTAG, SWD | PCB, IC |
| **Saleae** | Logic Analyzer | PCB |
| **Oscilloscope** | Analog | PCB, RF |

### Debug Session UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Debug Session: J-Link                                    [✕]   │
├─────────────────────────────────────────────────────────────────┤
│ Target: STM32F407VGT6 @ U1                                     │
│ Status: ● Connected (SWD @ 4MHz)                               │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Registers:                    │ Memory:                        │
│ ┌───────────────────────────┐ │ ┌────────────────────────────┐ │
│ │ R0:  0x00000000           │ │ │ 0x20000000: 00 00 00 00   │ │
│ │ R1:  0x20001234           │ │ │ 0x20000004: 12 34 56 78   │ │
│ │ R2:  0x08001000           │ │ │ 0x20000008: AB CD EF 00   │ │
│ │ SP:  0x20010000           │ │ │ 0x2000000C: 00 00 00 00   │ │
│ │ LR:  0x08000421           │ │ └────────────────────────────┘ │
│ │ PC:  0x08000512           │ │                                │
│ └───────────────────────────┘ │ Peripherals:                   │
│                               │ ┌────────────────────────────┐ │
│ Breakpoints:                  │ │ GPIOA_ODR: 0x0000         │ │
│ • 0x08000512 (main.c:45)     │ │ TIM2_CNT:  0x1234         │ │
│ • 0x08000600 (irq.c:12)      │ │ USART1_SR: 0x00C0         │ │
│                               │ └────────────────────────────┘ │
├───────────────────────────────┴────────────────────────────────┤
│ [Step] [Step Over] [Continue] [Halt] [Reset] [Disconnect]     │
└─────────────────────────────────────────────────────────────────┘
```

---

## Quantum Testing

### Gate Fidelity Measurement

```rust
QuantumTest {
    // Fidelity tests
    fidelity_tests: vec![
        FidelityTest {
            gate: "H",  // Hadamard
            qubit: "Q0",
            method: FidelityMethod::RandomizedBenchmarking,
            sequences: 100,
        },
        FidelityTest {
            gate: "CNOT",
            qubits: ("Q0", "Q1"),
            method: FidelityMethod::ProcessTomography,
        },
    ],
    
    // Coherence tests
    coherence_tests: vec![
        CoherenceTest::T1 { qubit: "Q0" },
        CoherenceTest::T2 { qubit: "Q0" },
        CoherenceTest::T2Echo { qubit: "Q0" },
    ],
    
    // Crosstalk tests
    crosstalk_tests: vec![
        CrosstalkTest {
            source: "Q0",
            target: "Q1",
            method: CrosstalkMethod::SimultaneousRB,
        },
    ],
}
```

### Quantum Test Results

```
┌─────────────────────────────────────────────────────────────────┐
│ Quantum Characterization Results                         [✕]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Gate Fidelities:                                               │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Gate    │ Qubit(s) │ Fidelity │ Error Rate │ Status        │ │
│ │ ────────┼──────────┼──────────┼────────────┼────────────── │ │
│ │ H       │ Q0       │ 99.92%   │ 0.08%      │ ✓ Excellent   │ │
│ │ X       │ Q0       │ 99.95%   │ 0.05%      │ ✓ Excellent   │ │
│ │ CNOT    │ Q0-Q1    │ 99.21%   │ 0.79%      │ ✓ Good        │ │
│ │ CZ      │ Q1-Q2    │ 98.85%   │ 1.15%      │ ⚠ Marginal    │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Coherence Times:                                               │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Qubit │ T1       │ T2       │ T2 Echo  │ Status            │ │
│ │ ──────┼──────────┼──────────┼──────────┼────────────────── │ │
│ │ Q0    │ 85 μs    │ 42 μs    │ 78 μs    │ ✓ Good            │ │
│ │ Q1    │ 92 μs    │ 38 μs    │ 72 μs    │ ✓ Good            │ │
│ │ Q2    │ 45 μs    │ 18 μs    │ 35 μs    │ ⚠ Low             │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [Export Report] [Recalibrate] [Compare to Baseline]            │
└─────────────────────────────────────────────────────────────────┘
```

---

## Test Coverage Analysis

```
┌─────────────────────────────────────────────────────────────────┐
│ Test Coverage Report                                     [✕]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Overall Coverage: 87.3%                                        │
│ ████████████████████░░░ 87.3%                                  │
│                                                                 │
│ By Category:                                                   │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Category          │ Covered │ Total │ Coverage             │ │
│ │ ──────────────────┼─────────┼───────┼───────────────────── │ │
│ │ Power Nets        │ 12/12   │ 12    │ ████████████ 100%   │ │
│ │ Ground Nets       │ 8/8     │ 8     │ ████████████ 100%   │ │
│ │ Clock Nets        │ 4/4     │ 4     │ ████████████ 100%   │ │
│ │ High-Speed Signals│ 24/28   │ 28    │ ██████████░░ 85.7%  │ │
│ │ General Signals   │ 82/98   │ 98    │ ██████████░░ 83.7%  │ │
│ │ Analog Signals    │ 5/8     │ 8     │ ███████░░░░░ 62.5%  │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Uncovered Nets:                                                │
│ • DATA[5], DATA[6], DATA[7] - No test point access            │
│ • ANALOG_IN1, ANALOG_IN2 - Requires probe access              │
│                                                                 │
│ [Improve Coverage] [Export Report] [Generate Test Program]    │
└─────────────────────────────────────────────────────────────────┘
```

---

## CLI Commands

```bash
# Test point management
hwt test points list
hwt test points insert --coverage 90
hwt test points report

# Boundary scan
hwt test jtag validate
hwt test jtag generate-tests
hwt test jtag export-svf

# DFT (IC)
hwt test scan insert --max-chain 500
hwt test bist insert --memories all
hwt test coverage report

# Debug
hwt debug connect --probe jlink
hwt debug read-memory 0x20000000 256
hwt debug program firmware.elf

# Quantum
hwt test quantum fidelity --gate H --qubit Q0
hwt test quantum coherence --qubit Q0
hwt test quantum crosstalk --source Q0 --target Q1
```

---

## Rust API

```rust
use hardware_tool::testing::*;

// Insert test points
let result = design.insert_test_points(TestPointConfig {
    coverage_target: 90.0,
    ..Default::default()
})?;
println!("Inserted {} test points", result.inserted);

// Generate JTAG tests
let jtag = design.jtag_chain()?;
let tests = jtag.generate_tests(JtagTestConfig::default())?;
tests.export_svf("boundary_scan.svf")?;

// Connect debug probe
let probe = DebugProbe::connect(ProbeType::JLink)?;
let target = probe.attach("STM32F407")?;
target.halt()?;
let pc = target.read_register(Register::PC)?;
println!("PC: 0x{:08X}", pc);
```

---

## Related Topics

- [Shared DRC Architecture](./shared-drc-architecture.md)
- [Manufacturing Integration](../core-architecture/manufacturing-integration.md)
- [PDK Integration](../core-architecture/pdk-process-integration.md)
- [Quantum Decoherence Calculators](../quantum-hardware/advanced-features/decoherence-and-fidelity-calculators.md)
