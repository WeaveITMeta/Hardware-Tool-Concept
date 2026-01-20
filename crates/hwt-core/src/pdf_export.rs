//! PDF Export Module.
//!
//! Generates PDF documents from schematic and PCB data.
//! Uses a simple PDF generation approach without external dependencies.

use std::fmt::Write;

use crate::schematic::SchematicSheet;
use crate::layout::Layout;

/// PDF document generator.
pub struct PdfGenerator {
    /// Page width in points (1 point = 1/72 inch)
    page_width: f64,
    /// Page height in points
    page_height: f64,
    /// Margin in points
    margin: f64,
    /// Title for the document
    title: String,
    /// Author
    author: Option<String>,
    /// Include grid
    include_grid: bool,
    /// Grid spacing in mm
    grid_spacing: f64,
}

/// PDF page size presets.
#[derive(Debug, Clone, Copy)]
pub enum PageSize {
    /// A4 (210 x 297 mm)
    A4,
    /// A3 (297 x 420 mm)
    A3,
    /// Letter (8.5 x 11 inches)
    Letter,
    /// Legal (8.5 x 14 inches)
    Legal,
    /// Tabloid (11 x 17 inches)
    Tabloid,
    /// Custom size in points
    Custom { width: f64, height: f64 },
}

impl PageSize {
    /// Get dimensions in points.
    pub fn dimensions(&self) -> (f64, f64) {
        match self {
            PageSize::A4 => (595.28, 841.89),      // 210 x 297 mm
            PageSize::A3 => (841.89, 1190.55),     // 297 x 420 mm
            PageSize::Letter => (612.0, 792.0),    // 8.5 x 11 inches
            PageSize::Legal => (612.0, 1008.0),    // 8.5 x 14 inches
            PageSize::Tabloid => (792.0, 1224.0),  // 11 x 17 inches
            PageSize::Custom { width, height } => (*width, *height),
        }
    }
}

/// PDF export options.
#[derive(Debug, Clone)]
pub struct PdfExportOptions {
    /// Page size
    pub page_size: PageSize,
    /// Landscape orientation
    pub landscape: bool,
    /// Margin in mm
    pub margin_mm: f64,
    /// Include title block
    pub include_title_block: bool,
    /// Include grid
    pub include_grid: bool,
    /// Grid spacing in mm
    pub grid_spacing_mm: f64,
    /// Black and white mode
    pub black_and_white: bool,
    /// Scale factor (1.0 = 100%)
    pub scale: f64,
}

impl Default for PdfExportOptions {
    fn default() -> Self {
        Self {
            page_size: PageSize::A4,
            landscape: true,
            margin_mm: 10.0,
            include_title_block: true,
            include_grid: false,
            grid_spacing_mm: 2.54,
            black_and_white: false,
            scale: 1.0,
        }
    }
}

impl PdfGenerator {
    /// Create a new PDF generator with default A4 landscape settings.
    pub fn new(title: impl Into<String>) -> Self {
        let (width, height) = PageSize::A4.dimensions();
        Self {
            page_width: height,  // Landscape
            page_height: width,
            margin: 28.35,       // 10mm
            title: title.into(),
            author: None,
            include_grid: false,
            grid_spacing: 2.54,
        }
    }

    /// Create with specific options.
    pub fn with_options(title: impl Into<String>, options: &PdfExportOptions) -> Self {
        let (width, height) = options.page_size.dimensions();
        let (page_width, page_height) = if options.landscape {
            (height, width)
        } else {
            (width, height)
        };

        Self {
            page_width,
            page_height,
            margin: options.margin_mm * 2.835,  // mm to points
            title: title.into(),
            author: None,
            include_grid: options.include_grid,
            grid_spacing: options.grid_spacing_mm,
        }
    }

    /// Set author.
    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    /// Generate PDF from a schematic sheet.
    pub fn generate_schematic(&self, sheet: &SchematicSheet) -> Vec<u8> {
        let mut pdf = PdfDocument::new();
        
        // Set document info
        pdf.set_title(&self.title);
        if let Some(ref author) = self.author {
            pdf.set_author(author);
        }

        // Create page
        let page_id = pdf.add_page(self.page_width, self.page_height);
        let mut content = String::new();

        // Draw border
        self.draw_border(&mut content);

        // Draw title block
        self.draw_title_block(&mut content, &sheet.name);

        // Draw grid if enabled
        if self.include_grid {
            self.draw_grid(&mut content);
        }

        // Draw schematic elements
        self.draw_schematic_content(&mut content, sheet);

        pdf.set_page_content(page_id, &content);
        pdf.generate()
    }

