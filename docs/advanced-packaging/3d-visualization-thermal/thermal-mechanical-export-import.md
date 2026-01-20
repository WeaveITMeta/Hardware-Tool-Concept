# Thermal-Mechanical Export/Import

## Overview

Hardware Tool supports import and export of thermal and mechanical simulation data for advanced packaging, enabling integration with FEA tools like ANSYS, COMSOL, and Abaqus.

## Export Formats

```rust
ThermalMechanicalExport {
    // Thermal export
    thermal: ThermalExport {
        formats: vec![
            ExportFormat::ANSYS_ICEPAK,
            ExportFormat::COMSOL,
            ExportFormat::FloTHERM,
        ],
        data: ThermalData {
            power_map: true,
            material_properties: true,
            boundary_conditions: true,
            mesh_hints: true,
        },
    },
    
    // Mechanical export
    mechanical: MechanicalExport {
        formats: vec![
            ExportFormat::ANSYS_Mechanical,
            ExportFormat::Abaqus,
            ExportFormat::COMSOL,
        ],
        data: MechanicalData {
            geometry: true,
            materials: true,
            constraints: true,
            loads: true,
        },
    },
    
    // Combined thermo-mechanical
    coupled: CoupledExport {
        temperature_profile: true,
        cte_mismatch: true,
        warpage_prediction: true,
    },
}
```

## Export UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Thermal-Mechanical Export                                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Export Type: [Thermal + Mechanical ▼]                          │
│                                                                 │
│ Target Tool: [ANSYS ▼]                                         │
│                                                                 │
│ Thermal Data:                                                   │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Component      │ Power (W) │ Area (mm²) │ Include           │ │
│ │ ───────────────┼───────────┼────────────┼────────────────── │ │
│ │ Compute die    │   150.0   │   100.0    │ ☑                │ │
│ │ HBM3 stack 1   │    20.0   │    80.0    │ ☑                │ │
│ │ HBM3 stack 2   │    20.0   │    80.0    │ ☑                │ │
│ │ Interposer     │     5.0   │  3025.0    │ ☑                │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Material Properties:                                            │ │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Material    │ k (W/mK) │ CTE (ppm/K) │ E (GPa)             │ │
│ │ ────────────┼──────────┼─────────────┼──────────────────── │ │
│ │ Silicon     │   150    │    2.6      │   170               │ │
│ │ Copper      │   400    │   17.0      │   120               │ │
│ │ Underfill   │    0.5   │   30.0      │     8               │ │
│ │ Solder      │    50    │   21.0      │    40               │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Boundary Conditions:                                            │
│   Ambient: 25°C    Heat sink: 0.1 °C/W    Max Tj: 105°C       │
│                                                                 │
│ Options:                                                        │
│ ☑ Include mesh hints                                           │
│ ☑ Include temperature-dependent properties                     │
│ ☑ Export warpage prediction model                              │
│                                                                 │
│ [Cancel]                                    [Export]            │
└─────────────────────────────────────────────────────────────────┘
```

## Import Results

```rust
SimulationResultsImport {
    // Thermal results
    thermal_results: ThermalResults {
        temperature_field: true,
        heat_flux: true,
        thermal_resistance: true,
    },
    
    // Mechanical results
    mechanical_results: MechanicalResults {
        displacement: true,
        stress: true,
        strain: true,
        warpage: true,
    },
    
    // Visualization
    visualization: ResultVisualization {
        overlay_on_3d: true,
        color_scale: ColorScale::Jet,
        contour_levels: 20,
    },
}
```

## Rust API

```rust
// Export for thermal simulation
let system = project.get_system("hpc_accelerator")?;

system.export_thermal(ThermalExportConfig {
    format: ExportFormat::ANSYS_ICEPAK,
    output: "thermal_model/",
    include_power_map: true,
    include_materials: true,
})?;

// Export for mechanical simulation
system.export_mechanical(MechanicalExportConfig {
    format: ExportFormat::ANSYS_Mechanical,
    output: "mechanical_model/",
    include_cte: true,
})?;

// Import simulation results
let thermal_results = system.import_thermal_results(
    "thermal_results.csv",
    ResultFormat::ANSYS,
)?;

// Visualize
system.overlay_thermal_results(&thermal_results)?;
println!("Max Tj: {}°C", thermal_results.max_temperature);
```

## Related Topics

- [3D Package Viewer](./3d-package-viewer.md)
- [Power Integrity Calculators](../advanced-features/power-integrity-calculators.md)
