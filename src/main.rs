use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};
use log::info;
use env_logger;
use std::error::Error;

mod app;
mod camera;
mod event_loop;
mod vertex;
mod uniforms;
mod chunk;
mod world;
mod world_update;
mod texture;

#[derive(Default)]
struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Correctly create the event loop
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::default();

    // Run the application
    let _ = event_loop.run_app(&mut app);

    Ok(())
}
