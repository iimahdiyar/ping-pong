#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    MainMenu,
    Controls,
    Countdown,
    Playing,
    Replay,
    GameOver,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
    OnePlayer,
    TwoPlayer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    pub fn ai_speed(&self) -> f32 {
        match self {
            Difficulty::Easy => 200.0,
            Difficulty::Medium => 320.0,
            Difficulty::Hard => 460.0,
        }
    }

    pub fn aim_error(&self) -> f32 {
        match self {
            Difficulty::Easy => 40.0,
            Difficulty::Medium => 18.0,
            Difficulty::Hard => 6.0,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Difficulty::Easy => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard => "Hard",
        }
    }

    pub fn cycle(&self) -> Difficulty {
        match self {
            Difficulty::Easy => Difficulty::Medium,
            Difficulty::Medium => Difficulty::Hard,
            Difficulty::Hard => Difficulty::Easy,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Winner {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScoreEvent {
    NoScore,
    LeftScored,
    RightScored,
    Blocked,
}
