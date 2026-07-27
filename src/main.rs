mod ball;
mod game_state;
mod paddle;
mod powerup;
mod replay;
mod score;

use ball::Ball;
use game_state::{Difficulty, GameMode, GameState, ScoreEvent, Winner};
use macroquad::prelude::*;
use paddle::Paddle;
use powerup::{PowerUp, PowerUpKind, POWERUP_SPAWN_INTERVAL};
use replay::{ReplayBuffer, Snapshot};
use score::Score;

const WIN_SCORE:u32=5;
const MENU_OPTIONS:[&str; 4]=["1 Player","2 Player","Controls", "Quit"];
const COUNTDOWN_SECONDS:f32=3.0;
const REPLAY_FRAME_TIME:f32=3.0 / 60.0;

fn window_conf()->Conf{
    Conf{
        window_title: "Rust Pong".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}


struct TimedBall{
    ball: Ball,
    timer: f32,
}
struct App{
    state:GameState,
    mode:GameMode,
    difficulty:Difficulty,
    menu_index: usize,

    paddle_left:Paddle,
    paddle_right: Paddle,
    ball:Ball,
    extra_balls:Vec<TimedBall>,

    score:Score,
    winner:Option<Winner>,


    countdown_timer:f32,
    serve_toward_right:bool,


    powerup:Option<PowerUp>,
    powerup_spawn_timer:  f32,

    replay_buffer:ReplayBuffer,
    replay_frames:Vec<Snapshot>,
    replay_index:usize,
    replay_playback_timer:f32,

    ai_aim_offset:f32,
    ai_retarget_timer:f32,
}

impl App {
    fn new() -> Self {
        App{
            state:GameState::MainMenu,
            mode:GameMode::OnePlayer,
            difficulty:Difficulty::Medium,
            menu_index: 0,

            paddle_left:Paddle::new(30.0, false),
            paddle_right:Paddle::new(screen_width() - 30.0 - paddle::PADDLE_WIDTH, true),
            ball:Ball::new(),
            extra_balls:Vec::new(),

            score:Score::new(),
            winner:None,

            countdown_timer:COUNTDOWN_SECONDS ,
            serve_toward_right:true,

            powerup:None,
            powerup_spawn_timer: POWERUP_SPAWN_INTERVAL,

            replay_buffer:ReplayBuffer::new(),
            replay_frames:Vec::new(),
            replay_index:0,
            replay_playback_timer:0.0,

            ai_aim_offset:0.0,
            ai_retarget_timer:0.0,
        }
    }

    fn start_new_match(&mut self){
        self.score. reset();
        self.winner=None;
        self.paddle_left.reset_for_new_match();
        self.paddle_right.reset_for_new_match();
        self.paddle_right.is_ai = matches!(self.mode, GameMode::OnePlayer);
        self.ball.reset_for_new_match(true);
        self.extra_balls.clear();
        self.powerup =None;
        self.powerup_spawn_timer = POWERUP_SPAWN_INTERVAL;
        self.replay_buffer.clear();
        self.serve_toward_right =true;
        self.countdown_timer =COUNTDOWN_SECONDS;
        self.state =GameState::Countdown;
    }

    fn update_menu(&mut self){
        if is_key_pressed(KeyCode::Down){
            self.menu_index =(self.menu_index + 1).min(MENU_OPTIONS.len() - 1);
        }
        if is_key_pressed(KeyCode::Up){
            self.menu_index =self.menu_index.saturating_sub(1);
        }
        if self.menu_index ==0 && is_key_pressed(KeyCode::Right){
            self.difficulty =self.difficulty.cycle();
        }
        if is_key_pressed(KeyCode::Enter){
            match self.menu_index{
                0 =>{
                    self.mode =GameMode::OnePlayer;
                    self.start_new_match();
                }
                1=>{
                    self.mode =GameMode::TwoPlayer;
                    self.start_new_match();
                }
                2=> self.state = GameState::Controls,
                _=> std::process::exit(0),
            }
        }
        if is_key_pressed(KeyCode::Escape){
            std::process::exit(0);
        }
    }

    fn draw_menu(&self){
        let title ="RUST PONG";
        let tw = measure_text(title, None, 60, 1.0).width;
        draw_text(title,screen_width() / 2.0 - tw / 2.0, 120.0, 60.0, WHITE);

        for (i, option) in MENU_OPTIONS.iter().enumerate() {
            let mut label =option.to_string();
            if i == 0{
                label =format!("{}  ({})", option, self.difficulty.label());
            }
            let color =if i == self.menu_index { YELLOW } else { GRAY };
            let w = measure_text(&label, None, 34, 1.0).width;
            draw_text(
                &label,
                screen_width() / 2.0 - w / 2.0,
                240.0 + i as f32 * 50.0,
                34.0,
                color,
            );
        }

        let hint = "Up/Down: select   Right: change difficulty   Enter: confirm";
        let hw = measure_text(hint, None, 18, 1.0).width;
        draw_text(hint,
            screen_width() / 2.0 - hw / 2.0,
            screen_height() - 40.0,
            18.0,
            DARKGRAY,
        );
    }

    fn update_controls(&mut self) {
        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Enter) {
            self.state = GameState::MainMenu;
        }
    }


    fn draw_controls(&self){
        let title ="CONTROLS";
        let tw =measure_text(title, None, 44, 1.0).width;
        draw_text(title,screen_width() / 2.0 - tw / 2.0, 100.0, 44.0, WHITE);

        let lines =[
            "Left paddle:  W (up)  /  S (down)",
            "Right paddle (2 Player only):  Up / Down arrows",
            "Right paddle (1 Player):  controlled by the AI",
            "Space: skip a replay early",
            "Escape: back / quit",
        ];
        for (i, line) in lines.iter().enumerate() {
            let w = measure_text(line,None,22,1.0).width;
            draw_text(line,
                screen_width() / 2.0 - w / 2.0,
                200.0 + i as f32 * 36.0,
                22.0,
                LIGHTGRAY,
            );
        }

        let hint = "Escape or Enter: back to menu";
        let hw = measure_text(hint, None, 18, 1.0).width;
        draw_text(
            hint,
            screen_width() / 2.0 - hw / 2.0,
            screen_height() - 40.0,
            18.0,
            DARKGRAY,
        );
    }

    fn update_countdown(&mut self, dt: f32){
        self.countdown_timer -= dt;
        if self.countdown_timer <= 0.0{
            self.ball.reset_for_serve(self.serve_toward_right);
            self.extra_balls.clear();
            self.state = GameState::Playing;
        }
    }

    fn draw_countdown(&self){
        self.draw_field();
        let remaining = self.countdown_timer.ceil() as i32;
        let text = if remaining > 0 {
            remaining.to_string()
        }else {
            "GO!".to_string()
        };

        let size = 80.0;
        let w =measure_text(&text, None, size as u16, 1.0).width;
        draw_text(
            &text,
            screen_width() / 2.0 - w / 2.0,
            screen_height() / 2.0 + size / 3.0,
            size,
            YELLOW,
        );
    }

    fn update_playing(&mut self, dt:f32) {
        if is_key_pressed(KeyCode::Escape) {
            self.state = GameState::MainMenu;
            return;
        }

        self.ai_retarget_timer -= dt;
        if self.ai_retarget_timer <= 0.0 {
            let err = self.difficulty.aim_error();
            self.ai_aim_offset = rand::gen_range(-err, err);
            self.ai_retarget_timer = 0.3;
        }

        let right_ball_y =if self.paddle_right.is_ai && self.ball.vel.x > 0.0 {
            Some(self.ball.pos.y)
        }else{
            None
        };

        self.paddle_left
            .update(dt, None, 0.0, 0.0, KeyCode::W, KeyCode::S);
        self.paddle_right.update(
            dt,
            right_ball_y,
            self.difficulty.ai_speed(),
            self.ai_aim_offset,
            KeyCode::Up,
            KeyCode::Down,
        );

        self.paddle_left.update_effects(dt);
        self.paddle_right.update_effects(dt);

        self.ball.update(dt);
        resolve_paddle_collision(&mut self.ball, &self.paddle_left, &self.paddle_right);

        for tb in self.extra_balls.iter_mut(){
            tb.ball.update(dt);
            resolve_paddle_collision(&mut tb.ball, &self.paddle_left, &self.paddle_right);
            tb.timer -= dt;
        }
        self.extra_balls.retain(|tb| {
            tb.timer > 0.0 && tb.ball.pos.x > -100.0 && tb.ball.pos.x < screen_width() + 100.0
        });

        self.update_powerups(dt);

        self.replay_buffer.push(Snapshot{
            ball_pos: self.ball.pos,
            paddle_left_y: self.paddle_left.y,
            paddle_left_h: self.paddle_left.height(),
            paddle_right_y: self.paddle_right.y,
            paddle_right_h: self.paddle_right.height(),
        });

        match self
            .score
            .update(&mut self.ball, &mut self.paddle_left, &mut self.paddle_right)
        {
            ScoreEvent::NoScore | ScoreEvent::Blocked => {}
            ScoreEvent::LeftScored => self.handle_score(false),
            ScoreEvent::RightScored => self.handle_score(true),
        }
    }

    fn update_powerups(&mut self,dt:f32){
        if self.powerup.is_none(){
            self.powerup_spawn_timer -= dt;
            if self.powerup_spawn_timer <= 0.0{
                self.powerup = Some(PowerUp::spawn_random());
                self.powerup_spawn_timer = POWERUP_SPAWN_INTERVAL;
            }
        }

        let mut hit: Option<(PowerUpKind, bool)> = None;
        if let Some(pu) = &self.powerup {
            if pu.overlaps_circle(self.ball.pos, self.ball.radius) {
                hit = Some((pu.kind, self.ball.vel.x > 0.0));
            }else{
                for tb in &self.extra_balls {
                    if pu.overlaps_circle(tb.ball.pos, tb.ball.radius) {
                        hit = Some((pu.kind, tb.ball.vel.x > 0.0));
                        break;
                    }
                }
            }
        }
        if let Some((kind, heading_right)) = hit{
            self.powerup = None;

            self.apply_powerup(kind, heading_right);
        }
    }

    fn apply_powerup(&mut self, kind: PowerUpKind, heading_right: bool){
        match kind {
            PowerUpKind::BigPaddle =>{
                if heading_right{
                    self.paddle_right.apply_big(kind.duration());
                }else{
                    self.paddle_left.apply_big(kind.duration());
                }
            }
            PowerUpKind::ShrinkOpponent =>{
                if heading_right{
                    self.paddle_left.apply_small(kind.duration());
                } else{
                    self.paddle_right.apply_small(kind.duration());
                }
            }
            PowerUpKind::FreezeOpponent => {
                if heading_right{
                    self.paddle_left.apply_freeze(kind.duration());
                }else{
                    self.paddle_right.apply_freeze(kind.duration());
                }
            }
            PowerUpKind::ReverseControls => {
                if heading_right {
                    self.paddle_left.apply_reverse(kind.duration());
                } else {
                    self.paddle_right.apply_reverse(kind.duration());
                }
            }
            PowerUpKind::Shield => {
                if heading_right {
                    self.paddle_right.apply_shield();
                } else {
                    self.paddle_left.apply_shield();
                }
            }

            PowerUpKind::SlowBall => self.ball.slow_timer=kind.duration(),
            PowerUpKind::FastBall => self.ball.fast_timer=kind.duration(),
            PowerUpKind::MultiBall => {
                let mut nb = Ball::new();
                nb.pos = self.ball.pos;
                nb.speed = self.ball.speed;
                let flip = if self.ball.vel.y.abs() < 0.1 {
                    1.0
                } else {
                    -self.ball.vel.y.signum()
                };
                nb.vel = vec2(-self.ball.vel.x.signum(), flip).normalize_or_zero();
                self.extra_balls.push(TimedBall {
                    ball: nb,
                    timer: kind.duration(),
                });
            }
        }
    }

    fn handle_score(&mut self,right_scored:bool){
        if right_scored{
            self.paddle_left.shrink_on_concede();
            self.serve_toward_right = true;
        } else{
            self.paddle_right.shrink_on_concede();
            self.serve_toward_right = false;
        }
        self.ball.speed_up();

        if self.score.left >= WIN_SCORE || self.score.right >= WIN_SCORE {
            self.winner = Some(if self.score.left > self.score.right {
                Winner::Left
            } else {
                Winner::Right
            });
            self.state = GameState::GameOver;
            return;
        }
        if self.replay_buffer.len() > 20{
            self.replay_frames = self.replay_buffer.snapshot_vec();
            self.replay_index = 0;
            self.replay_playback_timer = 0.0;
            self.state = GameState::Replay;
        }else {
            self.countdown_timer = COUNTDOWN_SECONDS;
            self.state = GameState::Countdown;
        }
    }

    fn draw_field(&self){
        clear_background(BLACK);
        let mut y = 0.0;
        while y < screen_height(){
            draw_rectangle(screen_width() / 2.0 - 2.0, y,4.0,14.0, DARKGRAY);
            y += 26.0;
        }

        self.paddle_left.draw();
        self.paddle_right.draw();
        self.ball.draw();
        for tb in &self.extra_balls {
            tb.ball.draw();
        }
        if let Some(pu) = &self.powerup {
            pu.draw();
        }

        let score_text = format!("{}   -   {}", self.score.left, self.score.right);
        let w = measure_text(&score_text, None, 40, 1.0).width;
        draw_text(&score_text, screen_width() / 2.0 - w / 2.0, 50.0, 40.0, WHITE);
        self.draw_effects_hud();
    }

    fn draw_effects_hud(&self){
        let mut y = 90.0;
        let entries: [(&str, f32, f32); 8]=[
            ("L Big", big_timer(&self.paddle_left), self.paddle_left.base_height),
            ("L Frozen", self.paddle_left.frozen_timer, 3.0),
            ("L Reversed", self.paddle_left.reversed_timer, 4.0),
            ("R Big", big_timer(&self.paddle_right), self.paddle_right.base_height),
            ("R Frozen", self.paddle_right.frozen_timer, 3.0),
            ("R Reversed", self.paddle_right.reversed_timer, 4.0),
            ("Ball Slow/Fast", self.ball.slow_timer.max(self.ball.fast_timer), 5.0),
            ("Multi Ball", multi_ball_timer(&self.extra_balls), 6.0),
        ];
        for (label, remaining, max_duration) in entries {
            if remaining > 0.0{
                draw_text(label, 10.0, y, 16.0, LIGHTGRAY);
                let ratio = (remaining / max_duration.max(0.01)).clamp(0.0, 1.0);
                draw_rectangle(140.0, y - 12.0, 60.0, 10.0, DARKGRAY);
                draw_rectangle(140.0, y - 12.0, 60.0 * ratio, 10.0, GREEN);
                y += 20.0;
            }
        }
    }

    fn update_replay(&mut self, dt: f32){
        if is_key_pressed(KeyCode::Space) {
            self.finish_replay();
            return;
        }

        self.replay_playback_timer += dt;
        if self.replay_playback_timer >= REPLAY_FRAME_TIME {
            self.replay_playback_timer = 0.0;
            self.replay_index += 1;
            if self.replay_index >= self.replay_frames.len() {
                self.finish_replay();
            }
        }
    }

    fn finish_replay(&mut self){
        self.countdown_timer = COUNTDOWN_SECONDS;
        self.state = GameState::Countdown;
    }

    fn draw_replay(&self){
        clear_background(BLACK);
        if self.replay_frames.is_empty() {
            return;
        }
        let idx = self.replay_index.min(self.replay_frames.len() - 1);
        let snap = self.replay_frames[idx];

        draw_rectangle(
            self.paddle_left.x,
            snap.paddle_left_y,
            self.paddle_left.width,
            snap.paddle_left_h,
            WHITE,
        );
        draw_rectangle(
            self.paddle_right.x,
            snap.paddle_right_y,
            self.paddle_right.width,
            snap.paddle_right_h,
            WHITE,
        );
        draw_circle(snap.ball_pos.x, snap.ball_pos.y, self.ball.radius, WHITE);

        let label = "REPLAY";
        let w = measure_text(label, None, 34, 1.0).width;
        draw_text(label, screen_width() / 2.0 - w / 2.0, 50.0, 34.0, YELLOW);

        let progress = idx as f32 / (self.replay_frames.len().max(1) - 1).max(1) as f32;
        let bar_w = 300.0;
        let bar_x = screen_width() / 2.0 - bar_w / 2.0;
        draw_rectangle(bar_x, screen_height() - 40.0, bar_w, 8.0, DARKGRAY);
        draw_rectangle(bar_x, screen_height() - 40.0, bar_w * progress, 8.0, YELLOW);


        let hint = "space: skip replay";
        let hw = measure_text(hint, None, 16, 1.0).width;

        draw_text(
            hint,
            screen_width() / 2.0 - hw / 2.0,
            screen_height() - 55.0,
            16.0,
            DARKGRAY,
        );
    }

    fn update_game_over(&mut self) {
        if is_key_pressed(KeyCode::Enter) {
            self.state = GameState::MainMenu;
        }
        if is_key_pressed(KeyCode::Escape) {
            std::process::exit(0);
        }
    }

    fn draw_game_over(&self){
        clear_background(BLACK);
        let winner_text=match self.winner{
            Some(Winner::Left) => "Left player wins!",
            Some(Winner::Right) => "Right player wins!",
            None => "Game over",
        };
        let w =measure_text(winner_text,None,44,1.0).width;

        draw_text(
            winner_text,
            screen_width() / 2.0 - w / 2.0,
            screen_height() / 2.0 - 20.0,
            44.0,
            YELLOW,
        );

        let hint = "Enter: back to menu   Escape: quit";
        let hw = measure_text(hint, None, 20, 1.0).width;

        draw_text(
            hint,
            screen_width() / 2.0 - hw / 2.0,
            screen_height() / 2.0 + 30.0,
            20.0,
            LIGHTGRAY,
        );
    }
}

