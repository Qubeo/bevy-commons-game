/// A coordinate on a hex grid, representing distances along the various directions of travel
/// Invariant: In order to represent a valid hex coordinate, q + r + s must equal 0
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct HexCoord {
    pub q: isize,
    pub r: isize,
    pub s: isize,
}

// The directions you can move on a hex grid
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Direction {
    None,
    Three,
    Six,
    Nine
}

// Q: Defining symmetries? Ideal "grammar"? What about PT?
impl Direction {
    pub fn opposite(&self) -> Self {
        use Direction::*;
        match self {
            None => None,
            Three => Six,
            Six => Nine,
            Nine => Three,            
        }
    }
}

/// All directions, for convenient enumeration
pub const DIRECTIONS: &[Direction] = &[Direction::Three, Direction::Six, Direction::Nine];

impl TriCoord {
    /// The origin of an infinite hex grid
    pub fn origin() -> Self { HexCoord { q: 0, r: 0, s: 0 } }
    /// Construct a tri coordinate from two pieces of information, enforcing the invariant on the third
    pub fn new(q: isize, r: isize) -> Self {
        TriCoord { q, r, s: -q - r }
    }

    /// The coordinate to the north
    pub fn three(&self) -> Self { Self::new(self.q + 0, self.r - 1) }
    /// The coordinate to the south
    pub fn six(&self)   -> Self { Self::new(self.q + 0, self.r + 1) }
    /// The coordinate to the northeast
    pub fn nine(&self)  -> Self { Self::new(self.q + 1, self.r - 1) }
    
    /// The coordinate in a specific direction
    pub fn neighbor(&self, dir: Direction) -> Self {
        use Direction::*;
        match dir {
            None => self.clone(),
            Three => self.three(),
            Six => self.six(),
            Nine => self.nine()            
        }
    }

    /// Yield the neighbor coordinates, starting from North and going clockwise
    pub fn neighbors<'a>(&'a self) -> impl Iterator<Item = TriCoord> + 'a {
        struct NeighborIter<'a> { c: &'a TriCoord, iter: std::slice::Iter<'a, Direction> }
        impl<'a> Iterator for NeighborIter<'a> {
            type Item = TriCoord;
            fn next(&mut self) -> Option<Self::Item> { self.iter.next().map(|d| self.c.neighbor(d.clone())) }
        }
        NeighborIter { c: self, iter: DIRECTIONS.iter() }
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn zero_sum_invariant() {
        let mut current = HexCoord::origin();
        let mut rand = rand::thread_rng();
        for _ in 0..1000 {
            let dir = DIRECTIONS[rand.gen_range(0..DIRECTIONS.len())].clone();
            current = current.neighbor(dir);
        }
    }

    #[test]
    fn enumerate_neighbors() {
        let mut rand = rand::thread_rng();
        let (q, r) = (rand.gen_range(-100..100), rand.gen_range(-100..100));
        let point = HexCoord::new(q, r);
        let neighbors: Vec<_> = point.neighbors().collect();
        assert_eq!(6, neighbors.len());
        for (neighbor, dir) in neighbors.iter().zip(DIRECTIONS) {
            assert_eq!(point, neighbor.neighbor(dir.opposite()));
        }
    }
}

*/