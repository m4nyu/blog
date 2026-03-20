#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u8)]
pub enum CellState {
    Dead,
    Alive,
}

impl CellState {
    pub fn is_alive(&self) -> bool {
        matches!(self, CellState::Alive)
    }

    pub fn toggle(&mut self) {
        *self = match self {
            CellState::Dead => CellState::Alive,
            CellState::Alive => CellState::Dead,
        };
    }
}

impl Default for CellState {
    fn default() -> Self {
        CellState::Dead
    }
}

impl From<bool> for CellState {
    fn from(alive: bool) -> Self {
        if alive {
            CellState::Alive
        } else {
            CellState::Dead
        }
    }
}

impl From<CellState> for bool {
    fn from(state: CellState) -> Self {
        state.is_alive()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub state: CellState,
}

impl Cell {
    pub fn new(state: CellState) -> Self {
        Self { state }
    }

    pub fn alive() -> Self {
        Self::new(CellState::Alive)
    }

    pub fn dead() -> Self {
        Self::new(CellState::Dead)
    }

    pub fn from_bool(alive: bool) -> Self {
        Self::new(CellState::from(alive))
    }

    pub fn is_alive(&self) -> bool {
        self.state.is_alive()
    }

    pub fn toggle(&mut self) {
        self.state.toggle();
    }

    pub fn set_alive(&mut self) {
        self.state = CellState::Alive;
    }

    pub fn set_dead(&mut self) {
        self.state = CellState::Dead;
    }

    pub fn set_state(&mut self, state: CellState) {
        self.state = state;
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::dead()
    }
}

impl From<bool> for Cell {
    fn from(alive: bool) -> Self {
        Self::from_bool(alive)
    }
}

impl From<Cell> for bool {
    fn from(cell: Cell) -> Self {
        cell.is_alive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cell_state_repr() {
        assert_eq!(std::mem::size_of::<CellState>(), 1);
    }

    #[test]
    fn default_is_dead() {
        assert!(!Cell::default().is_alive());
        assert_eq!(CellState::default(), CellState::Dead);
    }

    #[test]
    fn alive_dead_constructors() {
        assert!(Cell::alive().is_alive());
        assert!(!Cell::dead().is_alive());
    }

    #[test]
    fn toggle() {
        let mut c = Cell::dead();
        c.toggle();
        assert!(c.is_alive());
        c.toggle();
        assert!(!c.is_alive());
    }

    #[test]
    fn set_state() {
        let mut c = Cell::dead();
        c.set_alive();
        assert!(c.is_alive());
        c.set_dead();
        assert!(!c.is_alive());
    }

    #[test]
    fn bool_conversions() {
        assert_eq!(CellState::from(true), CellState::Alive);
        assert_eq!(CellState::from(false), CellState::Dead);
        assert!(bool::from(CellState::Alive));
        assert!(!bool::from(CellState::Dead));
        assert!(bool::from(Cell::alive()));
        assert!(!bool::from(Cell::dead()));
    }
}
