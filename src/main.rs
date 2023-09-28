use std::time::{Duration, Instant};
use std::thread;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas, RenderTarget};
use tetris_rs::{BLOCK_SIZE, GAME_WIDTH, GAME_HEIGHT, TetroType, Game, Cord, Pos, GameTetro};

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut game = Game::new();

    let window = video_subsystem.window("Tetros", BLOCK_SIZE * 17, BLOCK_SIZE * 22)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut last_down = Instant::now();

    let mut keys_down = vec![];

    'running: loop {
        let now = Instant::now();
        let delta_last_down = now - last_down;

        if delta_last_down.as_millis() > (if keys_down.contains(&Keycode::S) { 75 } else { 750 }) {
            let mut next_cord = game.dropping_tetro.cord;
            next_cord.1 += 1;

            if next_cord.is_outside_game(game.dropping_tetro.tetro_type, game.dropping_tetro.rotation) {
                game.petrify_dropping_tetro();
                game.next_tetro();
            } else {
                game.dropping_tetro.cord = next_cord;
            }

            last_down = Instant::now();
        }

        canvas.clear();

        draw(&mut canvas, &game);

        canvas.set_draw_color(Color::RGB(52, 73, 94));
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    keys_down.push(keycode);

                    match keycode {
                        Keycode::Q => {
                            let mut next_rotation = game.dropping_tetro.rotation;
                            if next_rotation == 3 {
                                next_rotation = 0;
                            } else {
                                next_rotation += 1;
                            }

                            if !game.dropping_tetro.cord.is_outside_game(game.dropping_tetro.tetro_type, next_rotation) {
                                game.dropping_tetro.rotation = next_rotation;
                            }
                        },
                        Keycode::E => {
                            let mut next_rotation = game.dropping_tetro.rotation;
                            if next_rotation == 0 {
                                next_rotation = 3;
                            } else {
                                next_rotation -= 1;
                            }

                            if !game.dropping_tetro.cord.is_outside_game(game.dropping_tetro.tetro_type, next_rotation) {
                                game.dropping_tetro.rotation = next_rotation;
                            }
                        },
                        Keycode::A => {
                            let mut next_cord = game.dropping_tetro.cord;
                            next_cord.0 -= 1;

                            if !next_cord.is_outside_game(game.dropping_tetro.tetro_type, game.dropping_tetro.rotation) {
                                game.dropping_tetro.cord = next_cord;
                            }
                        },
                        Keycode::D => {
                            let mut next_cord = game.dropping_tetro.cord;
                            next_cord.0 += 1;

                            if !next_cord.is_outside_game(game.dropping_tetro.tetro_type, game.dropping_tetro.rotation) {
                                game.dropping_tetro.cord = next_cord;
                            }
                        },
                        Keycode::C => {
                            let hold_piece = game.hold_tetro;

                            game.hold_tetro = Some(game.dropping_tetro.tetro_type);

                            let tetro_type = hold_piece.unwrap_or_else(|| {
                                if game.next_tetros.len() == 0 { game.next_tetros = TetroType::random_set() }
                                game.next_tetros.pop().unwrap()
                            });

                            last_down = Instant::now();

                            game.dropping_tetro = GameTetro::new(tetro_type, Cord(2, 0), 0);
                        },
                        _ => ()
                    };
                },
                Event::KeyUp { keycode: Some(keycode), .. } => {
                    keys_down.retain(|&a| a != keycode);
                },
                _ => ()
            }
        }

        thread::sleep(Duration::from_millis(1000 / 30))
    }
}

fn draw(canvas: &mut Canvas<impl RenderTarget>, game: &Game) {
    if let Some(hold_piece) = game.hold_tetro {
        let center_offset = (4 - hold_piece.shape_size()) as f64 / 2. * BLOCK_SIZE as f64;
        let center_pos = Pos(game.hold_screen_pos.0 + center_offset as i32, game.hold_screen_pos.1 + center_offset as i32);
        hold_piece.draw(canvas, center_pos, 0);
    }

    game.draw(canvas);
}

