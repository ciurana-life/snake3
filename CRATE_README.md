# snake3 🐍

This crate gives you the building blocks to create the classical snake game and at the same time allows you to introduce new entities to the game and define how they interact with the snake.

## Creating a new game
```rust
use snake3::SnakeGame;
let cols = 10;
let rows = 10;
let mut snake_game = SnakeGame::new(
    cols,
    rows,
    None, // snake_direction
    None  // starting_position
);
```

[`SnakeGame::new`] has 2 optional parameters that will get filled with some default values if not provided:
* `snake_direction` will default to [`snake::SnakeDirection::Right`].
* `starting_position` will default to the tuple `(cols/2, rows/2)`.

## Moving the snake
Using the method `snake::Snake::set_direction` we can change where the snake is headed:
``` rust
use snake3::snake::SnakeDirection;
snake_game.snake.set_direction(SnakeDirection::Up);
```
And then on our game loop we can call `snake::Snake::advance` to move in the las set direction:
``` rust
snake_game.snake.advance();
```

## Dealing with collisions
After we have advanced we have to check if we are hitting a wall, ourselfs or any other entity:
``` rust
use snake3::snake::Apple;
// Did we hit ourselfs or the wall?
if snake_game.check_collisions() {
    // End the game or custom logic
};
// Did we hit an entity?
if let Some(hit) = snake_game.check_entity_collision() {
    if let Some(_apple) = hit.downcast_ref::<Apple>() {
        // Make the snake bigger and add +1 to the score
    }
    // Or check for your custom entity
}
```

## Adding entities and customization
You can randomly add entities to the game with:
``` rust
new_game.generate_entity(named!(Apple));
```

The default game comes just with the [`snake::Apple`] entity, but you can add as many as you want, in the above examples you learned how to create and check for entities, here is how to add your own:
``` rust
pub struct Bomb {
    pub x: i16,
    pub y: i16,
}
impl_entity!(Bomb);

let mut new_game = SnakeGame::new(10, 10, None, None);
new_game.generate_entity(named!(Bomb));
```

## Working example
You can see an example implementation that runs in the terminal in [the repo](<https://github.com/ciurana-life/snake3/blob/main/src/main.rs>).

## WASM support
It uses the macroquad random module.
```bash
cargo build --release --target wasm32-unknown-unknown
```