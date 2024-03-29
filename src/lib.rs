use crate::tetros::TetroType;

pub mod game;
pub mod gui;
pub mod tetros;
pub mod controls;
pub mod bot;

pub const BLOCK_SIZE: i32 = 30;
pub const GAME_POS: Pos = Pos(6 * BLOCK_SIZE, 1 * BLOCK_SIZE);
pub const GAME_WIDTH: i32 = 10;
pub const GAME_HEIGHT: i32 = 20;
pub const FONT_CHAR_WIDTH: i32 = BLOCK_SIZE / 2;
pub const FONT_CHAR_HEIGHT: i32 = BLOCK_SIZE;


#[derive(Copy, Clone, Debug)]
pub struct Cord(pub i32, pub i32);

impl Cord {
    pub fn pos(&self) -> Pos {
        Pos(self.0 * BLOCK_SIZE + GAME_POS.0, self.1 * BLOCK_SIZE + GAME_POS.1)
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

