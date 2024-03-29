use std::time::Instant;

use crate::{Cord, GAME_WIDTH, GAME_HEIGHT};
use crate::tetros::{GameTetro, TetroType};

#[derive(Clone)]
pub struct Game {
    pub lines: i32,
    pub score: i32,
    pub next_tetros_sets: Vec<Vec<TetroType>>,
    pub dropping_tetro: GameTetro,
    pub hold_tetro: Option<TetroType>,
    pub blocks: [Option<TetroType>; (GAME_WIDTH * GAME_HEIGHT) as usize],
    pub is_soft_dropping: bool,
    pub last_drop_timing: Instant
}

impl Game {
    pub fn new() -> Self {
        let mut next_tetros_sets = vec![TetroType::random_set(), TetroType::random_set()];
        let dropping_tetro_type = next_tetros_sets[0].pop().unwrap();

        Self {
            lines: 0,
            score: 0,
            dropping_tetro: GameTetro::new(dropping_tetro_type, dropping_tetro_type.start_pos(), 0),
            next_tetros_sets,
            hold_tetro: None,
            blocks: [None; 10 * 20],
            is_soft_dropping: false,
            last_drop_timing: Instant::now(),
        }
    }

    pub fn next_frame(&mut self) {
        let now = Instant::now();
        let drop_delta = now - self.last_drop_timing;

        if drop_delta.as_millis() > (if self.is_soft_dropping { 50 } else { 750 }) {
            let mut next = self.dropping_tetro;
            next.cord.1 += 1;

            if self.is_soft_dropping { self.score += 1; };

            if is_tetro_colliding(self.blocks, next) {
                petrify_tetro(&mut self.blocks, self.dropping_tetro);

                let lines_cleared = clear_lines(&mut self.blocks);
                self.add_lines_cleared(lines_cleared);

                self.next_tetro();
            } else {
                self.dropping_tetro = next;
            }

            self.last_drop_timing = Instant::now();
        }
    }

    pub fn next_tetro(&mut self) {
        let tetro_type = self.next_tetros_sets[0].pop().unwrap_or_else(|| {
            self.next_tetros_sets.remove(0);
            self.next_tetros_sets.push(TetroType::random_set());
            self.next_tetros_sets[0].pop().unwrap()
        });

        self.dropping_tetro = GameTetro::new(tetro_type, tetro_type.start_pos(), 0);
        self.last_drop_timing = Instant::now();
    }

    pub fn get_next_tetro(&self) -> TetroType {
        self.next_tetros_sets[0].last().unwrap_or_else(|| {
            self.next_tetros_sets[1].last().unwrap()
        }).clone()
    }

    pub fn add_lines_cleared(&mut self, lines_cleared: usize) {
        let score_multiplier = match lines_cleared {
            1 => 100,
            2 => 300,
            3 => 500,
            4 => 800,
            _ => 0
        };

        self.score += (self.lines / 10 + 1) * score_multiplier;

        self.lines += lines_cleared as i32;
    }
}

pub fn petrify_tetro(blocks: &mut [Option<TetroType>; (GAME_WIDTH * GAME_HEIGHT) as usize], tetro: GameTetro) {
    let shape = tetro.tetro_type.shape(tetro.rotation);
    let shape_size = tetro.tetro_type.shape_size();

    for i in 0..shape_size * shape_size {
        let bit = shape >> i & 1;
        if bit == 0 { continue }

        let relative_cord = Cord(i as i32 % shape_size, i as i32 / shape_size);
        let cord = Cord(tetro.cord.0 + relative_cord.0, tetro.cord.1 + relative_cord.1);

        blocks[(cord.0 + cord.1 * GAME_WIDTH) as usize] = Some(tetro.tetro_type);
    }
}

pub fn is_tetro_colliding(blocks: [Option<TetroType>; (GAME_WIDTH * GAME_HEIGHT) as usize], tetro: GameTetro) -> bool {
    if tetro.cord.is_outside_game(tetro.tetro_type, tetro.rotation) { return true }

    let shape = tetro.tetro_type.shape(tetro.rotation);
    let shape_size = tetro.tetro_type.shape_size();

    for i in 0..shape_size * shape_size {
        let bit = shape >> i & 1;
        if bit == 0 { continue }

        let relative_cord = Cord(i as i32 % shape_size, i as i32 / shape_size);
        let cord = Cord(tetro.cord.0 + relative_cord.0, tetro.cord.1 + relative_cord.1);

        let block = blocks[(cord.0 + cord.1 * GAME_WIDTH) as usize];

        if block.is_some() { return true }
    }

    false
}

pub fn clear_lines(blocks: &mut [Option<TetroType>; (GAME_WIDTH * GAME_HEIGHT) as usize]) -> usize {
    let mut lines_to_remove = [false; GAME_HEIGHT as usize];

    for y in (0..GAME_HEIGHT as usize).rev() {
        let row_is_full = (0..GAME_WIDTH as usize).all(|x| {
            blocks[x + y * GAME_WIDTH as usize].is_some()
        });
        
        if row_is_full {
            lines_to_remove[y] = true;
        }
    }

    let mut lines_removed = 0;
    for y in (0..GAME_HEIGHT as usize).rev() {
        if lines_to_remove[y] {
            lines_removed += 1;
        } else if lines_removed != 0 {
            for x in 0..GAME_WIDTH as usize {
                blocks[x + (y + lines_removed) * GAME_WIDTH as usize] = blocks[x + y * GAME_WIDTH as usize];
            }
        }
    }

    lines_removed
}

