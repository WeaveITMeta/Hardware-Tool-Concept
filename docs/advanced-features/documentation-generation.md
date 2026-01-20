# Documentation Generation

## Overview

Hardware Tool provides **automatic documentation generation** for all hardware domains. Generate professional schematics PDFs, assembly drawings, fabrication notes, design review packages, and API documentation — all from your design files.

> **"One Hardware Tool That Does It All"** — The same documentation engine works for every hardware type.

---

## Document Types

| Document | Domain | Description |
|----------|--------|-------------|
| **Schematic PDF** | All | Multi-page schematic printout |
| **Assembly Drawing** | PCB, Packaging | Component placement guide |
| **Fabrication Notes** | PCB, IC | Manufacturing specifications |
| **BOM Report** | All | Bill of materials with pricing |
| **Design Review** | All | Complete design package |
| **API Documentation** | All | Code-first design docs |
| **Datasheet** | All | Component/module datasheet |

---

## Schematic PDF Generation

### Configuration

```rust
SchematicPdfConfig {
    // Page setup
    page_size: PageSize::A3,
    orientation: Orientation::Landscape,
    margins: Margins::mm(10.0),
    
    // Content
    include_title_block: true,
    include_revision_history: true,
    include_bom: true,
    include_net_list: false,
    
    // Styling
    color_mode: ColorMode::Color,  // or Monochrome
    line_width_scale: 1.0,
    font: "Arial",
    
    // Organization
    sheet_order: SheetOrder::Hierarchical,
    one_sheet_per_page: true,
}
```

### Title Block

```
┌─────────────────────────────────────────────────────────────────┐
│                                                                 │
│                      [SCHEMATIC CONTENT]                        │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│ Title: Smart Sensor Board          │ Rev: 2.1    │ Sheet 1/5   │
│ Project: SSB-2026                  │ Date: 2026-01-20          │
│ Author: Hardware Team              │ Approved: J. Smith        │
│ Company: Acme Electronics          │ File: main.hwt_sch        │
└─────────────────────────────────────────────────────────────────┘
```

---

## Assembly Drawing

### PCB Assembly Drawing

```
┌─────────────────────────────────────────────────────────────────┐
│ Assembly Drawing: Top Side                               [✕]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │                                                             │ │
│ │    ┌─────┐                    ┌─────────────┐               │ │
│ │    │ U1  │←─────────────────→ │ STM32F407   │               │ │
│ │    └─────┘                    │ LQFP-100    │               │ │
│ │                               │ θ = 0°      │               │ │
│ │    R1 R2 R3 R4                └─────────────┘               │ │
│ │    ═══════════                                              │ │
│ │                                                             │ │
│ │    C1  C2  C3  C4  C5                                       │ │
│ │    ▢   ▢   ▢   ▢   ▢                                        │ │
│ │                                                             │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Component List:                                                 │
│ • U1: STM32F407VGT6, LQFP-100, 0° rotation                     │
│ • R1-R4: 10K 0603, 0° rotation                                 │
│ • C1-C5: 100nF 0402, 0° rotation                               │
│                                                                 │
│ [Top Side] [Bottom Side] [Export PDF] [Export DXF]             │
└─────────────────────────────────────────────────────────────────┘
```

### Assembly Drawing Options

```rust
AssemblyDrawingConfig {
    // Views
    sides: vec![Side::Top, Side::Bottom],
    
    // Annotations
    show_reference_designators: true,
    show_polarity_marks: true,
    show_pin1_indicators: true,
    show_component_outlines: true,
    
    // Dimensions
    show_board_dimensions: true,
    show_mounting_holes: true,
    dimension_units: Units::Millimeters,
    
    // Layers
    include_silkscreen: true,
    include_fab_layer: true,
    include_courtyard: false,
    
    // Legend
    include_component_table: true,
    group_by: GroupBy::Value,
}
```

---

## Fabrication Notes

### Auto-Generated Fab Notes

