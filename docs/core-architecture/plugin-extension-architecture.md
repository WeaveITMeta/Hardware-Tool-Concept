# Plugin & Extension Architecture

## Overview

Hardware Tool provides a **robust plugin system** that enables third-party developers to extend functionality across all hardware domains. Whether you're adding new importers, exporters, simulation engines, DRC rules, or UI components — the same plugin API handles it all.

> **"One Hardware Tool That Does It All"** — And the community can make it do even more.

---

## Plugin Types

| Type | Description | Examples |
|------|-------------|----------|
| **Importer** | Add new file format support | Altium, OrCAD, Cadence |
| **Exporter** | Add new output formats | Custom Gerber, proprietary |
| **Simulator** | Add simulation engines | Custom SPICE, FEM solver |
| **DRC Rules** | Add design rule checks | Manufacturer-specific |
| **UI Component** | Add panels, tools, widgets | Custom property editors |
| **AI Provider** | Add AI model integrations | Custom LLM, local models |
| **Library Source** | Add component sources | Manufacturer libraries |
| **Manufacturer** | Add fab house integration | Quote, order, DFM |

---

## Plugin Structure

```
my_plugin/
├── plugin.toml              # Plugin manifest
├── src/
│   └── lib.rs               # Plugin entry point
├── assets/
│   ├── icon.svg             # Plugin icon
│   └── screenshots/         # Marketplace screenshots
├── tests/
│   └── integration_test.rs
├── Cargo.toml               # Rust dependencies
└── README.md                # Documentation
```

---

## Plugin Manifest

```toml
# plugin.toml
[plugin]
name = "altium-importer"
version = "1.0.0"
description = "Import Altium Designer projects into Hardware Tool"
author = "Community Contributor"
license = "MIT"
repository = "https://github.com/user/altium-importer"

[plugin.compatibility]
hardware_tool_version = ">=1.0.0"
domains = ["pcb"]  # or ["all"] for universal plugins

[plugin.type]
category = "importer"
file_extensions = [".PcbDoc", ".SchDoc", ".PrjPcb"]

[plugin.permissions]
filesystem = ["read"]      # read, write, or both
network = false            # Network access
subprocess = false         # Spawn external processes

[plugin.ui]
menu_item = "File > Import > Altium Project"
shortcut = "Ctrl+Shift+A"
icon = "assets/icon.svg"

[plugin.dependencies]
# Other plugins this depends on
required = []
optional = ["3d-model-converter"]
```

---

## Extension Points

### Importer Extension

```rust
use hardware_tool::plugin::*;

#[plugin]
pub struct AltiumImporter;

impl ImporterPlugin for AltiumImporter {
    fn name(&self) -> &str {
        "Altium Importer"
    }
    
    fn supported_extensions(&self) -> Vec<&str> {
        vec![".PcbDoc", ".SchDoc", ".PrjPcb"]
    }
    
    fn can_import(&self, path: &Path) -> bool {
        // Quick check if file is valid
        self.detect_altium_format(path)
    }
    
    fn import(&self, path: &Path, options: ImportOptions) -> Result<Design> {
        // Parse Altium file and convert to Hardware Tool format
        let altium_data = self.parse_altium(path)?;
        let design = self.convert_to_hwt(altium_data)?;
        Ok(design)
    }
    
    fn preview(&self, path: &Path) -> Result<ImportPreview> {
        // Generate preview without full import
        let preview = ImportPreview {
            sheets: 5,
            components: 120,
            nets: 85,
            warnings: vec!["Some 3D models may not convert"],
        };
        Ok(preview)
    }
}
```

### Exporter Extension

```rust
impl ExporterPlugin for CustomExporter {
    fn name(&self) -> &str {
        "Custom Format Exporter"
    }
    
    fn supported_formats(&self) -> Vec<ExportFormat> {
        vec![ExportFormat::Custom("myformat")]
    }
    
    fn export(&self, design: &Design, options: ExportOptions) -> Result<Vec<ExportedFile>> {
        // Convert design to custom format
        let output = self.generate_output(design)?;
        Ok(vec![ExportedFile {
            path: options.output_dir.join("design.myformat"),
            content: output,
        }])
    }
    
    fn configure_ui(&self) -> Option<ConfigPanel> {
        // Return custom configuration UI
        Some(ConfigPanel::new()
            .checkbox("include_metadata", "Include Metadata", true)
            .dropdown("compression", "Compression", vec!["none", "gzip", "lz4"]))
    }
}
```

### DRC Rule Extension

```rust
impl DrcPlugin for ManufacturerDrc {
    fn name(&self) -> &str {
        "JLCPCB DRC Rules"
    }
    
    fn rules(&self) -> Vec<DrcRule> {
        vec![
            DrcRule {
                id: "jlc_min_trace",
                name: "Minimum Trace Width",
                description: "JLCPCB requires minimum 0.127mm trace width",
                category: DrcCategory::Clearance,
                severity: Severity::Error,
                check: Box::new(|design| {
                    design.traces()
                        .filter(|t| t.width < 0.127)
                        .map(|t| DrcViolation {
                            rule: "jlc_min_trace",
                            location: t.location(),
                            message: format!("Trace width {} < 0.127mm", t.width),
                        })
                        .collect()
                }),
            },
            // More rules...
        ]
    }
}
```

### UI Component Extension

```rust
impl UiPlugin for CustomPanel {
    fn name(&self) -> &str {
        "BOM Analyzer"
    }
    
    fn panel_id(&self) -> &str {
        "bom_analyzer"
    }
    
    fn render(&self, ctx: &UiContext) -> UiElement {
        Panel::new("BOM Analyzer")
            .child(Table::new()
                .columns(vec!["Reference", "Value", "Footprint", "Price"])
                .rows(self.get_bom_data(ctx.design())))
            .child(Button::new("Export BOM").on_click(|| self.export_bom()))
    }
    
    fn dock_position(&self) -> DockPosition {
        DockPosition::Right
    }
}
```

