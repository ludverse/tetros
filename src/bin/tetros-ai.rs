use std::sync::{mpsc, Mutex, Arc};
use std::time::Duration;
use std::thread;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use tetros_rs::game::Game;
use tetros_rs::controls;
use tetros_rs::bot::{Bot, Weigths};
use tetros_rs::gui::GUI;

fn main() {
    let game = Game::new();
    let self_referential = GUI::build_self_referential();
    let mut gui = GUI::build(&self_referential, game, "Tetros");

    let bot = Bot {
        weights: Weigths {
            holes_penalty: 16000,
            bumpiness_penalty: 6,
            height_penalty: 2,
            line_clearing: [-90, -70, -50, 800]
        },
        depth: 1
    };

    let next_move: Arc<Mutex<Option<(bool, i32, usize)>>> = Arc::new(Mutex::new(None));
    let (mission_tx, mission_rx) = mpsc::channel();

    let mut hold_debounce = false;

    {
        let next_move = Arc::clone(&next_move);
        thread::spawn(move || {
            for game in mission_rx {
                let mut next_move = next_move.lock().unwrap();
                *next_move = Some(bot.best_move(&game).unwrap());
            }
        });
    }

    let mut slow = false;

    'running: loop {
        gui.game.next_frame();

        if let Ok(mut next_move) = next_move.try_lock() {
            if let Some(next_move_unwrapped) = *next_move {
                if !hold_debounce && next_move_unwrapped.0 { controls::hold_tetro(&mut gui.game) };
                hold_debounce = true;

                let rotate_times = next_move_unwrapped.2 as i32 - gui.game.dropping_tetro.rotation as i32;
                let shift_amount = next_move_unwrapped.1 - gui.game.dropping_tetro.cord.0;

                if shift_amount == 0 && rotate_times == 0 {
                    controls::hard_drop(&mut gui.game);
                    if slow { thread::sleep(Duration::from_millis(500)); };
                    *next_move = None;
                } else {
                    if shift_amount != 0 {
                        controls::shift_tetro(&mut gui.game, if shift_amount.is_positive() { 1 } else { -1 });
                    }
                    if rotate_times != 0 {
                        controls::rotate_tetro(&mut gui.game, rotate_times)
                    } 

                    if slow { thread::sleep(Duration::from_millis(35)); };
                }
            } else {
                hold_debounce = false;
                mission_tx.send(gui.game.clone()).unwrap();
            }
        }

        gui.canvas.clear();

        gui.draw();

        gui.canvas.set_draw_color(Color::RGB(52, 73, 94));
        gui.canvas.present();

        let mut event_pump = gui.sdl_context.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Space => {
                            slow = true
                        },
                        Keycode::A => {
                            slow = false
                        },
                        _ => ()
                    }
                },
                _ => ()
            }
        }

        thread::sleep(Duration::from_millis(1000 / 30));
    }
}
