use sdl2::Sdl;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::ttf::{Font, Sdl2TtfContext};

use crate::game::{self, Game};
use crate::{Cord, Pos, BLOCK_SIZE, GAME_POS, GAME_WIDTH, GAME_HEIGHT, FONT_CHAR_WIDTH, FONT_CHAR_HEIGHT};
use crate::tetros::TetroType;

pub struct GUI<'ttf> {
    pub sdl_context: Sdl,
    pub canvas: Canvas<Window>,
    pub font: Font<'ttf, 'static>,
    pub game: Game
}

impl<'ttf> GUI<'ttf> {
    pub fn build_self_referential() -> Sdl2TtfContext {
        sdl2::ttf::init().unwrap()
    }

    pub fn build(ttf_context: &'ttf Sdl2TtfContext, game: Game, window_title: &'static str) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let font = ttf_context.load_font("DOS-font.ttf", 128).unwrap();

        let window = video_subsystem.window(window_title, BLOCK_SIZE as u32 * 17, BLOCK_SIZE as u32 * 22).build().unwrap();
        let canvas = window.into_canvas().build().unwrap();

        Self {
            sdl_context,
            canvas,
            font,
            game
        }
    }

    pub fn draw(&mut self) {
        self.draw_tetro_box("HOLD", Pos(BLOCK_SIZE, BLOCK_SIZE * 2), self.game.hold_tetro);
        self.draw_tetro_box("NEXT", Pos(BLOCK_SIZE, BLOCK_SIZE * 8), Some(self.game.get_next_tetro()));

        self.draw_value_display("LEVEL", Pos(BLOCK_SIZE, BLOCK_SIZE * 16), self.game.lines / 10 + 1);
        self.draw_value_display("SCORE", Pos(BLOCK_SIZE, BLOCK_SIZE * 19), self.game.score);

        let game_border_rect = Rect::new(GAME_POS.0 - 4, GAME_POS.1 - 4, (BLOCK_SIZE * GAME_WIDTH + 8) as u32, (BLOCK_SIZE * GAME_HEIGHT + 8) as u32);
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.fill_rect(game_border_rect).unwrap();

        for (i, block) in self.game.blocks.iter().enumerate() {
            let cord = Cord(i as i32 % GAME_WIDTH, i as i32 / GAME_WIDTH);
            let pos = cord.pos();

            let mut block_colour = (Color::RGB(189, 195, 199), Color::RGB(181, 187, 191));

            if let Some(block) = block {
                block_colour = block.colour();
            }

            let border_size = BLOCK_SIZE / 10;

            let block_border_rect = Rect::new(pos.0, pos.1, BLOCK_SIZE as u32, BLOCK_SIZE as u32);
            self.canvas.set_draw_color(block_colour.1);
            self.canvas.fill_rect(block_border_rect).unwrap();

            let block_inner_rect = Rect::new(pos.0 + border_size, pos.1 + border_size, (BLOCK_SIZE - border_size * 2) as u32, (BLOCK_SIZE - border_size * 2) as u32);
            self.canvas.set_draw_color(block_colour.0);
            self.canvas.fill_rect(block_inner_rect).unwrap();
        }

        let mut ghost = self.game.dropping_tetro;
        for i in self.game.dropping_tetro.cord.1..GAME_HEIGHT {
            ghost.cord.1 = i;
            if game::is_tetro_colliding(self.game.blocks, ghost) { break }
        }
        ghost.cord.1 -= 1;

        self.game.dropping_tetro.tetro_type.draw(&mut self.canvas, ghost.cord.pos(), self.game.dropping_tetro.rotation, true);

        self.game.dropping_tetro.draw(&mut self.canvas);
    }

    fn draw_text(&mut self, pos: Pos, text: &str) {
        let texture_creator = self.canvas.texture_creator();

        let surface = self.font
            .render(text)
            .blended(Color::RGB(189, 195, 199))
            .unwrap();
        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();

        let title_rect = Rect::new(pos.0, pos.1, FONT_CHAR_WIDTH as u32 * text.len() as u32, FONT_CHAR_HEIGHT as u32);
        self.canvas.copy(&texture, None, Some(title_rect)).unwrap();
    }

    fn draw_tetro_box(&mut self, box_title: &'static str, box_pos: Pos, tetro: Option<TetroType>) {
        let mut title_pos = box_pos;
        title_pos.1 -= BLOCK_SIZE;
        self.draw_text(title_pos, box_title);

        self.canvas.set_draw_color(Color::RGB(189, 195, 199));
        self.canvas.fill_rect(Rect::new(box_pos.0, box_pos.1, BLOCK_SIZE as u32 * 4, BLOCK_SIZE as u32 * 4)).unwrap();

        if let Some(tetro) = tetro {
            tetro.draw_centered(&mut self.canvas, box_pos);
        }
    }

    fn draw_value_display(&mut self, display_title: &'static str, display_pos: Pos, display_value: i32) {
        self.draw_text(display_pos, display_title);

        let mut value_pos = display_pos;
        value_pos.1 += BLOCK_SIZE;

        self.draw_text(value_pos, display_value.to_string().as_str());
    }
}