fn big_timer(p: &Paddle) -> f32{
    match p.size_effect {
        paddle::SizeEffect::Big(t) => t,
        _=> 0.0,
    }
}

fn multi_ball_timer(extra_balls:&[TimedBall]) -> f32{
    extra_balls
        .iter()
        .map(|tb| tb.timer)
        .fold(0.0, |acc, t| if t > acc { t } else { acc })
}

fn resolve_paddle_collision(ball:&mut Ball,paddle_left:&Paddle,paddle_right:&Paddle){
    let ball_rect = Rect::new(
        ball.pos.x - ball.radius,
        ball.pos.y - ball.radius,
        ball.radius * 2.0,
        ball.radius * 2.0,
    );
    if ball.vel.x < 0.0 && ball_rect.overlaps(&paddle_left.rect()){
        ball.bounce_off_paddle(paddle_left, true);
    }else if ball.vel.x > 0.0 && ball_rect.overlaps(&paddle_right.rect()){
        ball.bounce_off_paddle(paddle_right, false);
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut app = App::new();


    loop {
        let dt = get_frame_time();
        
        match app.state {
            GameState::MainMenu => app.update_menu(),
            GameState::Controls => app.update_controls(),
            GameState::Countdown => app.update_countdown(dt),
            GameState::Playing => app.update_playing(dt),
            GameState::Replay => app.update_replay(dt),
            GameState::GameOver => app.update_game_over(),
        }
        match app.state{
            GameState::MainMenu => {
                clear_background(BLACK);
                app.draw_menu();
            }
            GameState::Controls =>{
                clear_background(BLACK);
                app.draw_controls();
            }
            GameState::Countdown => app.draw_countdown(),
            GameState::Playing => app.draw_field(),
            GameState::Replay => app.draw_replay(),
            GameState::GameOver => app.draw_game_over(),
        }
        next_frame().await;
    }
}
