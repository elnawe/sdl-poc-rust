extern crate sdl2;

pub fn new(title: &str, width: u32, height: u32) -> Window {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let sdl_window = video_subsystem.window(&title, width, height)
        .position_centered()
        .build()
        .unwrap();

    Window {
        context: sdl_context,
        frame: sdl_window
    }
}

pub struct Window {
    pub context: sdl2::Sdl,
    pub frame: sdl2::video::Window
}

impl Window {

}
