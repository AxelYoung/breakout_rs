use std::ops::{Mul, AddAssign};
use std::rc::Rc;

use num::clamp;
use num::complex::ComplexFloat;
use rand::Rng;
use winit::event::*;

pub const SCREEN_SIZE: Vec2i = Vec2i {x: 800, y: 800};

pub const PADDLE_BOUNDS_MIN: f32 = -SCREEN_SIZE.y as f32 + (PADDLE_SIZE.y / 2.0);
pub const PADDLE_BOUNDS_MAX: f32 = SCREEN_SIZE.y as f32 - (PADDLE_SIZE.y / 2.0);

pub const PADDLE_SIZE: Vec2 = Vec2 {x: 250.0, y: 50.0};
pub const BRICK_SIZE: Vec2 = Vec2 {x:150.0, y: 75.0};
pub const BALL_SIZE: Vec2 = Vec2 {x: 50.0, y: 50.0};

const TICKS_PER_SECOND: f32 = 60.0;
const TICK_TIME: f32 = 1.0 / TICKS_PER_SECOND;

pub const PADDLE_SPEED: f32 = 12.0;
pub const BALL_SPEED: f32 = 25.0;

pub const BRICK_HEATH: u8 = 5;

pub struct GameState {
    pub player: Entity,
    pub bricks: Vec<Brick>,
    pub ball: Entity,
    previous_time: instant::Instant,
    tick: f32
}

#[derive(PartialEq)]
pub struct Entity {
    pub quad: Quad,
    dir: Vec2
}

pub struct Brick {
    pub quad: Quad,
    pub health: u8
}

impl Brick {
    pub fn new<T: num::ToPrimitive>(x: T, y: T) -> Self {
        Brick {
            quad: Quad {
                pos: Vec2::new(x, y),
                size: BRICK_SIZE
            },
            health: rand::thread_rng().gen_range(3..=BRICK_HEATH)
        }
    }

    pub fn top(&self) -> f32 {
        self.quad.pos.y + (BRICK_SIZE.y / 2.0)
    }
    pub fn bottom(&self) -> f32 {
        self.quad.pos.y - (BRICK_SIZE.y / 2.0)
    }
    pub fn right(&self) -> f32 {
        self.quad.pos.x + (BRICK_SIZE.x / 2.0)
    }
    pub fn left(&self) -> f32 {
        self.quad.pos.x - (BRICK_SIZE.x / 2.0)
    }
}

impl Entity {
    pub fn add_position(&mut self, pos: Vec2) {
        self.quad.pos += pos;
    }
}

#[derive(PartialEq)]
pub struct Quad {
    pub pos: Vec2,
    pub size: Vec2
}

impl Quad {
    pub fn new(pos: Vec2, size: Vec2) -> Self {
        Self {
            pos,
            size
        }
    }

    pub fn interects(&self, b: &Quad) -> bool {
        let dx = (self.pos.x - b.pos.x).abs();
        let dy = (self.pos.y - b.pos.y).abs();

        let half_width = self.size.x / 2.0 + b.size.x / 2.0;
        let half_height = self.size.y / 2.0 + b.size.y / 2.0;

        dx < half_width && dy < half_height
    }
}

#[derive(Debug)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32
}

impl Mul<f32> for Vec2i {
    type Output = Vec2;

