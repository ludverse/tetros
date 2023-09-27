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
    pub fn pos(&self, game: &Game) -> Pos {
        Pos(self.0 * BLOCK_SIZE as i32 + game.game_pos.0, self.1 * BLOCK_SIZE as i32 + game.game_pos.1)
    }

    pub fn is_outside_game(&self) -> bool {
        self.0 < 0 ||
        self.0 > GAME_WIDTH as i32
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Pos(pub i32, pub i32);

impl Pos {
}

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
    pub fn shape(&self, rotation: usize) -> Vec<u8> {
        let mut shape = match self {
            TetroType::I => vec![
                0,0,0,0,
                1,1,1,1,
                0,0,0,0,
                0,0,0,0
            ],
            TetroType::J => vec![
                1,0,0,
                1,1,1,
                0,0,0
            ],
            TetroType::L => vec![
                0,0,1,
                1,1,1,
                0,0,0
            ],
            TetroType::O => vec![
                1,1,
                1,1
            ],
            TetroType::S => vec![
                0,1,1,
                1,1,0,
                0,0,0
            ],
            TetroType::T => vec![
                0,1,0,
                1,1,1,
                0,0,0
            ],
            TetroType::Z => vec![
                1,1,0,
                0,1,1,
                0,0,0
            ]
        };

        let size = f64::sqrt(shape.len() as f64) as i32;

        for _ in 0..rotation {
            shape = shape.iter().enumerate().map(|(i, _)| {
                let new_x = size - 1 - i as i32 / size;
                let new_y = i as i32 % size;
                shape[(new_x + new_y * size) as usize]
            }).collect();
        }

        shape
    }

    pub fn shape_size(&self) -> i32 {
        f64::sqrt(self.shape(0).len() as f64) as i32
    }

    pub fn colour(&self) -> (Color, Color) {
        match self {
            TetroType::I => (Color::RGB(69, 170, 242), Color::RGB(45, 152, 218)),
            TetroType::J => (Color::RGB(75, 123, 236), Color::RGB(56, 103, 214)),
            TetroType::L => (Color::RGB(253, 150, 68), Color::RGB(250, 130, 49)),
            TetroType::O => (Color::RGB(254, 211, 48), Color::RGB(247, 183, 49)),
            _ => (Color::RGB(38, 222, 129), Color::RGB(32, 191, 107)),
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<impl RenderTarget>, pos: Pos, rotation: usize) {
        let shape = self.shape(rotation);
        let shape_size = (shape.len() as f64).sqrt() as i32;

        let border_size = BLOCK_SIZE / 10;

        for (i, block) in shape.iter().enumerate() {
            if *block == 0 { continue }

            let relative_x = i as i32 % shape_size * BLOCK_SIZE as i32;
            let relative_y = i as i32 / shape_size * BLOCK_SIZE as i32;

            canvas.set_draw_color(self.colour().1);
            canvas.fill_rect(Rect::new(pos.0 + relative_x, pos.1 + relative_y, BLOCK_SIZE, BLOCK_SIZE)).unwrap();

            canvas.set_draw_color(self.colour().0);
            canvas.fill_rect(Rect::new(pos.0 + relative_x + border_size as i32, pos.1 + relative_y + border_size as i32, BLOCK_SIZE - border_size * 2, BLOCK_SIZE - border_size * 2)).unwrap();
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
        self.tetro_type.draw(canvas, self.cord.pos(game), self.rotation);
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
        let block_size = 20;
        let game_pos = Pos(block_size as i32 * 6, block_size as i32);

        let mut next_tetros = TetroType::random_set();

        Game {
            game_pos,
            hold_screen_pos: Pos(block_size as i32, block_size as i32),
            dropping_tetro: GameTetro::new(next_tetros.pop().unwrap(), Cord(4, 0), 0),
            hold_tetro: None,
            next_tetros,
            blocks: [None; 10 * 20]
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<impl RenderTarget>) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.fill_rect(Rect::new(self.game_pos.0 - 4, self.game_pos.1 - 4, BLOCK_SIZE * 10 + 8, BLOCK_SIZE * 20 + 8)).unwrap();

        canvas.set_draw_color(Color::RGB(189, 195, 199));
        canvas.fill_rect(Rect::new(self.game_pos.0, self.game_pos.1, BLOCK_SIZE * 10, BLOCK_SIZE * 20)).unwrap();

        let border_size = BLOCK_SIZE / 10;

        for (i, block) in self.blocks.iter().enumerate() {
            let relative_pos = Pos(i as i32 % 10 * BLOCK_SIZE as i32, i as i32 / 10 * BLOCK_SIZE as i32);

            let mut block_colour = (Color::RGB(189, 195, 199), Color::RGB(181, 187, 191));

            if let Some(block) = block {
                block_colour = block.colour();
            }

            canvas.set_draw_color(block_colour.1);
            canvas.fill_rect(Rect::new(self.game_pos.0 + relative_pos.0, self.game_pos.1 + relative_pos.1, BLOCK_SIZE, BLOCK_SIZE)).unwrap();

            canvas.set_draw_color(block_colour.0);
            canvas.fill_rect(Rect::new(self.game_pos.0 + relative_pos.0 + border_size as i32, self.game_pos.1 + relative_pos.1 + border_size as i32, BLOCK_SIZE - border_size * 2, BLOCK_SIZE - border_size * 2)).unwrap();
        }

        self.dropping_tetro.draw(canvas, self);
    }

    // pub fn petrify_tetro(&mut self, tetro: GameTetro) {
    //     for (i, block) in tetro.tetro_type.shape(tetro.rotation).iter().enumerate() {
    //
    //     }
    // }
}
