# Data Model Specification

## Overview

Hardware Tool uses a **unified data model** that represents all hardware designs across domains. This specification defines the schema, validation rules, versioning, and binary format for efficient storage and interchange.

> **"One Hardware Tool That Does It All"** — One data model for every hardware type.

---

## Data Model Hierarchy

```
Project
├── Metadata
│   ├── Name, Version, Author
│   ├── Domain (PCB, IC, Quantum, MEMS, RF, Packaging)
│   └── Settings
├── Design
│   ├── Sheets/Modules (hierarchical)
│   ├── Components/Instances
│   ├── Nets/Connections
│   └── Constraints
├── Libraries
│   ├── Symbols
│   ├── Footprints/Cells
│   └── Models
├── Layout
│   ├── Layers
│   ├── Geometry
│   ├── Routing
│   └── Zones
└── Simulation
    ├── Configurations
    └── Results
```

---

## Core Schema

### Project

```json
{
  "$schema": "https://hardwaretool.dev/schema/project/v1.json",
  "type": "object",
  "required": ["format_version", "project", "design"],
  "properties": {
    "format_version": {
      "type": "string",
      "pattern": "^\\d+\\.\\d+\\.\\d+$",
      "description": "Schema version (semver)"
    },
    "project": {
      "$ref": "#/definitions/ProjectMetadata"
    },
    "design": {
      "$ref": "#/definitions/Design"
    },
    "libraries": {
      "$ref": "#/definitions/Libraries"
    },
    "layout": {
      "$ref": "#/definitions/Layout"
    },
    "simulation": {
      "$ref": "#/definitions/Simulation"
    }
  }
}
```

### Project Metadata

```json
{
  "definitions": {
    "ProjectMetadata": {
      "type": "object",
      "required": ["name", "domain"],
      "properties": {
        "name": {
          "type": "string",
          "minLength": 1,
          "maxLength": 255
        },
        "version": {
          "type": "string",
          "pattern": "^\\d+\\.\\d+\\.\\d+$"
        },
        "domain": {
          "type": "string",
          "enum": ["pcb", "ic", "quantum", "mems", "rf", "packaging"]
        },
        "author": {
          "type": "string"
        },
        "created": {
          "type": "string",
          "format": "date-time"
        },
        "modified": {
          "type": "string",
          "format": "date-time"
        },
        "description": {
          "type": "string"
        },
        "license": {
          "type": "string"
        },
        "tags": {
          "type": "array",
          "items": { "type": "string" }
        }
      }
    }
  }
}
```

---

## Component Schema

### Universal Component

```json
{
  "definitions": {
    "Component": {
      "type": "object",
      "required": ["id", "type", "reference"],
      "properties": {
        "id": {
          "type": "string",
          "format": "uuid",
          "description": "Unique identifier"
        },
        "type": {
          "type": "string",
          "description": "Component type (domain-specific)"
        },
        "reference": {
          "type": "string",
          "pattern": "^[A-Z]+[0-9]+$",
          "description": "Reference designator"
        },
        "value": {
          "type": "string",
          "description": "Component value"
        },
        "symbol": {
          "type": "string",
          "description": "Symbol library reference"
        },
        "footprint": {
          "type": "string",
          "description": "Footprint/cell library reference"
        },
        "position": {
          "$ref": "#/definitions/Position"
        },
        "rotation": {
          "type": "number",
          "minimum": 0,
          "maximum": 360
        },
        "properties": {
          "type": "object",
          "additionalProperties": true
        },
        "pins": {
          "type": "array",
          "items": { "$ref": "#/definitions/Pin" }
        }
      }
    }
  }
}
```

### Domain-Specific Extensions

```json
{
  "definitions": {
    "PcbComponent": {
      "allOf": [
        { "$ref": "#/definitions/Component" },
        {
          "properties": {
            "layer": { "type": "string" },
            "dnp": { "type": "boolean" },
            "bom_exclude": { "type": "boolean" }
          }
        }
      ]
    },
    "IcCell": {
      "allOf": [
        { "$ref": "#/definitions/Component" },
        {
          "properties": {
            "cell_type": { "enum": ["standard", "macro", "io", "filler"] },
            "drive_strength": { "type": "string" },
            "timing_model": { "type": "string" }
          }
        }
      ]
    },
    "QuantumGate": {
      "allOf": [
        { "$ref": "#/definitions/Component" },
        {
          "properties": {
            "gate_type": { "enum": ["H", "X", "Y", "Z", "CNOT", "CZ", "T", "S"] },
            "qubits": { "type": "array", "items": { "type": "string" } },
            "parameters": { "type": "object" }
          }
        }
      ]
    }
  }
}
```

