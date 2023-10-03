use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget};

use crate::{Cord, Pos, BLOCK_SIZE, GAME_WIDTH};

#[derive(Copy, Clone, Debug)]
pub enum TetroType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl TetroType {
    pub fn shape(&self, rotation: usize) -> u16 {
        //let mut shape: u16 = match self {
        //TetroType::I => 0b0000111100000000,
        //TetroType::J => 0b100111000,
        //TetroType::L => 0b001111000,
        //TetroType::O => 0b1111,
        //TetroType::S => 0b011110000,
        //TetroType::T => 0b010111000,
        //TetroType::Z => 0b110011000,
        //};

        let mut shape: u16 = match self {
            TetroType::I => 0b0000000011110000,
            TetroType::J => 0b000111001,
            TetroType::L => 0b000111100,
            TetroType::O => 0b1111,
            TetroType::S => 0b000011110,
            TetroType::T => 0b000111010,
            TetroType::Z => 0b000110011,
        };

        let size = self.shape_size();

        for _ in 0..rotation {
            let mut rotated_shape = shape;

            for i in 0..size * size {
                let bit = shape >> i & 1;

                let new_x = size - 1 - i as i32 / size;
                let new_y = i as i32 % size;

                let new_index = (new_x + new_y * size) as usize;

                rotated_shape = (rotated_shape & !(1 << new_index)) | (bit << new_index);
            }

            shape = rotated_shape;
        }

        shape
    }

    pub fn shape_size(&self) -> i32 {
        match self {
            TetroType::I => 4,
            TetroType::J => 3,
            TetroType::L => 3,
            TetroType::O => 2,
            TetroType::S => 3,
            TetroType::T => 3,
            TetroType::Z => 3,
        }
    }

    pub fn colour(&self) -> (Color, Color) {
        match self {
            TetroType::I => (Color::RGB(69, 170, 242), Color::RGB(45, 152, 218)),
            TetroType::J => (Color::RGB(75, 123, 236), Color::RGB(56, 103, 214)),
            TetroType::L => (Color::RGB(253, 150, 68), Color::RGB(250, 130, 49)),
            TetroType::O => (Color::RGB(254, 211, 48), Color::RGB(247, 183, 49)),
            TetroType::S => (Color::RGB(38, 222, 129), Color::RGB(32, 191, 107)),
            TetroType::T => (Color::RGB(165, 94, 234), Color::RGB(136, 84, 208)),
            TetroType::Z => (Color::RGB(252, 92, 101), Color::RGB(235, 59, 90)),
        }
    }

    pub fn start_pos(&self) -> Cord {
        Cord((GAME_WIDTH as i32 - self.shape_size()) / 2, 0 )
    }

    pub fn draw(&self, canvas: &mut Canvas<impl RenderTarget>, pos: Pos, rotation: usize, ghost: bool) {
        let shape = self.shape(rotation);
        let shape_size = self.shape_size();

        let border_size = BLOCK_SIZE / 10;

        for i in 0..shape_size * shape_size {
            let bit = shape >> i & 1;
            if bit == 0 { continue; }

            let relative_cord = Cord(i as i32 % shape_size, i as i32 / shape_size);
            let pos = Pos(
                pos.0 + relative_cord.0 * BLOCK_SIZE,
                pos.1 + relative_cord.1 * BLOCK_SIZE,
            );

            canvas.set_draw_color(self.colour().1);
            canvas.fill_rect(Rect::new(pos.0, pos.1, BLOCK_SIZE as u32, BLOCK_SIZE as u32)).unwrap();

            if ghost {
                canvas.set_draw_color(Color::RGB(189, 195, 199));
            } else {
                canvas.set_draw_color(self.colour().0);
            }

            canvas.fill_rect(Rect::new(pos.0 + border_size as i32, pos.1 + border_size as i32, (BLOCK_SIZE - border_size * 2) as u32, (BLOCK_SIZE - border_size * 2) as u32)).unwrap();
        }
    }

    pub fn draw_centered(&self, canvas: &mut Canvas<impl RenderTarget>, origin_pos: Pos) {
        let center_offset = (4 - self.shape_size()) as f64 / 2. * BLOCK_SIZE as f64;
        let center_pos = Pos(origin_pos.0 + center_offset as i32, origin_pos.1 + center_offset as i32);

        self.draw(canvas, center_pos, 0, false);
    }

    pub fn random_tetro() -> TetroType {
        let mut rng = thread_rng();

        match rng.gen_range(0..=6) {
            0 => TetroType::I,
            1 => TetroType::J,
            2 => TetroType::L,
            3 => TetroType::O,
            4 => TetroType::S,
            5 => TetroType::T,
            6 => TetroType::Z,
            _ => panic!("invalid random value"),
        }
    }

    pub fn random_set() -> Vec<TetroType> {
        let mut rng = thread_rng();

        let mut set = vec![
            TetroType::I,
            TetroType::J,
            TetroType::L,
            TetroType::O,
            TetroType::S,
            TetroType::T,
            TetroType::Z,
        ];

        set.shuffle(&mut rng);

        set
    }
}

#[derive(Copy, Clone, Debug)]
pub struct GameTetro {
    pub tetro_type: TetroType,
    pub cord: Cord,
    pub rotation: usize,
}

impl GameTetro {
    pub fn new(tetro: TetroType, cord: Cord, rotation: usize) -> GameTetro {
        GameTetro {
            tetro_type: tetro,
            cord,
            rotation,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<impl RenderTarget>) {
        self.tetro_type.draw(canvas, self.cord.pos(), self.rotation, false);
    }
}

