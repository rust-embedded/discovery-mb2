use super::Prng;

use heapless::FnvIndexSet;

/// A single point on the grid.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Coords {
    // Signed ints to allow negative values (handy when checking if we have gone
    // off the top or left of the grid)
    pub row: i8,
    pub col: i8,
}

impl Coords {
    /// Get random coordinates within a grid. `exclude` is an optional set of
    /// coordinates which should be excluded from the output.
    pub fn random(rng: &mut Prng, exclude: Option<&FnvIndexSet<Coords, 32>>) -> Self {
        let mut coords = Coords {
            row: ((rng.random_u32() as usize) % 5) as i8,
            col: ((rng.random_u32() as usize) % 5) as i8,
        };
        while exclude.is_some_and(|exc| exc.contains(&coords)) {
            coords = Coords {
                row: ((rng.random_u32() as usize) % 5) as i8,
                col: ((rng.random_u32() as usize) % 5) as i8,
            }
        }
        coords
    }

    /// Whether the point is outside the bounds of the grid.
    pub fn is_out_of_bounds(&self) -> bool {
        self.row < 0 || self.row >= 5 || self.col < 0 || self.col >= 5
    }
}
