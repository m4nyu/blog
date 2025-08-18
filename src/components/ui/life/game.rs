use super::cell::Cell;
#[cfg(feature = "hydrate")]
use super::cell::CellState;

#[cfg(feature = "hydrate")]
use js_sys::Math;

// Leptos component imports
use leptos::*;

#[cfg(feature = "hydrate")]
use super::{CanvasConfig, CanvasRenderer, get_theme_colors};
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
#[cfg(feature = "hydrate")]
use leptos_dom::helpers::IntervalHandle;

#[derive(Clone, Debug)]
pub struct Universe {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Universe {
    pub fn new(width: usize, height: usize) -> Self {
        let cells = vec![Cell::default(); width * height];
        Self {
            width,
            height,
            cells,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn cells(&self) -> &[Cell] {
        &self.cells
    }

    pub fn cells_mut(&mut self) -> &mut [Cell] {
        &mut self.cells
    }

    fn get_index(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }

    pub fn get_cell(&self, row: usize, col: usize) -> Cell {
        if row < self.height && col < self.width {
            let idx = self.get_index(row, col);
            self.cells[idx]
        } else {
            Cell::dead()
        }
    }

    pub fn set_cell(&mut self, row: usize, col: usize, cell: Cell) {
        if row < self.height && col < self.width {
            let idx = self.get_index(row, col);
            self.cells[idx] = cell;
        }
    }

    pub fn toggle_cell(&mut self, row: usize, col: usize) {
        if row < self.height && col < self.width {
            let idx = self.get_index(row, col);
            self.cells[idx].toggle();
        }
    }

    pub fn set_cells(&mut self, cells: &[(usize, usize)]) {
        for &(row, col) in cells {
            self.set_cell(row, col, Cell::alive());
        }
    }

    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            cell.set_dead();
        }
    }

    #[allow(dead_code)]
    fn live_neighbor_count(&self, row: usize, col: usize) -> u8 {
        let mut count = 0;

        // Use wrapping arithmetic for toroidal topology
        let deltas = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),           (0, 1),
            (1, -1),  (1, 0),  (1, 1),
        ];

        for &(dr, dc) in &deltas {
            let neighbor_row = (row as isize + dr + self.height as isize) as usize % self.height;
            let neighbor_col = (col as isize + dc + self.width as isize) as usize % self.width;
            
            if self.get_cell(neighbor_row, neighbor_col).is_alive() {
                count += 1;
            }
        }

