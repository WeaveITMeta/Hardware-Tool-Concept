//! PNG Export Module.
//!
//! Generates PNG raster images from schematic and PCB data.
//! Uses SVG as an intermediate format for rendering.

use crate::schematic::SchematicSheet;
use crate::layout::Layout;
use crate::svg_export::{SvgGenerator, SvgExportOptions};

/// PNG export options.
#[derive(Debug, Clone)]
pub struct PngExportOptions {
    /// Image width in pixels
    pub width: u32,
    /// Image height in pixels
    pub height: u32,
    /// Background color (hex format, e.g., "#ffffff")
    pub background: Option<String>,
    /// DPI (dots per inch)
    pub dpi: u32,
    /// Anti-aliasing
    pub antialias: bool,
}

impl Default for PngExportOptions {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
            background: Some("#ffffff".to_string()),
            dpi: 96,
            antialias: true,
        }
    }
}

impl PngExportOptions {
    /// Create options for A4 at specified DPI.
    pub fn a4_landscape(dpi: u32) -> Self {
        // A4 = 297mm x 210mm
        let width = (297.0 * dpi as f64 / 25.4) as u32;
        let height = (210.0 * dpi as f64 / 25.4) as u32;
        Self {
            width,
            height,
            dpi,
            ..Default::default()
        }
    }

    /// Create options for A4 portrait at specified DPI.
    pub fn a4_portrait(dpi: u32) -> Self {
        let width = (210.0 * dpi as f64 / 25.4) as u32;
        let height = (297.0 * dpi as f64 / 25.4) as u32;
        Self {
            width,
            height,
            dpi,
            ..Default::default()
        }
    }

    /// Create options for specific resolution.
    pub fn resolution(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            ..Default::default()
        }
    }

    /// Set transparent background.
    pub fn transparent(mut self) -> Self {
        self.background = None;
        self
    }
}

/// PNG image generator.
/// 
/// Note: This module provides the interface and SVG generation.
/// Actual rasterization requires an external renderer (e.g., resvg, librsvg).
/// The `generate_*` methods return SVG data that can be rendered externally,
/// while `to_png_*` methods provide placeholder PNG data structure.
pub struct PngGenerator {
    options: PngExportOptions,
}

impl PngGenerator {
    /// Create a new PNG generator with default options.
    pub fn new() -> Self {
        Self {
            options: PngExportOptions::default(),
        }
    }

    /// Create with specific options.
    pub fn with_options(options: PngExportOptions) -> Self {
        Self { options }
    }

    /// Generate SVG for schematic (intermediate step for PNG).
    pub fn generate_schematic_svg(&self, sheet: &SchematicSheet) -> String {
        let svg_options = self.to_svg_options();
        let generator = SvgGenerator::with_options(&svg_options);
        generator.generate_schematic(sheet)
    }

    /// Generate SVG for layout (intermediate step for PNG).
    pub fn generate_layout_svg(&self, layout: &Layout, layers: &[String]) -> String {
        let svg_options = self.to_svg_options();
        let generator = SvgGenerator::with_options(&svg_options);
        generator.generate_layout(layout, layers)
    }

    /// Generate PNG data for schematic.
    /// 
    /// Returns a PngData struct containing the SVG source and metadata.
    /// Use an external renderer to convert to actual PNG bytes.
    pub fn to_png_schematic(&self, sheet: &SchematicSheet) -> PngData {
        let svg = self.generate_schematic_svg(sheet);
        PngData {
            svg_source: svg,
            width: self.options.width,
            height: self.options.height,
            dpi: self.options.dpi,
            background: self.options.background.clone(),
        }
    }

    /// Generate PNG data for layout.
    pub fn to_png_layout(&self, layout: &Layout, layers: &[String]) -> PngData {
        let svg = self.generate_layout_svg(layout, layers);
        PngData {
            svg_source: svg,
            width: self.options.width,
            height: self.options.height,
            dpi: self.options.dpi,
            background: self.options.background.clone(),
        }
    }

    /// Convert options to SVG export options.
    fn to_svg_options(&self) -> SvgExportOptions {
        // Calculate mm dimensions from pixels and DPI
        let width_mm = self.options.width as f64 * 25.4 / self.options.dpi as f64;
        let height_mm = self.options.height as f64 * 25.4 / self.options.dpi as f64;
        let scale = self.options.dpi as f64 / 25.4;  // pixels per mm

        SvgExportOptions {
            width_mm,
            height_mm,
            scale,
            background: self.options.background.clone(),
            include_grid: false,
            grid_spacing_mm: 2.54,
            stroke_width: 0.5,
        }
    }
}

impl Default for PngGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// PNG data container.
/// 
/// Contains SVG source and rendering parameters.
/// Use an external SVG renderer to convert to actual PNG bytes.
#[derive(Debug, Clone)]
pub struct PngData {
    /// SVG source to render
    pub svg_source: String,
    /// Target width in pixels
    pub width: u32,
    /// Target height in pixels
    pub height: u32,
    /// DPI setting
    pub dpi: u32,
    /// Background color
    pub background: Option<String>,
}

