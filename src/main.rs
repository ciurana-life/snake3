#[cfg(not(target_arch = "wasm32"))]
pub mod native {
    use crossterm::{
        ExecutableCommand,
        cursor::{Hide, MoveTo, Show},
        event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
        style::{self, Stylize},
        terminal::{self, Clear, ClearType},
    };
    use snake3::{
        GameState, SnakeGame, named,
        snake::{Apple, SnakeDirection},
    };
    use std::io::{self, Stdout, Write};
    use std::time::Duration;

    pub enum InputAction {
        Continue,
        Restart,
        Quit,
    }

    pub fn main() -> io::Result<()> {
        let mut stdout = io::stdout();
        enable_game_mode(&mut stdout)?;

        let (cols, rows) = get_terminal_size(&mut stdout);

        'main_loop: loop {
            clear_terminal(&mut stdout)?;

            let mut timer = 500;
            let mut snake_game = SnakeGame::new(cols as i16, rows as i16, None, None);
            snake_game.generate_entity(named!(Apple));
            snake_game.set_state(GameState::Playing);

            // GAME LOOP
            loop {
                // CLEAR
                clear_frame(&mut stdout, rows)?;
                // DRAW
                draw_frame(&mut stdout, &snake_game)?;
                // INPUT
                match handle_input(&mut snake_game, timer)? {
                    InputAction::Continue => {}
                    InputAction::Restart => break,
                    InputAction::Quit => break 'main_loop,
                }
                // LOGIC
                game_logic(&mut snake_game, &mut timer)?;
            }
        }

