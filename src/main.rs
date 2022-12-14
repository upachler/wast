extern crate sdl2; 


use rt::bridge::WasmerW4Process;
use sdl2::pixels::{Color, PixelFormatEnum, Palette};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::TextureAccess;
use sdl2::surface::Surface;
use w4::SCREEN_SIZE;

use std::{time::Duration};

use curl::easy::Easy;
use url::Url;

mod fb;
mod rt;
mod w4;

trait W4Process  {
    fn start(&mut self);
    fn update(&mut self);
    fn read_raw_palette(&self, buf: &mut [u8; 3*4]);
    fn write_raw_palette(&self, buf: &[u8; 3*4]);
    fn read_fb(&self, buf: &mut [u8]);
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

fn expand_fb_to_index8(fbtexdata: &mut [u8]) {
    assert!(fbtexdata.len() %4 == 0);

    for n in (0..fbtexdata.len()/4).rev() {
        let buf = fbtexdata[n];
        let m = 4*n+3;
        fbtexdata[m] = buf >> 6;
        let m = m-1;
        fbtexdata[m] = (buf >> 4) & 0b00000011;
        let m = m-1;
        fbtexdata[m] = (buf >> 2) & 0b00000011;
        let m = m-1;
        fbtexdata[m] = buf & 0b00000011;
    }
}

#[test]
fn test_expand_fb_to_index8() {
    let mut testfb = [0b11100100, 0b01100011, 0, 0, 0, 0, 0, 0];
    expand_fb_to_index8(&mut testfb);

    assert!(testfb == [
        0b00,
        0b01,
        0b10,
        0b11,
        0b11,
        0b00,
        0b10,
        0b01
    ]);
}

fn run_gameloop<C: W4Process>(cart: &mut C) {

    // initialize default palette
    cart.write_raw_palette(&[
        224, 248, 207,
        134, 192, 108,
        48, 104, 80,
        7, 24, 33
    ]);

    cart.start();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("WASM Station", 3*SCREEN_SIZE, 3*SCREEN_SIZE)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    let mut surface = Surface::new(SCREEN_SIZE, SCREEN_SIZE, PixelFormatEnum::Index8).unwrap();
    let tc = canvas.texture_creator();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut raw_colors = [0xffu8;3*4];
    let mut colors = Vec::with_capacity(256);
    colors.resize(256, Color::RGB(0,0,0));

    'running: loop {

        cart.update();

        // read palette for this frame
        for c in 0..4 {
            colors[c] = Color::RGB(raw_colors[3*c+0], raw_colors[3*c+1], raw_colors[3*c+2])
        }
        cart.read_raw_palette(&mut raw_colors);

        let fbdata =surface.without_lock_mut().unwrap();
        cart.read_fb(fbdata);
        expand_fb_to_index8(fbdata);


        let palette = Palette::with_colors(&colors).unwrap();
        surface.set_palette(&palette).unwrap();
        
        let fb_tex = tc.create_texture_from_surface(&surface).unwrap();
        canvas.copy(&fb_tex, None, Some(Rect::new(0, 0, 3*SCREEN_SIZE, 3*SCREEN_SIZE))).unwrap();

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