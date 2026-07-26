use crate::ball::Ball;
use crate::game_state::ScoreEvent;
use crate::paddle::Paddle;
use macroquad::prelude::*;

pub struct Score {
    pub left: u32,
    pub right: u32,
}

impl Score {
    pub fn new() -> Self {
        Score { left: 0, right: 0 }
    }

    pub fn reset(&mut self) {
        self.left = 0;
        self.right = 0;
    }

    pub fn update(
        &mut self,
        ball: &mut Ball,
        paddle_left: &mut Paddle,
        paddle_right: &mut Paddle,
    ) -> ScoreEvent {
        if ball.pos.x < -ball.radius {
            if paddle_left.consume_shield() {
                ball.pos.x = paddle_left.x + paddle_left.width + ball.radius;
                ball.bounce_off_wall_x(true);
                return ScoreEvent::Blocked;
            }
            self.right += 1;
            return ScoreEvent::RightScored;
        }

        if ball.pos.x > screen_width() + ball.radius {
            if paddle_right.consume_shield() {
                ball.pos.x = paddle_right.x - ball.radius;
                ball.bounce_off_wall_x(false);
                return ScoreEvent::Blocked;
            }
            self.left += 1;
            return ScoreEvent::LeftScored;
        }

        ScoreEvent::NoScore
    }
}
