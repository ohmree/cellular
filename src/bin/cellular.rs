use cellular::{Automaton, Neighborhood, MooreNeighborhood};
use ndarray::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum GameState {
    Alive,
    Dead,
}

impl GameState {
    pub fn symbol(&self) -> char {
        use GameState::*;
        match self {
            Alive => '#',
            Dead => ' ',
        }
    }

    pub fn is_alive(&self) -> bool {
        matches!(self, Alive)
    }

    pub fn is_dead(&self) -> bool {
        matches!(self, Dead)
    }
}

#[derive(Debug)]
pub struct GameOfLife {
    grid: Array2<GameState>,
}

impl GameOfLife {
    pub fn new(size: (usize, usize)) -> Self {
        use GameState::*;
        let mut grid = Array2::<GameState>::from_elem(size, Dead);
        GameOfLife { grid }
    }

    pub fn from_grid(grid: Array2<GameState>) -> Self {
        GameOfLife { grid }
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

    fn get_grid(&self) -> Array2<Self::State> { self.grid }

    fn next_state_of<T: Neighborhood>(&self, neighborhood: T) -> Self::State {
        use GameState::*;
        let cells = neighborhood.existing_cells();
        let living_neighbors = cells.iter().filter(|c| c.is_alive());
        Alive
    }

    fn step(&self) -> Self {
        // TODO: figure out exactly how to do this, I'm dumb.
        // I want to map over the array, have access to the index
        // and create a new array from the resulting iterator.
        // For now this doesn't compile.
        // Seems like I might need to implement FromIterator on my
        // state enum, but there might be a way around this.
        // Perhaps a more ergonomic way built into ndarray?

        // TODO: figure out if a for loop is better suited for this
        // for idx in (0..self.nrows()).zip(0..self.ncols()) {

        // }

        // FIXME: the map operation will probably fail on non-square grid sizes.
        // let grid: Array2<GameState> = (0..self.nrows()).zip(0..self.ncols())
        //     .map(|idx| self.next_state_of(self.moore_neighborhood_at(idx)))
        //     .collect::<Array2<GameState>>();

        // Self::from_grid(grid)

        *self
    }
}

pub fn main() {
    let mut game = GameOfLife::new((10, 10));
    // This is just dummy code to get this to compile
    game.print();
}
