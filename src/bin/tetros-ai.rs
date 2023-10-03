use std::time::{Duration, Instant};
use std::thread;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use tetris_rs::BLOCK_SIZE;
use tetris_rs::game::Game;
use tetris_rs::controls;
use tetris_rs::bot::{Bot, Weigths};

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut game = Game::new();
    let bot = Bot {
        weights: Weigths {
            holes_penalty: 16000,
            bumpiness_penalty: 2,
            height_penalty: 1,
            line_clearing: [1, 1, 1, 8]
        }
    };

    let window = video_subsystem.window("Tetros", BLOCK_SIZE as u32 * 17, BLOCK_SIZE as u32 * 22)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut next_move: Option<(bool, i32, usize)> = None;

    'running: loop {
        game.next_frame();

        if let Some(next_move_unwrapped) = next_move {
            let rotate_times = next_move_unwrapped.2 as i32 - game.dropping_tetro.rotation as i32;
            let shift_amount = next_move_unwrapped.1 - game.dropping_tetro.cord.0;
            if shift_amount != 0 {
                controls::shift_tetro(&mut game, if shift_amount.is_positive() { 1 } else { -1 });
            }
            if rotate_times != 0 {
                controls::rotate_tetro(&mut game, rotate_times)
            } 
            if shift_amount == 0 && rotate_times == 0 {
                controls::hard_drop(&mut game);
                next_move = None;
            }
        } else {
            next_move = Some(bot.best_move(&game).unwrap());

            if next_move.unwrap().0 { controls::hold_tetro(&mut game) };
        }

        canvas.clear();

        game.draw(&mut canvas, false);

        canvas.set_draw_color(Color::RGB(52, 73, 94));
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Space => {
                        }
                        _ => ()
                    }
                },
                _ => ()
            }
        }

        thread::sleep(Duration::from_millis(1000 / 30));
    }
}
