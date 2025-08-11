use super::game::Universe;

#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[derive(Clone, Debug)]
pub struct CanvasConfig {
    pub cell_size: f64,
    pub gap_size: f64,
    pub alive_color: String,
    pub dead_color: String,
    pub grid_color: String,
    pub show_grid: bool,
}

impl Default for CanvasConfig {
    fn default() -> Self {
        Self {
            cell_size: 12.0,
            gap_size: 0.0,
            alive_color: "#374151".to_string(),
            dead_color: "#f9fafb".to_string(),
            grid_color: "#e5e7eb".to_string(),
            show_grid: false,
        }
    }
}

impl CanvasConfig {
    pub fn new(cell_size: f64) -> Self {
        Self {
            cell_size,
            ..Default::default()
        }
    }

    pub fn with_colors(cell_size: f64, alive_color: &str, dead_color: &str) -> Self {
        Self {
            cell_size,
            alive_color: alive_color.to_string(),
            dead_color: dead_color.to_string(),
            ..Default::default()
        }
    }

    pub fn dark_theme() -> Self {
        Self {
            alive_color: "#ffffff".to_string(),
            dead_color: "#000000".to_string(),
            grid_color: "#333333".to_string(),
            ..Default::default()
        }
    }

    pub fn light_theme() -> Self {
        Self {
            alive_color: "#374151".to_string(),
            dead_color: "#f9fafb".to_string(),
            grid_color: "#e5e7eb".to_string(),
            ..Default::default()
        }
    }

    pub fn canvas_width(&self, universe_width: usize) -> f64 {
        universe_width as f64 * self.cell_size
    }

    pub fn canvas_height(&self, universe_height: usize) -> f64 {
        universe_height as f64 * self.cell_size
    }
}

#[derive(Clone)]
#[cfg(feature = "hydrate")]
pub struct CanvasRenderer {
    config: CanvasConfig,
}

