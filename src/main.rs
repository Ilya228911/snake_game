use std::io::{stdout, Write};
use std::time::{Duration, Instant};
use std::thread;
use rand::Rng;
use crossterm::{
    cursor,
    event::{self, KeyCode, KeyEvent},
    execute,
    terminal::{self, ClearType},
};

const SCREEN_WIDTH: u16 = 40;
const SCREEN_HEIGHT: u16 = 20;
const INITIAL_SNAKE_LENGTH: usize = 3;
const UPDATE_INTERVAL: Duration = Duration::from_millis(100);
const RENDER_INTERVAL: Duration = Duration::from_millis(10);

fn generate_food(snake: &[(u16, u16)]) -> (u16, u16) {
    let mut rng = rand::thread_rng();
    loop {
        let new_food = (rng.gen_range(1..SCREEN_HEIGHT - 1), rng.gen_range(1..SCREEN_WIDTH - 1));
        if !snake.contains(&new_food) {
            return new_food;
        }
    }
}

fn main() {
    let mut stdout = stdout();
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide).unwrap();
    terminal::enable_raw_mode().unwrap();

    let mut snake: Vec<(u16, u16)> = vec![];
    for i in 0..INITIAL_SNAKE_LENGTH {
        snake.push((SCREEN_HEIGHT / 2, SCREEN_WIDTH / 4 - i as u16));
    }

    let mut food = generate_food(&snake);
    let mut score = 0;
    let mut direction = KeyCode::Right;

    let mut last_update_time = Instant::now();
    let mut last_render_time = Instant::now();

    loop {
        if event::poll(Duration::from_millis(0)).unwrap() {
            if let event::Event::Key(key_event) = event::read().unwrap() {
                direction = match key_event.code {
                    KeyCode::Left if direction != KeyCode::Right => KeyCode::Left,
                    KeyCode::Right if direction != KeyCode::Left => KeyCode::Right,
                    KeyCode::Up if direction != KeyCode::Down => KeyCode::Up,
                    KeyCode::Down if direction != KeyCode::Up => KeyCode::Down,
                    _ => direction,
                };
            }
        }

        if last_update_time.elapsed() >= UPDATE_INTERVAL {
            let head = snake[0];
            let new_head = match direction {
                KeyCode::Right => (head.0, head.1 + 1),
                KeyCode::Left => (head.0, head.1 - 1),
                KeyCode::Up => (head.0 - 1, head.1),
                KeyCode::Down => (head.0 + 1, head.1),
                _ => head,
            };
            
            if new_head == food {
                score += 1;
                food = generate_food(&snake);
            } else {
                snake.pop();
            }
            
            if snake.contains(&new_head) || new_head.0 == 0 || new_head.0 == SCREEN_HEIGHT - 1 ||
                new_head.1 == 0 || new_head.1 == SCREEN_WIDTH - 1 {
                break;
            }
            
            snake.insert(0, new_head);
            last_update_time = Instant::now();
        }

        if last_render_time.elapsed() >= RENDER_INTERVAL {
            execute!(stdout, cursor::MoveTo(0, 0), terminal::Clear(ClearType::All)).unwrap();
            
            for y in 0..SCREEN_HEIGHT {
                for x in 0..SCREEN_WIDTH {
                    if (y == 0 || y == SCREEN_HEIGHT - 1) || (x == 0 || x == SCREEN_WIDTH - 1) {
                        print!("#");
                    } else if snake.contains(&(y, x)) {
                        print!("O");
                    } else if (y, x) == food {
                        print!("*");
                    } else {
                        print!(" ");
                    }
                }
                println!("");
            }
            println!("Score: {}", score);
            
            last_render_time = Instant::now();
        }

        thread::sleep(Duration::from_millis(1));
    }

    execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show).unwrap();
}
