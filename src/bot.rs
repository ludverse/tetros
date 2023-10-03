use crate::{Cord, GAME_WIDTH, GAME_HEIGHT};
use crate::game::{self, Game};
use crate::tetros::{TetroType, GameTetro};

pub struct Weigths {
    pub holes_penalty: i32,
    pub bumpiness_penalty: i32,
    pub height_penalty: i32,
    pub line_clearing: [i32; 4]
}

pub struct Bot {
    pub weights: Weigths
}

impl Bot {
    fn fitness_function(&self, blocks: [Option<TetroType>; (GAME_WIDTH * GAME_HEIGHT) as usize], lines_cleared: usize) -> i32 {
        let mut holes_amount = 0;
        let mut bumpiness = 0;
        let mut last_col_height = None;
        let mut height_penalty = 0;

        for x in 0..GAME_WIDTH as usize {
            let mut col_height = 0;
            for y in 0..GAME_HEIGHT as i32 {
                if blocks[x as usize + y as usize * GAME_WIDTH as usize].is_some() {
                    if col_height == 0 {
                        col_height = GAME_HEIGHT as i32 - y;
                        height_penalty += GAME_HEIGHT as i32 - y;
                    }

                } else if col_height != 0 {
                    holes_amount += 1;
                }
            }

            if let Some(last_col_height) = last_col_height {
                bumpiness += i32::abs(last_col_height - col_height);
            }
            last_col_height = Some(col_height);
        }

        -holes_amount * self.weights.holes_penalty +
        -bumpiness * self.weights.bumpiness_penalty +
        -height_penalty * self.weights.height_penalty + 
        if lines_cleared != 0 { lines_cleared as i32 * self.weights.line_clearing[lines_cleared - 1] } else { 0 }
    }

    pub fn best_move(&self, game: &Game) -> Option<(bool, i32, usize)> {
        let mut best_move: Option<(i32, (bool, i32, usize))> = None;

        let mut best_move_for_tetro = |use_hold| {
            let tetro_type = if use_hold {
                if let Some(hold_tetro) = game.hold_tetro {
                    hold_tetro
                } else {
                    game.what_is_the_next_tetro_type_comming_up()
                }
            } else {
                game.dropping_tetro.tetro_type
            };

            for rotation in 0..4usize {
                for x in -2..GAME_WIDTH as i32 {
                    let mut dropped = GameTetro::new(tetro_type, Cord(x as i32, 0), rotation);

                    let mut blocks = game.blocks;

                    if game::is_tetro_colliding(blocks, dropped) { continue; }

                    for y in 0..GAME_HEIGHT as i32 {
                        dropped.cord.1 = y;
                        if game::is_tetro_colliding(blocks, dropped) { break }
                    }
                    dropped.cord.1 -= 1;

                    game::petrify_tetro(&mut blocks, dropped);
                    let lines_cleared = game::clear_lines(&mut blocks);


                    let move_score = self.fitness_function(blocks, lines_cleared);
                    println!(" {} - hold: {}, x: {}, rot: {}", move_score, use_hold, x, rotation);
                    if best_move.is_none() || move_score > best_move.unwrap().0 {
                        best_move = Some((move_score, (use_hold, x as i32, rotation)));
                    }
                }
            }
        };

        best_move_for_tetro(false);
        best_move_for_tetro(true);

        println!("SELECTED {} - hold: {}, x: {}, rot: {}", best_move?.0, best_move?.1.0, best_move?.1.1, best_move?.1.2);

        Some(best_move?.1)
    }
}