    fn mul(self, mul: f32) -> Vec2 {
        Vec2::new(self.x as f32 * mul, self.y as f32 * mul)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, mul: f32) -> Vec2 {
        Vec2::new(self.x as f32 * mul, self.y as f32 * mul)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

impl Vec2 {
    pub fn new<T: num::ToPrimitive>(x: T, y: T) -> Self {
        Vec2 {
            x: x.to_f32().unwrap(),
            y: y.to_f32().unwrap()
        }
    }

    pub fn zero() -> Self {
        Vec2::new(0,0)
    }

    pub fn normalize(&mut self) -> Self {
        let mag = (self.x * self.x + self.y * self.y).sqrt();
        if mag != 0.0 { 
            self.x /= mag;
            self.y /= mag;
        }
        *self
    }
}

impl GameState {
    pub fn new() -> Self {

        let player = Entity {
            quad: Quad::new(Vec2::new(-130, -700), PADDLE_SIZE),
            dir: Vec2::zero()
        };

        let bricks = vec![
            Brick::new(0,200),
            Brick::new(150,200),
            Brick::new(-150,200),
            Brick::new(0,275),
            Brick::new(150,275),
            Brick::new(-150,275),
            Brick::new(-0,125),
            Brick::new(-150,125),
            Brick::new(150,125),
            
            Brick::new(500, 350),
            Brick::new(500,275),
            Brick::new(500,200),
            Brick::new(500,125),
            Brick::new(500, 50),

            Brick::new(-500, 350),
            Brick::new(-500,275),
            Brick::new(-500,200),
            Brick::new(-500,125),
            Brick::new(-500, 50),

            Brick::new(-500, 425),
            Brick::new(-350, 425),
            Brick::new(-200, 425),

            Brick::new(500, 425),
            Brick::new(350, 425),
            Brick::new(200, 425),

            Brick::new(-500, -25),
            Brick::new(-350, -25),
            Brick::new(-200, -25),

            Brick::new(500, -25),
            Brick::new(350, -25),
            Brick::new(200, -25),

        ];
        
        let ball = Entity {
            quad: Quad::new(Vec2::new(0, 0), BALL_SIZE),
            dir: Vec2::new(0, -1)
        };
 
        GameState {
            player,
            bricks,
            ball,
            previous_time: instant::Instant::now(),
            tick: 0.0,
        }
    }

    pub fn update(&mut self) {
        let current_time = instant::Instant::now();
        let elapsed_time = current_time.duration_since(self.previous_time).as_secs_f32();
        self.previous_time = current_time;

        self.tick += elapsed_time;

        if self.tick > TICK_TIME {
            
            GameState::paddle_move(&mut self.player);

            self.ball_bounce();

            self.score_keep();
            
            self.tick -= TICK_TIME;
        }
    }

    fn paddle_move(paddle: &mut Entity) {
        paddle.add_position(paddle.dir * PADDLE_SPEED);
        paddle.quad.pos.y = clamp(paddle.quad.pos.y, PADDLE_BOUNDS_MIN, PADDLE_BOUNDS_MAX);
    }

    fn check_player_collision(&self) -> Option<&Quad> {
        if self.player.quad.interects(&self.ball.quad) { return Some(&self.player.quad) }
        None
    }

    fn check_brick_collision(&self) -> Option<usize> {
        for (i, brick) in self.bricks.iter().enumerate() {
            if brick.quad.interects(&self.ball.quad) { 
                return Some(i) 
            }
        }
        None
    }

    fn ball_bounce(&mut self) {
        if self.ball.quad.pos.x > SCREEN_SIZE.x as f32 - (self.ball.quad.size.x  / 2.0) {
            self.ball.dir = Vec2::new(-(self.ball.dir.x.abs()), self.ball.dir.y);
        } else if self.ball.quad.pos.x < -SCREEN_SIZE.x as f32 + (self.ball.quad.size.x / 2.0) {
            self.ball.dir = Vec2::new(self.ball.dir.x.abs(), self.ball.dir.y);
        } else if self.ball.quad.pos.y > SCREEN_SIZE.y as f32 - (self.ball.quad.size.y / 2.0) {
            self.ball.dir = Vec2::new(self.ball.dir.x, -(self.ball.dir.y.abs()));
        }

        if let Some(col) = self.check_player_collision() {
            let dist = col.pos.x - self.ball.quad.pos.x;
            
            let normalized_dist = dist / (PADDLE_SIZE.x / 2.0);

            let angle = normalized_dist * std::f32::consts::FRAC_PI_4;

            let bounce_x = BALL_SPEED * -angle.sin();

            let mut bounce_y = BALL_SPEED * angle.cos();

            if col.pos.y > self.ball.quad.pos.y {
                bounce_y *= -1.0;
            }

            self.ball.dir = Vec2::new(bounce_x, bounce_y).normalize();
        }

        if let Some(brick) = self.check_brick_collision() {

            let x_dist: f32;
            let y_dist: f32;

            if self.ball.quad.pos.y > self.bricks[brick].quad.pos.y {
                y_dist = self.ball.quad.pos.y - self.bricks[brick].top();
            } else {
                y_dist = self.ball.quad.pos.y - self.bricks[brick].bottom();
            }

            if self.ball.quad.pos.x > self.bricks[brick].quad.pos.x {
                x_dist = self.ball.quad.pos.x - self.bricks[brick].right();
            } else {
                x_dist = self.ball.quad.pos.x - self.bricks[brick].left();
            }

            if y_dist.abs() < x_dist.abs() {
                if self.ball.quad.pos.y >= self.bricks[brick].top() {
                    self.ball.dir = Vec2::new(self.ball.dir.x, self.ball.dir.y.abs());
                    println!("top");
                } else if self.ball.quad.pos.y <= self.bricks[brick].bottom() {
                    self.ball.dir = Vec2::new(self.ball.dir.x, -(self.ball.dir.y.abs()));
                    println!("bot");
                }
            } else {
                if self.ball.quad.pos.x >= self.bricks[brick].right() {
                    self.ball.dir = Vec2::new(self.ball.dir.x.abs(), self.ball.dir.y);
                    println!("right");
                } else if self.ball.quad.pos.x <= self.bricks[brick].left() {
                    self.ball.dir = Vec2::new(-(self.ball.dir.x.abs()), self.ball.dir.y);
                    println!("left");
                }
            }

            self.bricks[brick].health -= 1;

            if(self.bricks[brick].health <= 0) {
                self.bricks.remove(brick);
            }
        }

        self.ball.add_position(self.ball.dir * BALL_SPEED);
    }

    fn score_keep(&mut self) {
        if self.ball.quad.pos.y < -SCREEN_SIZE.y as f32 + self.ball.quad.size.y {
            self.ball.quad.pos = Vec2::zero();
            self.ball.dir = Vec2::new(0,-1);
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Space),
                        ..
                    },
                ..
            } => {
                return true;
            }
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Left),
                        ..
                    },
                ..
            } => {
                self.player.dir = Vec2::new(-1, 0);
                return true;
            }
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Right),
                        ..
                    },
                ..
            } => {
                self.player.dir = Vec2::new(1, 0);
                return true;
            }
            _ => { 
                self.player.dir = Vec2::zero();
                return false;
            }
        }
    }
}
