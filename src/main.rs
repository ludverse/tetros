use std::time::{Duration, Instant};
use std::thread;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use tetris_rs::BLOCK_SIZE;
use tetris_rs::game::Game;
use tetris_rs::controls;

#[derive(Copy, Clone, Debug)]
struct Key(Keycode, Instant);

impl Key {
    fn repeat_key(keycode: Keycode, game: &mut Game) {
        match keycode {
            Keycode::Q => controls::rotate_tetro(game, -1),
            Keycode::E => controls::rotate_tetro(game, 1),
            Keycode::A => controls::shift_tetro(game, -1),
            Keycode::D => controls::shift_tetro(game, 1),
            _ => ()
        };
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();

    let mut font = ttf_context.load_font("DOS-font.ttf", 128).unwrap();

    let mut game = Game::new();

    let window = video_subsystem.window("Tetros", BLOCK_SIZE as u32 * 17, BLOCK_SIZE as u32 * 22)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut keys_down: Vec<Key> = vec![];

    'running: loop {
        game.next_frame();

        canvas.clear();

        game.draw(&mut canvas, &font);

        canvas.set_draw_color(Color::RGB(52, 73, 94));
        canvas.present();

        for key in &keys_down {
            if key.1.elapsed().as_millis() > 250 {
                Key::repeat_key(key.0, &mut game);
            }
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    if keys_down.iter().position(|key| key.0 == keycode).is_none() {
                        keys_down.push(Key(keycode, Instant::now()));

                        match keycode {
                            Keycode::S => game.is_soft_dropping = true,
                            Keycode::Return => controls::hard_drop(&mut game),
                            Keycode::C => controls::hold_tetro(&mut game),
                            _ => Key::repeat_key(keycode, &mut game)
                        }

                    }
                },
                Event::KeyUp { keycode: Some(keycode), .. } => {
                    keys_down.retain(|key| key.0 != keycode);

                    match keycode {
                        Keycode::S => game.is_soft_dropping = false,
                        _ => ()
                    }
                },
                _ => ()
            }
        }

        thread::sleep(Duration::from_millis(1000 / 30));
    }
}

