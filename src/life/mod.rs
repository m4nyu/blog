//! Conway's Game of Life on a torus (edges wrap), plus a small simulation
//! driver that reseeds the board when it stagnates.
//!
//! Pure Rust, no browser types: everything here is unit tested natively.
//! The canvas renderer lives in [`view`].

pub mod view;

/// Xorshift64* PRNG. Deterministic for tests, seeded from the clock in the browser.
#[derive(Clone, Debug)]
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        // A zero state would stay zero forever.
        Self(seed.max(1))
    }

    pub fn next_u64(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// Uniform in [0, 1).
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }

    pub fn chance(&mut self, p: f64) -> bool {
        self.next_f64() < p
    }

    /// Uniform in [0, n). Returns 0 when n is 0.
    pub fn below(&mut self, n: usize) -> usize {
        if n == 0 {
            0
        } else {
            (self.next_u64() % n as u64) as usize
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Universe {
    width: usize,
    height: usize,
    cells: Vec<bool>,
    next: Vec<bool>,
}

impl Universe {
    /// An empty board. Dimensions are clamped to at least 1x1.
    pub fn new(width: usize, height: usize) -> Self {
        let (width, height) = (width.max(1), height.max(1));
        Self {
            width,
            height,
            cells: vec![false; width * height],
            next: vec![false; width * height],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn cells(&self) -> &[bool] {
        &self.cells
    }

    pub fn is_alive(&self, row: usize, col: usize) -> bool {
        row < self.height && col < self.width && self.cells[row * self.width + col]
    }

    /// Out-of-range coordinates are ignored.
    pub fn set(&mut self, row: usize, col: usize, alive: bool) {
        if row < self.height && col < self.width {
            self.cells[row * self.width + col] = alive;
        }
    }

    pub fn population(&self) -> usize {
        self.cells.iter().filter(|&&c| c).count()
    }

    pub fn clear(&mut self) {
        self.cells.fill(false);
    }

    /// Advances one generation and returns how many cells changed state.
    pub fn tick(&mut self) -> usize {
        let (w, h) = (self.width, self.height);
        let mut changed = 0;
        for row in 0..h {
            let up = (row + h - 1) % h;
            let down = (row + 1) % h;
            for col in 0..w {
                let left = (col + w - 1) % w;
                let right = (col + 1) % w;
                let neighbours = [
                    (up, left),
                    (up, col),
                    (up, right),
                    (row, left),
                    (row, right),
                    (down, left),
                    (down, col),
                    (down, right),
                ]
                .iter()
                .filter(|&&(r, c)| self.cells[r * w + c])
                .count();
                let idx = row * w + col;
                let alive = self.cells[idx];
                let next = matches!((alive, neighbours), (true, 2) | (_, 3));
                changed += usize::from(next != alive);
                self.next[idx] = next;
            }
        }
        std::mem::swap(&mut self.cells, &mut self.next);
        changed
    }

    /// Every cell becomes alive with probability `density` (clamped to [0, 1]).
    pub fn randomize(&mut self, density: f64, rng: &mut Rng) {
        let p = density.clamp(0.0, 1.0);
        for cell in &mut self.cells {
            *cell = rng.chance(p);
        }
    }

    /// Fills a `size` x `size` square (wrapping) at the given corner with random cells.
    pub fn seed_patch(&mut self, row: usize, col: usize, size: usize, density: f64, rng: &mut Rng) {
        let p = density.clamp(0.0, 1.0);
        for dr in 0..size {
            for dc in 0..size {
                let r = (row + dr) % self.height;
                let c = (col + dc) % self.width;
                if rng.chance(p) {
                    self.cells[r * self.width + c] = true;
                }
            }
        }
    }

    /// Changes the board size, keeping the overlapping region and seeding
    /// only the newly exposed cells.
    pub fn resize(&mut self, width: usize, height: usize, density: f64, rng: &mut Rng) {
        let (width, height) = (width.max(1), height.max(1));
        if width == self.width && height == self.height {
            return;
        }
        let p = density.clamp(0.0, 1.0);
        let mut cells = vec![false; width * height];
        for row in 0..height {
            for col in 0..width {
                cells[row * width + col] = if row < self.height && col < self.width {
                    self.cells[row * self.width + col]
                } else {
                    rng.chance(p)
                };
            }
        }
        self.width = width;
        self.height = height;
        self.cells = cells;
        self.next = vec![false; width * height];
    }

    pub fn add_glider(&mut self, row: usize, col: usize) {
        for (dr, dc) in [(0, 1), (1, 2), (2, 0), (2, 1), (2, 2)] {
            self.set((row + dr) % self.height, (col + dc) % self.width, true);
        }
    }
}

/// Fraction of cells changing per generation below which the board counts as stagnant.
const STAGNANT_ACTIVITY: f64 = 0.002;
/// Generations to wait after a (re)seed before judging stagnation.
const SETTLE_GENERATIONS: u32 = 60;
const PATCH_SIZE: usize = 12;
const PATCH_DENSITY: f64 = 0.4;
const PATCHES: usize = 3;

/// Drives a [`Universe`]: tracks activity and drops fresh random patches
/// when life settles into still lifes and blinkers.
#[derive(Clone, Debug)]
pub struct Simulation {
    pub universe: Universe,
    rng: Rng,
    density: f64,
    activity: f64,
    since_seed: u32,
}

impl Simulation {
    pub fn new(width: usize, height: usize, density: f64, seed: u64) -> Self {
        let mut sim = Self {
            universe: Universe::new(width, height),
            rng: Rng::new(seed),
            density,
            activity: 1.0,
            since_seed: 0,
        };
        sim.reseed(density);
        sim
    }

    /// Clears and repopulates the whole board at `density`.
    pub fn reseed(&mut self, density: f64) {
        self.density = density;
        self.universe.randomize(density, &mut self.rng);
        if self.universe.width() > 10 && self.universe.height() > 10 {
            self.universe.add_glider(2, 2);
        }
        self.activity = 1.0;
        self.since_seed = 0;
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.universe
            .resize(width, height, self.density, &mut self.rng);
    }

    /// Advances one generation. Returns true when stagnation triggered a reseed.
    pub fn step(&mut self) -> bool {
        let changed = self.universe.tick();
        let total = self.universe.cells().len() as f64;
        self.activity = self.activity * 0.9 + (changed as f64 / total) * 0.1;
        self.since_seed = self.since_seed.saturating_add(1);

        if self.since_seed >= SETTLE_GENERATIONS && self.activity < STAGNANT_ACTIVITY {
            for _ in 0..PATCHES {
                let row = self.rng.below(self.universe.height());
                let col = self.rng.below(self.universe.width());
                self.universe
                    .seed_patch(row, col, PATCH_SIZE, PATCH_DENSITY, &mut self.rng);
            }
            self.activity = 1.0;
            self.since_seed = 0;
            return true;
        }
        false
    }

    /// Brings a cell to life. Returns false when it was alive already or out of range.
    pub fn set_alive(&mut self, row: usize, col: usize) -> bool {
        let in_range = row < self.universe.height() && col < self.universe.width();
        if !in_range || self.universe.is_alive(row, col) {
            return false;
        }
        self.universe.set(row, col, true);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn with_cells(w: usize, h: usize, cells: &[(usize, usize)]) -> Universe {
        let mut u = Universe::new(w, h);
        for &(r, c) in cells {
            u.set(r, c, true);
        }
        u
    }

    #[test]
    fn empty_stays_empty() {
        let mut u = Universe::new(8, 8);
        assert_eq!(u.tick(), 0);
        assert_eq!(u.population(), 0);
    }

    #[test]
    fn lonely_cell_dies() {
        let mut u = with_cells(5, 5, &[(2, 2)]);
        assert_eq!(u.tick(), 1);
        assert_eq!(u.population(), 0);
    }

    #[test]
    fn block_is_still() {
        let block = [(2, 2), (2, 3), (3, 2), (3, 3)];
        let mut u = with_cells(6, 6, &block);
        let before = u.clone();
        assert_eq!(u.tick(), 0);
        assert_eq!(u.cells(), before.cells());
    }

    #[test]
    fn blinker_has_period_two() {
        let mut u = with_cells(5, 5, &[(2, 1), (2, 2), (2, 3)]);
        let start = u.cells().to_vec();
        u.tick();
        assert!(u.is_alive(1, 2) && u.is_alive(2, 2) && u.is_alive(3, 2));
        assert!(!u.is_alive(2, 1) && !u.is_alive(2, 3));
        u.tick();
        assert_eq!(u.cells(), start.as_slice());
    }

    #[test]
    fn glider_translates_one_cell_diagonally_every_four_generations() {
        let mut u = Universe::new(10, 10);
        u.add_glider(0, 0);
        for _ in 0..4 {
            u.tick();
        }
        let mut expected = Universe::new(10, 10);
        expected.add_glider(1, 1);
        assert_eq!(u.cells(), expected.cells());
    }

    #[test]
    fn edges_wrap_around() {
        // A blinker straddling the left/right edge must keep oscillating.
        let mut u = with_cells(6, 6, &[(3, 5), (3, 0), (3, 1)]);
        u.tick();
        assert!(u.is_alive(2, 0) && u.is_alive(3, 0) && u.is_alive(4, 0));
        assert_eq!(u.population(), 3);
    }

    #[test]
    fn glider_survives_crossing_the_edge() {
        let mut u = Universe::new(8, 8);
        u.add_glider(0, 0);
        // 8x8 torus: after 32 generations the glider is back where it started.
        for _ in 0..32 {
            u.tick();
        }
        let mut expected = Universe::new(8, 8);
        expected.add_glider(0, 0);
        assert_eq!(u.cells(), expected.cells());
    }

    #[test]
    fn out_of_range_is_dead_and_ignored() {
        let mut u = Universe::new(4, 4);
        u.set(10, 10, true);
        assert!(!u.is_alive(10, 10));
        assert_eq!(u.population(), 0);
    }

    #[test]
    fn zero_dimensions_are_clamped() {
        let mut u = Universe::new(0, 0);
        assert_eq!((u.width(), u.height()), (1, 1));
        u.tick();
    }

    #[test]
    fn randomize_extremes() {
        let mut rng = Rng::new(7);
        let mut u = Universe::new(50, 40);
        u.randomize(0.0, &mut rng);
        assert_eq!(u.population(), 0);
        u.randomize(1.0, &mut rng);
        assert_eq!(u.population(), 2000);
        u.randomize(2.0, &mut rng);
        assert_eq!(u.population(), 2000);
    }

    #[test]
    fn randomize_matches_density_roughly() {
        let mut rng = Rng::new(42);
        let mut u = Universe::new(200, 200);
        u.randomize(0.1, &mut rng);
        let ratio = u.population() as f64 / 40_000.0;
        assert!((0.08..0.12).contains(&ratio), "ratio {ratio}");
    }

    #[test]
    fn resize_keeps_overlap() {
        let mut rng = Rng::new(1);
        let mut u = with_cells(10, 10, &[(1, 1), (8, 8)]);
        u.resize(20, 15, 0.0, &mut rng);
        assert_eq!((u.width(), u.height()), (20, 15));
        assert!(u.is_alive(1, 1) && u.is_alive(8, 8));
        assert_eq!(u.population(), 2);

        u.resize(5, 5, 0.0, &mut rng);
        assert!(u.is_alive(1, 1));
        assert_eq!(u.population(), 1);
    }

    #[test]
    fn resize_seeds_only_new_area() {
        let mut rng = Rng::new(3);
        let mut u = Universe::new(10, 10);
        u.resize(20, 10, 1.0, &mut rng);
        for row in 0..10 {
            for col in 0..20 {
                assert_eq!(u.is_alive(row, col), col >= 10, "({row},{col})");
            }
        }
    }

    #[test]
    fn rng_is_deterministic_and_in_range() {
        let (mut a, mut b) = (Rng::new(9), Rng::new(9));
        for _ in 0..1000 {
            let x = a.next_f64();
            assert_eq!(x, b.next_f64());
            assert!((0.0..1.0).contains(&x));
            assert!(a.below(7) < 7);
            b.below(7);
        }
        assert_eq!(Rng::new(0).next_u64(), Rng::new(1).next_u64());
    }

    #[test]
    fn stagnant_board_gets_reseeded() {
        let mut sim = Simulation::new(40, 30, 0.0, 5);
        sim.universe.clear();
        // A lone block: zero activity forever.
        for (r, c) in [(5, 5), (5, 6), (6, 5), (6, 6)] {
            sim.universe.set(r, c, true);
        }
        let reseeded = (0..SETTLE_GENERATIONS + 40).any(|_| sim.step());
        assert!(reseeded);
        assert!(sim.universe.population() > 4);
    }

    #[test]
    fn busy_board_is_left_alone() {
        let mut sim = Simulation::new(120, 80, 0.3, 11);
        // A random soup stays busy for far longer than the settle window.
        let reseeded = (0..SETTLE_GENERATIONS * 5).any(|_| sim.step());
        assert!(!reseeded);
    }

    #[test]
    fn set_alive_reports_change() {
        let mut sim = Simulation::new(10, 10, 0.0, 2);
        sim.universe.clear();
        assert!(sim.set_alive(3, 3));
        assert!(!sim.set_alive(3, 3));
        assert!(!sim.set_alive(99, 99));
    }
}
