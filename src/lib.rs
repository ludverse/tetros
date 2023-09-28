use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget};

use crate::tetros::{TetroType, GameTetro};

pub mod tetros;

pub const BLOCK_SIZE: u32 = 40;
pub const GAME_WIDTH: u32 = 10;
pub const GAME_HEIGHT: u32 = 20;

#[derive(Copy, Clone, Debug)]
pub struct Cord(pub i32, pub i32);

impl Cord {
    pub fn pos(&self, game_pos: Pos) -> Pos {
        Pos(self.0 * BLOCK_SIZE as i32 + game_pos.0, self.1 * BLOCK_SIZE as i32 + game_pos.1)
    }

    pub fn is_outside_game(&self, tetro_type: TetroType, rotation: usize) -> bool {
        let shape = tetro_type.shape(rotation);
        let shape_size = tetro_type.shape_size();

        for i in 0..shape_size * shape_size {
            let bit = shape >> i & 1;
            if bit == 0 { continue }

            let relative_cord = Cord(i as i32 % shape_size as i32, i as i32 / shape_size as i32);

            let is_outside = self.0 + relative_cord.0 < 0 ||
                self.0 + relative_cord.0 >= GAME_WIDTH as i32 ||
                self.1 + relative_cord.1 >= GAME_HEIGHT as i32;

            if is_outside { return true }
        }

        false
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Pos(pub i32, pub i32);

pub struct Game {
    pub game_pos: Pos,
    pub hold_screen_pos: Pos,
    pub next_tetros: Vec<TetroType>,
    pub dropping_tetro: GameTetro,
    pub hold_tetro: Option<TetroType>,
    pub blocks: [Option<TetroType>; 10 * 20]
}

impl Game {
    pub fn new() -> Game {
        let game_pos = Pos(BLOCK_SIZE as i32 * 6, BLOCK_SIZE as i32);

        let mut next_tetros = TetroType::random_set();

        Game {
            game_pos,
            hold_screen_pos: Pos(BLOCK_SIZE as i32, BLOCK_SIZE as i32),
            dropping_tetro: GameTetro::new(next_tetros.pop().unwrap(), Cord(2, 0), 0),
            hold_tetro: None,
            next_tetros,
            blocks: [None; 10 * 20]
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<impl RenderTarget>) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.fill_rect(Rect::new(self.game_pos.0 - 4, self.game_pos.1 - 4, BLOCK_SIZE * GAME_WIDTH + 8, BLOCK_SIZE * GAME_HEIGHT + 8)).unwrap();

        canvas.set_draw_color(Color::RGB(189, 195, 199));
        canvas.fill_rect(Rect::new(self.game_pos.0, self.game_pos.1, BLOCK_SIZE * GAME_WIDTH, BLOCK_SIZE * GAME_HEIGHT)).unwrap();

        let border_size = BLOCK_SIZE / 10;

        for (i, block) in self.blocks.iter().enumerate() {
            let relative_cord = Cord(i as i32 % GAME_WIDTH as i32, i as i32 / GAME_WIDTH as i32);
            let relative_pos = relative_cord.pos(self.game_pos);

            let mut block_colour = (Color::RGB(189, 195, 199), Color::RGB(181, 187, 191));

            if let Some(block) = block {
                block_colour = block.colour();
            }

            canvas.set_draw_color(block_colour.1);
            canvas.fill_rect(Rect::new(relative_pos.0, relative_pos.1, BLOCK_SIZE, BLOCK_SIZE)).unwrap();

            canvas.set_draw_color(block_colour.0);
            canvas.fill_rect(Rect::new(relative_pos.0 + border_size as i32, relative_pos.1 + border_size as i32, BLOCK_SIZE - border_size * 2, BLOCK_SIZE - border_size * 2)).unwrap();
        }

        self.dropping_tetro.draw(canvas, self);
    }

    pub fn next_tetro(&mut self) {
        let tetro_type = self.next_tetros.pop().unwrap_or_else(|| {
            if self.next_tetros.len() == 0 { self.next_tetros = TetroType::random_set() }
            self.next_tetros.pop().unwrap()
        });

        self.dropping_tetro = GameTetro::new(tetro_type, Cord(0, 0), 0);
    }

    pub fn petrify_dropping_tetro(&mut self) {
        let cord = self.dropping_tetro.cord;

        let shape = self.dropping_tetro.tetro_type.shape(self.dropping_tetro.rotation);
        let shape_size = self.dropping_tetro.tetro_type.shape_size();

        for i in 0..shape_size * shape_size {
            let bit = shape >> i & 1;
            if bit == 0 { continue }

            let relative_cord = Cord(i as i32 % shape_size as i32, i as i32 / shape_size as i32);
            let cord = Cord(cord.0 + relative_cord.0, cord.1 + relative_cord.1);

            self.blocks[(cord.0 + cord.1 * GAME_WIDTH as i32) as usize] = Some(self.dropping_tetro.tetro_type);
        }
    }

    pub fn is_tetro_colliding(&self, tetro_type: TetroType, cord: Cord, rotation: usize) -> bool {
        if cord.is_outside_game(tetro_type, rotation) { return true }

        let shape = tetro_type.shape(rotation);
        let shape_size = tetro_type.shape_size();

        for i in 0..shape_size * shape_size {
            let bit = shape >> i & 1;
            if bit == 0 { continue }

            let relative_cord = Cord(i as i32 % shape_size as i32, i as i32 / shape_size as i32);
            let cord = Cord(cord.0 + relative_cord.0, cord.1 + relative_cord.1);

            let block = self.blocks[(cord.0 + cord.1 * GAME_WIDTH as i32) as usize];

            if block.is_some() { return true }
        }

        false
        // for i in (0..GAME_HEIGHT).rev() {
        // }
    }
}
