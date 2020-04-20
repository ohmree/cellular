use crate::Automaton;
use ndarray::prelude::*;
// use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
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
        matches!(self, Self::Alive)
    }

    pub fn is_dead(&self) -> bool {
        matches!(self, Self::Dead)
    }
}

#[derive(Debug)]
pub struct GameOfLife {
    grid: Array2<GameState>,
}

impl GameOfLife {
    pub fn new(size: (usize, usize)) -> Self {
        use GameState::*;
        let grid = Array2::<GameState>::from_elem(size, Dead);
        GameOfLife { grid }
    }

    pub fn print(&self) {
        for col in self.grid.outer_iter() {
            for elem in col.iter() {
                print!("{}", elem.symbol());
            }
            println!();
        }
    }
}

impl Automaton for GameOfLife {
    type State = GameState;

    fn get_view(&self) -> ArrayView2<Self::State> {
        self.grid.view()
    }

    fn from_array2(grid: Array2<Self::State>) -> Self {
        GameOfLife { grid }
    }

    fn next_state_of(&self, idx: (usize, usize)) -> Self::State {
        use GameState::*;

        let neighborhood = self.moore_neighborhood_of(idx);
        let living_cells: Vec<_> = neighborhood
            .values()
            .into_iter()
            .filter(|state| **state == Alive)
            .collect();

        match living_cells.len() {
            n if n < 2 || n > 3 => Dead,
            _ => Alive,
        }
    }
}
