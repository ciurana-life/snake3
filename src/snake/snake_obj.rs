/// Every tick of the game we move to the current direction <br>
/// the snake is pointing at, this is changed by player movement.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SnakeDirection {
    Up,
    Down,
    Left,
    Right,
}

impl SnakeDirection {
    pub fn is_opposite(&self, other: &SnakeDirection) -> bool {
        matches!(
            (self, other),
            (SnakeDirection::Up, SnakeDirection::Down)
                | (SnakeDirection::Down, SnakeDirection::Up)
                | (SnakeDirection::Left, SnakeDirection::Right)
                | (SnakeDirection::Right, SnakeDirection::Left)
        )
    }
}

/// Point of the snake on the game grid.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct SnakeBodyPoint {
    pub x: i16,
    pub y: i16,
}

/// Player.
pub struct Snake {
    direction: SnakeDirection,
    pub body: Vec<SnakeBodyPoint>,
}

impl Snake {
    pub fn new(starting_position: (i16, i16), initial_direction: SnakeDirection) -> Self {
        let (x, y) = starting_position;
        let first_body_part = SnakeBodyPoint { x, y };
        Snake {
            direction: initial_direction,
            body: vec![first_body_part],
        }
    }
    pub fn get_direction(&self) -> SnakeDirection {
        self.direction
    }
    pub fn set_direction(&mut self, new_direction: SnakeDirection) {
        if !self.direction.is_opposite(&new_direction) {
            self.direction = new_direction;
        }
    }
    /// Removes the last body point from [`Snake::body`] and adds a new <br>
    /// one in the current snake direction.
    pub fn advance(&mut self) {
        let head: SnakeBodyPoint = self.body[0];
        let new_head = match self.direction {
            SnakeDirection::Up => SnakeBodyPoint {
                x: head.x,
                y: head.y + 1,
            },
            SnakeDirection::Down => SnakeBodyPoint {
                x: head.x,
                y: head.y - 1,
            },
            SnakeDirection::Left => SnakeBodyPoint {
                x: head.x - 1,
                y: head.y,
            },
            SnakeDirection::Right => SnakeBodyPoint {
                x: head.x + 1,
                y: head.y,
            },
        };
        self.body.insert(0, new_head);
        self.body.pop();
    }
    /// Adds a new body point to [`Snake::body`].
    pub fn grow(&mut self) {
        if self.body.len() < 2 {
            let tail = self.body[0];
            let new_tail = match self.direction {
                SnakeDirection::Up => SnakeBodyPoint {
                    x: tail.x,
                    y: tail.y + 1,
                },
                SnakeDirection::Down => SnakeBodyPoint {
                    x: tail.x,
                    y: tail.y - 1,
                },
                SnakeDirection::Left => SnakeBodyPoint {
                    x: tail.x + 1,
                    y: tail.y,
                },
                SnakeDirection::Right => SnakeBodyPoint {
                    x: tail.x - 1,
                    y: tail.y,
                },
            };
            self.body.push(new_tail);
        } else {
            let last = self.body[self.body.len() - 1];
            let before_last = self.body[self.body.len() - 2];
            let dx = last.x - before_last.x;
            let dy = last.y - before_last.y;
            let new_tail = SnakeBodyPoint {
                x: last.x + dx,
                y: last.y + dy,
            };
            self.body.push(new_tail);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn snake_advance() {
        let mut snake = Snake::new((10, 10), SnakeDirection::Down);
        // Move down
        assert_eq!(snake.body[0].x, 10);
        assert_eq!(snake.body[0].y, 10);
        snake.advance();
        assert_eq!(snake.body[0].x, 10);
        assert_eq!(snake.body[0].y, 9);
        // Move Up
        snake.direction = SnakeDirection::Up;
        snake.advance();
        assert_eq!(snake.body[0].x, 10);
        assert_eq!(snake.body[0].y, 10);
        // Move left
        snake.direction = SnakeDirection::Left;
        snake.advance();
        assert_eq!(snake.body[0].x, 9);
        assert_eq!(snake.body[0].y, 10);
        // Move right
        snake.direction = SnakeDirection::Right;
        snake.advance();
        assert_eq!(snake.body[0].x, 10);
        assert_eq!(snake.body[0].y, 10);
    }

    #[test]
    fn snake_set_direction() {
        let mut snake = Snake::new((10, 10), SnakeDirection::Down);
        // Prevent oposites
        snake.set_direction(SnakeDirection::Up);
        assert_eq!(snake.get_direction(), SnakeDirection::Down);

        snake.set_direction(SnakeDirection::Left);
        snake.set_direction(SnakeDirection::Up);
        snake.set_direction(SnakeDirection::Down);
        assert_eq!(snake.get_direction(), SnakeDirection::Up);

        snake.set_direction(SnakeDirection::Left);
        snake.set_direction(SnakeDirection::Right);
        assert_eq!(snake.get_direction(), SnakeDirection::Left);

        snake.set_direction(SnakeDirection::Up);
        snake.set_direction(SnakeDirection::Right);
        snake.set_direction(SnakeDirection::Left);
        assert_eq!(snake.get_direction(), SnakeDirection::Right);

        // Change direction and same is fine
        snake.set_direction(SnakeDirection::Right);
        assert_eq!(snake.get_direction(), SnakeDirection::Right);

        snake.set_direction(SnakeDirection::Up);
        snake.set_direction(SnakeDirection::Up);
        assert_eq!(snake.get_direction(), SnakeDirection::Up);

        snake.set_direction(SnakeDirection::Left);
        snake.set_direction(SnakeDirection::Left);
        assert_eq!(snake.get_direction(), SnakeDirection::Left);

        snake.set_direction(SnakeDirection::Down);
        snake.set_direction(SnakeDirection::Down);
        assert_eq!(snake.get_direction(), SnakeDirection::Down);
    }

    #[test]
    fn snake_grow_when_snake_has_one_segment() {
        let mut snake = Snake::new((5, 5), SnakeDirection::Right);
        assert_eq!(snake.body.len(), 1);

        snake.grow();
        assert_eq!(snake.body.len(), 2);

        let expected_tail = SnakeBodyPoint { x: 4, y: 5 };
        assert_eq!(snake.body[1], expected_tail);
    }

    #[test]
    fn snake_grow_when_snake_has_multiple_segments() {
        let mut snake = Snake::new((5, 5), SnakeDirection::Right);
        // Manually extend the body to simulate a moving snake
        snake.body.push(SnakeBodyPoint { x: 4, y: 5 });
        snake.body.push(SnakeBodyPoint { x: 3, y: 5 });
        assert_eq!(snake.body.len(), 3);

        snake.grow();
        assert_eq!(snake.body.len(), 4);

        let expected_tail = SnakeBodyPoint { x: 2, y: 5 };
        assert_eq!(snake.body[3], expected_tail);
    }

    #[test]
    fn snake_grow_tail_extension_correctness_for_vertical_snake() {
        let mut snake = Snake::new((5, 5), SnakeDirection::Up);
        snake.body.push(SnakeBodyPoint { x: 5, y: 6 });
        snake.body.push(SnakeBodyPoint { x: 5, y: 7 });

        snake.grow();
        assert_eq!(snake.body.len(), 4);

        let expected_tail = SnakeBodyPoint { x: 5, y: 8 };
        assert_eq!(snake.body[3], expected_tail);
    }

    #[test]
    fn snake_grow_multiple_times() {
        let mut snake = Snake::new((10, 10), SnakeDirection::Left);
        snake.grow();
        snake.grow();
        snake.grow();

        assert_eq!(snake.body.len(), 4);
        // Check the last segment is consistent with direction
        let l = snake.body.len();
        let dx = snake.body[l - 1].x - snake.body[l - 2].x;
        let dy = snake.body[l - 1].y - snake.body[l - 2].y;

        assert_eq!((dx, dy), (1, 0));
    }

    #[test]
    fn snake_grow_with_one_segment_up() {
        let mut snake = Snake::new((3, 3), SnakeDirection::Up);
        snake.grow();

        assert_eq!(snake.body.len(), 2);
        let expected_tail = SnakeBodyPoint { x: 3, y: 4 };
        assert_eq!(snake.body[1], expected_tail);
    }

    #[test]
    fn snake_grow_with_one_segment_down() {
        let mut snake = Snake::new((3, 3), SnakeDirection::Down);
        snake.grow();

        assert_eq!(snake.body.len(), 2);
        let expected_tail = SnakeBodyPoint { x: 3, y: 2 };
        assert_eq!(snake.body[1], expected_tail);
    }

    #[test]
    fn snake_grow_with_one_segment_left() {
        let mut snake = Snake::new((3, 3), SnakeDirection::Left);
        snake.grow();

        assert_eq!(snake.body.len(), 2);
        let expected_tail = SnakeBodyPoint { x: 4, y: 3 };
        assert_eq!(snake.body[1], expected_tail);
    }
}
