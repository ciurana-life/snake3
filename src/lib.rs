#![allow(dead_code, unused)]

/// Represents the state of the game.
#[derive(Debug, PartialEq)]
pub enum SnakeGameState {
    /// A new game that has not started yet.
    New,
    /// The game is currently in progress and the ticker should be counting.
    Playing,
    /// The game is paused; the ticker should not be counting.
    Paused,
    /// The game has ended due to player failure or a perfect run
    /// at this point, points should be saved and a new game can start.
    Ended,
}

/// Holds all the data related to a game
pub struct SnakeGame {
    pub state: SnakeGameState,
    pub current_score: u16,
    pub columns: u16,
    pub rows: u16,
    private_value: &'static str, // Just for fun on docs.
}

impl SnakeGame {
    /// Returns a new game with [`SnakeGameState::New`] and the desired dimensions.
    /// # Examples
    /// ```
    /// use snake3::{SnakeGame, SnakeGameState};
    ///
    /// let new_game = SnakeGame::new(10, 10);
    /// assert_eq!(new_game.state, SnakeGameState::New);
    /// assert_eq!(new_game.dimensions(), (10, 10));
    /// ```
    pub fn new(columns: u16, rows: u16) -> Self {
        SnakeGame {
            state: SnakeGameState::New,
            current_score: 0,
            private_value: "easter_egg",
            columns,
            rows,
        }
    }
    /// Returns a tuple ([columns](`SnakeGame::columns`), [rows](`SnakeGame::rows`))
    pub fn dimensions(&self) -> (u16, u16) {
        (self.columns, self.rows)
    }
}