        count
    }

    pub fn tick(&mut self) {
        // HYPER-OPTIMIZED tick with pre-allocated buffer and batch operations
        static mut TEMP_BUFFER: Vec<Cell> = Vec::new();
        
        unsafe {
            // Reuse buffer to avoid allocations (major performance gain)
            let temp_buffer = &raw mut TEMP_BUFFER;
            if (&*temp_buffer).len() != self.cells.len() {
                (&mut *temp_buffer).resize(self.cells.len(), Cell::default());
            }
            
            // Batch copy current state to temp buffer
            (&mut *temp_buffer).copy_from_slice(&self.cells);
            
            // ULTRA-FAST computation with optimized loops
            let width = self.width;
            let height = self.height;
            
            // Process in cache-friendly order with minimal function calls
            for row in 0..height {
                let row_offset = row * width;
                
                for col in 0..width {
                    let idx = row_offset + col;
                    let is_alive = self.cells[idx].is_alive();
                    
                    // OPTIMIZED neighbor counting with bounds checking eliminated
                    let mut live_neighbors = 0u8;
                    
                    // Unrolled neighbor loop for maximum performance
                    if row > 0 {
                        if col > 0 && self.cells[idx - width - 1].is_alive() { live_neighbors += 1; }
                        if self.cells[idx - width].is_alive() { live_neighbors += 1; }
                        if col < width - 1 && self.cells[idx - width + 1].is_alive() { live_neighbors += 1; }
                    }
                    
                    if col > 0 && self.cells[idx - 1].is_alive() { live_neighbors += 1; }
                    if col < width - 1 && self.cells[idx + 1].is_alive() { live_neighbors += 1; }
                    
                    if row < height - 1 {
                        if col > 0 && self.cells[idx + width - 1].is_alive() { live_neighbors += 1; }
                        if self.cells[idx + width].is_alive() { live_neighbors += 1; }
                        if col < width - 1 && self.cells[idx + width + 1].is_alive() { live_neighbors += 1; }
                    }
                    
                    // Optimized state transition with lookup table approach
                    let next_alive = match (is_alive, live_neighbors) {
                        (true, 2) | (true, 3) => true,  // Survive
                        (false, 3) => true,             // Birth
                        _ => false,                     // Die or stay dead
                    };
                    
                    (&mut *temp_buffer)[idx] = if next_alive { Cell::alive() } else { Cell::dead() };
                }
            }
            
            // Single memory swap (much faster than clone)
            std::mem::swap(&mut self.cells, &mut *temp_buffer);
        }
    }

    #[cfg(feature = "hydrate")]
    pub fn randomize(&mut self, probability: f64) {
        let mut alive_count = 0;
        let total_cells = self.width * self.height;
        
        // Fast randomization
        for cell in self.cells.iter_mut() {
            if Math::random() < probability {
                cell.set_state(CellState::Alive);
                alive_count += 1;
            } else {
                cell.set_state(CellState::Dead);
            }
        }
        
        // Ensure minimum population for visual interest
        if alive_count < 3 && total_cells > 10 {
            let min_cells = (total_cells / 50).max(3).min(10);
            for _ in alive_count..min_cells {
                let idx = (Math::random() * total_cells as f64) as usize;
                if idx < self.cells.len() {
                    self.cells[idx].set_state(CellState::Alive);
                    alive_count += 1;
                }
            }
        }
    }

    #[cfg(not(feature = "hydrate"))]
    pub fn randomize(&mut self, _probability: f64) {
        // No-op for SSR - could use a deterministic pattern instead
        self.add_glider(5, 5);
        self.add_blinker(15, 15);
    }

    // Predefined patterns
    pub fn add_glider(&mut self, start_row: usize, start_col: usize) {
        let pattern = [
            (0, 1),
            (1, 2),
            (2, 0), (2, 1), (2, 2)
        ];
        
        for &(dr, dc) in &pattern {
            let row = start_row + dr;
            let col = start_col + dc;
            if row < self.height && col < self.width {
                self.set_cell(row, col, Cell::alive());
            }
        }
    }

    pub fn add_blinker(&mut self, start_row: usize, start_col: usize) {
        let pattern = [(0, 0), (0, 1), (0, 2)];
        
        for &(dr, dc) in &pattern {
            let row = start_row + dr;
            let col = start_col + dc;
            if row < self.height && col < self.width {
                self.set_cell(row, col, Cell::alive());
            }
        }
    }

    pub fn add_toad(&mut self, start_row: usize, start_col: usize) {
        let pattern = [
            (0, 1), (0, 2), (0, 3),
            (1, 0), (1, 1), (1, 2)
        ];
        
        for &(dr, dc) in &pattern {
            let row = start_row + dr;
            let col = start_col + dc;
            if row < self.height && col < self.width {
                self.set_cell(row, col, Cell::alive());
            }
        }
    }

    pub fn add_beacon(&mut self, start_row: usize, start_col: usize) {
        let pattern = [
            (0, 0), (0, 1),
            (1, 0), (1, 1),
            (2, 2), (2, 3),
            (3, 2), (3, 3)
        ];
        
        for &(dr, dc) in &pattern {
            let row = start_row + dr;
            let col = start_col + dc;
            if row < self.height && col < self.width {
                self.set_cell(row, col, Cell::alive());
            }
        }
    }

    pub fn add_pulsar(&mut self, start_row: usize, start_col: usize) {
        let pattern = [
            // Top
            (2, 4), (2, 5), (2, 6), (2, 10), (2, 11), (2, 12),
            (4, 2), (4, 7), (4, 9), (4, 14),
            (5, 2), (5, 7), (5, 9), (5, 14),
            (6, 2), (6, 7), (6, 9), (6, 14),
            (7, 4), (7, 5), (7, 6), (7, 10), (7, 11), (7, 12),
            // Bottom
            (9, 4), (9, 5), (9, 6), (9, 10), (9, 11), (9, 12),
            (10, 2), (10, 7), (10, 9), (10, 14),
            (11, 2), (11, 7), (11, 9), (11, 14),
            (12, 2), (12, 7), (12, 9), (12, 14),
            (14, 4), (14, 5), (14, 6), (14, 10), (14, 11), (14, 12),
        ];
        
        for &(dr, dc) in &pattern {
            let row = start_row + dr;
            let col = start_col + dc;
            if row < self.height && col < self.width {
                self.set_cell(row, col, Cell::alive());
            }
        }
    }

    pub fn is_stable(&self, previous: &Universe) -> bool {
        if self.width != previous.width || self.height != previous.height {
            return false;
        }
        
        self.cells.iter().zip(previous.cells.iter())
            .all(|(current, prev)| current.state == prev.state)
    }

    pub fn count_living_cells(&self) -> usize {
        self.cells.iter().filter(|cell| cell.is_alive()).count()
    }

    #[cfg(feature = "hydrate")]
    pub fn resize_and_redistribute(&mut self, new_width: usize, new_height: usize, density: f64) {
        if self.width == new_width && self.height == new_height {
            return; // No change needed
        }

        // Fast resize: just clear and regenerate
        self.width = new_width;
        self.height = new_height;
        self.cells = vec![Cell::dead(); new_width * new_height];
        
        // Quick randomization
        self.randomize(density);
        
        // Add guaranteed patterns for visual interest
        if new_width > 15 && new_height > 15 {
            self.add_glider(5, 5);
            if new_width > 30 {
                self.add_blinker(new_width - 10, 8);
            }
        }
    }


    #[cfg(not(feature = "hydrate"))]
    pub fn resize_and_redistribute(&mut self, new_width: usize, new_height: usize, _density: f64) {
        // Simple resize for SSR
        self.width = new_width;
        self.height = new_height;
        self.cells = vec![Cell::dead(); new_width * new_height];
        
        // Add some patterns
        if new_width > 20 && new_height > 20 {
            self.add_glider(5, 5);
            self.add_blinker(15, 15);
        }
    }
}

