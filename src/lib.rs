use ndarray::prelude::*;

// TODO: maybe use hashmaps instead of hard-coded structs.
pub trait Neighborhood {
    type State: Copy;
    fn empty() -> Self;
    // This is what the hash version could look like:
    // fn as_hash(&self) -> HashMap<&str, Self::State>;
    // Or maybe even without 3 separate structs and a trait.

    // We might be able to generate this using a macro.
    fn as_vec(&self) -> Vec<Option<Self::State>>;
    fn existing_cells(&self) -> Vec<Self::State> {
        let maybe_cells: Vec<Option<Self::State>> = self.as_vec();
        maybe_cells.into_iter().flatten().collect()
    }
}

// Moore neighborhood:
// ###
// ###
// ###
#[derive(Copy, Debug, Clone)]
pub struct MooreNeighborhood<T> {
    pub n: Option<T>,
    pub ne: Option<T>,
    pub e: Option<T>,
    pub se: Option<T>,
    pub s: Option<T>,
    pub sw: Option<T>,
    pub w: Option<T>,
    pub nw: Option<T>,
}

impl<T: Copy> Neighborhood for MooreNeighborhood<T> {
    type State = T;

    fn empty() -> Self {
        Self {
            n: None,
            ne: None,
            e: None,
            se: None,
            s: None,
            sw: None,
            w: None,
            nw: None,
        }
    }

    fn as_vec(&self) -> Vec<Option<T>> {
        vec![
            self.n, self.ne, self.e, self.se, self.s, self.sw, self.w, self.nw,
        ]
    }
}

// von Neumann neighborhood:
//  #
// ###
//  #
#[derive(Copy, Debug, Clone)]
pub struct VonNeumannNeighborhood<T> {
    pub n: Option<T>,
    pub e: Option<T>,
    pub s: Option<T>,
    pub w: Option<T>,
}

impl<T: Copy> Neighborhood for VonNeumannNeighborhood<T> {
    type State = T;

    fn empty() -> Self {
        Self {
            n: None,
            e: None,
            s: None,
            w: None,
        }
    }
    fn as_vec(&self) -> Vec<Option<Self::State>> {
        vec![self.n, self.e, self.s, self.w]
    }
}

// Extended von Neumann neighborhood:
//   #
//   #
// #####
//   #
//   #
#[derive(Copy, Debug, Clone)]
pub struct ExtendedVnNeighborhood<T> {
    pub n: Option<T>,
    pub n2: Option<T>,
    pub e: Option<T>,
    pub e2: Option<T>,
    pub s: Option<T>,
    pub s2: Option<T>,
    pub w: Option<T>,
    pub w2: Option<T>,
}

impl<T: Copy> Neighborhood for ExtendedVnNeighborhood<T> {
    type State = T;

    fn empty() -> Self {
        Self {
            n: None,
            n2: None,
            e: None,
            e2: None,
            s: None,
            s2: None,
            w: None,
            w2: None,
        }
    }
    fn as_vec(&self) -> Vec<Option<Self::State>> {
        vec![
            self.n, self.n2, self.e, self.e2, self.s, self.s2, self.w, self.w2,
        ]
    }
}

pub trait Automaton {
    // TODO: maybe restrict this trait to types that implement a custom,
    // indexing operator and then use it instead of custom getters and setters.

    type State: Copy;

    // The 2D array where the automaton state is stored.
    // I think this returns a copy of the array, which
    // might be good since I'm trying to do this functionally?
    fn get_grid(&self) -> Array2<Self::State>;

    // Takes an index, returns the next state for the cell at the index.
    fn next_state_of<T: Neighborhood>(&self, neighborhood: T) -> Self::State;
    // Returns the next iteration of the automaton
    fn step(&self) -> Self;

    fn moore_neighborhood_at(&self, idx: (usize, usize)) -> MooreNeighborhood<Self::State> {
        let grid = &self.get_grid();
        let (row, col) = idx;
        let mut ret = MooreNeighborhood::<Self::State>::empty();

        if row != 0 {
            ret.n = Some(grid[(row + 1, col)]);
        }

        if row != grid.nrows() {
            ret.s = Some(grid[(row - 1, col)]);
        }

        if col != 0 {
            ret.w = Some(grid[(row, col - 1)]);
        }

        if col != grid.ncols() {
            ret.e = Some(grid[(row, col + 1)]);
        }

        // TODO: find out a less fugly way to implement this logic.
        if ret.n.is_some() {
            if ret.w.is_some() {
                ret.nw = Some(grid[(row - 1, col - 1)])
            }
            if ret.e.is_some() {
                ret.ne = Some(grid[(row + 1, col - 1)])
            }
        }

        if ret.s.is_some() {
            if ret.w.is_some() {
                ret.sw = Some(grid[(row - 1, col + 1)])
            }
            if ret.e.is_some() {
                ret.se = Some(grid[(row + 1, col + 1)])
            }
        }

        ret
    }

    fn vn_neighborhood_at(&self, idx: (usize, usize)) -> VonNeumannNeighborhood<Self::State> {
        let grid = &self.get_grid();
        let (row, col) = idx;
        let mut ret = VonNeumannNeighborhood::<Self::State>::empty();

        if row != 0 {
            ret.n = Some(grid[(row + 1, col)]);
        }

        if row != grid.nrows() {
            ret.s = Some(grid[(row - 1, col)]);
        }

        if col != 0 {
            ret.w = Some(grid[(row, col - 1)]);
        }

        if col != grid.ncols() {
            ret.e = Some(grid[(row, col + 1)]);
        }

        ret
    }

    fn extended_vn_neighborhood_at(
        &self,
        idx: (usize, usize),
    ) -> ExtendedVnNeighborhood<Self::State> {
        let grid = &self.get_grid();
        let (row, col) = idx;
        let mut ret = ExtendedVnNeighborhood::<Self::State>::empty();

        if row != 0 {
            ret.n = Some(grid[(row + 1, col)]);
        }

        if row != 1 {
            ret.n2 = Some(grid[(row + 2, col)]);
        }

        if row != grid.nrows() {
            ret.s = Some(grid[(row - 1, col)]);
        }

        if row != grid.nrows() - 1 {
            ret.s2 = Some(grid[(row - 2, col)]);
        }

        if col != 0 {
            ret.w = Some(grid[(row, col - 1)]);
        }

        if col != 1 {
            ret.w2 = Some(grid[(row, col - 2)]);
        }

        if col != grid.ncols() {
            ret.e = Some(grid[(row, col + 1)]);
        }

        if col != grid.ncols() - 1 {
            ret.e2 = Some(grid[(row, col + 2)]);
        }

        ret
    }
}
