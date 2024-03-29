use std::time::{Instant, Duration};

use crate::{Cord, TETRO_TYPES_AMOUNT, GAME_WIDTH, GAME_HEIGHT};
use crate::tetros::{GameTetro, TetroType};

#[derive(Clone)]
pub struct Game {
    pub lines: i32,
    pub score: i32,
    pub tetro_queue: Vec<TetroType>,
    pub dropping_tetro: GameTetro,
    pub hold_tetro: Option<TetroType>,
    pub blocks: [Option<TetroType>; (GAME_WIDTH * GAME_HEIGHT) as usize],
    pub is_soft_dropping: bool,
    pub last_drop_timing: Instant,
    pub lock_delay: Instant
}

impl Game {
    pub fn new() -> Self {
        let mut tetro_queue = TetroType::random_set();
        tetro_queue.append(&mut TetroType::random_set());
        let dropping_tetro_type = tetro_queue.pop().unwrap();

        Self {
            lines: 0,
            score: 0,
            dropping_tetro: GameTetro::new(dropping_tetro_type, dropping_tetro_type.start_pos(), 0),
            tetro_queue,
            hold_tetro: None,
            blocks: [None; 10 * 20],
            is_soft_dropping: false,
            last_drop_timing: Instant::now(),
            lock_delay: Instant::now()
        }
    }

    pub fn next_frame(&mut self) {
        let drop_delta = self.last_drop_timing.elapsed();

        let mut next = self.dropping_tetro;
        next.cord.1 += 1;

        if is_tetro_colliding(self.blocks, next) {
            if self.lock_delay.elapsed() > Duration::from_millis(500) {
                petrify_tetro(&mut self.blocks, self.dropping_tetro);

                let lines_cleared = clear_lines(&mut self.blocks);
                self.add_lines_cleared(lines_cleared);

                self.next_tetro();

                self.last_drop_timing = Instant::now();
                self.lock_delay = Instant::now();
            }
        } else {
            if drop_delta > Duration::from_millis(if self.is_soft_dropping { 50 } else { 750 }) {
                self.dropping_tetro = next;

                if self.is_soft_dropping { self.score += 1; };

                self.last_drop_timing = Instant::now();
                self.lock_delay = Instant::now();
            }
        }
    }

    pub fn next_tetro(&mut self) {
        let tetro_type = self.tetro_queue.pop().unwrap();

        if self.tetro_queue.len() < TETRO_TYPES_AMOUNT {
            self.tetro_queue.append(&mut TetroType::random_set());
        }

        self.dropping_tetro = GameTetro::new(tetro_type, tetro_type.start_pos(), 0);
        self.last_drop_timing = Instant::now();
    }

    pub fn get_next_tetro(&self) -> TetroType {
        *self.tetro_queue.last().unwrap()
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

