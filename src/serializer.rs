use std::{time::Instant, fs, process::Command};

use serde::{Serialize, Deserialize};

use crate::{
    game::Game,
    tetros::{TetroType, GameTetro},
    GAME_WIDTH,
    GAME_HEIGHT
};

#[derive(Debug, Serialize, Deserialize)]
pub struct GameData {
    pub lines: i32,
    pub score: i32,
    pub tetro_queue: Vec<TetroType>,
    pub dropping_tetro: GameTetro,
    pub hold_tetro: Option<TetroType>,
    pub blocks: Vec<Option<TetroType>>
}

impl GameData {
    pub fn from_game(game: &Game) -> Self {
        Self {
            lines: game.lines,
            score: game.score,
            tetro_queue: game.tetro_queue.clone(),
            dropping_tetro: game.dropping_tetro,
            hold_tetro: game.hold_tetro,
            blocks: game.blocks.iter().cloned().collect()
        }
    }

    pub fn to_game(&self) -> Game {
        let mut blocks = [None; (GAME_WIDTH * GAME_HEIGHT) as usize];

        for (i, game_block) in self.blocks.iter().enumerate() {
            blocks[i] = game_block.clone();
        }

        Game {
            lines: self.lines,
            score: self.score,
            dropping_tetro: self.dropping_tetro,
            tetro_queue: self.tetro_queue.clone(),
            hold_tetro: self.hold_tetro,
            blocks,
            is_soft_dropping: false,
            last_drop_timing: Instant::now(),
            lock_delay: Instant::now(),
            is_playing: false
        }
    }
}

 pub fn show_debug_game(game: &Game) {
    let game_data = GameData::from_game(&game.clone());

    fs::write("failed.json", serde_json::to_string(&game_data).unwrap())
        .expect("failed to write");

    Command::new("cargo")
        .args(["run", "--bin", "tetros-viewer", "--", "failed.json"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
 }
