use std::any::Any;

use crate::impl_entity;

/// Allows new entities to be created and added to the game. <br>
/// # Examples
/// ```
/// # use snake3::{SnakeGame, GameState, named, impl_entity};
/// # use snake3::snake::{self, Apple, Entity};
/// pub struct Bomb {
///     pub x: i16,
///     pub y: i16,
/// }
/// impl_entity!(Bomb);
///
/// let mut new_game = SnakeGame::new(10, 10, None, None);
/// new_game.generate_entity(named!(Bomb));
/// ```
pub trait Entity: Any {
    fn as_any(&self) -> &dyn Any;
    fn x(&self) -> i16;
    fn y(&self) -> i16;
}

impl dyn Entity {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Apple {
    pub x: i16,
    pub y: i16,
}

impl_entity!(Apple);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_downcast_ref_correct_type() {
        let apple = Apple { x: 3, y: 7 };
        let entity: &dyn Entity = &apple;

        let downcasted = entity.downcast_ref::<Apple>();
        assert!(downcasted.is_some());
        let unwrapped = downcasted.unwrap();
        assert_eq!(unwrapped.x, 3);
        assert_eq!(unwrapped.y, 7);
    }

    #[test]
    fn test_downcast_ref_wrong_type() {
        #[derive(Debug)]
        struct DummyEntity;
        impl Entity for DummyEntity {
            fn as_any(&self) -> &dyn Any {
                self
            }
            fn x(&self) -> i16 {
                0
            }
            fn y(&self) -> i16 {
                0
            }
        }

        let dummy = DummyEntity;
        let entity: &dyn Entity = &dummy;

        // Attempt to downcast to Apple, which should fail
        let downcasted = entity.downcast_ref::<Apple>();
        assert!(downcasted.is_none());
    }
}
