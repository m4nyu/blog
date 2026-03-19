use super::cell::Cell;

use rand::Rng;

// Leptos component imports
use leptos::*;

#[derive(Clone, Debug)]
pub struct Universe {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
    scratch: Vec<Cell>,
}

impl Universe {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Self {
            width,
            height,
            cells: vec![Cell::default(); size],
            scratch: vec![Cell::default(); size],
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

    fn get_index(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }

    pub fn get_cell(&self, row: usize, col: usize) -> Cell {
        if row < self.height && col < self.width {
            self.cells[self.get_index(row, col)]
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

    #[inline]
    fn count_neighbors(&self, row: usize, col: usize) -> u8 {
        let width = self.width;
        let idx = row * width + col;
        let mut live_neighbors = 0u8;

        if row > 0 {
            if col > 0 && self.cells[idx - width - 1].is_alive() {
                live_neighbors += 1;
            }
            if self.cells[idx - width].is_alive() {
                live_neighbors += 1;
            }
            if col < width - 1 && self.cells[idx - width + 1].is_alive() {
                live_neighbors += 1;
            }
        }

        if col > 0 && self.cells[idx - 1].is_alive() {
            live_neighbors += 1;
        }
        if col < width - 1 && self.cells[idx + 1].is_alive() {
            live_neighbors += 1;
        }

        if row < self.height - 1 {
            if col > 0 && self.cells[idx + width - 1].is_alive() {
                live_neighbors += 1;
            }
            if self.cells[idx + width].is_alive() {
                live_neighbors += 1;
            }
            if col < width - 1 && self.cells[idx + width + 1].is_alive() {
                live_neighbors += 1;
            }
        }

        live_neighbors
    }

    pub fn tick(&mut self) {
        // Copy current state to scratch buffer
        self.scratch.copy_from_slice(&self.cells);

        let width = self.width;
        let height = self.height;

        for row in 0..height {
            for col in 0..width {
                let idx = row * width + col;
                let is_alive = self.cells[idx].is_alive();
                let live_neighbors = self.count_neighbors(row, col);

                let next_alive = match (is_alive, live_neighbors) {
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                };

                self.scratch[idx] = if next_alive {
                    Cell::alive()
                } else {
                    Cell::dead()
                };
            }
        }

        // Swap cells and scratch
        std::mem::swap(&mut self.cells, &mut self.scratch);
    }

    pub fn randomize(&mut self, probability: f64) {
        let mut rng = rand::thread_rng();
        for cell in &mut self.cells {
            if rng.gen_bool(probability.clamp(0.0, 1.0)) {
                cell.set_alive();
            } else {
                cell.set_dead();
            }
        }
    }

    pub fn resize_and_redistribute(&mut self, new_width: usize, new_height: usize, density: f64) {
        if self.width == new_width && self.height == new_height {
            return;
        }

        self.width = new_width;
        self.height = new_height;
        let size = new_width * new_height;
        self.cells = vec![Cell::dead(); size];
        self.scratch = vec![Cell::dead(); size];

        self.randomize(density);

        if new_width > 15 && new_height > 15 {
            self.add_glider(5, 5);
            if new_width > 30 {
                self.add_blinker(new_width - 10, 8);
            }
        }
    }

    // Predefined patterns
    pub fn add_glider(&mut self, start_row: usize, start_col: usize) {
        let pattern = [(0, 1), (1, 2), (2, 0), (2, 1), (2, 2)];
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
        let pattern = [(0, 1), (0, 2), (0, 3), (1, 0), (1, 1), (1, 2)];
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
            (0, 0), (0, 1), (1, 0), (1, 1),
            (2, 2), (2, 3), (3, 2), (3, 3),
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
            (2, 4), (2, 5), (2, 6), (2, 10), (2, 11), (2, 12),
            (4, 2), (4, 7), (4, 9), (4, 14),
            (5, 2), (5, 7), (5, 9), (5, 14),
            (6, 2), (6, 7), (6, 9), (6, 14),
            (7, 4), (7, 5), (7, 6), (7, 10), (7, 11), (7, 12),
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
        self.cells
            .iter()
            .zip(previous.cells.iter())
            .all(|(current, prev)| current.state == prev.state)
    }

    pub fn count_living_cells(&self) -> usize {
        self.cells.iter().filter(|cell| cell.is_alive()).count()
    }
}

// ─── Hydrate (WASM) Life component ──────────────────────────────────────────

#[cfg(feature = "hydrate")]
fn is_dark_mode() -> bool {
    web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.document_element())
        .map_or(false, |el| el.class_list().contains("dark"))
}

#[cfg(feature = "hydrate")]
#[component]
pub fn Life(animation_speed: RwSignal<u64>, population_density: RwSignal<f64>) -> impl IntoView {
    use std::cell::RefCell;
    use std::rc::Rc;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;
    use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MutationObserver, MutationObserverInit};

