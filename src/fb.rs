
pub (crate) struct Framebuffer;

impl Framebuffer {
    pub fn blit(&mut self, target: &mut [u8], sprite: &[u8], x: i32, y: i32, width: u32, height: u32, flags: u32) {
    }
    
    /// Copies a subregion within a larger sprite atlas to the framebuffer.
    #[allow(clippy::too_many_arguments)]
    pub fn blit_sub(
        sprite: &[u8],
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        src_x: u32,
        src_y: u32,
        stride: u32,
        flags: u32,
    ) {
    
        //todo
    }

}

