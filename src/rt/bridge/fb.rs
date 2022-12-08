// Brige functionality for exposing host framebuffer functionality to carts

use std::ops::{Range};

use wasmer::{FunctionEnvMut, WasmPtr, WasmSlice, ValueType, AsStoreRef};

use crate::{w4::{SCREEN_SIZE, BLIT_2BPP, FRAMEBUFFER_ADDR}, fb::{Source, Sink}};

use super::WasmerW4StateRef;





pub (crate) fn blit(mut env: FunctionEnvMut<WasmerW4StateRef>, sprite: WasmPtr<u8>, x: i32, y: i32, width: u32, height: u32, flags: u32) {
    blit_sub(env, sprite, x, y, width, height, 0, 0, width, flags)
}


struct WasmSliceSinkSource<'a, T> 
where T: ValueType + Copy
{
    slice: WasmSlice<'a, T>
}

impl <'a,T> Source<T> for WasmSliceSinkSource<'a, T>
where T: ValueType + Copy
{
    fn item_at(&self, offset: usize) -> T {
       self.slice.index(offset as u64).read().unwrap()
    }
}

impl <'a,T> Sink<T> for WasmSliceSinkSource<'a, T>
where T: ValueType + Copy
{
    fn set_item_at(&mut self, offset: usize, item: T) {
//       self.slice.index(offset as u64).write(item).expect("writing to wasm memory failed");
self.slice.index(0).write(item).expect("writing to wasm memory failed");
}
}

#[allow(clippy::too_many_arguments)]
pub (crate) fn blit_sub(
    mut env: FunctionEnvMut<WasmerW4StateRef>, 
    sprite: WasmPtr<u8>,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    src_x: u32,
    src_y: u32,
    stride: u32,
    flags: u32,
) {
    let view = env.data().mem.view(&env.as_store_ref());
    let num_bits = stride * (height + src_y) * crate::fb::pixel_width_of_flags(flags);
    let len = (num_bits + 7) / 8;
    let sprite_slice = sprite.slice(&view, len).unwrap();

    let fb_len = (SCREEN_SIZE * SCREEN_SIZE / 4) as u32;
    let fb_slice = WasmPtr::<u8>::new(FRAMEBUFFER_ADDR as u32).slice(&view, fb_len).unwrap();

    let src = WasmSliceSinkSource {slice: sprite_slice};
    let mut tgt = WasmSliceSinkSource {slice: fb_slice};

    crate::fb::blit_sub_impl(&mut tgt, &src, x, y, width, height, src_x, src_y, stride, flags);
}


/// Draws a line between two points.
pub (crate) fn line(fbenv: &FunctionEnvMut<WasmerW4StateRef>, x1: i32, y1: i32, x2: i32, y2: i32) {
    //todo
}

/// Draws an oval (or circle).
pub (crate) fn oval(fbenv: &FunctionEnvMut<WasmerW4StateRef>, x: i32, y: i32, width: u32, height: u32) {
    //todo
}

/// Draws a rectangle.
pub (crate) fn rect(fbenv: &FunctionEnvMut<WasmerW4StateRef>, x: i32, y: i32, width: u32, height: u32) {
    //todo
}

/// Draws text using the built-in system font.
pub (crate) fn text<T: AsRef<str>>(fbenv: &FunctionEnvMut<WasmerW4StateRef>, text: T, x: i32, y: i32) {
    //todo
}

/// Draws text using the built-in system font.
pub (crate) fn text_len(fbenv: FunctionEnvMut<WasmerW4StateRef>, text_ptr: WasmPtr<u8>, len: u32, x: i32, y: i32) {
    //todo
}

/// Draws a vertical line
pub (crate) fn vline(fbenv: &FunctionEnvMut<WasmerW4StateRef>, x: i32, y: i32, len: u32) {
    //todo
}

/// Draws a horizontal line
pub (crate) fn hline(fbenv: &FunctionEnvMut<WasmerW4StateRef>, x: i32, y: i32, len: u32) {
    //todo
}