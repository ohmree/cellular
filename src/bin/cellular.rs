use cellular::Automaton;
use ndarray::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum GameState {
    Alive,
    Dead,
}

impl GameState {
    pub fn symbol(self) -> char {
        use GameState::*;
        match self {
            Alive => '#',
            Dead => ' ',
        }
    }
}

#[derive(Debug)]
pub struct GameOfLife {
    grid: Array2<GameState>,
}

impl GameOfLife {
    pub fn new(size: (usize, usize)) -> Self {
        use GameState::*;
        GameOfLife {
            grid: Array2::<GameState>::from_elem(size, Dead),
        }
    }

    fn print(&self) {
        for col in self.grid.outer_iter() {
            for elem in col.iter() {
                print!("{}", elem.symbol());
            }
            println!("");
        }
    }
}

impl Automaton for GameOfLife {
    type State = GameState;

    fn cell_at(&self, idx: (usize, usize)) -> Self::State { self.grid[idx] }
    fn nrows(&self) -> usize { self.grid.nrows() }
    fn ncols(&self) -> usize { self.grid.ncols() }
}

pub fn main() {
    let game = GameOfLife::new((10, 10));
    // game.step();
    game.print();
}