### Simulator Extension

```rust
impl SimulatorPlugin for CustomSpice {
    fn name(&self) -> &str {
        "Custom SPICE Engine"
    }
    
    fn supported_analyses(&self) -> Vec<AnalysisType> {
        vec![
            AnalysisType::DC,
            AnalysisType::AC,
            AnalysisType::Transient,
            AnalysisType::Noise,
        ]
    }
    
    fn simulate(&self, netlist: &Netlist, config: SimConfig) -> Result<SimResults> {
        // Run simulation
        let results = self.run_engine(netlist, config)?;
        Ok(results)
    }
}
```

---

## Plugin Permissions & Sandboxing

```rust
/// Plugin permission model
PluginPermissions {
    // Filesystem access
    filesystem: FilesystemPermission {
        read: vec!["/project/**"],     // Read project files
        write: vec!["/project/output/**"], // Write to output only
    },
    
    // Network access
    network: NetworkPermission {
        allowed_hosts: vec!["api.manufacturer.com"],
        allow_localhost: false,
    },
    
    // System access
    system: SystemPermission {
        subprocess: false,              // No spawning processes
        environment: false,             // No env var access
        clipboard: true,                // Clipboard allowed
    },
    
    // Hardware Tool API access
    api: ApiPermission {
        design_read: true,
        design_write: true,
        settings_read: true,
        settings_write: false,
    },
}
```

---

## Plugin Marketplace

### Marketplace UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Plugin Marketplace                                       [✕]   │
├─────────────────────────────────────────────────────────────────┤
│ Search: [altium__________________] [🔍] [Category ▼] [Sort ▼]  │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ 📦 Altium Importer                           ⭐⭐⭐⭐⭐ (4.8) │ │
│ │ Import Altium Designer projects              Downloads: 12K │ │
│ │ by Community • v2.1.0 • Updated 3 days ago                  │ │
│ │ [Install]                                                   │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ 📦 JLCPCB Integration                        ⭐⭐⭐⭐⭐ (4.9) │ │
│ │ DFM checks, instant quotes, order PCBs       Downloads: 25K │ │
│ │ by JLCPCB • v3.0.0 • Official                               │ │
│ │ [Install]                                                   │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ 📦 OrCAD Importer                            ⭐⭐⭐⭐ (4.2)  │ │
│ │ Import OrCAD Capture and PCB Editor          Downloads: 8K  │ │
│ │ by Community • v1.5.0                                       │ │
│ │ [Install]                                                   │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│ Showing 1-10 of 156 plugins                    [< 1 2 3 ... >] │
└─────────────────────────────────────────────────────────────────┘
```

### Publishing Plugins

```bash
# Build plugin
hwt plugin build

# Test plugin
hwt plugin test

# Package for distribution
hwt plugin package

# Publish to marketplace
hwt plugin publish --token <api_token>
```

---

## Plugin Configuration

### User Settings

```toml
# ~/.hwt/plugins.toml
[plugins]
# Enabled plugins
enabled = [
    "altium-importer",
    "jlcpcb-integration",
    "custom-drc",
]

# Plugin-specific settings
[plugins.jlcpcb-integration]
api_key = "your_api_key"
default_service = "standard"

[plugins.altium-importer]
convert_3d_models = true
preserve_designators = true
```

### Project-Level Overrides

```toml
# project.hwt
[plugins]
# Disable specific plugins for this project
disabled = ["experimental-feature"]

# Plugin overrides
[plugins.jlcpcb-integration]
service = "express"  # Override default
```

---

## CLI Commands

```bash
# Plugin management
hwt plugin list                    # List installed plugins
hwt plugin install <name>          # Install from marketplace
hwt plugin install ./plugin.zip    # Install from file
hwt plugin update <name>           # Update plugin
hwt plugin remove <name>           # Remove plugin
hwt plugin enable <name>           # Enable plugin
hwt plugin disable <name>          # Disable plugin

# Plugin development
hwt plugin new <name>              # Create new plugin project
hwt plugin build                   # Build plugin
hwt plugin test                    # Run plugin tests
hwt plugin package                 # Package for distribution
hwt plugin publish                 # Publish to marketplace

# Marketplace
hwt plugin search <query>          # Search marketplace
hwt plugin info <name>             # Show plugin details
```

---

## Plugin Development SDK

```rust
use hardware_tool::plugin::prelude::*;

// Plugin entry point macro
#[plugin]
pub struct MyPlugin;

// Implement required trait
impl Plugin for MyPlugin {
    fn info(&self) -> PluginInfo {
        PluginInfo {
            name: "My Plugin",
            version: "1.0.0",
            description: "Does something useful",
        }
    }
    
    fn on_load(&self, ctx: &PluginContext) -> Result<()> {
        // Called when plugin is loaded
        ctx.register_command("my_command", self.my_command)?;
        Ok(())
    }
    
    fn on_unload(&self) -> Result<()> {
        // Cleanup
        Ok(())
    }
}
```

---

## Security Model

1. **Code Signing** — All marketplace plugins are signed
2. **Permission Review** — Permissions displayed before install
3. **Sandboxing** — Plugins run in isolated environment
4. **Audit Logging** — All plugin actions logged
5. **Revocation** — Malicious plugins can be remotely disabled

---

## Related Topics

- [Command-Line Interface](../advanced-features/cli.md)
- [Shared Export/Import Architecture](../advanced-features/shared-export-import-architecture.md)
- [Shared DRC Architecture](../advanced-features/shared-drc-architecture.md)
- [Manufacturing Integration](./manufacturing-integration.md)
