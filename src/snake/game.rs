use std::collections::HashSet;

use rand::Rng;

use super::{Snake, SnakeDirection, entities::Entity};

/// Represents the state of the game.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GameState {
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

/// Holds all the data related to a game.
#[allow(unused)]
pub struct SnakeGame {
    state: GameState,
    pub score: u16,
    pub columns: i16,
    pub rows: i16,
    pub snake: Snake,
    private_value: &'static str, // Just for fun on docs.
    pub entities: Vec<Box<dyn Entity>>,
    game_board: Vec<(i16, i16)>,
}

impl SnakeGame {
    fn game_board(columns: &i16, rows: &i16) -> Vec<(i16, i16)> {
        let mut board = Vec::new();
        for x in 0..*columns {
            for y in 0..*rows {
                board.push((x, y));
            }
        }
        board
    }
    /// Returns a new game with [`GameState::New`] and the desired dimensions.
    /// # Examples
    /// ```
    /// # use snake3::{SnakeGame, GameState};
    /// let new_game = SnakeGame::new(10, 10, None, None);
    /// ```
    /// # Panics
    /// - If you try to create a snake exceeding the values of: ([columns](`SnakeGame::columns`), [rows](`SnakeGame::rows`)).
    pub fn new(
        columns: i16,
        rows: i16,
        snake_direction: Option<SnakeDirection>,
        starting_position: Option<(i16, i16)>,
    ) -> Self {
        let starting_position = starting_position.unwrap_or((columns / 2, rows / 2));
        if starting_position.0 > columns || starting_position.1 > rows {
            panic!("You can't create a snake outside of columns or rows range.")
        }
        SnakeGame {
            state: GameState::New,
            score: 0,
            private_value: "easter_egg",
            snake: Snake::new(
                starting_position,
                snake_direction.unwrap_or(SnakeDirection::Right),
            ),
            entities: Vec::new(),
            game_board: SnakeGame::game_board(&columns, &rows),
            columns,
            rows,
        }
    }
    /// Returns a tuple ([columns](`SnakeGame::columns`), [rows](`SnakeGame::rows`)).
    /// # Examples
    /// ```
    /// # use snake3::{SnakeGame, GameState};
    /// # let new_game = SnakeGame::new(10, 10, None, None);
    /// let (col, row) = new_game.dimensions();
    /// ```
    pub fn dimensions(&self) -> (i16, i16) {
        (self.columns, self.rows)
    }
    /// Change the game [state](`GameState`) to a new one.
    /// # Examples
    /// ```
    /// # use snake3::{SnakeGame, GameState};
    /// # let mut new_game = SnakeGame::new(10, 10, None, None);
    /// new_game.set_state(GameState::Playing);
    /// ```
    /// # Panics
    /// - Trying to set the state to [`GameState::New`].
    /// - Trying to set the game to anything after is beeing set to [`GameState::Ended`].
    /// - Trying to set twice the same state.
    pub fn set_state(&mut self, state: GameState) {
        if state == GameState::New {
            panic!("Can't set to New.")
        }
        if self.state == GameState::Ended {
            panic!("Can't set the sate after it is beeing set to Ended.")
        }
        if self.state == state {
            panic!("Can't set the same state twice.")
        }
        self.state = state
    }
    /// Returns the current state of the game.
    pub fn get_state(&self) -> GameState {
        self.state
    }
    /// Check if our snake is in contact with the wall or itself.
    pub fn check_collisions(&self) -> bool {
        // Are we hitting a wall
        let head = &self.snake.body[0];
        if head.x > self.columns || head.y > self.rows || head.x < 0 || head.y < 0 {
            return true;
        }
        // Is the snake eating itself
        self.snake.body[1..]
            .iter()
            .any(|point| point.x == head.x && point.y == head.y)
    }
    /// Randomly place a struct implementing [`Entity`] into the game [`SnakeGame::entities`].<br>
    /// If there was no space left to place an entity it returns `false`.
    /// # Examples
    /// ```
    /// # use snake3::{SnakeGame, GameState, named};
    /// # use snake3::snake::{self, Apple};
    /// # let mut new_game = SnakeGame::new(10, 10, None, None);
    /// new_game.generate_entity(named!(Apple));
    /// ```
    pub fn generate_entity<F>(&mut self, make_entity: F) -> bool
    where
        F: Fn(i16, i16) -> Box<dyn Entity>,
    {
        let empty_spots = self.empty_spots();
        if empty_spots.is_empty() {
            return false;
        }
        let mut rng = rand::rng();
        let new_position = empty_spots[rng.random_range(0..empty_spots.len())];
        let entity = make_entity(new_position.0, new_position.1);
        self.entities.push(entity);
        true
    }
    /// If the snake head collides with an [`Entity`] it gets removed from the [`SnakeGame::entities`] and <br>
    /// is returned to us as an [`Option`] so that we can check what action to take.
    /// # Examples
    /// ```
    /// # use snake3::{SnakeGame, GameState, named};
    /// # use snake3::snake::{self, Apple};
    /// # let mut new_game = SnakeGame::new(10, 10, None, None);
    /// if let Some(hit) = new_game.check_entity_collision() {
    ///     if let Some(apple) = hit.downcast_ref::<Apple>() {
    ///         new_game.snake.grow();
    ///         new_game.score += 1;
    ///     }
    ///     // If we had a `Bomb` struct that implemented `Entity`
    ///     // if let Some(bomb) = hid.downcast_ref::<Bomb>() {}
    /// }
    /// ```
    pub fn check_entity_collision(&mut self) -> Option<Box<dyn Entity>> {
        let mut remove_index = None;

        for (i, entity) in self.entities.iter().enumerate() {
            if self.snake.body[0].x == entity.x() && self.snake.body[0].y == entity.y() {
                remove_index = Some(i);
                break;
            }
        }

        remove_index.map(|i| self.entities.remove(i))
    }
    fn empty_spots(&self) -> Vec<(i16, i16)> {
        let snake_set: HashSet<(i16, i16)> =
            self.snake.body.iter().map(|seg| (seg.x, seg.y)).collect();
        self.game_board
            .iter()
            .cloned()
            .filter(|pos| !snake_set.contains(pos))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        named,
        snake::{Apple, snake_obj::SnakeBodyPoint},
    };