    use super::{CanvasConfig, CanvasRenderer};

    let canvas_ref = NodeRef::<leptos::html::Canvas>::new();
    let cell_size = 12.0_f64;

    // ── Shared state via Rc<RefCell<..>> ──

    let universe = Rc::new(RefCell::new({
        let (gw, gh, _cw, _ch) = viewport_grid_size(cell_size);
        let mut u = Universe::new(gw, gh);
        u.randomize(population_density.get_untracked());
        u.add_glider(5, 5);
        u.add_blinker(10, 8);
        u
    }));

    let renderer = Rc::new(RefCell::new(CanvasRenderer::new(
        CanvasConfig::for_theme(cell_size, is_dark_mode()),
    )));

    let cached_ctx: Rc<RefCell<Option<CanvasRenderingContext2d>>> = Rc::new(RefCell::new(None));
    let needs_render = Rc::new(RefCell::new(true));
    let canvas_dirty = Rc::new(RefCell::new(true));
    let last_tick = Rc::new(RefCell::new(0.0_f64));

    // ── requestAnimationFrame loop ──

    let raf_closure: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None));
    let raf_id: Rc<RefCell<i32>> = Rc::new(RefCell::new(0));

    {
        let raf_closure_clone = raf_closure.clone();
        let raf_id_clone = raf_id.clone();
        let universe = universe.clone();
        let renderer = renderer.clone();
        let cached_ctx = cached_ctx.clone();
        let needs_render = needs_render.clone();
        let canvas_dirty = canvas_dirty.clone();
        let last_tick = last_tick.clone();

        let closure = Closure::wrap(Box::new(move |timestamp: f64| {
            let speed = animation_speed.get_untracked() as f64;
            let elapsed = timestamp - *last_tick.borrow();

            let mut ticked = false;
            if elapsed >= speed {
                *last_tick.borrow_mut() = timestamp;
                universe.borrow_mut().tick();
                ticked = true;
            }

            if ticked || *needs_render.borrow() {
                // Obtain or cache the 2D context
                let mut ctx_ref = cached_ctx.borrow_mut();
                if ctx_ref.is_none() {
                    if let Some(canvas) = leptos::document()
                        .query_selector("canvas")
                        .ok()
                        .flatten()
                    {
                        let canvas_el: HtmlCanvasElement = canvas.unchecked_into();
                        if let Ok(Some(ctx)) = canvas_el.get_context("2d") {
                            if let Ok(ctx2d) = ctx.dyn_into::<CanvasRenderingContext2d>() {
                                *ctx_ref = Some(ctx2d);
                            }
                        }
                    }
                }

                if let Some(ctx) = ctx_ref.as_ref() {
                    // Set canvas dimensions only when dirty
                    if *canvas_dirty.borrow() {
                        let (_, _, cw, ch) = viewport_grid_size(cell_size);
                        if let Some(canvas) = leptos::document()
                            .query_selector("canvas")
                            .ok()
                            .flatten()
                        {
                            let canvas_el: HtmlCanvasElement = canvas.unchecked_into();
                            canvas_el.set_width(cw);
                            canvas_el.set_height(ch);
                            let style = canvas_el.style();
                            let _ = style.set_property("width", &format!("{cw}px"));
                            let _ = style.set_property("height", &format!("{ch}px"));
                        }
                        *canvas_dirty.borrow_mut() = false;
                    }

                    let uni = universe.borrow();
                    let (_, _, cw, ch) = viewport_grid_size(cell_size);
                    renderer.borrow_mut().draw(ctx, &uni, cw, ch);
                }

                *needs_render.borrow_mut() = false;
            }

            // Schedule next frame
            if let Some(ref cb) = *raf_closure_clone.borrow() {
                if let Some(window) = web_sys::window() {
                    if let Ok(id) = window.request_animation_frame(cb.as_ref().unchecked_ref()) {
                        *raf_id_clone.borrow_mut() = id;
                    }
                }
            }
        }) as Box<dyn FnMut(f64)>);

        *raf_closure.borrow_mut() = Some(closure);
    }

    // Kick off the first frame
    {
        if let Some(ref cb) = *raf_closure.borrow() {
            if let Some(window) = web_sys::window() {
                if let Ok(id) = window.request_animation_frame(cb.as_ref().unchecked_ref()) {
                    *raf_id.borrow_mut() = id;
                }
            }
        }
    }

    // Cleanup rAF
    {
        let raf_closure = raf_closure.clone();
        let raf_id = raf_id.clone();
        on_cleanup(move || {
            if let Some(window) = web_sys::window() {
                window.cancel_animation_frame(*raf_id.borrow()).ok();
            }
            // Break Rc cycle
            *raf_closure.borrow_mut() = None;
        });
    }

    // ── Window resize listener ──
    {
        let universe = universe.clone();
        let canvas_dirty = canvas_dirty.clone();
        let needs_render = needs_render.clone();

        let resize_cb = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            let density = population_density.get_untracked();
            let (gw, gh, _, _) = viewport_grid_size(cell_size);
            universe.borrow_mut().resize_and_redistribute(gw, gh, density);
            *canvas_dirty.borrow_mut() = true;
            *needs_render.borrow_mut() = true;
        }) as Box<dyn FnMut(_)>);

        if let Some(window) = web_sys::window() {
            let js_fn = resize_cb.as_ref().unchecked_ref::<js_sys::Function>().clone();
            let _ = window.add_event_listener_with_callback("resize", &js_fn);

            let js_fn_cleanup = js_fn.clone();
            on_cleanup(move || {
                if let Some(window) = web_sys::window() {
                    let _ = window.remove_event_listener_with_callback("resize", &js_fn_cleanup);
                }
                drop(resize_cb);
            });
        }
    }

    // ── Mouse handler ──
    {
        let universe = universe.clone();
        let needs_render = needs_render.clone();

        let mouse_cb = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let mouse_x = event.client_x() as f64;
            let mouse_y = event.client_y() as f64;
            let col = (mouse_x / cell_size) as usize;
            let row = (mouse_y / cell_size) as usize;

            let mut uni = universe.borrow_mut();
            if row < uni.height() && col < uni.width() {
                // Only set needs_render when cell was dead (avoid unnecessary renders)
                if !uni.get_cell(row, col).is_alive() {
                    uni.set_cell(row, col, Cell::alive());
                    *needs_render.borrow_mut() = true;
                }
            }
        }) as Box<dyn FnMut(_)>);

        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            let js_fn = mouse_cb.as_ref().unchecked_ref::<js_sys::Function>().clone();
            let _ = document.add_event_listener_with_callback("mousemove", &js_fn);

            let js_fn_cleanup = js_fn.clone();
            on_cleanup(move || {
                if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                    let _ = document.remove_event_listener_with_callback("mousemove", &js_fn_cleanup);
                }
                drop(mouse_cb);
            });
        }
    }

    // ── MutationObserver for theme detection ──
    {
        let renderer = renderer.clone();
        let needs_render = needs_render.clone();

        let mutation_cb = Closure::wrap(Box::new(move |_mutations: js_sys::Array, _observer: MutationObserver| {
            let dark = is_dark_mode();
            renderer.borrow_mut().update_theme(dark);
            *needs_render.borrow_mut() = true;
        }) as Box<dyn FnMut(js_sys::Array, MutationObserver)>);

        if let Some(doc_el) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.document_element())
        {
            if let Ok(observer) = MutationObserver::new(mutation_cb.as_ref().unchecked_ref()) {
                let opts = MutationObserverInit::new();
                opts.set_attributes(true);
                opts.set_attribute_filter(&js_sys::Array::of1(&JsValue::from_str("class")));
                let _ = observer.observe_with_options(&doc_el, &opts);

                on_cleanup(move || {
                    observer.disconnect();
                    drop(mutation_cb);
                });
            }
        }
    }

    // ── Population density watcher ──
    {
        let universe = universe.clone();
        let needs_render = needs_render.clone();

        create_effect(move |previous_density: Option<f64>| {
            let current_density = population_density.get();
            if let Some(prev) = previous_density {
                if (current_density - prev).abs() > f64::EPSILON {
                    let mut uni = universe.borrow_mut();
                    uni.clear();
                    uni.randomize(current_density);
                    uni.add_glider(5, 5);
                    uni.add_blinker(10, 8);
                    *needs_render.borrow_mut() = true;
                }
            }
            current_density
        });
    }

    view! {
        <div class="fixed inset-0 w-screen h-screen z-0 pointer-events-none">
            <canvas
                node_ref=canvas_ref
                class="w-full h-full block bg-background"
            />
        </div>
    }
}

#[cfg(feature = "hydrate")]
fn viewport_grid_size(cell_size: f64) -> (usize, usize, u32, u32) {
    if let Some(window) = web_sys::window() {
        let width = window.inner_width().unwrap().as_f64().unwrap();
        let height = window.inner_height().unwrap().as_f64().unwrap();
        let grid_width = (width / cell_size).ceil() as usize;
        let grid_height = (height / cell_size).ceil() as usize;
        let canvas_width = width as u32;
        let canvas_height = height as u32;
        (grid_width, grid_height, canvas_width, canvas_height)
    } else {
        (100, 60, 1200, 720)
    }
}

// ─── SSR version (no-op) ────────────────────────────────────────────────────

#[cfg(not(feature = "hydrate"))]
#[component]
pub fn Life(animation_speed: RwSignal<u64>, population_density: RwSignal<f64>) -> impl IntoView {
    let _ = animation_speed;
    let _ = population_density;
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