#[cfg(feature = "hydrate")]
impl CanvasRenderer {
    pub fn new(config: CanvasConfig) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &CanvasConfig {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut CanvasConfig {
        &mut self.config
    }

    pub fn update_theme(&mut self, is_dark: bool) {
        #[cfg(feature = "hydrate")]
        {
            use web_sys::window;
            
            // Try to read theme colors from CSS variables
            if let Some(window) = window() {
                if let Some(document) = window.document() {
                    if let Some(element) = document.document_element() {
                        if let Ok(styles) = window.get_computed_style(&element) {
                            if let Some(styles) = styles {
                                let fg_color = styles.get_property_value("--color-foreground").unwrap_or_default();
                                let border_color = styles.get_property_value("--color-border").unwrap_or_default();
                                
                                if !fg_color.is_empty() {
                                    self.config.alive_color = fg_color;
                                    
                                    if is_dark {
                                        self.config.dead_color = "hsl(0 0% 0%)".to_string(); // Force pure black
                                    } else {
                                        self.config.dead_color = "hsl(0 0% 100%)".to_string(); // Force pure white
                                    }
                                    
                                    if !border_color.is_empty() {
                                        self.config.grid_color = border_color;
                                    }
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Fallback if CSS variables can't be read
        if is_dark {
            self.config.alive_color = "hsl(0 0% 98%)".to_string();
            self.config.dead_color = "hsl(0 0% 0%)".to_string();
            self.config.grid_color = "hsl(0 0% 14.9%)".to_string();
        } else {
            self.config.alive_color = "hsl(0 0% 3.9%)".to_string();
            self.config.dead_color = "hsl(0 0% 100%)".to_string();
            self.config.grid_color = "hsl(0 0% 89.8%)".to_string();
        }
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d, universe: &Universe) {
        let canvas_width = self.config.canvas_width(universe.width());
        let canvas_height = self.config.canvas_height(universe.height());

        // Clear canvas with background color
        ctx.set_fill_style_str(&self.config.dead_color);
        ctx.fill_rect(0.0, 0.0, canvas_width, canvas_height);

        // Draw grid if enabled
        if self.config.show_grid {
            self.draw_grid(ctx, universe);
        }

        // Draw living cells
        ctx.set_fill_style_str(&self.config.alive_color);
        self.draw_cells(ctx, universe);
    }

    fn draw_grid(&self, ctx: &CanvasRenderingContext2d, universe: &Universe) {
        ctx.set_stroke_style_str(&self.config.grid_color);
        ctx.set_line_width(0.5);
        ctx.begin_path();

        let canvas_width = self.config.canvas_width(universe.width());
        let canvas_height = self.config.canvas_height(universe.height());

        // Vertical lines
        for i in 0..=universe.width() {
            let x = i as f64 * self.config.cell_size;
            ctx.move_to(x, 0.0);
            ctx.line_to(x, canvas_height);
        }

        // Horizontal lines
        for i in 0..=universe.height() {
            let y = i as f64 * self.config.cell_size;
            ctx.move_to(0.0, y);
            ctx.line_to(canvas_width, y);
        }

        ctx.stroke();
    }

    fn draw_cells(&self, ctx: &CanvasRenderingContext2d, universe: &Universe) {
        for row in 0..universe.height() {
            for col in 0..universe.width() {
                let cell = universe.get_cell(row, col);
                if cell.is_alive() {
                    self.draw_cell(ctx, row, col);
                }
            }
        }
    }

    fn draw_cell(&self, ctx: &CanvasRenderingContext2d, row: usize, col: usize) {
        let x = col as f64 * self.config.cell_size + self.config.gap_size;
        let y = row as f64 * self.config.cell_size + self.config.gap_size;
        let size = self.config.cell_size - self.config.gap_size * 2.0;

        if size > 0.0 {
            ctx.fill_rect(x, y, size, size);
        }
    }

    pub fn canvas_to_grid_coords(&self, canvas_x: f64, canvas_y: f64) -> (usize, usize) {
        let col = (canvas_x / self.config.cell_size) as usize;
        let row = (canvas_y / self.config.cell_size) as usize;
        (row, col)
    }

    pub fn setup_canvas(&self, canvas: &HtmlCanvasElement, universe: &Universe) {
        let canvas_width = self.config.canvas_width(universe.width()) as u32;
        let canvas_height = self.config.canvas_height(universe.height()) as u32;
        
        canvas.set_width(canvas_width);
        canvas.set_height(canvas_height);

        // Set CSS size to match canvas resolution for sharp rendering
        let style = canvas.style();
        let _ = style.set_property("width", &format!("{}px", canvas_width));
        let _ = style.set_property("height", &format!("{}px", canvas_height));
        
        // Disable image smoothing for pixel-perfect rendering
        if let Ok(Some(ctx)) = canvas.get_context("2d") {
            if let Ok(ctx) = ctx.dyn_into::<CanvasRenderingContext2d>() {
                let _ = ctx.set_image_smoothing_enabled(false);
            }
        }
    }
}

#[cfg(feature = "hydrate")]
pub fn get_theme_colors() -> (String, String) {
    use web_sys::window;
    
    if let Some(window) = window() {
        if let Some(document) = window.document() {
            if let Some(element) = document.document_element() {
                // Get computed styles to read CSS custom properties
                if let Ok(styles) = window.get_computed_style(&element) {
                    if let Some(styles) = styles {
                        // Read directly from your theme CSS variables
                        let bg_color = styles.get_property_value("--color-background").unwrap_or_default();
                        let fg_color = styles.get_property_value("--color-foreground").unwrap_or_default();
                        
                        leptos::logging::log!("Reading theme CSS vars - bg: '{}', fg: '{}'", bg_color, fg_color);
                        
                        if !bg_color.is_empty() && !fg_color.is_empty() {
                            let class_list = element.class_list();
                            if class_list.contains("dark") {
                                // Dark mode: force pure black background, use theme foreground for cells
                                return ("hsl(0 0% 0%)".to_string(), fg_color);
                            } else {
                                // Light mode: force pure white background, use theme foreground for cells
                                return ("hsl(0 0% 100%)".to_string(), fg_color);
                            }
                        }
                    }
                }
                
                // Fallback if CSS variables can't be read
                let class_list = element.class_list();
                if class_list.contains("dark") {
                    leptos::logging::log!("Dark mode fallback");
                    return ("hsl(0 0% 0%)".to_string(), "hsl(0 0% 98%)".to_string());
                } else {
                    leptos::logging::log!("Light mode fallback");
                    return ("hsl(0 0% 100%)".to_string(), "hsl(0 0% 3.9%)".to_string());
                }
            }
        }
    }
    
    leptos::logging::log!("Final fallback - light mode");
    ("hsl(0 0% 100%)".to_string(), "hsl(0 0% 3.9%)".to_string())
}

#[cfg(not(feature = "hydrate"))]
pub fn get_theme_colors() -> (String, String) {
    // Light mode: pure white background with theme foreground
    ("hsl(0 0% 100%)".to_string(), "hsl(0 0% 3.9%)".to_string())
}

// Animation utilities
#[cfg(feature = "hydrate")]
pub struct AnimationState {
    pub is_playing: bool,
    pub speed_ms: u32,
    pub generation: usize,
    pub fps: f64,
    last_frame_time: f64,
    frame_count: usize,
}

#[cfg(feature = "hydrate")]
impl AnimationState {
    pub fn new(speed_ms: u32) -> Self {
        Self {
            is_playing: false,
            speed_ms,
            generation: 0,
            fps: 0.0,
            last_frame_time: 0.0,
            frame_count: 0,
        }
    }

    pub fn update_fps(&mut self, current_time: f64) {
        if self.last_frame_time > 0.0 {
            let delta = current_time - self.last_frame_time;
            if delta > 0.0 {
                self.fps = 1000.0 / delta; // Convert to FPS
                self.frame_count += 1;
            }
        }
        self.last_frame_time = current_time;
    }

    pub fn reset_stats(&mut self) {
        self.generation = 0;
        self.fps = 0.0;
        self.frame_count = 0;
        self.last_frame_time = 0.0;
    }

    pub fn next_generation(&mut self) {
        self.generation += 1;
    }
}

// Pattern utilities
pub struct PatternManager;

impl PatternManager {
    pub fn get_pattern_names() -> Vec<&'static str> {
        vec![
            "Empty",
            "Random",
            "Glider",
            "Blinker", 
            "Toad",
            "Beacon",
            "Pulsar",
        ]
    }

    pub fn apply_pattern(universe: &mut Universe, pattern_name: &str) {
        universe.clear();
        
        match pattern_name {
            "Empty" => {},
            "Random" => universe.randomize(0.25),
            "Glider" => {
                universe.add_glider(5, 5);
                universe.add_glider(15, 25);
                universe.add_glider(25, 15);
            },
            "Blinker" => {
                universe.add_blinker(10, 10);
                universe.add_blinker(20, 20);
                universe.add_blinker(30, 30);
            },
            "Toad" => {
                universe.add_toad(10, 10);
                universe.add_toad(20, 20);
            },
            "Beacon" => {
                universe.add_beacon(10, 10);
                universe.add_beacon(20, 20);
            },
            "Pulsar" => {
                universe.add_pulsar(5, 5);
            },
            _ => universe.randomize(0.25),
        }
    }
}