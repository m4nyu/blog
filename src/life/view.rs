//! Full-viewport canvas that runs the [`Simulation`](super::Simulation).
//!
//! Rendering: one canvas pixel per cell, scaled up by CSS with
//! `image-rendering: pixelated`, so a frame writes `cols * rows` pixels
//! instead of the whole viewport and stays sharp on HiDPI screens.

use leptos::prelude::*;

/// Edge length of one cell in CSS pixels.
pub const CELL_PX: f64 = 12.0;

#[component]
pub fn Life() -> impl IntoView {
    let canvas = NodeRef::<leptos::html::Canvas>::new();

    #[cfg(feature = "hydrate")]
    {
        let settings = expect_context::<crate::app::Settings>();
        Effect::new(move |_| {
            if let Some(el) = canvas.get() {
                client::start(el, settings);
            }
        });
    }

    view! { <canvas node_ref=canvas class="life" aria-hidden="true"></canvas> }
}

#[cfg(feature = "hydrate")]
mod client {
    use super::CELL_PX;
    use crate::app::Settings;
    use crate::life::Simulation;
    use leptos::prelude::*;
    use send_wrapper::SendWrapper;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;
    use wasm_bindgen::{prelude::*, Clamped, JsCast};
    use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData, MediaQueryList};

    type FrameCallback = Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>>;

    struct State {
        sim: Simulation,
        canvas: HtmlCanvasElement,
        ctx: CanvasRenderingContext2d,
        pixels: Vec<u8>,
        rgb: [u8; 3],
        last_tick: f64,
        dirty: bool,
        animate: bool,
    }

    impl State {
        /// Matches the canvas backing store to the grid and its CSS box to the cells.
        fn fit_canvas(&mut self) {
            let (cols, rows) = (self.sim.universe.width(), self.sim.universe.height());
            self.canvas.set_width(cols as u32);
            self.canvas.set_height(rows as u32);
            let style = web_sys::HtmlElement::style(&self.canvas);
            let _ = style.set_property("width", &format!("{}px", cols as f64 * CELL_PX));
            let _ = style.set_property("height", &format!("{}px", rows as f64 * CELL_PX));
            self.pixels = vec![0; cols * rows * 4];
            self.dirty = true;
        }

        fn resize(&mut self) {
            let (cols, rows) = grid_size();
            if (cols, rows) != (self.sim.universe.width(), self.sim.universe.height()) {
                self.sim.resize(cols, rows);
                self.fit_canvas();
            }
        }

        fn refresh_color(&mut self) {
            self.rgb = cell_rgb();
            self.dirty = true;
        }

        fn poke(&mut self, x: i32, y: i32) {
            if x < 0 || y < 0 {
                return;
            }
            let col = (f64::from(x) / CELL_PX) as usize;
            let row = (f64::from(y) / CELL_PX) as usize;
            if self.sim.set_alive(row, col) {
                self.dirty = true;
            }
        }

        fn frame(&mut self, now: f64, interval_ms: u32) {
            if self.animate && now - self.last_tick >= f64::from(interval_ms) {
                self.last_tick = now;
                self.sim.step();
                self.dirty = true;
            }
            if self.dirty {
                self.draw();
                self.dirty = false;
            }
        }

        fn draw(&mut self) {
            let [r, g, b] = self.rgb;
            for (px, &alive) in self
                .pixels
                .chunks_exact_mut(4)
                .zip(self.sim.universe.cells())
            {
                px.copy_from_slice(&if alive { [r, g, b, 255] } else { [0, 0, 0, 0] });
            }
            let (cols, rows) = (
                self.sim.universe.width() as u32,
                self.sim.universe.height() as u32,
            );
            if let Ok(img) =
                ImageData::new_with_u8_clamped_array_and_sh(Clamped(&self.pixels), cols, rows)
            {
                let _ = self.ctx.put_image_data(&img, 0.0, 0.0);
            }
        }
    }

    /// Grid that covers the viewport, rounded up.
    fn grid_size() -> (usize, usize) {
        let window = window();
        let px = |v: Result<JsValue, JsValue>| v.ok().and_then(|v| v.as_f64()).unwrap_or(0.0);
        let cols = (px(window.inner_width()) / CELL_PX).ceil().max(1.0) as usize;
        let rows = (px(window.inner_height()) / CELL_PX).ceil().max(1.0) as usize;
        (cols, rows)
    }

    /// Reads `--life-rgb: R G B` from the root element, so the palette lives in CSS.
    fn cell_rgb() -> [u8; 3] {
        let fallback = [128, 128, 128];
        let Some(root) = document().document_element() else {
            return fallback;
        };
        let Ok(Some(style)) = window().get_computed_style(&root) else {
            return fallback;
        };
        let value = style.get_property_value("--life-rgb").unwrap_or_default();
        let parts: Vec<u8> = value
            .split_whitespace()
            .filter_map(|p| p.parse().ok())
            .collect();
        match parts.as_slice() {
            [r, g, b] => [*r, *g, *b],
            _ => fallback,
        }
    }

    fn media(query: &str) -> Option<MediaQueryList> {
        window().match_media(query).ok().flatten()
    }

    fn request_frame(cb: &Closure<dyn FnMut(f64)>) {
        let _ = window().request_animation_frame(cb.as_ref().unchecked_ref());
    }

    pub fn start(canvas: HtmlCanvasElement, settings: Settings) {
        let Some(ctx) = canvas
            .get_context("2d")
            .ok()
            .flatten()
            .and_then(|c| c.dyn_into::<CanvasRenderingContext2d>().ok())
        else {
            return;
        };

        let (cols, rows) = grid_size();
        let seed = (js_sys::Math::random() * 2f64.powi(53)) as u64 ^ js_sys::Date::now() as u64;
        let animate = !media("(prefers-reduced-motion: reduce)").is_some_and(|m| m.matches());
        let state = Rc::new(RefCell::new(State {
            sim: Simulation::new(cols, rows, settings.density.get_untracked(), seed),
            canvas,
            ctx,
            pixels: Vec::new(),
            rgb: cell_rgb(),
            last_tick: 0.0,
            dirty: true,
            animate,
        }));
        state.borrow_mut().fit_canvas();

        // Animation loop. `running` stops it; dropping the closure breaks the Rc cycle.
        let running = Rc::new(Cell::new(true));
        let frame: FrameCallback = Rc::new(RefCell::new(None));
        *frame.borrow_mut() = Some(Closure::new({
            let (state, frame, running) = (state.clone(), frame.clone(), running.clone());
            move |now: f64| {
                if !running.get() {
                    return;
                }
                state
                    .borrow_mut()
                    .frame(now, settings.speed.get_untracked());
                if let Some(cb) = frame.borrow().as_ref() {
                    request_frame(cb);
                }
            }
        }));
        if let Some(cb) = frame.borrow().as_ref() {
            request_frame(cb);
        }

        let on_resize = window_event_listener(leptos::ev::resize, {
            let state = state.clone();
            move |_| state.borrow_mut().resize()
        });
        let on_move = window_event_listener(leptos::ev::pointermove, {
            let state = state.clone();
            move |e| state.borrow_mut().poke(e.client_x(), e.client_y())
        });
        let on_down = window_event_listener(leptos::ev::pointerdown, {
            let state = state.clone();
            move |e| state.borrow_mut().poke(e.client_x(), e.client_y())
        });

        // Palette follows the theme setting and, in system mode, the OS scheme.
        Effect::new({
            let state = state.clone();
            move |_| {
                settings.theme.track();
                // The class on <html> is updated by another effect in the same tick.
                let state = state.clone();
                request_animation_frame(move || state.borrow_mut().refresh_color());
            }
        });
        let scheme = media("(prefers-color-scheme: dark)");
        let on_scheme: Closure<dyn FnMut()> = Closure::new({
            let state = state.clone();
            move || state.borrow_mut().refresh_color()
        });
        if let Some(mq) = &scheme {
            let _ =
                mq.add_event_listener_with_callback("change", on_scheme.as_ref().unchecked_ref());
        }

        // New density from the settings dialog: fresh board.
        Effect::new({
            let state = state.clone();
            move |prev: Option<f64>| {
                let density = settings.density.get();
                if prev.is_some_and(|p| p != density) {
                    let mut s = state.borrow_mut();
                    s.sim.reseed(density);
                    s.dirty = true;
                }
                density
            }
        });

        let owned = SendWrapper::new((running, frame, scheme, on_scheme));
        on_cleanup(move || {
            let (running, frame, scheme, on_scheme) = owned.take();
            running.set(false);
            frame.borrow_mut().take();
            if let Some(mq) = scheme {
                let _ = mq.remove_event_listener_with_callback(
                    "change",
                    on_scheme.as_ref().unchecked_ref(),
                );
            }
            on_resize.remove();
            on_move.remove();
            on_down.remove();
        });
    }
}