    /// Generate PDF from a PCB layout.
    pub fn generate_layout(&self, layout: &Layout, layers: &[String]) -> Vec<u8> {
        let mut pdf = PdfDocument::new();
        
        pdf.set_title(&self.title);
        if let Some(ref author) = self.author {
            pdf.set_author(author);
        }

        let page_id = pdf.add_page(self.page_width, self.page_height);
        let mut content = String::new();

        // Draw border
        self.draw_border(&mut content);

        // Draw title block
        self.draw_title_block(&mut content, &self.title);

        // Draw PCB content
        self.draw_layout_content(&mut content, layout, layers);

        pdf.set_page_content(page_id, &content);
        pdf.generate()
    }

    /// Draw page border.
    fn draw_border(&self, content: &mut String) {
        let x = self.margin;
        let y = self.margin;
        let w = self.page_width - 2.0 * self.margin;
        let h = self.page_height - 2.0 * self.margin;

        writeln!(content, "q").unwrap();
        writeln!(content, "0.5 w").unwrap();  // Line width
        writeln!(content, "0 0 0 RG").unwrap();  // Black stroke
        writeln!(content, "{:.2} {:.2} {:.2} {:.2} re S", x, y, w, h).unwrap();
        writeln!(content, "Q").unwrap();
    }

    /// Draw title block.
    fn draw_title_block(&self, content: &mut String, sheet_name: &str) {
        let block_width = 180.0;
        let block_height = 40.0;
        let x = self.page_width - self.margin - block_width;
        let y = self.margin;

        writeln!(content, "q").unwrap();
        writeln!(content, "0.5 w").unwrap();
        writeln!(content, "0 0 0 RG").unwrap();
        
        // Title block border
        writeln!(content, "{:.2} {:.2} {:.2} {:.2} re S", x, y, block_width, block_height).unwrap();
        
        // Horizontal divider
        writeln!(content, "{:.2} {:.2} m {:.2} {:.2} l S", x, y + 20.0, x + block_width, y + 20.0).unwrap();
        
        // Title text
        writeln!(content, "BT").unwrap();
        writeln!(content, "/F1 10 Tf").unwrap();
        writeln!(content, "{:.2} {:.2} Td", x + 5.0, y + 25.0).unwrap();
        writeln!(content, "({}) Tj", self.escape_pdf_string(&self.title)).unwrap();
        writeln!(content, "ET").unwrap();
        
        // Sheet name
        writeln!(content, "BT").unwrap();
        writeln!(content, "/F1 8 Tf").unwrap();
        writeln!(content, "{:.2} {:.2} Td", x + 5.0, y + 5.0).unwrap();
        writeln!(content, "(Sheet: {}) Tj", self.escape_pdf_string(sheet_name)).unwrap();
        writeln!(content, "ET").unwrap();
        
        writeln!(content, "Q").unwrap();
    }

    /// Draw grid.
    fn draw_grid(&self, content: &mut String) {
        let grid_pts = self.grid_spacing * 2.835;  // mm to points
        let x_start = self.margin;
        let y_start = self.margin;
        let x_end = self.page_width - self.margin;
        let y_end = self.page_height - self.margin;

        writeln!(content, "q").unwrap();
        writeln!(content, "0.1 w").unwrap();
        writeln!(content, "0.8 0.8 0.8 RG").unwrap();  // Light gray

        // Vertical lines
        let mut x = x_start;
        while x <= x_end {
            writeln!(content, "{:.2} {:.2} m {:.2} {:.2} l S", x, y_start, x, y_end).unwrap();
            x += grid_pts;
        }

        // Horizontal lines
        let mut y = y_start;
        while y <= y_end {
            writeln!(content, "{:.2} {:.2} m {:.2} {:.2} l S", x_start, y, x_end, y).unwrap();
            y += grid_pts;
        }

        writeln!(content, "Q").unwrap();
    }

