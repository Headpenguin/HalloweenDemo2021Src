extern crate sdl2;

/*use sdl2::image::LoadTexture;
use sdl2::rect::Rect;*/
use sdl2::hint;
use sdl2::keyboard::KeyboardState;
use sdl2::mixer::{self, Music, DEFAULT_FORMAT};
use sdl2::pixels::Color;
use sdl2::event::Event;

/*use std::thread;
use std::time::Duration;*/

mod PlayerMod;
mod SpriteLoader;
mod MapMod;
mod CollisionMod;

use MapMod::*;
use PlayerMod::*;

const WIDTH: u32 = 850;
const HEIGHT: u32 = 600;

pub fn main() {
    let context = sdl2::init().unwrap();
    let videoSubsystem = context.video().unwrap();
    //let mixerContext = mixer::init(InitFlag::all());

    mixer::open_audio(44100, DEFAULT_FORMAT, 2, 1024).unwrap();

    let music = Music::from_file(&"Resources/Music/hauntedhouseorgan.wav").unwrap();

    music.play(-1).unwrap();

    if !hint::set("SDL_RENDER_SCALE_QUALITY", "0") {
        eprintln!("Warning: Linear texture filtering may not be enabled.");
    }

    //let _imageContext = image::init(InitFlag::PNG);

    let window = videoSubsystem.window("Halloween Demo", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();
    
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    let creator = canvas.texture_creator();

    let mut player = Player::new(&creator, 50, 50);

    let map = Map::new(TILES, &creator);

    canvas.set_draw_color(Color::RGB(0xff, 0x80, 0x00));
    canvas.clear();
    player.draw(&mut canvas);
    canvas.present();

    let mut events = context.event_pump().unwrap();

    let mut state: Option<KeyboardState>;

    let mut keyUpdate = false;
 
    'main: loop {
        for event in events.poll_iter() {
            if let Event::Quit{..} = event {break 'main;}
            if let Event::KeyDown{..} | Event::KeyUp{..} = event {keyUpdate = true;}
        }
        if keyUpdate {state = Some(events.keyboard_state());}
        else {state = None}
        player.update(state, &events, &map);
        canvas.clear();
        map.render(&mut canvas);
        player.draw(&mut canvas);
        canvas.present();
        keyUpdate = false;
        //thread::sleep(Duration::from_nanos(16666667));
    }

}

const TILES: [[usize; 17]; 12] = [
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];