    use super::*;

    #[test]
    fn snake_game_game_board() {
        let columns = 2;
        let rows = 2;
        let game_board = SnakeGame::game_board(&columns, &rows);
        assert_eq!(game_board.len(), 4);
        assert_eq!(game_board, vec![(0, 0), (0, 1), (1, 0), (1, 1)]);
    }

    #[test]
    fn snake_game_new() {
        let new_game = SnakeGame::new(42, 24, None, None);
        assert_eq!(new_game.score, 0);
        assert_eq!(new_game.columns, 42);
        assert_eq!(new_game.rows, 24);
        assert_eq!(new_game.state, GameState::New);
        assert_eq!(new_game.private_value, "easter_egg");
    }

    #[test]
    fn snake_game_new_custom_direction_and_starting_positions() {
        let new_game = SnakeGame::new(42, 24, Some(SnakeDirection::Left), Some((10, 20)));
        assert_eq!(new_game.snake.get_direction(), SnakeDirection::Left);
        assert_eq!(new_game.snake.body[0].x, 10);
        assert_eq!(new_game.snake.body[0].y, 20)
    }

    #[test]
    #[should_panic(expected = "You can't create a snake outside of columns or rows range.")]
    fn snake_game_new_custom_position_out_of_range() {
        let _new_game = SnakeGame::new(42, 24, Some(SnakeDirection::Left), Some((10, 25)));
    }

    #[test]
    fn snake_game_dimensions() {
        let new_game = SnakeGame::new(42, 24, None, None);
        assert_eq!(new_game.dimensions(), (42, 24))
    }

    #[test]
    fn snake_game_get_state() {
        let new_game = SnakeGame::new(42, 24, None, None);
        assert_eq!(new_game.get_state(), GameState::New)
    }