        disable_game_mode(&mut stdout)?;
        println!("The game was closed, have a nice day :)");
        Ok(())
    }

    fn clear_frame(stdout: &mut Stdout, rows: u16) -> io::Result<()> {
        for y in 0..rows + 1 {
            stdout
                .execute(MoveTo(0, y))?
                .execute(Clear(ClearType::CurrentLine))?;
        }
        Ok(())
    }

    fn draw_frame(stdout: &mut Stdout, snake_game: &SnakeGame) -> io::Result<()> {
        // Snake
        if snake_game.get_state() != GameState::Ended {
            for i in 0..snake_game.snake.body.len() {
                let current = &snake_game.snake.body[i];
                let ch = if i == 0 {
                    match snake_game.snake.get_direction() {
                        SnakeDirection::Up => 'v',
                        SnakeDirection::Down => '^',
                        SnakeDirection::Left => '<',
                        SnakeDirection::Right => '>',
                    }
                } else {
                    let prev = &snake_game.snake.body[i - 1];
                    if current.x == prev.x {
                        '|'
                    } else if current.y == prev.y {
                        '-'
                    } else {
                        's'
                    }
                };

                stdout
                    .execute(MoveTo(current.x as u16, current.y as u16))?
                    .execute(style::PrintStyledContent(ch.green()))?;
            }
            // Entities
            for entity in &snake_game.entities {
                stdout
                    .execute(MoveTo(entity.x() as u16, entity.y() as u16))?
                    .execute(style::PrintStyledContent("o".red()))?;
            }
        }

        // Paused screen
        if snake_game.get_state() == GameState::Paused {
            let x_third = (snake_game.rows / 3) as u16;
            let y_third = (snake_game.columns / 3) as u16;
            let lines = "*".repeat(y_third as usize);
            let lines2 = "*".repeat(y_third as usize);
            let text = "Game is puased";
            let text2 = "press <p> to resume";
            stdout
                .execute(MoveTo(y_third + 2, x_third + 1))?
                .execute(style::PrintStyledContent(text.red()))?
                .execute(MoveTo(y_third + 2, x_third + 2))?
                .execute(style::PrintStyledContent(text2.red()))?
                .execute(MoveTo(y_third, x_third - 1))?
                .execute(style::PrintStyledContent(lines.red()))?
                .execute(MoveTo(y_third, x_third + 4))?
                .execute(style::PrintStyledContent(lines2.red()))?;
        }

        // Game ended
        if snake_game.get_state() == GameState::Ended {
            let end_text_1 = format!(
                "Your game ended with a score of {} points",
                snake_game.score
            );
            let end_text_2 = "Press <y> to play a new game, to close press <q>";
            stdout
                .execute(MoveTo(0, 0))?
                .execute(style::PrintStyledContent(end_text_1.red()))?
                .execute(MoveTo(0, 1))?
                .execute(style::PrintStyledContent(end_text_2.red()))?;
        }

        // Info text
        let snake_rows = snake_game.rows as u16;
        let info_text = "Move with keyboard arrows, press <q> or <Ctrl+C> to exit, press <p> to pause and resume.";
        let division = "-".repeat(snake_game.columns as usize);
        let score = format!("Score: {}", &snake_game.score.to_string());
        stdout
            .execute(MoveTo(0, snake_rows + 1))?
            .execute(style::PrintStyledContent(division.dark_grey()))?;
        stdout
            .execute(MoveTo(0, snake_rows + 3))?
            .execute(style::PrintStyledContent(info_text.dark_grey()))?;
        stdout
            .execute(MoveTo(0, snake_rows + 2))?
            .execute(style::PrintStyledContent(score.cyan()))?;
        stdout.flush()?;
        Ok(())
    }

    fn handle_input(snake_game: &mut SnakeGame, timer: u64) -> io::Result<InputAction> {
        if event::poll(Duration::from_millis(timer))? {
            if let Event::Key(KeyEvent {
                code, modifiers, ..
            }) = event::read()?
            {
                match code {
                    KeyCode::Char('q') => return Ok(InputAction::Quit),
                    KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(InputAction::Quit);
                    }
                    KeyCode::Left => {
                        snake_game.snake.set_direction(SnakeDirection::Left);
                    }
                    KeyCode::Right => {
                        snake_game.snake.set_direction(SnakeDirection::Right);
                    }
                    KeyCode::Up => {
                        snake_game.snake.set_direction(SnakeDirection::Down);
                    }
                    KeyCode::Down => {
                        snake_game.snake.set_direction(SnakeDirection::Up);
                    }
                    KeyCode::Char('p') => {
                        if snake_game.get_state() == GameState::Playing {
                            snake_game.set_state(GameState::Paused);
                        } else if snake_game.get_state() == GameState::Paused {
                            snake_game.set_state(GameState::Playing);
                        }
                    }
                    KeyCode::Char('y') if snake_game.get_state() == GameState::Ended => {
                        return Ok(InputAction::Restart);
                    }
                    _ => {}
                }
            }
        }
        Ok(InputAction::Continue)
    }

    fn game_logic(snake_game: &mut SnakeGame, timer: &mut u64) -> io::Result<()> {
        if snake_game.get_state() == GameState::Playing {
            snake_game.snake.advance();
            if snake_game.check_collisions() {
                snake_game.set_state(GameState::Ended);
                return Ok(());
            };
            if let Some(hit) = snake_game.check_entity_collision() {
                if let Some(_apple) = hit.downcast_ref::<Apple>() {
                    snake_game.snake.grow();
                    snake_game.score += 1;
                    if *timer > 100 {
                        *timer -= 10;
                    }
                }
            }
            if snake_game.entities.is_empty() && !snake_game.generate_entity(named!(Apple)) {
                snake_game.set_state(GameState::Ended);
                return Ok(());
            }
        }
        Ok(())
    }

    fn get_terminal_size(stdout: &mut Stdout) -> (u16, u16) {
        let (cols, rows) = terminal::size().unwrap_or((0, 0));
        if cols < 84 || rows < 24 {
            let _ = disable_game_mode(stdout);
            println!("\n*****\n");
            println!("You should have a minimum 84x28 terminal size in terms of rows and columns");
            println!("but you have {rows} rows and {cols} columns");
            println!("please resize your terminal and try again");
            println!("\n*****\n");
            std::process::exit(1);
        } else {
            let cols = cols - 1; // Collision does not have to wait an extra frame
            let rows = rows - 4; // Space for text
            (cols, rows)
        }
    }

    fn clear_terminal(stdout: &mut Stdout) -> io::Result<()> {
        stdout
            .execute(Clear(ClearType::All))?
            .execute(Clear(ClearType::Purge))?
            .execute(MoveTo(0, 0))?;
        Ok(())
    }

    fn enable_game_mode(stdout: &mut Stdout) -> io::Result<()> {
        // Prevents input to be forwaded to the screen but also disables Ctrl+C
        terminal::enable_raw_mode()?;
        // Hide the cursor
        stdout.execute(Hide)?;
        // Full clear
        stdout
            .execute(Clear(ClearType::All))?
            .execute(Clear(ClearType::Purge))?;
        Ok(())
    }

    fn disable_game_mode(stdout: &mut Stdout) -> io::Result<()> {
        // Enable normal input again
        terminal::disable_raw_mode()?;
        // Show cursor again
        stdout.execute(Show)?;
        // Clear terminal screen
        clear_terminal(stdout)?;
        Ok(())
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> std::io::Result<()> {
    native::main()
}

#[cfg(target_arch = "wasm32")]
fn main() {
    println!(":)");
}
