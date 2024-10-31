use application::Application;
use winit::event_loop::EventLoop;

mod application;
mod cell;
mod pty;
mod window;

fn main() {
    let event_loop = EventLoop::new().expect("create event loop");
    event_loop
        .run_app(&mut Application::new())
        .expect("run event loop");
}
