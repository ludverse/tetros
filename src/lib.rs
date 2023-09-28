use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget};

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

            if is_outside {
                return true;
            }
        }

        false
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Pos(pub i32, pub i32);

#[derive(Copy, Clone, Debug)]
pub enum TetroType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z
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
            TetroType::I => 00000000011110000,
            TetroType::J => 0b000111001,
            TetroType::L => 0b000111100,
            TetroType::O => 0b1111,
            TetroType::S => 0b000011110,
            TetroType::T => 0b000111010,
            TetroType::Z => 0b000110011
        };

        let size = self.shape_size();

        for _ in 0..rotation {
            let mut rotated = shape;

            for i in 0..size * size {
                let bit = shape >> i & 1;

                let new_x = size - 1 - i as i32 / size;
                let new_y = i as i32 % size;

                let new_pos = (new_x + new_y * size) as usize;

                rotated = (rotated & !(1 << new_pos)) | (bit << new_pos);
            }

            shape = rotated;
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

    pub fn draw(&self, canvas: &mut Canvas<impl RenderTarget>, pos: Pos, rotation: usize) {
        let shape = self.shape(rotation);
        let shape_size = self.shape_size();

        let border_size = BLOCK_SIZE / 10;

        for i in 0..shape_size * shape_size {
            let bit = shape >> i & 1;
            if bit == 0 { continue }

            let relative_cord = Cord(i as i32 % shape_size, i as i32 / shape_size);

            canvas.set_draw_color(self.colour().1);
            canvas.fill_rect(Rect::new(pos.0 + relative_cord.0 * BLOCK_SIZE as i32, pos.1 + relative_cord.1 * BLOCK_SIZE as i32, BLOCK_SIZE, BLOCK_SIZE)).unwrap();

            canvas.set_draw_color(self.colour().0);
            canvas.fill_rect(Rect::new(pos.0 + relative_cord.0 * BLOCK_SIZE as i32 + border_size as i32, pos.1 + relative_cord.1 * BLOCK_SIZE as i32 + border_size as i32, BLOCK_SIZE - border_size * 2, BLOCK_SIZE - border_size * 2)).unwrap();
        }
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
            _ => panic!("invalid random value")
        }
    }

    pub fn random_set() -> Vec<TetroType> {
        let mut rng = thread_rng();

        let mut set = vec![TetroType::I, TetroType::J, TetroType::L, TetroType::O, TetroType::S, TetroType::T, TetroType::Z];

        set.shuffle(&mut rng);

        set
    }
}

pub struct GameTetro {
    pub tetro_type: TetroType,
    pub cord: Cord,
    pub rotation: usize
}

impl GameTetro {
    pub fn new(tetro: TetroType, cord: Cord, rotation: usize) -> GameTetro {
        GameTetro {
            tetro_type: tetro,
            cord,
            rotation
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<impl RenderTarget>, game: &Game) {
        self.tetro_type.draw(canvas, self.cord.pos(game.game_pos), self.rotation);
    }
}

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

            self.blocks[(cord.0 + relative_cord.0 + (cord.1 + relative_cord.1) * GAME_WIDTH as i32) as usize] = Some(self.dropping_tetro.tetro_type);
        }
    }
}