    #[test]
    fn snake_game_set_state_valid_path() {
        let mut new_game = SnakeGame::new(42, 24, None, None);

        new_game.set_state(GameState::Playing);
        assert_eq!(new_game.state, GameState::Playing);

        new_game.set_state(GameState::Paused);
        assert_eq!(new_game.state, GameState::Paused);

        new_game.set_state(GameState::Playing);
        assert_eq!(new_game.state, GameState::Playing);

        new_game.set_state(GameState::Paused);
        assert_eq!(new_game.state, GameState::Paused);

        new_game.set_state(GameState::Playing);
        assert_eq!(new_game.state, GameState::Playing);

        new_game.set_state(GameState::Ended);
        assert_eq!(new_game.state, GameState::Ended)
    }

    #[test]
    #[should_panic(expected = "Can't set to New.")]
    fn snake_game_set_state_new() {
        let mut new_game = SnakeGame::new(42, 24, None, None);
        new_game.set_state(GameState::New)
    }

    #[test]
    #[should_panic(expected = "Can't set the sate after it is beeing set to Ended.")]
    fn snake_game_set_state_ended() {
        let mut new_game = SnakeGame::new(42, 24, None, None);
        new_game.set_state(GameState::Ended);
        new_game.set_state(GameState::Playing)
    }

    #[test]
    #[should_panic(expected = "Can't set the same state twice.")]
    fn snake_game_set_state_same() {
        let mut new_game = SnakeGame::new(42, 24, None, None);
        new_game.set_state(GameState::Playing);
        new_game.set_state(GameState::Playing)
    }

    #[test]
    fn snake_game_check_collisions_false() {
        let new_game = SnakeGame::new(42, 24, None, None);
        assert_eq!(false, new_game.check_collisions())
    }

    #[test]
    fn snake_game_check_collisions_wall() {
        let mut new_game = SnakeGame::new(42, 24, Some(SnakeDirection::Left), Some((0, 0)));
        new_game.snake.body[0].y = -1;
        assert_eq!(true, new_game.check_collisions());
        new_game.snake.body[0].y = 0;
        new_game.snake.body[0].x = new_game.columns + 1;
        assert_eq!(true, new_game.check_collisions());
        new_game.snake.body[0].y = new_game.rows + 1;
        new_game.snake.body[0].x = 0;
        assert_eq!(true, new_game.check_collisions());
        new_game.snake.body[0].y = 0;
        new_game.snake.body[0].x = -1;
        assert_eq!(true, new_game.check_collisions())
    }

    #[test]
    fn snake_game_check_collisions_self() {
        let mut new_game = SnakeGame::new(42, 24, Some(SnakeDirection::Left), Some((10, 10)));
        new_game.snake.body.push(SnakeBodyPoint { x: 9, y: 10 });
        new_game.snake.body.push(SnakeBodyPoint { x: 9, y: 11 });
        new_game.snake.advance();
        assert_eq!(true, new_game.check_collisions())
    }

    #[test]
    fn snake_game_generate_entity() {
        let mut new_game = SnakeGame::new(42, 24, None, None);
        assert_eq!(new_game.entities.len(), 0);
        new_game.generate_entity(named!(Apple));
        assert_eq!(new_game.entities.len(), 1);
    }

    #[test]
    fn snake_game_generate_entity_no_space() {
        let mut game = SnakeGame::new(1, 1, None, None);
        assert_eq!(game.empty_spots().len(), 0);
        let result = game.generate_entity(named!(Apple));
        assert!(!result);
        assert_eq!(game.entities.len(), 0);
    }

    #[test]
    fn snake_game_check_entity_collision() {
        let mut new_game = SnakeGame::new(42, 24, None, None);
        new_game.generate_entity(named!(Apple));
        assert_eq!(new_game.entities.len(), 1);
        let current_entity = &new_game.entities[0];
        new_game.snake.body[0].x = current_entity.x();
        new_game.snake.body[0].y = current_entity.y();
        new_game.check_entity_collision();
        assert_eq!(new_game.entities.len(), 0);
    }

    #[test]
    fn snake_game_empty_spots() {
        let new_game = SnakeGame::new(2, 2, None, None);
        assert_eq!(new_game.empty_spots(), vec![(0, 0), (0, 1), (1, 0)])
    }
}
