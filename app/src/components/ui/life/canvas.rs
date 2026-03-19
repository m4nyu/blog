use super::game::Universe;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

#[derive(Clone, Copy, Debug)]
pub struct RgbaColor {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl RgbaColor {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn to_bytes(self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

// Theme colors matching global.css HSL values:
// Light: foreground = hsl(0 0% 3.9%) ~= rgb(10,10,10), background = hsl(0 0% 100%) = rgb(255,255,255)
// Dark:  foreground = hsl(0 0% 98%)  ~= rgb(250,250,250), background = hsl(0 0% 3.9%) ~= rgb(10,10,10)
const LIGHT_ALIVE: RgbaColor = RgbaColor::new(10, 10, 10, 255);
const LIGHT_DEAD: RgbaColor = RgbaColor::new(255, 255, 255, 255);
const DARK_ALIVE: RgbaColor = RgbaColor::new(250, 250, 250, 255);
const DARK_DEAD: RgbaColor = RgbaColor::new(10, 10, 10, 255);

#[derive(Clone, Debug)]
pub struct CanvasConfig {
    pub cell_size: f64,
    pub alive_color: RgbaColor,
    pub dead_color: RgbaColor,
}

impl CanvasConfig {
    pub fn for_theme(cell_size: f64, is_dark: bool) -> Self {
        if is_dark {
            Self {
                cell_size,
                alive_color: DARK_ALIVE,
                dead_color: DARK_DEAD,
            }
        } else {
            Self {
                cell_size,
                alive_color: LIGHT_ALIVE,
                dead_color: LIGHT_DEAD,
            }
        }
    }
}

impl Default for CanvasConfig {
    fn default() -> Self {
        Self::for_theme(12.0, false)
    }
}

pub struct CanvasRenderer {
    config: CanvasConfig,
    pixel_buffer: Vec<u8>,
    buffer_width: u32,
    buffer_height: u32,
}

impl CanvasRenderer {
    pub fn new(config: CanvasConfig) -> Self {
        Self {
            config,
            pixel_buffer: Vec::new(),
            buffer_width: 0,
            buffer_height: 0,
        }
    }

    pub fn update_theme(&mut self, is_dark: bool) {
        if is_dark {
            self.config.alive_color = DARK_ALIVE;
            self.config.dead_color = DARK_DEAD;
        } else {
            self.config.alive_color = LIGHT_ALIVE;
            self.config.dead_color = LIGHT_DEAD;
        }
    }

    pub fn cell_size(&self) -> f64 {
        self.config.cell_size
    }

    pub fn draw(
        &mut self,
        ctx: &CanvasRenderingContext2d,
        universe: &Universe,
        canvas_width: u32,
        canvas_height: u32,
    ) {
        let w = canvas_width;
        let h = canvas_height;

        // Resize pixel buffer if dimensions changed
        if w != self.buffer_width || h != self.buffer_height {
            let size = (w as usize) * (h as usize) * 4;
            self.pixel_buffer.resize(size, 0);
            self.buffer_width = w;
            self.buffer_height = h;
        }

        // Fill entire buffer with dead color
        let dead_bytes = self.config.dead_color.to_bytes();
        for chunk in self.pixel_buffer.chunks_exact_mut(4) {
            chunk[0] = dead_bytes[0];
            chunk[1] = dead_bytes[1];
            chunk[2] = dead_bytes[2];
            chunk[3] = dead_bytes[3];
        }

        // For each alive cell, fill its cell_size x cell_size pixel rectangle
        let cell_size = self.config.cell_size as u32;
        let alive_bytes = self.config.alive_color.to_bytes();

        for row in 0..universe.height() {
            for col in 0..universe.width() {
                if !universe.get_cell(row, col).is_alive() {
                    continue;
                }

                let px_start = col as u32 * cell_size;
                let py_start = row as u32 * cell_size;

                let px_end = (px_start + cell_size).min(w);
                let py_end = (py_start + cell_size).min(h);

                for py in py_start..py_end {
                    for px in px_start..px_end {
                        let offset = ((py * w + px) * 4) as usize;
                        if offset + 3 < self.pixel_buffer.len() {
                            self.pixel_buffer[offset] = alive_bytes[0];
                            self.pixel_buffer[offset + 1] = alive_bytes[1];
                            self.pixel_buffer[offset + 2] = alive_bytes[2];
                            self.pixel_buffer[offset + 3] = alive_bytes[3];
                        }
                    }
                }
            }
        }

        // Single FFI call: create ImageData from pixel buffer and blit to canvas
        if let Ok(image_data) = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(self.pixel_buffer.as_mut_slice()),
            w,
            h,
        ) {
            let _ = ctx.put_image_data(&image_data, 0.0, 0.0);
        }
    }
}