```
┌─────────────────────────────────────────────────────────────────┐
│ FABRICATION NOTES                                               │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ 1. GENERAL                                                      │
│    • Material: FR-4 TG150                                       │
│    • Layers: 4                                                  │
│    • Finished Thickness: 1.6mm ±10%                            │
│    • Copper Weight: 1oz outer, 0.5oz inner                     │
│                                                                 │
│ 2. DIMENSIONS                                                   │
│    • Board Size: 100.0 x 80.0 mm                               │
│    • Tolerance: ±0.15mm                                        │
│                                                                 │
│ 3. DRILL                                                        │
│    • Minimum Drill: 0.30mm                                     │
│    • Finished Hole Tolerance: ±0.08mm                          │
│    • Plating: 25μm minimum                                     │
│                                                                 │
│ 4. SOLDER MASK                                                  │
│    • Color: Green                                               │
│    • Type: LPI                                                  │
│    • Both Sides                                                 │
│                                                                 │
│ 5. SILKSCREEN                                                   │
│    • Color: White                                               │
│    • Top Side Only                                              │
│                                                                 │
│ 6. SURFACE FINISH                                               │
│    • Type: HASL Lead-Free                                       │
│                                                                 │
│ 7. IMPEDANCE CONTROL                                            │
│    • 50Ω single-ended (±10%)                                   │
│    • 90Ω differential (±10%)                                   │
│    • Reference: Stackup drawing                                 │
│                                                                 │
│ 8. STANDARDS                                                    │
│    • IPC-6012 Class 2                                          │
│    • IPC-A-600 Class 2                                         │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Design Review Package

### Package Contents

```rust
DesignReviewPackage {
    // Documents to include
    documents: vec![
        Document::SchematicPdf,
        Document::LayoutPdf,
        Document::AssemblyDrawing,
        Document::FabricationNotes,
        Document::Bom,
        Document::DrcReport,
        Document::SimulationResults,
        Document::DesignNotes,
    ],
    
    // Review checklist
    checklist: ReviewChecklist {
        schematic_review: true,
        layout_review: true,
        signal_integrity: true,
        power_integrity: true,
        thermal_analysis: true,
        dfm_check: true,
    },
    
    // Output
    output_format: OutputFormat::Zip,
    include_source_files: false,
}
```

### Design Review UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Generate Design Review Package                           [✕]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Project: Smart Sensor Board v2.1                               │
│ Review Type: [Pre-Production ▼]                                │
│                                                                 │
│ Documents to Include:                                          │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ ☑ Schematic PDF (5 sheets)                                 │ │
│ │ ☑ Layout PDF (top, bottom, inner layers)                   │ │
│ │ ☑ Assembly Drawings (top, bottom)                          │ │
│ │ ☑ Fabrication Notes                                        │ │
│ │ ☑ Bill of Materials                                        │ │
│ │ ☑ DRC Report (0 errors, 2 warnings)                        │ │
│ │ ☑ Stackup Diagram                                          │ │
│ │ ☐ Simulation Results                                       │ │
│ │ ☐ 3D Renderings                                            │ │
│ │ ☐ Source Files                                             │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Review Checklist:                                              │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ ✓ Schematic review complete                                │ │
│ │ ✓ Layout review complete                                   │ │
│ │ ✓ DFM check passed                                         │ │
│ │ ⚠ Signal integrity: 2 warnings                             │ │
│ │ ✓ Power integrity verified                                 │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Output: [./review/SSB_v2.1_review.zip]                         │
│                                                                 │
│ [Generate Package] [Preview] [Cancel]                          │
└─────────────────────────────────────────────────────────────────┘
```

---

## BOM Report

### BOM Formats

```rust
BomReportConfig {
    // Format
    format: BomFormat::Html,  // or Csv, Xlsx, Pdf
    
    // Grouping
    group_by: vec![GroupBy::Value, GroupBy::Footprint],
    sort_by: SortBy::Reference,
    
    // Columns
    columns: vec![
        BomColumn::Reference,
        BomColumn::Quantity,
        BomColumn::Value,
        BomColumn::Footprint,
        BomColumn::Manufacturer,
        BomColumn::MPN,
        BomColumn::Supplier,
        BomColumn::SupplierPN,
        BomColumn::UnitPrice,
        BomColumn::ExtendedPrice,
    ],
    
    // Pricing
    include_pricing: true,
    pricing_source: PricingSource::Octopart,
    quantity_breaks: vec![1, 10, 100, 1000],
    
    // Variants
    include_variants: true,
    active_variant: "production",
}
```

### BOM Report Output

