use crate::paddle::Paddle;
use macroquad::prelude::*;

pub const BALL_RADIUS: f32 = 8.0;
pub const BALL_BASE_SPEED: f32 = 320.0;
pub const BALL_MAX_SPEED: f32 = 750.0;
pub const BALL_SPEED_INCREMENT: f32 = 22.0;
pub const MAX_BOUNCE_ANGLE_DEG: f32 = 60.0;

pub struct Ball {
    pub pos: Vec2,
    pub vel: Vec2,
    pub radius: f32,
    pub speed: f32,
    pub slow_timer: f32,
    pub fast_timer: f32,
}

impl Ball {
    pub fn new() -> Self {
        Ball {
            pos: vec2(screen_width() / 2.0, screen_height() / 2.0),
            vel: vec2(0.0, 0.0),
            radius: BALL_RADIUS,
            speed: BALL_BASE_SPEED,
            slow_timer: 0.0,
            fast_timer: 0.0,
        }
    }

    pub fn effective_speed(&self) -> f32 {
        let mut s = self.speed;
        if self.slow_timer > 0.0 {
            s *= 0.5;
        }
        if self.fast_timer > 0.0 {
            s *= 2.0;
        }
        s
    }

    pub fn reset_for_serve(&mut self, serve_toward_right: bool) {
        self.pos = vec2(screen_width() / 2.0, screen_height() / 2.0);
        let dir = if serve_toward_right { 1.0 } else { -1.0 };
        self.vel = vec2(dir, 0.0);
        self.slow_timer = 0.0;
        self.fast_timer = 0.0;
    }

    pub fn reset_for_new_match(&mut self, serve_toward_right: bool) {
        self.speed = BALL_BASE_SPEED;
        self.reset_for_serve(serve_toward_right);
    }

    pub fn speed_up(&mut self) {
        self.speed = (self.speed + BALL_SPEED_INCREMENT).min(BALL_MAX_SPEED);
    }

    pub fn update(&mut self, dt: f32) {
        if self.slow_timer > 0.0 {
            self.slow_timer = (self.slow_timer - dt).max(0.0);
        }
        if self.fast_timer > 0.0 {
            self.fast_timer = (self.fast_timer - dt).max(0.0);
        }

        let dir = self.vel.normalize_or_zero();
        self.pos += dir * self.effective_speed() * dt;

        if self.pos.y - self.radius <= 0.0 {
            self.pos.y = self.radius;
            self.vel.y = self.vel.y.abs();
        }
        if self.pos.y + self.radius >= screen_height() {
            self.pos.y = screen_height() - self.radius;
            self.vel.y = -self.vel.y.abs();
        }
    }

    pub fn bounce_off_paddle(&mut self, paddle: &Paddle, moving_right: bool) {
        let paddle_center = paddle.y + paddle.height() / 2.0;
        let relative = (paddle_center - self.pos.y) / (paddle.height() / 2.0);
        let relative = relative.clamp(-1.0, 1.0);
        let bounce_angle = relative * MAX_BOUNCE_ANGLE_DEG.to_radians();

        let dir_x = if moving_right { 1.0 } else { -1.0 };
        self.vel = vec2(dir_x * bounce_angle.cos(), -bounce_angle.sin()).normalize_or_zero();

        if moving_right {
            self.pos.x = paddle.x + paddle.width + self.radius;
        } else {
            self.pos.x = paddle.x - self.radius;
        }
    }

    pub fn bounce_off_wall_x(&mut self, moving_right: bool) {
        let speed = self.vel.length().max(0.001);
        let dir_x = if moving_right { speed } else { -speed };
        self.vel.x = dir_x;
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.radius, WHITE);
    }
}
