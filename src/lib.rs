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

impl<T> MooreNeighborhood<T> {
    pub fn empty() -> Self {
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

impl<T> VonNeumannNeighborhood<T> {
    pub fn empty() -> Self {
        Self {
            n: None,
            e: None,
            s: None,
            w: None,
        }
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

impl<T> ExtendedVnNeighborhood<T> {
    pub fn empty() -> Self {
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
}

pub trait Automaton {
    // TODO: maybe restrict this trait to types that implement a custom,
    // indexing operator and then use it instead of custom getters and setters.

    type State: Copy;

    // Grid helpers, implement these on your automaton.
    fn cell_at(&self, idx: (usize, usize)) -> Self::State;
    fn nrows(&self) -> usize;
    fn ncols(&self) -> usize;

    fn step(&self) -> &Self {
        // TODO: advance all cells.
        // Should look like this:
        // for (idx, cell) in self.grid.indexed_iter() {
        //     let neighborhood = self.get_*_neighborhood(idx);
        //     self.set_cell_at(idx, get_next_state(self.cell_at(idx), neighborhood));
        // }
        self
    }
    fn get_moore_neighborhood(&self, idx: (usize, usize)) -> MooreNeighborhood<Self::State> {
        let (row, col) = idx;
        let mut n: Option<Self::State> = None;
        let mut ne: Option<Self::State> = None;
        let mut e: Option<Self::State> = None;
        let mut se: Option<Self::State> = None;
        let mut s: Option<Self::State> = None;
        let mut sw: Option<Self::State> = None;
        let mut w: Option<Self::State> = None;
        let mut nw: Option<Self::State> = None;

        if row != 0 {
            e = Some(self.cell_at((row + 1, col)));
        }
        if row != self.nrows() {
            w = Some(self.cell_at((row - 1, col)));
        }
        if col != 0 {
            n = Some(self.cell_at((row, col - 1)));
        }
        if col != self.ncols() {
            s = Some(self.cell_at((row, col + 1)));
        }
        if n.is_some() {
            if w.is_some() { nw = Some(self.cell_at((row - 1, col - 1))) }
            if e.is_some() { ne = Some(self.cell_at((row + 1, col - 1))) }
        }
        if s.is_some() {
            if w.is_some() { sw = Some(self.cell_at((row - 1, col + 1))) }
            if e.is_some() { se = Some(self.cell_at((row + 1, col + 1))) }
        }

        MooreNeighborhood::<Self::State> {
            n,
            ne,
            e,
            se,
            s,
            sw,
            w,
            nw,
        }
    }

    fn get_vn_neighborhood(&self, idx: (usize, usize)) -> VonNeumannNeighborhood<Self::State> {
        let (row, col) = idx;
        let mut n: Option<Self::State> = None;
        let mut e: Option<Self::State> = None;
        let mut s: Option<Self::State> = None;
        let mut w: Option<Self::State> = None;

        if row != 0 {
            e = Some(self.cell_at((row + 1, col)));
        }
        if row != self.nrows() {
            w = Some(self.cell_at((row - 1, col)));
        }
        if col != 0 {
            n = Some(self.cell_at((row, col - 1)));
        }
        if col != self.ncols() {
            s = Some(self.cell_at((row, col + 1)));
        }

        VonNeumannNeighborhood::<Self::State> { n, e, s, w }
    }

    fn get_extended_vn_neighborhood(
        &self,
        idx: (usize, usize),
    ) -> ExtendedVnNeighborhood<Self::State> {
        let (row, col) = idx;
        let mut n: Option<Self::State> = None;
        let mut n2: Option<Self::State> = None;
        let mut e: Option<Self::State> = None;
        let mut e2: Option<Self::State> = None;
        let mut s: Option<Self::State> = None;
        let mut s2: Option<Self::State> = None;
        let mut w: Option<Self::State> = None;
        let mut w2: Option<Self::State> = None;

        if row != 0 {
            e = Some(self.cell_at((row + 1, col)));
        }
        if row != self.nrows() {
            w = Some(self.cell_at((row - 1, col)));
        }
        if col != 0 {
            n = Some(self.cell_at((row, col - 1)));
        }
        if col != self.ncols() {
            s = Some(self.cell_at((row, col + 1)));
        }
        if row != 1 {
            e2 = Some(self.cell_at((row + 2, col)));
        }
        if row != self.nrows() - 1 {
            w2 = Some(self.cell_at((row - 2, col)));
        }
        if col != 1 {
            n2 = Some(self.cell_at((row, col - 2)));
        }
        if col != self.ncols() - 1 {
            s2 = Some(self.cell_at((row, col + 2)));
        }

        ExtendedVnNeighborhood::<Self::State> { n, n2, e, e2, s, s2, w, w2 }
    }
}