    /// Draw schematic content.
    fn draw_schematic_content(&self, content: &mut String, sheet: &SchematicSheet) {
        let scale = 2.835;  // mm to points (assuming schematic units are mm)
        let offset_x = self.margin + 20.0;
        let offset_y = self.page_height - self.margin - 20.0;

        writeln!(content, "q").unwrap();

        // Draw wires
        writeln!(content, "0.5 w").unwrap();
        writeln!(content, "0 0 0 RG").unwrap();
        for wire in &sheet.wires {
            let x1 = offset_x + wire.start.x * scale;
            let y1 = offset_y - wire.start.y * scale;
            let x2 = offset_x + wire.end.x * scale;
            let y2 = offset_y - wire.end.y * scale;
            writeln!(content, "{:.2} {:.2} m {:.2} {:.2} l S", x1, y1, x2, y2).unwrap();
        }

        // Draw junctions
        writeln!(content, "0 0 0 rg").unwrap();
        for junction in &sheet.junctions {
            let x = offset_x + junction.position.x * scale;
            let y = offset_y - junction.position.y * scale;
            // Draw filled circle
            writeln!(content, "{:.2} {:.2} 2 0 360 arc f", x, y).unwrap();
        }

        // Draw symbols (simplified as rectangles with reference)
        for symbol in &sheet.symbols {
            let x = offset_x + symbol.position.x * scale;
            let y = offset_y - symbol.position.y * scale;
            
            // Draw symbol box
            writeln!(content, "0.3 w").unwrap();
            writeln!(content, "{:.2} {:.2} 20 15 re S", x - 10.0, y - 7.5).unwrap();
            
            // Draw reference
            writeln!(content, "BT").unwrap();
            writeln!(content, "/F1 8 Tf").unwrap();
            writeln!(content, "{:.2} {:.2} Td", x - 8.0, y + 10.0).unwrap();
            writeln!(content, "({}) Tj", self.escape_pdf_string(&symbol.reference)).unwrap();
            writeln!(content, "ET").unwrap();
            
            // Draw value
            writeln!(content, "BT").unwrap();
            writeln!(content, "/F1 6 Tf").unwrap();
            writeln!(content, "{:.2} {:.2} Td", x - 8.0, y - 12.0).unwrap();
            writeln!(content, "({}) Tj", self.escape_pdf_string(&symbol.value)).unwrap();
            writeln!(content, "ET").unwrap();
        }

        // Draw labels
        for label in &sheet.labels {
            let x = offset_x + label.position.x * scale;
            let y = offset_y - label.position.y * scale;
            
            writeln!(content, "BT").unwrap();
            writeln!(content, "/F1 8 Tf").unwrap();
            writeln!(content, "{:.2} {:.2} Td", x, y).unwrap();
            writeln!(content, "({}) Tj", self.escape_pdf_string(&label.name)).unwrap();
            writeln!(content, "ET").unwrap();
        }

        writeln!(content, "Q").unwrap();
    }

    /// Draw layout content.
    fn draw_layout_content(&self, content: &mut String, layout: &Layout, layers: &[String]) {
        let scale = 2.835;  // mm to points
        let offset_x = self.margin + 20.0;
        let offset_y = self.page_height - self.margin - 20.0;

        writeln!(content, "q").unwrap();

        // Draw traces on specified layers
        for trace in &layout.traces {
            if layers.is_empty() || layers.contains(&trace.layer) {
                let x1 = offset_x + trace.start.x * scale;
                let y1 = offset_y - trace.start.y * scale;
                let x2 = offset_x + trace.end.x * scale;
                let y2 = offset_y - trace.end.y * scale;
                let width = trace.width * scale;
                
                writeln!(content, "{:.2} w", width).unwrap();
                writeln!(content, "0 0.5 0 RG").unwrap();  // Green for copper
                writeln!(content, "{:.2} {:.2} m {:.2} {:.2} l S", x1, y1, x2, y2).unwrap();
            }
        }

        // Draw vias
        writeln!(content, "0.5 w").unwrap();
        for via in &layout.vias {
            let x = offset_x + via.position.x * scale;
            let y = offset_y - via.position.y * scale;
            let r = via.pad * scale / 2.0;
            
            // Outer circle
            writeln!(content, "0 0 0 RG").unwrap();
            self.draw_circle(content, x, y, r);
            
            // Inner circle (drill)
            let drill_r = via.drill * scale / 2.0;
            writeln!(content, "1 1 1 rg").unwrap();
            self.draw_filled_circle(content, x, y, drill_r);
        }

        // Draw components
        for component in &layout.components {
            let x = offset_x + component.position.x * scale;
            let y = offset_y - component.position.y * scale;
            
            // Draw component outline
            writeln!(content, "0.3 w").unwrap();
            writeln!(content, "0 0 0 RG").unwrap();
            writeln!(content, "{:.2} {:.2} 15 10 re S", x - 7.5, y - 5.0).unwrap();
            
            // Draw reference
            writeln!(content, "BT").unwrap();
            writeln!(content, "/F1 6 Tf").unwrap();
            writeln!(content, "{:.2} {:.2} Td", x - 6.0, y - 2.0).unwrap();
            writeln!(content, "({}) Tj", self.escape_pdf_string(&component.reference)).unwrap();
            writeln!(content, "ET").unwrap();
        }

        writeln!(content, "Q").unwrap();
    }

