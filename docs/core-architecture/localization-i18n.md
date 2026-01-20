# Localization & Internationalization (i18n)

## Overview

Hardware Tool provides **full internationalization support** to serve designers worldwide. The interface, documentation, and output formats adapt to local languages, units, and regional standards.

> **"One Hardware Tool That Does It All"** — In any language, with any unit system.

---

## Supported Languages

| Language | Code | Status | Coverage |
|----------|------|--------|----------|
| **English** | en | ✓ Complete | 100% |
| **Chinese (Simplified)** | zh-CN | ✓ Complete | 100% |
| **Chinese (Traditional)** | zh-TW | ✓ Complete | 100% |
| **Japanese** | ja | ✓ Complete | 100% |
| **Korean** | ko | ✓ Complete | 100% |
| **German** | de | ✓ Complete | 100% |
| **French** | fr | ✓ Complete | 100% |
| **Spanish** | es | ✓ Complete | 100% |
| **Portuguese** | pt-BR | ✓ Complete | 100% |
| **Russian** | ru | ✓ Complete | 100% |
| **Italian** | it | ○ Partial | 85% |
| **Polish** | pl | ○ Partial | 80% |
| **Turkish** | tr | ○ Partial | 75% |
| **Vietnamese** | vi | ○ Partial | 70% |
| **Arabic** | ar | ○ Partial | 60% |

---

## Language Configuration

### User Settings

```toml
# ~/.hwt/settings.toml

[locale]
# UI language
language = "zh-CN"

# Fallback language
fallback = "en"

# Date/time format
date_format = "YYYY-MM-DD"
time_format = "24h"

# Number format
decimal_separator = "."
thousands_separator = ","

# First day of week
first_day_of_week = "monday"
```

### Per-Project Override

```toml
# project.hwt

[locale]
# Override for this project
language = "de"

# Documentation language
documentation_language = "en"

# Export language (for generated files)
export_language = "en"
```

---

## Unit Systems

### Supported Units

| Category | Metric | Imperial | Domain-Specific |
|----------|--------|----------|-----------------|
| **Length** | mm, μm, nm | mil, inch | — |
| **Area** | mm², μm² | sq mil, sq inch | — |
| **Angle** | degrees, radians | degrees | — |
| **Temperature** | °C, K | °F | — |
| **Frequency** | Hz, kHz, MHz, GHz | — | — |
| **Resistance** | Ω, kΩ, MΩ | — | — |
| **Capacitance** | pF, nF, μF | — | — |
| **Current** | mA, A | — | — |
| **Voltage** | mV, V | — | — |

### Unit Configuration

```toml
# ~/.hwt/settings.toml

[units]
# Primary unit system
system = "metric"  # or "imperial"

# Length units by domain
[units.length]
pcb = "mm"
ic = "um"
quantum = "um"
mems = "um"
rf = "mm"
packaging = "um"

# Display precision
[units.precision]
length = 3      # decimal places
angle = 1
temperature = 1

# Automatic conversion
auto_convert = true
show_both_units = false
```

### Unit Display

```
┌─────────────────────────────────────────────────────────────────┐
│ Properties: R1                                           [✕]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Position:                                                       │
│   X: 25.400 mm (1000.0 mil)    ← Both units shown              │
│   Y: 12.700 mm (500.0 mil)                                     │
│                                                                 │
│ Rotation: 45.0°                                                │
│                                                                 │
│ Pad Size:                                                      │
│   Width:  1.200 mm                                             │
│   Height: 0.800 mm                                             │
│                                                                 │
│ [Unit: mm ▼]  ☑ Show imperial equivalent                       │
└─────────────────────────────────────────────────────────────────┘
```

---

## Translation System

### String Resources

```rust
/// Translation file structure
// locales/zh-CN/main.ftl (Fluent format)

// Menu items
menu-file = 文件
menu-edit = 编辑
menu-view = 视图
menu-tools = 工具
menu-help = 帮助

// Common actions
action-save = 保存
action-open = 打开
action-export = 导出
action-import = 导入

// Component properties
property-reference = 参考标识
property-value = 值
property-footprint = 封装

// DRC messages
drc-clearance-violation = 间距违规：{$actual}mm < {$required}mm
drc-trace-width = 走线宽度：{$width}mm
```

### Pluralization

```rust
// locales/en/main.ftl
components-selected = { $count ->
    [one] {$count} component selected
   *[other] {$count} components selected
}

// locales/zh-CN/main.ftl
components-selected = 已选择 {$count} 个元件

// locales/ru/main.ftl
components-selected = { $count ->
    [one] {$count} компонент выбран
    [few] {$count} компонента выбрано
   *[other] {$count} компонентов выбрано
}
```

---

## Regional Standards

### Manufacturing Standards by Region

| Region | PCB Standard | IC Standard | Documentation |
|--------|--------------|-------------|---------------|
| **USA** | IPC-2221 | JEDEC | English |
| **EU** | IPC + EN | JEDEC | Local language |
| **China** | GB/T | GB | Chinese + English |
| **Japan** | JIS | JEITA | Japanese |
| **Korea** | KS | — | Korean |

### Standard Configuration

```toml
# project.hwt

[standards]
# Regional standard set
region = "china"

# Specific standards
pcb_standard = "GB/T 4588"
component_marking = "GB/T 5094"

# Documentation requirements
documentation_bilingual = true
primary_language = "zh-CN"
secondary_language = "en"
```

---

## Right-to-Left (RTL) Support

### RTL Languages

