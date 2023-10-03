use std::time::Instant;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::ttf::Font;

use crate::{Cord, Pos, BLOCK_SIZE, GAME_POS, GAME_WIDTH, GAME_HEIGHT, HOLD_SCREEN_POS, FONT_CHAR_WIDTH, FONT_CHAR_HEIGHT};
use crate::tetros::{GameTetro, TetroType};

pub struct Game {
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

            if is_tetro_colliding(self.blocks, next) {
                petrify_tetro(&mut self.blocks, self.dropping_tetro);
                clear_lines(&mut self.blocks);
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

    pub fn draw(&self, canvas: &mut Canvas<Window>, font: &Font) {
        let texture_creator = canvas.texture_creator();

        let surface = font
            .render("HOLD")
            .blended(Color::RGBA(255, 0, 0, 255))
            .unwrap();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();

        canvas.copy(&texture, None, Some(Rect::new(20, 20, FONT_CHAR_WIDTH as u32 * 4, FONT_CHAR_HEIGHT as u32)));

        if let Some(hold_piece) = self.hold_tetro {
            let center_offset = (4 - hold_piece.shape_size()) as f64 / 2. * BLOCK_SIZE as f64;
            let center_pos = Pos(HOLD_SCREEN_POS.0 + center_offset as i32, HOLD_SCREEN_POS.1 + center_offset as i32);
            hold_piece.draw(canvas, center_pos, 0, false);
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.fill_rect(Rect::new(GAME_POS.0 - 4, GAME_POS.1 - 4, (BLOCK_SIZE * GAME_WIDTH + 8) as u32, (BLOCK_SIZE * GAME_HEIGHT + 8) as u32)).unwrap();

        canvas.set_draw_color(Color::RGB(189, 195, 199));
        canvas.fill_rect(Rect::new(GAME_POS.0, GAME_POS.1, (BLOCK_SIZE * GAME_WIDTH) as u32, (BLOCK_SIZE * GAME_HEIGHT) as u32)).unwrap();

        let border_size = BLOCK_SIZE / 10;

        for (i, block) in self.blocks.iter().enumerate() {
            let relative_cord = Cord(i as i32 % GAME_WIDTH as i32, i as i32 / GAME_WIDTH as i32);
            let relative_pos = relative_cord.pos();

            let mut block_colour = (Color::RGB(189, 195, 199), Color::RGB(181, 187, 191));

            if let Some(block) = block {
                block_colour = block.colour();
            }

            canvas.set_draw_color(block_colour.1);
            canvas.fill_rect(Rect::new(relative_pos.0, relative_pos.1, BLOCK_SIZE as u32, BLOCK_SIZE as u32)).unwrap();

            canvas.set_draw_color(block_colour.0);
            canvas.fill_rect(Rect::new(relative_pos.0 + border_size as i32, relative_pos.1 + border_size as i32, (BLOCK_SIZE - border_size * 2) as u32, (BLOCK_SIZE - border_size * 2) as u32)).unwrap();
        }

        let mut ghost = self.dropping_tetro;
        for i in self.dropping_tetro.cord.1..GAME_HEIGHT as i32 {
            ghost.cord.1 = i;
            if is_tetro_colliding(self.blocks, ghost) { break }
        }
        ghost.cord.1 -= 1;

        self.dropping_tetro.tetro_type.draw(canvas, ghost.cord.pos(), self.dropping_tetro.rotation, true);

        self.dropping_tetro.draw(canvas);
    }
}

pub fn petrify_tetro(blocks: &mut [Option<TetroType>; (GAME_WIDTH * GAME_HEIGHT) as usize], tetro: GameTetro) {
    let shape = tetro.tetro_type.shape(tetro.rotation);
    let shape_size = tetro.tetro_type.shape_size();

    for i in 0..shape_size * shape_size {
        let bit = shape >> i & 1;
        if bit == 0 { continue }

        let relative_cord = Cord(i as i32 % shape_size as i32, i as i32 / shape_size as i32);
        let cord = Cord(tetro.cord.0 + relative_cord.0, tetro.cord.1 + relative_cord.1);

        blocks[(cord.0 + cord.1 * GAME_WIDTH as i32) as usize] = Some(tetro.tetro_type);
    }
}

pub fn is_tetro_colliding(blocks: [Option<TetroType>; (GAME_WIDTH * GAME_HEIGHT) as usize], tetro: GameTetro) -> bool {
    if tetro.cord.is_outside_game(tetro.tetro_type, tetro.rotation) { return true }

    let shape = tetro.tetro_type.shape(tetro.rotation);
    let shape_size = tetro.tetro_type.shape_size();

    for i in 0..shape_size * shape_size {
        let bit = shape >> i & 1;
        if bit == 0 { continue }

        let relative_cord = Cord(i as i32 % shape_size as i32, i as i32 / shape_size as i32);
        let cord = Cord(tetro.cord.0 + relative_cord.0, tetro.cord.1 + relative_cord.1);

        let block = blocks[(cord.0 + cord.1 * GAME_WIDTH as i32) as usize];

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

