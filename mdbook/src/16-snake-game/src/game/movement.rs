use super::Coords;

/// Define the directions the snake can move.
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// What direction the snake should turn.
#[derive(Debug, Copy, Clone)]
pub enum Turn {
    Left,
    Right,
    None,
}

/// The current status of the game.
pub enum GameStatus {
    Won,
    Lost,
    Ongoing,
}

/// The outcome of a single move/step.
pub enum StepOutcome {
    /// Grid full (player wins)
    Full,
    /// Snake has collided with itself (player loses)
    Collision,
    /// Snake has eaten some food
    Eat(Coords),
    /// Snake has moved (and nothing else has happened)
    Move(Coords),
}
