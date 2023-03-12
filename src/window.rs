extern crate winit;

use winit::{dpi::LogicalSize, event::{Event, WindowEvent}};

pub struct Window {
    title: &'static str,
    width: u32,
    height: u32,
    window: winit::window::Window,
    event_loop: winit::event_loop::EventLoop<()>,
}

impl Window {

    pub fn new(title: &'static str, width: u32, height: u32) -> Self
    {
        let event_loop = winit::event_loop::EventLoop::new();
        let window = winit::window::WindowBuilder::new()
            .with_title(title)
            .with_inner_size(LogicalSize::new(f64::from(width), f64::from(height)))
            .build(&event_loop)
            .expect("Failed to create the window.");
        Window { title: title, width: width, height: height, window: window, event_loop: event_loop }
    }

    pub fn run<'a>(self, callback: impl Fn() + 'static)
    {
        self.event_loop.run(move |event, _, control_flow|
        {
            control_flow.set_poll();
            callback();

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    control_flow.set_exit();
                },
                _ => ()
            }
        });  
    }
}

