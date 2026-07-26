use macroquad::prelude::*;

pub const POWERUP_SPAWN_INTERVAL: f32 = 8.0;
pub const POWERUP_RADIUS: f32 = 12.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerUpKind {
    BigPaddle,
    ShrinkOpponent,
    SlowBall,
    FastBall,
    FreezeOpponent,
    ReverseControls,
    Shield,
    MultiBall,
}

impl PowerUpKind {
    pub fn duration(&self) -> f32 {
        match self {
            PowerUpKind::BigPaddle => 5.0,
            PowerUpKind::ShrinkOpponent => 5.0,
            PowerUpKind::SlowBall => 5.0,
            PowerUpKind::FastBall => 4.0,
            PowerUpKind::FreezeOpponent => 3.0,
            PowerUpKind::ReverseControls => 4.0,
            PowerUpKind::Shield => 0.0,
            PowerUpKind::MultiBall => 6.0,
        }
    }

    pub fn color(&self) -> Color {
        match self {
            PowerUpKind::BigPaddle => GREEN,
            PowerUpKind::ShrinkOpponent => RED,
            PowerUpKind::SlowBall => BLUE,
            PowerUpKind::FastBall => YELLOW,
            PowerUpKind::FreezeOpponent => SKYBLUE,
            PowerUpKind::ReverseControls => PURPLE,
            PowerUpKind::Shield => ORANGE,
            PowerUpKind::MultiBall => PINK,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            PowerUpKind::BigPaddle => "BIG",
            PowerUpKind::ShrinkOpponent => "SHRINK",
            PowerUpKind::SlowBall => "SLOW",
            PowerUpKind::FastBall => "FAST",
            PowerUpKind::FreezeOpponent => "FREEZE",
            PowerUpKind::ReverseControls => "REVERSE",
            PowerUpKind::Shield => "SHIELD",
            PowerUpKind::MultiBall => "MULTI",
        }
    }

    pub fn random() -> Self {
        match macroquad::rand::gen_range(0, 8) {
            0 => PowerUpKind::BigPaddle,
            1 => PowerUpKind::ShrinkOpponent,
            2 => PowerUpKind::SlowBall,
            3 => PowerUpKind::FastBall,
            4 => PowerUpKind::FreezeOpponent,
            5 => PowerUpKind::ReverseControls,
            6 => PowerUpKind::Shield,
            _ => PowerUpKind::MultiBall,
        }
    }
}

pub struct PowerUp {
    pub pos: Vec2,
    pub kind: PowerUpKind,
    pub radius: f32,
}

impl PowerUp {
    pub fn spawn_random() -> Self {
        let margin = 40.0;
        let y = macroquad::rand::gen_range(margin, screen_height() - margin);
        PowerUp {
            pos: vec2(screen_width() / 2.0, y),
            kind: PowerUpKind::random(),
            radius: POWERUP_RADIUS,
        }
    }

    pub fn overlaps_circle(&self, other_pos: Vec2, other_radius: f32) -> bool {
        self.pos.distance(other_pos) <= self.radius + other_radius
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.radius, self.kind.color());
        let w = measure_text(self.kind.label(), None, 14, 1.0).width;
        draw_text(
            self.kind.label(),
            self.pos.x - w / 2.0,
            self.pos.y - self.radius - 6.0,
            14.0,
            WHITE,
        );
    }
}
