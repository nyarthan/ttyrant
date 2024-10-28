use std::fs::File;
use std::io::Read;

use application::Application;
use nix::pty::{forkpty, ForkptyResult};
use nix::unistd::execvp;
use winit::event_loop::EventLoop;

mod application;
mod pty;
mod window;

fn main() {
    let event_loop = EventLoop::new().expect("create event loop");
    event_loop
        .run_app(&mut Application::new())
        .expect("run event loop");
}
