use std::time::{Duration, Instant};
use std::thread;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use tetros::game::Game;
use tetros::controls;
use tetros::gui::GUI;

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
    let game = Game::new();

    let sdl_context = sdl2::init().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
    let mut gui = GUI::build(&sdl_context, &ttf_context, game, "Tetros");

    let mut event_pump = gui.sdl_context.event_pump().unwrap();

    let mut keys_down: Vec<Key> = vec![];

    'running: loop {
        if gui.game.is_playing { gui.game.next_frame(); };

        gui.canvas.clear();
        gui.draw();

        gui.canvas.set_draw_color(Color::RGB(52, 73, 94));
        gui.canvas.present();

        for key in &keys_down {
            if key.1.elapsed().as_millis() > 350 {
                Key::repeat_key(key.0, &mut gui.game);
            }
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(keycode), .. }
                    if gui.game.is_playing => {

                    if keys_down.iter().position(|key| key.0 == keycode).is_none() {
                        keys_down.push(Key(keycode, Instant::now()));

                        match keycode {
                            Keycode::S => gui.game.is_soft_dropping = true,
                            Keycode::Return => controls::hard_drop(&mut gui.game),
                            Keycode::F => controls::hold_tetro(&mut gui.game),
                            _ => Key::repeat_key(keycode, &mut gui.game)
                        }

                    }
                },
                Event::KeyUp { keycode: Some(keycode), .. } => {
                    keys_down.retain(|key| key.0 != keycode);

                    match keycode {
                        Keycode::S => gui.game.is_soft_dropping = false,
                        _ => ()
                    }
                },
                _ => ()
            }
        }

        thread::sleep(Duration::from_millis(1000 / 30));
    }
}

