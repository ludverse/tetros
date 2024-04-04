use std::time::Instant;

use rand::{thread_rng, Rng};

use crate::{
    GAME_WIDTH,
    GAME_HEIGHT,
    controls,
    game::{self, Game},
    tetros::{TetroType, GameTetro},
    serializer::show_debug_game
};

#[derive(Clone, Copy, Debug, Default)]
pub struct Weigths {
    pub holes_penalty: i32,
    pub bumpiness_penalty: i32,
    pub height_penalty: i32,
    pub line_clearing: [i32; 4]
}

impl Weigths {
    pub fn mutate(&self, diff: i32) -> Self {
        let mut rng = thread_rng();

        let mut mutate_num = |diff_multiplier, value| rng.gen_range(value - diff * diff_multiplier..value + diff * diff_multiplier);

        Self {
            holes_penalty: mutate_num(1, self.holes_penalty),
            bumpiness_penalty: mutate_num(1, self.holes_penalty),
            height_penalty: mutate_num(1, self.holes_penalty),
            line_clearing: [
                mutate_num(4, self.line_clearing[0]),
                mutate_num(4, self.line_clearing[1]),
                mutate_num(4, self.line_clearing[2]),
                mutate_num(4, self.line_clearing[3])
            ]
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Bot {
    pub weights: Weigths,
    pub depth: usize
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
        if lines_cleared != 0 { self.weights.line_clearing[lines_cleared - 1] } else { 0 }
    }

    fn alternate_universe(&self, mut game: Game, depth: usize) -> (i32, (bool, i32, usize)) {
        let mut best_move: Option<(i32, (bool, i32, usize))> = None;

        for use_hold in 0..=1 {
            let use_hold = use_hold == 1;

            for rotation in 0..4usize {
                for x in -2..GAME_WIDTH as i32 {
                    let mut game = game.clone();

                    if use_hold { controls::hold_tetro(&mut game); };

                    game.dropping_tetro.cord = game.dropping_tetro.tetro_type.start_pos();
                    game.dropping_tetro.cord.0 = x;
                    game.dropping_tetro.rotation = rotation;

                    if game::is_tetro_colliding(game.blocks, game.dropping_tetro) { continue; };

                    for _ in 0..GAME_HEIGHT as i32 {
                        game.dropping_tetro.cord.1 += 1;
                        if game::is_tetro_colliding(game.blocks, game.dropping_tetro) { break; };
                    }
                    game.dropping_tetro.cord.1 -= 1;

                    game::petrify_tetro(&mut game.blocks, game.dropping_tetro);
                    let lines_cleared = game::clear_lines(&mut game.blocks);

                    game.next_tetro();

                    let game_score = self.fitness_function(game.blocks, lines_cleared);
                    if depth == 0 || !game.is_playing {
                        if best_move.is_none() || game_score > best_move.unwrap().0 {
                            best_move = Some((game_score, (use_hold, x as i32, rotation)));
                        }
                    } else {
                        let universe_score = self.alternate_universe(game, depth - 1);
                        let game_score = universe_score.0 + game_score;

                        if best_move.is_none() || game_score > best_move.unwrap().0 {
                            best_move = Some((game_score, (use_hold, x as i32, rotation)));
                        }
                    }
                }
            }
        }

        if best_move.is_none() {
            dbg!(depth);
            dbg!(game.blocks, game.dropping_tetro);

            show_debug_game(&game);
        }

        best_move.unwrap()
    }

    pub fn best_move(&self, game: &Game) -> Option<(bool, i32, usize)> {
        // let started = Instant::now();

        let best_move = self.alternate_universe(game.clone(), self.depth);

        // println!(
        //     "SELECTED {} - hold: {}, x: {}, rot: {} - {} ms",
        //     best_move.0,
        //     best_move.1.0,
        //     best_move.1.1,
        //     best_move.1.2,
        //     started.elapsed().as_millis()
        // );

        Some(best_move.1)
    }
}

impl Default for Bot {
    fn default() -> Self {
        Self {
            weights: Weigths::default(),
            depth: 1
        }
    }
}