---

## Net Schema

```json
{
  "definitions": {
    "Net": {
      "type": "object",
      "required": ["id", "name"],
      "properties": {
        "id": {
          "type": "string",
          "format": "uuid"
        },
        "name": {
          "type": "string"
        },
        "class": {
          "type": "string",
          "description": "Net class reference"
        },
        "type": {
          "type": "string",
          "enum": ["signal", "power", "ground", "clock", "differential"]
        },
        "connections": {
          "type": "array",
          "items": { "$ref": "#/definitions/Connection" }
        },
        "properties": {
          "type": "object",
          "properties": {
            "impedance": { "type": "number" },
            "max_current": { "type": "number" },
            "max_voltage": { "type": "number" },
            "differential_pair": { "type": "string" }
          }
        }
      }
    },
    "Connection": {
      "type": "object",
      "required": ["component_id", "pin"],
      "properties": {
        "component_id": { "type": "string", "format": "uuid" },
        "pin": { "type": "string" }
      }
    }
  }
}
```

---

## Geometry Schema

```json
{
  "definitions": {
    "Position": {
      "type": "object",
      "required": ["x", "y"],
      "properties": {
        "x": { "type": "number" },
        "y": { "type": "number" },
        "z": { "type": "number" },
        "unit": { "type": "string", "enum": ["mm", "mil", "um", "nm"] }
      }
    },
    "Geometry": {
      "oneOf": [
        { "$ref": "#/definitions/Line" },
        { "$ref": "#/definitions/Arc" },
        { "$ref": "#/definitions/Circle" },
        { "$ref": "#/definitions/Rectangle" },
        { "$ref": "#/definitions/Polygon" }
      ]
    },
    "Line": {
      "type": "object",
      "required": ["type", "start", "end"],
      "properties": {
        "type": { "const": "line" },
        "start": { "$ref": "#/definitions/Position" },
        "end": { "$ref": "#/definitions/Position" },
        "width": { "type": "number" }
      }
    },
    "Polygon": {
      "type": "object",
      "required": ["type", "points"],
      "properties": {
        "type": { "const": "polygon" },
        "points": {
          "type": "array",
          "items": { "$ref": "#/definitions/Position" },
          "minItems": 3
        },
        "filled": { "type": "boolean" }
      }
    }
  }
}
```

---

## Validation Rules

### Schema Validation

```rust
/// Validate design against schema
ValidationRules {
    // Schema validation
    schema: SchemaValidation {
        version: "1.0.0",
        strict: true,
        additional_properties: false,
    },
    
    // Semantic validation
    semantic: SemanticValidation {
        // Reference integrity
        check_references: true,
        check_dangling_nets: true,
        check_unconnected_pins: true,
        
        // Domain rules
        domain_rules: true,
        
        // Naming conventions
        naming_conventions: NamingConventions {
            reference_pattern: "^[A-Z]+[0-9]+$",
            net_pattern: "^[A-Za-z_][A-Za-z0-9_]*$",
        },
    },
    
    // Warnings
    warnings: WarningConfig {
        unused_components: true,
        floating_pins: true,
        duplicate_references: true,
    },
}
```

### Validation Report

```
┌─────────────────────────────────────────────────────────────────┐
│ Data Model Validation                                    [✕]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Schema Version: 1.0.0                                          │
│ Result: ✗ 2 errors, 3 warnings                                 │
│                                                                 │
│ ✗ Errors                                                       │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ ✗ Component "U99" references non-existent symbol           │ │
│ │   Path: /design/components[45]/symbol                      │ │
│ │                                                             │ │
│ │ ✗ Net "VCC" has invalid connection                         │ │
│ │   Path: /design/nets[12]/connections[3]                    │ │
│ │   Component "C99" does not exist                           │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ ⚠ Warnings                                                     │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ ⚠ Component "R5" has no footprint assigned                 │ │
│ │ ⚠ Net "UNUSED_NET" has no connections                      │ │
│ │ ⚠ Pin "U1.NC" is not connected                             │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [Fix Errors] [Export Report] [Ignore Warnings]                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Version Migration

### Migration System

```rust
/// Migrate between schema versions
MigrationSystem {
    // Supported versions
    versions: vec!["0.9.0", "1.0.0", "1.1.0", "2.0.0"],
    
    // Migration path
    migrations: vec![
        Migration {
            from: "0.9.0",
            to: "1.0.0",
            transform: migrate_0_9_to_1_0,
        },
        Migration {
            from: "1.0.0",
            to: "1.1.0",
            transform: migrate_1_0_to_1_1,
        },
        Migration {
            from: "1.1.0",
            to: "2.0.0",
            transform: migrate_1_1_to_2_0,
        },
    ],
    
    // Backward compatibility
    backward_compatible: true,
    min_supported_version: "0.9.0",
}