// Life Component for background animation
#[cfg(feature = "hydrate")]
#[component]
pub fn Life(
    animation_speed: RwSignal<u64>,
    population_density: RwSignal<f64>,
) -> impl IntoView {
    leptos::logging::log!("Life component initializing");
    
    let canvas_ref = NodeRef::<leptos::html::Canvas>::new();
    let universe = RwSignal::new(None::<Universe>);
    let viewport_size = RwSignal::new((2560, 1440)); // Default size
    
    // Fixed cell size - bigger pixels
    let cell_size = 12.0;
    
    // Create canvas config and renderer using the existing canvas module
    let (bg_color, cell_color) = get_theme_colors();
    leptos::logging::log!("Theme colors - bg: {}, cell: {}", bg_color, cell_color);
    let config = CanvasConfig::with_colors(cell_size, &cell_color, &bg_color);
    let renderer = CanvasRenderer::new(config);
    
    // Grid size calculation
    let calculate_grid_size = move || -> (usize, usize) {
        if let Some(window) = web_sys::window() {
            let width = window.inner_width().unwrap().as_f64().unwrap();
            let height = window.inner_height().unwrap().as_f64().unwrap();
            
            let grid_width = (width / cell_size).ceil() as usize;
            let grid_height = (height / cell_size).ceil() as usize;
            
            viewport_size.set((width as u32, height as u32));
            
            (grid_width, grid_height)
        } else {
            (100, 60)
        }
    };
    
    // Universe initialization - one time only
    create_effect(move |_| {
        if universe.get_untracked().is_none() {
            let (grid_width, grid_height) = calculate_grid_size();
            let density = population_density.get_untracked();
            
            leptos::logging::log!("Initializing universe: {}x{} with density {}", grid_width, grid_height, density);
            
            let mut new_universe = Universe::new(grid_width, grid_height);
            new_universe.randomize(density);
            
            // Add some guaranteed patterns for visual interest
            new_universe.add_glider(5, 5);
            new_universe.add_blinker(10, 8);
            
            universe.set(Some(new_universe));
            leptos::logging::log!("Universe initialized successfully");
        }
    });
    
    // Window resize handling
    create_effect(move |_| {
        if let Some(window) = web_sys::window() {
            let resize_handler = wasm_bindgen::closure::Closure::wrap(Box::new(move |_event: web_sys::Event| {
                let (new_grid_width, new_grid_height) = calculate_grid_size();
                let density = population_density.get_untracked();
                
                universe.update(|u| {
                    if let Some(ref mut universe_ref) = u {
                        universe_ref.resize_and_redistribute(new_grid_width, new_grid_height, density);
                    }
                });
            }) as Box<dyn FnMut(_)>);
            
            let _ = window.add_event_listener_with_callback("resize", resize_handler.as_ref().unchecked_ref());
            resize_handler.forget();
        }
    });
    
    // Density change tracking
    create_effect(move |previous_density: Option<f64>| {
        let current_density = population_density.get();
        
        // Skip the first initialization run
        if let Some(prev_density) = previous_density {
            if current_density != prev_density && universe.get_untracked().is_some() {
                universe.update(|u| {
                    if let Some(ref mut universe_ref) = u {
                        universe_ref.clear();
                        universe_ref.randomize(current_density);
                        universe_ref.add_glider(5, 5);
                        universe_ref.add_blinker(10, 8);
                    }
                });
            }
        }
        
        current_density
    });
    
    // Animation with speed control
    let animation_handle = RwSignal::new(None::<IntervalHandle>);
    
    let restart_animation = move || {
        // Clear existing animation
        animation_handle.update(|handle| {
            if let Some(h) = handle.take() {
                h.clear();
            }
        });
        
        // Start new animation with current speed
        let speed = animation_speed.get_untracked();
        
        let new_handle = leptos::set_interval_with_handle(
            move || {
                universe.update(|u| {
                    if let Some(ref mut u) = u {
                        u.tick();
                    }
                });
            },
            std::time::Duration::from_millis(speed),
        ).expect("Failed to create animation");
        
        animation_handle.set(Some(new_handle));
    };
    
    // Watch for speed changes
    create_effect(move |_| {
        let _speed = animation_speed.get();
        restart_animation();
    });
    
    // Initial animation start
    restart_animation();
    
    // Track theme changes and update renderer
    let theme_signal = RwSignal::new(false);
    let renderer_signal = RwSignal::new(renderer);
    
    // Check theme periodically and update renderer colors
    let _theme_interval = leptos::set_interval_with_handle(
        move || {
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(element) = document.document_element() {
                        let is_dark = element.class_list().contains("dark");
                        if theme_signal.get_untracked() != is_dark {
                            theme_signal.set(is_dark);
                            
                            // Update renderer with new theme colors
                            renderer_signal.update(|r| {
                                r.update_theme(is_dark);
                            });
                        }
                    }
                }
            }
        },
        std::time::Duration::from_millis(500),
    ).expect("Failed to create theme check interval");
    
    // Canvas drawing using the CanvasRenderer - redraw when content changes
    create_effect(move |_| {
        // Track universe, theme, and viewport changes
        universe.track();
        theme_signal.track();
        viewport_size.track();
        
        leptos::logging::log!("Canvas drawing effect triggered");
        
        if let Some(canvas) = canvas_ref.get() {
            let canvas_el: HtmlCanvasElement = (*canvas).clone().unchecked_into();
            leptos::logging::log!("Canvas element found: {}x{}", canvas_el.width(), canvas_el.height());
            
            // Get universe WITH tracking so we re-render on changes
            if let Some(current_universe) = universe.get() {
                leptos::logging::log!("Drawing universe: {}x{}", current_universe.width(), current_universe.height());
                
                // Get current viewport size
                let (canvas_width, canvas_height) = viewport_size.get();
                
                // Set canvas dimensions to match viewport
                canvas_el.set_width(canvas_width);
                canvas_el.set_height(canvas_height);
                
                // Use the CanvasRenderer to draw the universe
                if let Ok(Some(ctx)) = canvas_el.get_context("2d") {
                    if let Ok(ctx) = ctx.dyn_into::<CanvasRenderingContext2d>() {
                        let current_renderer = renderer_signal.get_untracked();
                        
                        // Use the CanvasRenderer setup_canvas method first
                        current_renderer.setup_canvas(&canvas_el, &current_universe);
                        
                        // Clear canvas with background color
                        ctx.clear_rect(0.0, 0.0, canvas_width as f64, canvas_height as f64);
                        
                        // Draw directly using the current universe 
                        current_renderer.draw(&ctx, &current_universe);
                        leptos::logging::log!("Canvas drawing completed");
                    } else {
                        leptos::logging::log!("Failed to get 2D context");
                    }
                } else {
                    leptos::logging::log!("Failed to get canvas context");
                }
            } else {
                leptos::logging::log!("No universe available for drawing");
            }
        } else {
            leptos::logging::log!("Canvas element not available");
        }
    });
    
    // Mouse handler for clicking to add cells
    create_effect(move |_| {
        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            let mouse_handler = wasm_bindgen::closure::Closure::wrap(Box::new({
                let universe = universe.clone();
                move |event: web_sys::MouseEvent| {
                    if let Some(current_universe) = universe.get_untracked() {
                        let mouse_x = event.client_x() as f64;
                        let mouse_y = event.client_y() as f64;
                        
                        // Convert mouse position to grid coordinates
                        let col = (mouse_x / cell_size) as usize;
                        let row = (mouse_y / cell_size) as usize;
                        
                        if row < current_universe.height() && col < current_universe.width() {
                            universe.update(|u| {
                                if let Some(ref mut universe) = u {
                                    universe.set_cell(row, col, Cell::alive());
                                }
                            });
                        }
                    }
                }
            }) as Box<dyn FnMut(_)>);
            
            let _ = document.add_event_listener_with_callback("mousemove", mouse_handler.as_ref().unchecked_ref());
            mouse_handler.forget();
        }
    });
    
    view! {
        <div class="fixed inset-0 w-screen h-screen z-0 pointer-events-none">
            <canvas
                node_ref=canvas_ref
                class="w-full h-full block bg-background"
            />
        </div>
    }
}

// SSR version (no-op)
#[cfg(not(feature = "hydrate"))]
#[component]
pub fn Life(
    animation_speed: RwSignal<u64>,
    population_density: RwSignal<f64>,
) -> impl IntoView {
    let _ = animation_speed; // Suppress unused warning  
    let _ = population_density; // Suppress unused warning
    view! {
        <div class="fixed inset-0 w-screen h-screen z-0 pointer-events-none">
            <canvas
                width="2560"
                height="1440"
                class="w-full h-full block bg-background"
            />
        </div>
    }
}