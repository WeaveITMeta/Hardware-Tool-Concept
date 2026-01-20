//! SVG Export Module.
//!
//! Generates SVG (Scalable Vector Graphics) from schematic and PCB data.
//! SVG is ideal for web display and high-quality vector output.

use std::fmt::Write;

use crate::schematic::SchematicSheet;
use crate::layout::Layout;

/// SVG document generator.
pub struct SvgGenerator {
    /// Canvas width in mm
    width: f64,
    /// Canvas height in mm
    height: f64,
    /// Scale factor (pixels per mm)
    scale: f64,
    /// Background color
    background: Option<String>,
    /// Include grid
    include_grid: bool,
    /// Grid spacing in mm
    grid_spacing: f64,
    /// Stroke width for lines
    stroke_width: f64,
}

/// SVG export options.
#[derive(Debug, Clone)]
pub struct SvgExportOptions {
    /// Canvas width in mm
    pub width_mm: f64,
    /// Canvas height in mm
    pub height_mm: f64,
    /// Scale factor (pixels per mm)
    pub scale: f64,
    /// Background color (None for transparent)
    pub background: Option<String>,
    /// Include grid
    pub include_grid: bool,
    /// Grid spacing in mm
    pub grid_spacing_mm: f64,
    /// Stroke width
    pub stroke_width: f64,
}

impl Default for SvgExportOptions {
    fn default() -> Self {
        Self {
            width_mm: 297.0,   // A4 landscape width
            height_mm: 210.0,  // A4 landscape height
            scale: 3.78,       // ~96 DPI
            background: Some("#ffffff".to_string()),
            include_grid: false,
            grid_spacing_mm: 2.54,
            stroke_width: 0.5,
        }
    }
}

impl SvgGenerator {
    /// Create a new SVG generator with default A4 landscape settings.
    pub fn new() -> Self {
        Self {
            width: 297.0,
            height: 210.0,
            scale: 3.78,
            background: Some("#ffffff".to_string()),
            include_grid: false,
            grid_spacing: 2.54,
            stroke_width: 0.5,
        }
    }

    /// Create with specific options.
    pub fn with_options(options: &SvgExportOptions) -> Self {
        Self {
            width: options.width_mm,
            height: options.height_mm,
            scale: options.scale,
            background: options.background.clone(),
            include_grid: options.include_grid,
            grid_spacing: options.grid_spacing_mm,
            stroke_width: options.stroke_width,
        }
    }

    /// Set canvas size.
    pub fn with_size(mut self, width_mm: f64, height_mm: f64) -> Self {
        self.width = width_mm;
        self.height = height_mm;
        self
    }

    /// Set scale factor.
    pub fn with_scale(mut self, scale: f64) -> Self {
        self.scale = scale;
        self
    }

    /// Set background color.
    pub fn with_background(mut self, color: impl Into<String>) -> Self {
        self.background = Some(color.into());
        self
    }

    /// Set transparent background.
    pub fn transparent(mut self) -> Self {
        self.background = None;
        self
    }