    /// Draw a circle (stroke only).
    fn draw_circle(&self, content: &mut String, x: f64, y: f64, r: f64) {
        // Approximate circle with bezier curves
        let k = 0.5522847498;  // Magic number for circle approximation
        let kr = k * r;
        
        writeln!(content, "{:.2} {:.2} m", x + r, y).unwrap();
        writeln!(content, "{:.2} {:.2} {:.2} {:.2} {:.2} {:.2} c", 
            x + r, y + kr, x + kr, y + r, x, y + r).unwrap();
        writeln!(content, "{:.2} {:.2} {:.2} {:.2} {:.2} {:.2} c", 
            x - kr, y + r, x - r, y + kr, x - r, y).unwrap();
        writeln!(content, "{:.2} {:.2} {:.2} {:.2} {:.2} {:.2} c", 
            x - r, y - kr, x - kr, y - r, x, y - r).unwrap();
        writeln!(content, "{:.2} {:.2} {:.2} {:.2} {:.2} {:.2} c S", 
            x + kr, y - r, x + r, y - kr, x + r, y).unwrap();
    }

    /// Draw a filled circle.
    fn draw_filled_circle(&self, content: &mut String, x: f64, y: f64, r: f64) {
        let k = 0.5522847498;
        let kr = k * r;
        
        writeln!(content, "{:.2} {:.2} m", x + r, y).unwrap();
        writeln!(content, "{:.2} {:.2} {:.2} {:.2} {:.2} {:.2} c", 
            x + r, y + kr, x + kr, y + r, x, y + r).unwrap();
        writeln!(content, "{:.2} {:.2} {:.2} {:.2} {:.2} {:.2} c", 
            x - kr, y + r, x - r, y + kr, x - r, y).unwrap();
        writeln!(content, "{:.2} {:.2} {:.2} {:.2} {:.2} {:.2} c", 
            x - r, y - kr, x - kr, y - r, x, y - r).unwrap();
        writeln!(content, "{:.2} {:.2} {:.2} {:.2} {:.2} {:.2} c f", 
            x + kr, y - r, x + r, y - kr, x + r, y).unwrap();
    }

    /// Escape special characters for PDF strings.
    fn escape_pdf_string(&self, s: &str) -> String {
        s.replace('\\', "\\\\")
         .replace('(', "\\(")
         .replace(')', "\\)")
    }
}

/// Simple PDF document builder.
struct PdfDocument {
    objects: Vec<String>,
    pages: Vec<usize>,
    page_contents: Vec<(usize, String)>,
    title: Option<String>,
    author: Option<String>,
}

impl PdfDocument {
    fn new() -> Self {
        Self {
            objects: Vec::new(),
            pages: Vec::new(),
            page_contents: Vec::new(),
            title: None,
            author: None,
        }
    }

    fn set_title(&mut self, title: &str) {
        self.title = Some(title.to_string());
    }

    fn set_author(&mut self, author: &str) {
        self.author = Some(author.to_string());
    }

    fn add_page(&mut self, width: f64, height: f64) -> usize {
        let page_id = self.pages.len();
        self.pages.push(page_id);
        self.objects.push(format!(
            "<< /Type /Page /MediaBox [0 0 {:.2} {:.2}] >>",
            width, height
        ));
        page_id
    }

    fn set_page_content(&mut self, page_id: usize, content: &str) {
        self.page_contents.push((page_id, content.to_string()));
    }