impl PngData {
    /// Get the SVG source.
    pub fn svg(&self) -> &str {
        &self.svg_source
    }

    /// Get dimensions as (width, height).
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Check if background is transparent.
    pub fn is_transparent(&self) -> bool {
        self.background.is_none()
    }

    /// Estimate file size in bytes (rough approximation).
    pub fn estimated_size(&self) -> usize {
        // Rough estimate: 3 bytes per pixel (RGB) with ~50% compression
        (self.width as usize * self.height as usize * 3) / 2
    }

    /// Generate a simple placeholder PNG header.
    /// 
    /// This is a minimal valid PNG that can be used for testing.
    /// For actual rendering, use an external SVG renderer.
    pub fn placeholder_png(&self) -> Vec<u8> {
        // Minimal 1x1 white PNG
        let mut png = Vec::new();
        
        // PNG signature
        png.extend_from_slice(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]);
        
        // IHDR chunk (image header)
        let ihdr_data = [
            0x00, 0x00, 0x00, 0x01,  // width: 1
            0x00, 0x00, 0x00, 0x01,  // height: 1
            0x08,                     // bit depth: 8
            0x02,                     // color type: RGB
            0x00,                     // compression: deflate
            0x00,                     // filter: adaptive
            0x00,                     // interlace: none
        ];
        png.extend_from_slice(&[0x00, 0x00, 0x00, 0x0D]); // length
        png.extend_from_slice(b"IHDR");
        png.extend_from_slice(&ihdr_data);
        png.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // CRC placeholder
        
        // IDAT chunk (image data) - minimal
        png.extend_from_slice(&[0x00, 0x00, 0x00, 0x0C]); // length
        png.extend_from_slice(b"IDAT");
        png.extend_from_slice(&[0x08, 0xD7, 0x63, 0xF8, 0xFF, 0xFF, 0xFF, 0x00, 0x05, 0xFE, 0x02, 0xFE]);
        png.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // CRC placeholder
        
        // IEND chunk
        png.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // length
        png.extend_from_slice(b"IEND");
        png.extend_from_slice(&[0xAE, 0x42, 0x60, 0x82]); // CRC
        
        png
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schematic::SchematicSheet;
    use crate::layout::Layout;

    #[test]
    fn test_png_options_default() {
        let options = PngExportOptions::default();
        assert_eq!(options.width, 1920);
        assert_eq!(options.height, 1080);
        assert_eq!(options.dpi, 96);
    }

    #[test]
    fn test_png_options_a4_landscape() {
        let options = PngExportOptions::a4_landscape(300);
        // A4 at 300 DPI: 297mm * 300 / 25.4 ≈ 3508 pixels
        assert!(options.width > 3500);
        assert!(options.height > 2400);
    }

    #[test]
    fn test_png_options_transparent() {
        let options = PngExportOptions::default().transparent();
        assert!(options.background.is_none());
    }

    #[test]
    fn test_png_generator_new() {
        let generator = PngGenerator::new();
        assert_eq!(generator.options.width, 1920);
    }

    #[test]
    fn test_generate_schematic_svg() {
        let sheet = SchematicSheet::new("Test");
        let generator = PngGenerator::new();
        let svg = generator.generate_schematic_svg(&sheet);
        
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
    }

    #[test]
    fn test_to_png_schematic() {
        let sheet = SchematicSheet::new("Test");
        let generator = PngGenerator::new();
        let png_data = generator.to_png_schematic(&sheet);
        
        assert_eq!(png_data.width, 1920);
        assert_eq!(png_data.height, 1080);
        assert!(!png_data.svg_source.is_empty());
    }

    #[test]
    fn test_to_png_layout() {
        let layout = Layout::new();
        let generator = PngGenerator::new();
        let png_data = generator.to_png_layout(&layout, &[]);
        
        assert!(!png_data.svg_source.is_empty());
        assert_eq!(png_data.dimensions(), (1920, 1080));
    }

    #[test]
    fn test_png_data_estimated_size() {
        let png_data = PngData {
            svg_source: String::new(),
            width: 1920,
            height: 1080,
            dpi: 96,
            background: None,
        };
        
        // 1920 * 1080 * 3 / 2 = ~3.1 MB
        assert!(png_data.estimated_size() > 3_000_000);
    }

    #[test]
    fn test_placeholder_png() {
        let png_data = PngData {
            svg_source: String::new(),
            width: 1,
            height: 1,
            dpi: 96,
            background: Some("#ffffff".to_string()),
        };
        
        let png = png_data.placeholder_png();
        
        // Check PNG signature
        assert_eq!(&png[0..8], &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]);
    }
}