fn migrate_0_9_to_1_0(data: &mut Value) -> Result<()> {
    // Rename fields
    if let Some(project) = data.get_mut("project") {
        // Old: "project_name" -> New: "name"
        if let Some(name) = project.get("project_name") {
            project["name"] = name.clone();
            project.as_object_mut().unwrap().remove("project_name");
        }
    }
    
    // Add new required fields with defaults
    if data.get("format_version").is_none() {
        data["format_version"] = json!("1.0.0");
    }
    
    Ok(())
}
```

---

## Binary Format

### Efficient Storage

```rust
/// Binary format for large designs
BinaryFormat {
    // Header
    header: BinaryHeader {
        magic: [0x48, 0x57, 0x54, 0x42],  // "HWTB"
        version: 1,
        compression: Compression::LZ4,
        checksum: Checksum::XXH3,
    },
    
    // Sections
    sections: vec![
        Section::Metadata,
        Section::Components,
        Section::Nets,
        Section::Geometry,
        Section::Layers,
        Section::Properties,
    ],
    
    // Indexing
    index: IndexConfig {
        component_index: true,
        net_index: true,
        spatial_index: true,  // R-tree for geometry
    },
}
```

### Binary vs JSON Comparison

| Aspect | JSON (.hwt_json) | Binary (.hwt_bin) |
|--------|------------------|-------------------|
| **Size** | 100% | ~15-25% |
| **Load Time** | 100% | ~10-20% |
| **Human Readable** | Yes | No |
| **Diff-able** | Yes | No |
| **Use Case** | Version control | Large designs |

---

## Interchange Formats

### Circuit JSON (Universal)

```json
{
  "format": "circuit-json",
  "version": "1.0.0",
  "components": [
    {
      "id": "R1",
      "type": "resistor",
      "value": "10k",
      "pins": ["1", "2"]
    }
  ],
  "nets": [
    {
      "name": "VCC",
      "connections": [
        {"component": "R1", "pin": "1"},
        {"component": "U1", "pin": "VDD"}
      ]
    }
  ]
}
```

### Domain Interchange

| Domain | Interchange Format |
|--------|-------------------|
| **PCB** | KiCad JSON, Altium JSON |
| **IC** | OpenAccess, LEF/DEF |
| **Quantum** | OpenQASM, Qiskit JSON |
| **MEMS** | GDS JSON |
| **RF** | Touchstone, MDIF |

---

## CLI Commands

```bash
# Validate data model
hwt data validate design.hwt
hwt data validate design.hwt --strict

# Migrate versions
hwt data migrate design.hwt --to 2.0.0
hwt data migrate design.hwt --to latest

# Convert formats
hwt data convert design.hwt --to json
hwt data convert design.hwt --to binary
hwt data convert design.hwt_bin --to json

# Schema operations
hwt data schema show
hwt data schema export --output schema.json
hwt data schema validate design.hwt
```

---

## Rust API

```rust
use hardware_tool::data::*;

// Load and validate
let design = Design::load("project.hwt")?;
let report = design.validate(ValidationConfig::strict())?;

if !report.is_valid() {
    for error in report.errors() {
        println!("Error: {} at {}", error.message, error.path);
    }
}

// Migrate
let migrated = design.migrate_to("2.0.0")?;

// Convert to binary
design.save_binary("project.hwt_bin", BinaryConfig::default())?;

// Access schema
let schema = Schema::current();
println!("Schema version: {}", schema.version());
```

---

## Related Topics

- [Circuit JSON IR](./circuit-json-ir.md)
- [Shared Project Architecture](./shared-project-architecture.md)
- [Netlist Formats](./netlist-formats.md)
- [Shared Export/Import Architecture](../advanced-features/shared-export-import-architecture.md)
