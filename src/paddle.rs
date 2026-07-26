use macroquad::prelude::*;

pub const PADDLE_WIDTH: f32 = 15.0;
pub const PADDLE_HEIGHT_DEFAULT: f32 = 100.0;
pub const PADDLE_HEIGHT_MIN: f32 = 30.0;
pub const PADDLE_SHRINK_AMOUNT: f32 = 12.0;
pub const PADDLE_SPEED: f32 = 420.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SizeEffect {
    Normal,
    Big(f32),
    Small(f32),
}

pub struct Paddle {
    pub x: f32,
    pub y: f32,
    pub base_height: f32,
    pub width: f32,
    pub speed: f32,
    pub is_ai: bool,
    pub size_effect: SizeEffect,
    pub frozen_timer: f32,
    pub reversed_timer: f32,
    pub shield: bool,
    pub low_height_flash_timer: f32,
}

impl Paddle {
    pub fn new(x: f32, is_ai: bool) -> Self {
        Paddle {
            x,
            y: screen_height() / 2.0 - PADDLE_HEIGHT_DEFAULT / 2.0,
            base_height: PADDLE_HEIGHT_DEFAULT,
            width: PADDLE_WIDTH,
            speed: PADDLE_SPEED,
            is_ai,
            size_effect: SizeEffect::Normal,
            frozen_timer: 0.0,
            reversed_timer: 0.0,
            shield: false,
            low_height_flash_timer: 0.0,
        }
    }

    pub fn height(&self) -> f32 {
        let h = match self.size_effect {
            SizeEffect::Normal => self.base_height,
            SizeEffect::Big(_) => self.base_height * 2.0,
            SizeEffect::Small(_) => self.base_height * 0.5,
        };
        h.clamp(10.0, screen_height())
    }

    pub fn update(
        &mut self,
        dt: f32,
        ball_y: Option<f32>,
        ai_speed: f32,
        aim_offset: f32,
        up_key: KeyCode,
        down_key: KeyCode,
    ) {
        if self.frozen_timer > 0.0 {
            self.y = self.y.clamp(0.0, screen_height() - self.height());
            return;
        }

        let mut dir: f32 = 0.0;
        let mut speed = self.speed;

        if self.is_ai {
            speed = ai_speed;
            if let Some(target) = ball_y {
                let center = self.y + self.height() / 2.0;
                let diff = (target + aim_offset) - center;
                if diff.abs() > 2.0 {
                    dir = diff.signum();
                }
            }
        } else {
            if is_key_down(up_key) {
                dir -= 1.0;
            }
            if is_key_down(down_key) {
                dir += 1.0;
            }
        }

        if self.reversed_timer > 0.0 {
            dir = -dir;
        }

        self.y += dir * speed * dt;
        self.y = self.y.clamp(0.0, screen_height() - self.height());
    }

    pub fn apply_big(&mut self, duration: f32) {
        self.size_effect = SizeEffect::Big(duration);
    }

    pub fn apply_small(&mut self, duration: f32) {
        self.size_effect = SizeEffect::Small(duration);
    }

    pub fn apply_freeze(&mut self, duration: f32) {
        self.frozen_timer = duration;
    }

    pub fn apply_reverse(&mut self, duration: f32) {
        self.reversed_timer = duration;
    }

    pub fn apply_shield(&mut self) {
        self.shield = true;
    }

    pub fn consume_shield(&mut self) -> bool {
        if self.shield {
            self.shield = false;
            true
        } else {
            false
        }
    }

    pub fn update_effects(&mut self, dt: f32) {
        self.size_effect = match self.size_effect {
            SizeEffect::Big(t) => {
                let r = t - dt;
                if r <= 0.0 {
                    SizeEffect::Normal
                } else {
                    SizeEffect::Big(r)
                }
            }
            SizeEffect::Small(t) => {
                let r = t - dt;
                if r <= 0.0 {
                    SizeEffect::Normal
                } else {
                    SizeEffect::Small(r)
                }
            }
            SizeEffect::Normal => SizeEffect::Normal,
        };

        if self.frozen_timer > 0.0 {
            self.frozen_timer = (self.frozen_timer - dt).max(0.0);
        }
        if self.reversed_timer > 0.0 {
            self.reversed_timer = (self.reversed_timer - dt).max(0.0);
        }

        if self.base_height <= PADDLE_HEIGHT_MIN + 0.5 {
            self.low_height_flash_timer = (self.low_height_flash_timer + dt) % 1.0;
        } else {
            self.low_height_flash_timer = 0.0;
        }

        self.y = self.y.clamp(0.0, screen_height() - self.height());
    }

    pub fn shrink_on_concede(&mut self) {
        self.base_height = (self.base_height - PADDLE_SHRINK_AMOUNT).max(PADDLE_HEIGHT_MIN);
    }

    pub fn reset_for_new_match(&mut self) {
        self.base_height = PADDLE_HEIGHT_DEFAULT;
        self.size_effect = SizeEffect::Normal;
        self.frozen_timer = 0.0;
        self.reversed_timer = 0.0;
        self.shield = false;
        self.low_height_flash_timer = 0.0;
        self.y = screen_height() / 2.0 - self.height() / 2.0;
    }

    pub fn rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.width, self.height())
    }

    pub fn draw(&self) {
        let color = if self.low_height_flash_timer > 0.0 && self.low_height_flash_timer < 0.5 {
            RED
        } else {
            WHITE
        };

        let h = self.height();
        let third = h / 3.0;
        draw_rectangle(self.x, self.y, self.width, third, Color::new(color.r, color.g, color.b, 0.6));
        draw_rectangle(self.x, self.y + third, self.width, third, color);
        draw_rectangle(self.x, self.y + third * 2.0, self.width, third, Color::new(color.r, color.g, color.b, 0.6));

        if self.shield {
            let shield_x = if self.x < screen_width() / 2.0 {
                0.0
            } else {
                screen_width() - 4.0
            };
            draw_rectangle(shield_x, 0.0, 4.0, screen_height(), SKYBLUE);
        }
    }
}
