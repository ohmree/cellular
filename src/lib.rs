use ndarray::prelude::*;
use std::{fmt::Debug, collections::HashMap};

#[derive(Debug, Clone, Copy)]
pub enum GameState {
    Alive,
    Dead,
}

pub trait Automaton {
    type State: Copy + Debug;

    // The 2D array where the automaton state is stored.
    // I think this returns a copy of the array, which
    // might be good since I'm trying to do this functionally?
    fn get_view(&self) -> ArrayView2<Self::State>;

    // Returns a new struct with the given array.
    fn from_array2(grid: Array2<Self::State>) -> Self;

    // Takes an index, returns the next state for the cell at the index.
    fn next_state_of(&self, idx: (usize, usize)) -> Self::State;

    // Returns the next iteration of the automaton
    fn step(&self) -> Self
    where
        Self: std::marker::Sized,
    {
        // TODO: figure out exactly how to do this, I'm dumb.
        let view = self.get_view();
        let cells = view
            .indexed_iter()
            .map(|(idx, _)| self.next_state_of(idx))
            .collect();

        let shape = view.shape();
        let res = Array2::from_shape_vec((shape[0], shape[1]), cells).unwrap();
        Self::from_array2(res)
    }

    fn moore_neighborhood_of(&self, idx: (usize, usize)) -> HashMap<&str, Self::State> {
        let grid = self.get_view();
        let (row, col) = idx;
        let nrows = grid.nrows();
        let ncols = grid.ncols();
        let mut ret = HashMap::with_capacity(8);

        if row != 0 {
            ret.insert("n", grid[(row - 1, col)]);
        }

        if row != grid.nrows() {
            ret.insert("s", grid[(row + 1, col)]);
        }

        if col != 0 {
            ret.insert("w", grid[(row, col - 1)]);
        }

        if col != grid.ncols() {
            ret.insert("e", grid[(row, col + 1)]);
        }

        // TODO: find out a less fugly way to implement this logic.
        if ret.contains_key("n") {
            if ret.contains_key("w") {
                ret.insert("nw", grid[(row - 1, col - 1)]);
            }
            if ret.contains_key("e") {
                ret.insert("ne", grid[(row + 1, col - 1)]);
            }
        }

        if ret.contains_key("s") {
            if ret.contains_key("w") {
                ret.insert("sw", grid[(row - 1, col + 1)]);
            }
            if ret.contains_key("e") {
                ret.insert("se", grid[(row + 1, col + 1)]);
            }
        }

        ret
    }

    fn vn_neighborhood_of(&self, idx: (usize, usize)) -> HashMap<&str, Self::State> {
        let grid = &self.get_view();
        let (row, col) = idx;
        let mut ret = HashMap::with_capacity(4);

        if row != 0 {
            ret.insert("n", grid[(row + 1, col)]);
        }

        if row != grid.nrows() {
            ret.insert("s", grid[(row - 1, col)]);
        }

        if col != 0 {
            ret.insert("w", grid[(row, col - 1)]);
        }

        if col != grid.ncols() {
            ret.insert("e", grid[(row, col + 1)]);
        }

        ret
    }

    fn extended_vn_neighborhood_of(&self, idx: (usize, usize)) -> HashMap<&str, Self::State> {
        let grid = &self.get_view();
        let (row, col) = idx;
        let mut ret = HashMap::with_capacity(8);

        if row != 0 {
            ret.insert("n", grid[(row + 1, col)]);
        }

        if row != 1 {
            ret.insert("n2", grid[(row + 2, col)]);
        }

        if row != grid.nrows() {
            ret.insert("s", grid[(row - 1, col)]);
        }

        if row != grid.nrows() - 1 {
            ret.insert("s2", grid[(row - 2, col)]);
        }

        if col != 0 {
            ret.insert("w", grid[(row, col - 1)]);
        }

        if col != 1 {
            ret.insert("w2", grid[(row, col - 2)]);
        }

        if col != grid.ncols() {
            ret.insert("e", grid[(row, col + 1)]);
        }

        if col != grid.ncols() - 1 {
            ret.insert("e2", grid[(row, col + 2)]);
        }

        ret
    }
}

pub mod automata;
