pub mod entities;
pub mod game;
pub mod macros;
pub mod snake_obj;

pub use entities::{Apple, Entity};
pub use game::{GameState, SnakeGame};
pub use snake_obj::{Snake, SnakeDirection};
