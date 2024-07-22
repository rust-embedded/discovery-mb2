use super::{Coords, Direction, FnvIndexSet, Turn};

use heapless::spsc::Queue;

pub struct Snake {
    /// Coordinates of the snake's head.
    pub head: Coords,
    /// Queue of coordinates of the rest of the snake's body. The end of the tail is
    /// at the front.
    pub tail: Queue<Coords, 32>,
    /// A set containing all coordinates currently occupied by the snake (for fast
    /// collision checking).
    pub coord_set: FnvIndexSet<Coords, 32>,
    /// The direction the snake is currently moving in.
    pub direction: Direction,
}

impl Snake {
    pub fn make_snake() -> Self {
        let head = Coords { row: 2, col: 2 };
        let initial_tail = Coords { row: 2, col: 1 };
        let mut tail = Queue::new();
        tail.enqueue(initial_tail).unwrap();
        let mut coord_set: FnvIndexSet<Coords, 32> = FnvIndexSet::new();
        coord_set.insert(head).unwrap();
        coord_set.insert(initial_tail).unwrap();
        Self {
            head,
            tail,
            coord_set,
            direction: Direction::Right,
        }
    }

    /// Move the snake onto the tile at the given coordinates. If `extend` is false,
    /// the snake's tail vacates the rearmost tile.
    pub fn move_snake(&mut self, coords: Coords, extend: bool) {
        // Location of head becomes front of tail
        self.tail.enqueue(self.head).unwrap();
        // Head moves to new coords
        self.head = coords;
        self.coord_set.insert(coords).unwrap();
        if !extend {
            let back = self.tail.dequeue().unwrap();
            self.coord_set.remove(&back);
        }
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn turn_left(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    pub fn turn(&mut self, direction: Turn) {
        match direction {
            Turn::Left => self.turn_left(),
            Turn::Right => self.turn_right(),
            Turn::None => (),
        }
    }
}
