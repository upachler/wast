extern crate sdl2; 


use rt::bridge::WasmerW4Process;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::{time::Duration};

use curl::easy::Easy;
use url::Url;

mod fb;
mod rt;
mod w4;

trait W4Process  {
    fn start(&mut self);
    fn update(&mut self);
}



pub fn main() {

    
    let wa_module_url = Url::from_directory_path(std::env::current_dir().unwrap())
    .unwrap()
    .join("./hello.wasm")
    .unwrap();

    let wa_module_bytes = read_wasm_from_url(&wa_module_url);


    let mut cart = WasmerW4Process::new(wa_module_bytes);
    run_gameloop(&mut cart);
}

fn read_wasm_from_url(url: &Url) -> Vec<u8> {
    let mut curl = Easy::new();
    curl.url(url.as_str()).unwrap();
    let mut wa_module = Vec::<u8>::new();
    let mut curl = curl.transfer();
    curl.write_function(|data|{
        wa_module.extend_from_slice(data);
        Ok(data.len())
    }).unwrap();
    curl.perform().unwrap();
    drop(curl);

    wa_module
}

fn run_gameloop<C: W4Process>(cart: &mut C) {

    cart.start();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("WASM Station", 640, 640)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {

        cart.update();

        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => ()
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}