    /// Generate SVG from a schematic sheet.
    pub fn generate_schematic(&self, sheet: &SchematicSheet) -> String {
        let mut svg = String::new();
        let pixel_width = self.width * self.scale;
        let pixel_height = self.height * self.scale;

        // SVG header
        writeln!(svg, r#"<?xml version="1.0" encoding="UTF-8"?>"#).unwrap();
        writeln!(svg, r#"<svg xmlns="http://www.w3.org/2000/svg" version="1.1""#).unwrap();
        writeln!(svg, r#"     width="{:.0}" height="{:.0}""#, pixel_width, pixel_height).unwrap();
        writeln!(svg, r#"     viewBox="0 0 {:.2} {:.2}">"#, self.width, self.height).unwrap();

        // Title
        writeln!(svg, r#"  <title>{}</title>"#, self.escape_xml(&sheet.name)).unwrap();

        // Definitions (for reusable elements)
        writeln!(svg, r#"  <defs>"#).unwrap();
        writeln!(svg, r#"    <marker id="arrowhead" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">"#).unwrap();
        writeln!(svg, "      <polygon points=\"0 0, 10 3.5, 0 7\" fill=\"#000\"/>").unwrap();
        writeln!(svg, r#"    </marker>"#).unwrap();
        writeln!(svg, r#"  </defs>"#).unwrap();

        // Background
        if let Some(ref bg) = self.background {
            writeln!(svg, r#"  <rect width="100%" height="100%" fill="{}"/>"#, bg).unwrap();
        }

        // Grid
        if self.include_grid {
            self.write_grid(&mut svg);
        }

        // Border
        writeln!(svg, "  <rect x=\"5\" y=\"5\" width=\"{:.2}\" height=\"{:.2}\" fill=\"none\" stroke=\"#000\" stroke-width=\"0.5\"/>\n", 
            self.width - 10.0, self.height - 10.0).unwrap();

        // Schematic content group
        writeln!(svg, "  <g id=\"schematic\" stroke=\"#000\" stroke-width=\"{:.2}\" fill=\"none\">", self.stroke_width).unwrap();

        // Draw wires
        for wire in &sheet.wires {
            writeln!(svg, r#"    <line x1="{:.2}" y1="{:.2}" x2="{:.2}" y2="{:.2}"/>"#,
                wire.start.x, wire.start.y, wire.end.x, wire.end.y).unwrap();
        }

        // Draw junctions
        for junction in &sheet.junctions {
            writeln!(svg, "    <circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"1\" fill=\"#000\"/>",
                junction.position.x, junction.position.y).unwrap();
        }

        writeln!(svg, r#"  </g>"#).unwrap();

        // Symbols group
        writeln!(svg, r#"  <g id="symbols">"#).unwrap();

        for symbol in &sheet.symbols {
            self.write_symbol(&mut svg, symbol);
        }

        writeln!(svg, r#"  </g>"#).unwrap();

        // Labels group
        writeln!(svg, r#"  <g id="labels" font-family="sans-serif" font-size="3">"#).unwrap();

        for label in &sheet.labels {
            writeln!(svg, r#"    <text x="{:.2}" y="{:.2}">{}</text>"#,
                label.position.x, label.position.y, self.escape_xml(&label.name)).unwrap();
        }

        writeln!(svg, r#"  </g>"#).unwrap();

        // Power symbols
        writeln!(svg, r#"  <g id="power" font-family="sans-serif" font-size="2.5">"#).unwrap();

        for power in &sheet.power_symbols {
            writeln!(svg, r#"    <text x="{:.2}" y="{:.2}" text-anchor="middle">{}</text>"#,
                power.position.x, power.position.y, self.escape_xml(&power.net_name)).unwrap();
        }

        writeln!(svg, r#"  </g>"#).unwrap();

        // Close SVG
        writeln!(svg, r#"</svg>"#).unwrap();

        svg
    }

    /// Generate SVG from a PCB layout.
    pub fn generate_layout(&self, layout: &Layout, layers: &[String]) -> String {
        let mut svg = String::new();
        let pixel_width = self.width * self.scale;
        let pixel_height = self.height * self.scale;

        // SVG header
        writeln!(svg, r#"<?xml version="1.0" encoding="UTF-8"?>"#).unwrap();
        writeln!(svg, r#"<svg xmlns="http://www.w3.org/2000/svg" version="1.1""#).unwrap();
        writeln!(svg, r#"     width="{:.0}" height="{:.0}""#, pixel_width, pixel_height).unwrap();
        writeln!(svg, r#"     viewBox="0 0 {:.2} {:.2}">"#, self.width, self.height).unwrap();

        writeln!(svg, r#"  <title>PCB Layout</title>"#).unwrap();

        // Background
        if let Some(ref bg) = self.background {
            writeln!(svg, r#"  <rect width="100%" height="100%" fill="{}"/>"#, bg).unwrap();
        }

        // Traces group
        writeln!(svg, "  <g id=\"traces\" stroke=\"#00aa00\" fill=\"none\">").unwrap();

        for trace in &layout.traces {
            if layers.is_empty() || layers.contains(&trace.layer) {
                writeln!(svg, r#"    <line x1="{:.2}" y1="{:.2}" x2="{:.2}" y2="{:.2}" stroke-width="{:.2}"/>"#,
                    trace.start.x, trace.start.y, trace.end.x, trace.end.y, trace.width).unwrap();
            }
        }

        writeln!(svg, r#"  </g>"#).unwrap();

        // Vias group
        writeln!(svg, r#"  <g id="vias">"#).unwrap();

        for via in &layout.vias {
            let outer_r = via.pad / 2.0;
            let inner_r = via.drill / 2.0;
            writeln!(svg, "    <circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"{:.2}\" fill=\"#888\" stroke=\"#000\" stroke-width=\"0.1\"/>",
                via.position.x, via.position.y, outer_r).unwrap();
            writeln!(svg, "    <circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"{:.2}\" fill=\"#fff\"/>",
                via.position.x, via.position.y, inner_r).unwrap();
        }

        writeln!(svg, r#"  </g>"#).unwrap();

        // Components group
        writeln!(svg, r#"  <g id="components" font-family="sans-serif" font-size="2">"#).unwrap();

        for component in &layout.components {
            writeln!(svg, "    <rect x=\"{:.2}\" y=\"{:.2}\" width=\"8\" height=\"5\" fill=\"none\" stroke=\"#000\" stroke-width=\"0.2\"/>",
                component.position.x - 4.0, component.position.y - 2.5).unwrap();
            writeln!(svg, r#"    <text x="{:.2}" y="{:.2}" text-anchor="middle">{}</text>"#,
                component.position.x, component.position.y + 0.7, self.escape_xml(&component.reference)).unwrap();
        }

        writeln!(svg, r#"  </g>"#).unwrap();

        // Close SVG
        writeln!(svg, r#"</svg>"#).unwrap();

        svg
    }

    /// Write grid to SVG.
    fn write_grid(&self, svg: &mut String) {
        writeln!(svg, "  <g id=\"grid\" stroke=\"#ddd\" stroke-width=\"0.1\">").unwrap();

        // Vertical lines
        let mut x = 0.0;
        while x <= self.width {
            writeln!(svg, r#"    <line x1="{:.2}" y1="0" x2="{:.2}" y2="{:.2}"/>"#, x, x, self.height).unwrap();
            x += self.grid_spacing;
        }

        // Horizontal lines
        let mut y = 0.0;
        while y <= self.height {
            writeln!(svg, r#"    <line x1="0" y1="{:.2}" x2="{:.2}" y2="{:.2}"/>"#, y, self.width, y).unwrap();
            y += self.grid_spacing;
        }

        writeln!(svg, r#"  </g>"#).unwrap();
    }

    /// Write a symbol to SVG.
    fn write_symbol(&self, svg: &mut String, symbol: &crate::schematic::PlacedSymbol) {
        let x = symbol.position.x;
        let y = symbol.position.y;

        writeln!(svg, r#"    <g transform="translate({:.2},{:.2}) rotate({:.0})">"#, x, y, symbol.rotation).unwrap();
        
        // Symbol body (simplified rectangle)
        writeln!(svg, "      <rect x=\"-5\" y=\"-4\" width=\"10\" height=\"8\" fill=\"none\" stroke=\"#000\" stroke-width=\"0.3\"/>").unwrap();
        
        // Reference
        writeln!(svg, r#"      <text x="0" y="-5" text-anchor="middle" font-family="sans-serif" font-size="2.5">{}</text>"#,
            self.escape_xml(&symbol.reference)).unwrap();
        
        // Value
        writeln!(svg, r#"      <text x="0" y="7" text-anchor="middle" font-family="sans-serif" font-size="2">{}</text>"#,
            self.escape_xml(&symbol.value)).unwrap();

        writeln!(svg, r#"    </g>"#).unwrap();
    }

    /// Escape XML special characters.
    fn escape_xml(&self, s: &str) -> String {
        s.replace('&', "&amp;")
         .replace('<', "&lt;")
         .replace('>', "&gt;")
         .replace('"', "&quot;")
         .replace('\'', "&apos;")
    }
}

impl Default for SvgGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schematic::SchematicSheet;
    use crate::layout::Layout;

    #[test]
    fn test_svg_generator_new() {
        let generator = SvgGenerator::new();
        assert!(generator.width > 0.0);
        assert!(generator.height > 0.0);
    }

    #[test]
    fn test_svg_export_options_default() {
        let options = SvgExportOptions::default();
        assert!((options.width_mm - 297.0).abs() < 0.1);
        assert!((options.height_mm - 210.0).abs() < 0.1);
    }

    #[test]
    fn test_generate_schematic_svg() {
        let sheet = SchematicSheet::new("Test Sheet");
        let generator = SvgGenerator::new();
        let svg = generator.generate_schematic(&sheet);
        
        assert!(svg.contains("<?xml version"));
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
        assert!(svg.contains("Test Sheet"));
    }

    #[test]
    fn test_generate_layout_svg() {
        let layout = Layout::new();
        let generator = SvgGenerator::new();
        let svg = generator.generate_layout(&layout, &[]);
        
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
        assert!(svg.contains("PCB Layout"));
    }

    #[test]
    fn test_svg_with_options() {
        let options = SvgExportOptions {
            width_mm: 100.0,
            height_mm: 80.0,
            scale: 5.0,
            background: None,
            include_grid: true,
            grid_spacing_mm: 5.0,
            stroke_width: 1.0,
        };
        
        let generator = SvgGenerator::with_options(&options);
        assert!((generator.width - 100.0).abs() < 0.1);
        assert!((generator.height - 80.0).abs() < 0.1);
        assert!(generator.background.is_none());
    }

    #[test]
    fn test_svg_transparent_background() {
        let sheet = SchematicSheet::new("Test");
        let generator = SvgGenerator::new().transparent();
        let svg = generator.generate_schematic(&sheet);
        
        // Should not contain background rect with fill
        assert!(!svg.contains("fill=\"#ffffff\""));
    }

    #[test]
    fn test_svg_escape_xml() {
        let generator = SvgGenerator::new();
        let escaped = generator.escape_xml("<test & \"value\">");
        assert_eq!(escaped, "&lt;test &amp; &quot;value&quot;&gt;");
    }
}