    fn generate(&self) -> Vec<u8> {
        let mut output = String::new();
        let mut xref_offsets = Vec::new();

        // Header
        writeln!(output, "%PDF-1.4").unwrap();
        // Binary marker comment (indicates binary content)
        writeln!(output, "%PDF Binary").unwrap();

        // Object 1: Catalog
        xref_offsets.push(output.len());
        writeln!(output, "1 0 obj").unwrap();
        writeln!(output, "<< /Type /Catalog /Pages 2 0 R >>").unwrap();
        writeln!(output, "endobj").unwrap();

        // Object 2: Pages
        xref_offsets.push(output.len());
        writeln!(output, "2 0 obj").unwrap();
        let page_refs: Vec<String> = (0..self.pages.len())
            .map(|i| format!("{} 0 R", i + 4))
            .collect();
        writeln!(output, "<< /Type /Pages /Kids [{}] /Count {} >>", 
            page_refs.join(" "), self.pages.len()).unwrap();
        writeln!(output, "endobj").unwrap();

        // Object 3: Font
        xref_offsets.push(output.len());
        writeln!(output, "3 0 obj").unwrap();
        writeln!(output, "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>").unwrap();
        writeln!(output, "endobj").unwrap();

        // Pages and their content streams
        let mut obj_num = 4;
        for (i, _) in self.pages.iter().enumerate() {
            let content_obj = obj_num + 1;
            
            // Page object
            xref_offsets.push(output.len());
            writeln!(output, "{} 0 obj", obj_num).unwrap();
            writeln!(output, "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 841.89 595.28] /Contents {} 0 R /Resources << /Font << /F1 3 0 R >> >> >>", content_obj).unwrap();
            writeln!(output, "endobj").unwrap();
            obj_num += 1;

            // Content stream
            let content = self.page_contents.iter()
                .find(|(id, _)| *id == i)
                .map(|(_, c)| c.as_str())
                .unwrap_or("");
            
            xref_offsets.push(output.len());
            writeln!(output, "{} 0 obj", obj_num).unwrap();
            writeln!(output, "<< /Length {} >>", content.len()).unwrap();
            writeln!(output, "stream").unwrap();
            write!(output, "{}", content).unwrap();
            writeln!(output, "endstream").unwrap();
            writeln!(output, "endobj").unwrap();
            obj_num += 1;
        }

        // Info object
        let info_obj = obj_num;
        xref_offsets.push(output.len());
        writeln!(output, "{} 0 obj", obj_num).unwrap();
        let mut info = String::from("<< ");
        if let Some(ref title) = self.title {
            write!(info, "/Title ({}) ", title).unwrap();
        }
        if let Some(ref author) = self.author {
            write!(info, "/Author ({}) ", author).unwrap();
        }
        write!(info, "/Producer (HardwareTool) >>").unwrap();
        writeln!(output, "{}", info).unwrap();
        writeln!(output, "endobj").unwrap();
        obj_num += 1;

        // Cross-reference table
        let xref_start = output.len();
        writeln!(output, "xref").unwrap();
        writeln!(output, "0 {}", obj_num).unwrap();
        writeln!(output, "0000000000 65535 f ").unwrap();
        for offset in &xref_offsets {
            writeln!(output, "{:010} 00000 n ", offset).unwrap();
        }

        // Trailer
        writeln!(output, "trailer").unwrap();
        writeln!(output, "<< /Size {} /Root 1 0 R /Info {} 0 R >>", obj_num, info_obj).unwrap();
        writeln!(output, "startxref").unwrap();
        writeln!(output, "{}", xref_start).unwrap();
        writeln!(output, "%%EOF").unwrap();

        output.into_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schematic::SchematicSheet;
    use crate::layout::Layout;

    #[test]
    fn test_pdf_generator_new() {
        let generator = PdfGenerator::new("Test Schematic");
        assert!(generator.page_width > 0.0);
        assert!(generator.page_height > 0.0);
    }

    #[test]
    fn test_pdf_page_sizes() {
        let (w, h) = PageSize::A4.dimensions();
        assert!((w - 595.28).abs() < 0.1);
        assert!((h - 841.89).abs() < 0.1);

        let (w, h) = PageSize::Letter.dimensions();
        assert!((w - 612.0).abs() < 0.1);
        assert!((h - 792.0).abs() < 0.1);
    }

    #[test]
    fn test_pdf_export_options_default() {
        let options = PdfExportOptions::default();
        assert!(options.landscape);
        assert!(!options.black_and_white);
        assert!((options.scale - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_generate_schematic_pdf() {
        let sheet = SchematicSheet::new("Test Sheet");
        let generator = PdfGenerator::new("Test Schematic");
        let pdf = generator.generate_schematic(&sheet);
        
        // Check PDF header
        assert!(pdf.starts_with(b"%PDF-1.4"));
        // Check PDF footer
        let pdf_str = String::from_utf8_lossy(&pdf);
        assert!(pdf_str.contains("%%EOF"));
    }

    #[test]
    fn test_generate_layout_pdf() {
        let layout = Layout::new();
        let generator = PdfGenerator::new("Test Layout");
        let pdf = generator.generate_layout(&layout, &[]);
        
        assert!(pdf.starts_with(b"%PDF-1.4"));
    }

    #[test]
    fn test_pdf_with_author() {
        let sheet = SchematicSheet::new("Test");
        let generator = PdfGenerator::new("Test").with_author("Test Author");
        let pdf = generator.generate_schematic(&sheet);
        
        let pdf_str = String::from_utf8_lossy(&pdf);
        assert!(pdf_str.contains("Test Author"));
    }
}
