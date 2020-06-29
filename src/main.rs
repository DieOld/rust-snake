extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

use std::collections::LinkedList;
use std::iter::FromIterator;
use rand::Rng;

#[derive(Clone, PartialEq)]
enum Direction {
    Right, Left, Up, Down
}

struct Snake {
    body: LinkedList<(i32, i32)>,
    dir: Direction,
}

struct Food {
    pos: (i32, i32)
}

impl Food {
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        let red: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
        let food_item = graphics::rectangle::square((self.pos.0 * 20) as f64, (self.pos.1 * 20) as f64, 20_f64);
        gl.draw(args.viewport(), |c, gl| {
            graphics::rectangle(red, food_item, c.transform, gl)
        });
    }
}

impl Snake {
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        let red: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        
        let squares: Vec<graphics::types::Rectangle> = self.body
            .iter()
            .map(|&(x, y)| graphics::rectangle::square((x * 20) as f64, (y * 20) as f64, 20_f64)).collect();

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            squares.into_iter()
                .for_each(|square| graphics::rectangle(red, square, transform, gl))
        });
    }

    fn update(&mut self){
        self.add_head();
        self.body.pop_back().unwrap();
    }
    
    fn add_head(&mut self) {
        let mut new_head = (*self.body.front().expect("Snale has no body")).clone();
        match self.dir {
            Direction::Left => new_head.0 -= 1,
            Direction::Right => new_head.0 += 1,
            Direction::Up => new_head.1 -= 1,
            Direction::Down => new_head.1 += 1,
        }
        self.body.push_front(new_head);
    }
}

struct Game {
    gl: GlGraphics,
    snake: Snake,
    food: Food,
    is_game: bool
}

impl Game {
    fn render(&mut self, arg: &RenderArgs) {
        let green: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(green, gl);
        });

        self.food.render(&mut self.gl, arg);
        self.snake.render(&mut self.gl, arg);
    }

    fn calculate_food_pos(&mut self) -> (i32, i32) {
        let mut rng = rand::thread_rng();
        let mut pos = (rng.gen_range(0, 20), rng.gen_range(0, 20));
        while self.snake.body.contains(&pos) {
            pos = (rng.gen_range(0, 20), rng.gen_range(0, 20))
        }
        pos
    }

    fn update(&mut self) {
        let head = *self.snake.body.front().unwrap();
        self.check_for_lose(head);
        if head == self.food.pos {
            self.snake.add_head();
            self.food.pos = self.calculate_food_pos();
        }
        if self.is_game {
            self.snake.update();
        }; 
    }
    
    fn check_for_lose(&mut self, head: (i32, i32)) {
        let mut body_without_head = self.snake.body.clone();
        body_without_head.pop_front();
        if body_without_head.contains(&head) || ((head.0 < 0 || head.0 > 19) || (head.1 < 0 || head.1 > 19)) {
            self.is_game = false;
        }

    }

    fn pressed(&mut self, btn: &Button) {
        let last_direction = self.snake.dir.clone();
        self.snake.dir = match btn {
            &Button::Keyboard(Key::Up)
                if last_direction != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::Down)
                if last_direction != Direction::Up => Direction::Down,
            &Button::Keyboard(Key::Left)
                if last_direction != Direction::Right => Direction::Left,
            &Button::Keyboard(Key::Right)
                if last_direction != Direction::Left => Direction::Right,
            _ => last_direction
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let mut rng = rand::thread_rng();
    let mut window: GlutinWindow = WindowSettings::new(
        "Snake Game",
        [400, 400]
    ).graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    
    let mut game = Game {
        gl: GlGraphics::new(opengl),
        snake: Snake{
            body: LinkedList::from_iter((vec![(0, 0), (0, 1)]).into_iter()),
            dir: Direction::Right },
        food: Food{
            pos: (rng.gen_range(0, 10), rng.gen_range(0, 10))
        },
        is_game: true

    }; 
    let mut events = Events::new(EventSettings::new()).ups(8);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(_u) = e.update_args() {
            if game.is_game { game.update();}
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button);
            }
        }
    }

}
