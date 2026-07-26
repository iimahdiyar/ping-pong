use macroquad::prelude::*;
use std::collections::VecDeque;

pub const MAX_SNAPSHOTS: usize = 300;

#[derive(Debug, Clone, Copy)]
pub struct Snapshot {
    pub ball_pos: Vec2,
    pub paddle_left_y: f32,
    pub paddle_left_h: f32,
    pub paddle_right_y: f32,
    pub paddle_right_h: f32,
}

pub struct ReplayBuffer {
    frames: VecDeque<Snapshot>,
}

impl ReplayBuffer {
    pub fn new() -> Self {
        ReplayBuffer {
            frames: VecDeque::with_capacity(MAX_SNAPSHOTS),
        }
    }

    pub fn push(&mut self, snap: Snapshot) {
        if self.frames.len() >= MAX_SNAPSHOTS {
            self.frames.pop_front();
        }
        self.frames.push_back(snap);
    }

    pub fn clear(&mut self) {
        self.frames.clear();
    }

    pub fn len(&self) -> usize {
        self.frames.len()
    }

    pub fn snapshot_vec(&self) -> Vec<Snapshot> {
        self.frames.iter().cloned().collect()
    }
}