```rust
RtlSupport {
    // RTL languages
    rtl_languages: vec!["ar", "he", "fa", "ur"],
    
    // UI mirroring
    mirror_ui: true,
    mirror_icons: false,  // Keep icons LTR
    
    // Text handling
    text_direction: TextDirection::Auto,
    number_direction: TextDirection::LTR,  // Numbers always LTR
    
    // Schematic
    schematic_direction: SchematicDirection::LTR,  // Keep schematics LTR
}
```

### RTL UI Example

```
┌─────────────────────────────────────────────────────────────────┐
│                                              أداة الأجهزة [✕]  │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│                                              :المكونات المحددة  │
│                                                                 │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │                                           R1 المقاومة   │   │
│  │                                           10K :القيمة   │   │
│  │                                        0603 :البصمة     │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                 │
│                              [إلغاء] [حفظ]                      │
└─────────────────────────────────────────────────────────────────┘
```

---

## Font Support

### Unicode Coverage

```rust
FontConfig {
    // Primary font
    primary: "Inter",
    
    // CJK fallback
    cjk_fallback: "Noto Sans CJK",
    
    // Arabic fallback
    arabic_fallback: "Noto Sans Arabic",
    
    // Monospace (for code)
    monospace: "JetBrains Mono",
    monospace_cjk: "Noto Sans Mono CJK",
    
    // Symbol font
    symbols: "Noto Sans Symbols",
    
    // Schematic font (must support all characters)
    schematic: "Hardware Tool Schematic",
}
```

---

## Keyboard Layouts

### Input Method Support

| Language | Input Method | Status |
|----------|--------------|--------|
| **Chinese** | Pinyin, Wubi | ✓ Supported |
| **Japanese** | Romaji, Kana | ✓ Supported |
| **Korean** | Hangul | ✓ Supported |
| **Russian** | Cyrillic | ✓ Supported |
| **Arabic** | Arabic keyboard | ✓ Supported |

### Keyboard Shortcuts

```toml
# Keyboard shortcuts adapt to layout

[shortcuts.en]
save = "Ctrl+S"
undo = "Ctrl+Z"
redo = "Ctrl+Y"

[shortcuts.de]
# German keyboard: Z and Y are swapped
save = "Ctrl+S"
undo = "Ctrl+Z"
redo = "Ctrl+Y"  # Same logical key

[shortcuts.zh-CN]
# Chinese: same as English
save = "Ctrl+S"
undo = "Ctrl+Z"
redo = "Ctrl+Y"
```

---

## Export Localization

### Localized Output

```rust
ExportLocalization {
    // BOM language
    bom_language: "zh-CN",
    bom_headers: BomHeaders {
        reference: "参考标识",
        value: "值",
        footprint: "封装",
        quantity: "数量",
    },
    
    // Fabrication notes
    fab_notes_language: "en",  // Usually English for manufacturers
    
    // Assembly drawings
    assembly_language: "bilingual",
    
    // Documentation
    documentation_language: "zh-CN",
}
```

### Bilingual Documentation

```
┌─────────────────────────────────────────────────────────────────┐
│ 制造说明 / Fabrication Notes                                    │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ 1. 材料 / Material                                              │
│    FR-4 TG150                                                   │
│                                                                 │
│ 2. 层数 / Layers                                                │
│    4层 / 4 layers                                               │
│                                                                 │
│ 3. 成品厚度 / Finished Thickness                                │
│    1.6mm ±10%                                                   │
│                                                                 │
│ 4. 铜厚 / Copper Weight                                         │
│    外层 1oz / Outer 1oz                                         │
│    内层 0.5oz / Inner 0.5oz                                     │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Contributing Translations

### Translation Workflow

```bash
# Extract strings for translation
hwt i18n extract --output locales/messages.pot

# Create new language
hwt i18n init --language vi

# Update existing translations
hwt i18n update --language zh-CN

# Validate translations
hwt i18n validate --language zh-CN

# Check coverage
hwt i18n coverage
```

### Translation File Format

```ftl
# locales/vi/main.ftl
# Vietnamese translation

# Metadata
-brand-name = Hardware Tool

# Menu
menu-file = Tệp
menu-edit = Chỉnh sửa
menu-view = Xem
menu-tools = Công cụ
menu-help = Trợ giúp

# Actions
action-save = Lưu
action-open = Mở
action-new = Mới
action-close = Đóng
```

---

## CLI Commands

```bash
# Language settings
hwt config set locale.language zh-CN
hwt config set units.system metric

# Check current locale
hwt config get locale

# List available languages
hwt i18n languages

# Switch language temporarily
hwt --language de open project.hwt

# Export with specific language
hwt export --language en --output ./output/
```

---

## Rust API

```rust
use hardware_tool::i18n::*;

// Set locale
Locale::set("zh-CN")?;

// Get translated string
let msg = t!("drc-clearance-violation", actual = 0.1, required = 0.15);
// Output: "间距违规：0.1mm < 0.15mm"

// Format with units
let length = Length::mm(25.4);
println!("{}", length.format(UnitSystem::Imperial));
// Output: "1000.0 mil"

// Pluralization
let msg = t!("components-selected", count = 5);
// Output (en): "5 components selected"
// Output (zh-CN): "已选择 5 个元件"
```

---

## Related Topics

- [Accessibility & Theming](../ux-ui-design/accessibility-and-theming.md)
- [Documentation Generation](../advanced-features/documentation-generation.md)
- [Keyboard Shortcuts](../ux-ui-design/keyboard-shortcuts-reference.md)
- [Manufacturing Integration](./manufacturing-integration.md)
