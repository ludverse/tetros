use std::time::Instant;
use crate::{Cord, BLOCK_SIZE, GAME_POS, GAME_WIDTH, GAME_HEIGHT};
use crate::game::{self, Game};
use crate::tetros::GameTetro;

pub fn shift_tetro(game: &mut Game, x_amount: i32) {
    let mut next = game.dropping_tetro;
    next.cord.0 += x_amount;

    if !game::is_tetro_colliding(game.blocks, next) {
        game.dropping_tetro = next;
    }
}

pub fn rotate_tetro(game: &mut Game, rotate_times: i32) {
    let mut next = game.dropping_tetro;
    next.rotation = (next.rotation as i32 + rotate_times).rem_euclid(3 + 1) as usize; // calculate modulus of new rotation, NOT remainder which is %

    if !game::is_tetro_colliding(game.blocks, next) {
        game.dropping_tetro = next;
    }
}

pub fn hard_drop(game: &mut Game) {
    let mut next = game.dropping_tetro;

    let mut cells_dropped = 0;
    for i in game.dropping_tetro.cord.1..GAME_HEIGHT as i32 {
        next.cord.1 = i;
        if game::is_tetro_colliding(game.blocks, next) { break }

        cells_dropped += 1;
    }
    next.cord.1 -= 1;

    game.score += cells_dropped * 2;

    game::petrify_tetro(&mut game.blocks, next);
    let lines_cleared = game::clear_lines(&mut game.blocks);
    game.add_lines_cleared(lines_cleared);

    game.next_tetro();
}

pub fn hold_tetro(game: &mut Game) {
    let hold_tetro = game.hold_tetro;

    game.hold_tetro = Some(game.dropping_tetro.tetro_type);

    if let Some(hold_tetro) = hold_tetro {
        game.dropping_tetro = GameTetro::new(hold_tetro, hold_tetro.start_pos(), 0);
        game.last_drop_timing = Instant::now();
    } else {
        game.next_tetro();
    }

}

