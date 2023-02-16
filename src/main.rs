use ruscii::app::{App, State};
use ruscii::drawing::{Pencil, RectCharset};
use ruscii::gui::FPSCounter;
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::Vec2;
use ruscii::terminal::Window;

use rand::{self, prelude::*};

struct SnakeState {
    pub head: Vec2,
    pub direction: Vec2,
    pub speed: usize,
    pub tail: Vec<Vec2>,
}

impl SnakeState {
    pub fn update(&mut self, frame: usize) {
        if frame % (30 / self.speed) == 0 {
            let mut last_position = self.head.clone();
            self.head += self.direction;
            for part in self.tail.iter_mut() {
                let aux = part.clone();
                *part = last_position;
                last_position = aux.clone();
            }
        }
    }

    pub fn draw(&mut self, mut pencil: Pencil) {
        pencil.draw_char('@', self.head);

        for part in self.tail.iter_mut() {
            pencil.draw_char('#', *part);
        }
    }

    pub fn add_tail(&mut self, position: Vec2) {
        self.tail.push(position)
    }
}

struct GameState {
    pub snake: SnakeState,
    pub food_position: Vec2,
}

impl GameState {
    pub fn new(winsize: Vec2) -> Self {
        let center_of_win = Vec2::xy(winsize.x / 2, winsize.y / 2);
        Self {
            snake: SnakeState {
                head: center_of_win,
                direction: Vec2::xy(0, -1),
                speed: 5,
                tail: vec![
                    Vec2::xy(center_of_win.x, center_of_win.y + 1),
                    Vec2::xy(center_of_win.x, center_of_win.y + 2),
                ],
            },
            food_position: Self::random_food_position(winsize),
        }
    }

    pub fn random_food_position(winsize: Vec2) -> Vec2 {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..winsize.x);
        let y = rng.gen_range(0..winsize.y);
        Vec2::xy(x, y)
    }

    pub fn update(&mut self, frame: usize, winsize: Vec2) {
        self.snake.update(frame);
        if self.snake.head == self.food_position {
            self.snake.add_tail(self.food_position.clone());
            self.food_position = Self::random_food_position(winsize);
        }
    }
}

fn main() {
    let mut fps_counter = FPSCounter::new();
    let mut app = App::new();
    let winsize = app.window().size();
    let mut state = GameState::new(winsize);

    app.run(|app_state: &mut State, window: &mut Window| {
        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                KeyEvent::Pressed(Key::Q) => app_state.stop(),
                KeyEvent::Pressed(Key::A) => {
                    state.snake.direction = Vec2::xy(-1, 0);
                }
                KeyEvent::Pressed(Key::S) => {
                    state.snake.direction = Vec2::xy(0, 1);
                }
                KeyEvent::Pressed(Key::D) => {
                    state.snake.direction = Vec2::xy(1, 0);
                }
                KeyEvent::Pressed(Key::W) => {
                    state.snake.direction = Vec2::xy(0, -1);
                }
                _ => (),
            }
        }

        fps_counter.update();
        state.update(app_state.step(), winsize);

        let mut pencil = Pencil::new(window.canvas_mut());
        pencil
            .draw_text(&format!("FPS: {}", fps_counter.count()), Vec2::xy(1, 1))
            .draw_char('a', state.food_position)
            .draw_rect(
                &RectCharset::double_lines(),
                Vec2::xy(1, 1),
                Vec2::xy(winsize.x - 1, winsize.y - 1),
            );
        state.snake.draw(pencil)
    });
}