```
┌─────────────────────────────────────────────────────────────────┐
│ Bill of Materials: Smart Sensor Board v2.1                     │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Summary: 48 unique parts, 156 total components                 │
│ Estimated Cost: $12.45 @ qty 1, $8.92 @ qty 100               │
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Ref     │ Qty │ Value    │ Footprint │ MPN        │ Price  │ │
│ │ ────────┼─────┼──────────┼───────────┼────────────┼─────── │ │
│ │ C1-C20  │ 20  │ 100nF    │ 0402      │ GRM155R71H│ $0.40  │ │
│ │ C21-C25 │ 5   │ 10uF     │ 0805      │ GRM21BR61│ $0.25  │ │
│ │ R1-R30  │ 30  │ 10K      │ 0402      │ RC0402FR-│ $0.30  │ │
│ │ U1      │ 1   │ STM32F407│ LQFP-100  │ STM32F407│ $5.20  │ │
│ │ U2      │ 1   │ LM1117   │ SOT-223   │ LM1117MP-│ $0.45  │ │
│ │ ...     │     │          │           │           │        │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [Export CSV] [Export Excel] [Export PDF] [Update Pricing]      │
└─────────────────────────────────────────────────────────────────┘
```

---

## API Documentation (Code-First)

### Auto-Generated from Code

```rust
/// Hardware Tool automatically generates documentation from
/// code-first designs using Rust doc comments.

/// # Power Supply Module
/// 
/// Provides regulated 3.3V and 1.8V rails from 5V input.
/// 
/// ## Specifications
/// - Input: 4.5V - 5.5V
/// - Output 1: 3.3V @ 500mA
/// - Output 2: 1.8V @ 200mA
/// - Efficiency: >85%
/// 
/// ## Usage
/// ```rust
/// let psu = PowerSupply::new()
///     .input_voltage(5.0)
///     .output_3v3(true)
///     .output_1v8(true);
/// ```
#[module]
pub struct PowerSupply {
    /// Input voltage (4.5V - 5.5V)
    #[pin(power)]
    pub vin: PowerPin,
    
    /// 3.3V regulated output
    #[pin(power)]
    pub vout_3v3: PowerPin,
    
    /// 1.8V regulated output  
    #[pin(power)]
    pub vout_1v8: PowerPin,
    
    /// Ground reference
    #[pin(ground)]
    pub gnd: GroundPin,
}
```

### Generated Documentation

```markdown
# PowerSupply Module

Provides regulated 3.3V and 1.8V rails from 5V input.

## Specifications
- Input: 4.5V - 5.5V
- Output 1: 3.3V @ 500mA
- Output 2: 1.8V @ 200mA
- Efficiency: >85%

## Pins

| Pin | Type | Description |
|-----|------|-------------|
| `vin` | Power | Input voltage (4.5V - 5.5V) |
| `vout_3v3` | Power | 3.3V regulated output |
| `vout_1v8` | Power | 1.8V regulated output |
| `gnd` | Ground | Ground reference |

## Schematic Symbol

[Auto-generated symbol image]

## Example Usage

```rust
let psu = PowerSupply::new()
    .input_voltage(5.0)
    .output_3v3(true)
    .output_1v8(true);
```
```

---

## CLI Commands

```bash
# Generate documents
hwt docs schematic --output schematic.pdf
hwt docs assembly --output assembly.pdf
hwt docs fab-notes --output fab_notes.pdf
hwt docs bom --format xlsx --output bom.xlsx
hwt docs review-package --output review.zip

# Options
hwt docs schematic --page-size a3 --color
hwt docs bom --include-pricing --quantity 100
hwt docs assembly --side top --include-dimensions

# API documentation
hwt docs api --output docs/
hwt docs api --format html --open

# All documents
hwt docs all --output ./documentation/
```

---

## Rust API

```rust
use hardware_tool::docs::*;

// Generate schematic PDF
let pdf = design.generate_schematic_pdf(SchematicPdfConfig {
    page_size: PageSize::A3,
    color_mode: ColorMode::Color,
    ..Default::default()
})?;
pdf.save("schematic.pdf")?;

// Generate BOM
let bom = design.generate_bom(BomConfig {
    format: BomFormat::Xlsx,
    include_pricing: true,
    ..Default::default()
})?;
bom.save("bom.xlsx")?;

// Generate design review package
let package = design.generate_review_package(ReviewPackageConfig {
    documents: vec![
        Document::SchematicPdf,
        Document::LayoutPdf,
        Document::Bom,
        Document::DrcReport,
    ],
    ..Default::default()
})?;
package.save("review.zip")?;
```

---

## Related Topics

- [Shared Export/Import Architecture](./shared-export-import-architecture.md)
- [BOM & Pick-Place](../manufacturing-output/bom-pick-place.md)
- [Manufacturing Integration](../core-architecture/manufacturing-integration.md)
- [Programmatic Design](../core-architecture/programmatic-design.md)
