use std::{
    thread,
    time::Duration,
    fs,
    env
};
use sdl2::pixels::Color;
use sdl2::event::Event;
use tetros::{
    gui::GUI,
    serializer::GameData
};

fn main() {
    let mut args = env::args();
    args.next();
    
    let filename = args
        .next()
        .expect("no input GameData file");

    dbg!(&filename);
    let game_data = fs::read_to_string(filename)
        .expect("failed to read file");
    let game_data: GameData = serde_json::from_str(&game_data[..])
        .expect("invalid GameData format");

    dbg!(&game_data);

    let game = game_data.to_game();

    let sdl_context = sdl2::init().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
    let mut gui = GUI::build(&sdl_context, &ttf_context, game, "Tetros position viewer");

    let mut event_pump = gui.sdl_context.event_pump().unwrap();

    gui.game.is_playing = false;

    'running: loop {
        gui.canvas.clear();
        gui.draw();

        gui.canvas.set_draw_color(Color::RGB(52, 73, 94));
        gui.canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running
                },
                _ => ()
            }
        }

        thread::sleep(Duration::from_millis(1000 / 30));
    }
}

