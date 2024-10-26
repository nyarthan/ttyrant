use std::fs::File;
use std::io::Read;

use application::Application;
use nix::pty::{forkpty, ForkptyResult};
use nix::unistd::execvp;
use winit::event_loop::EventLoop;

mod application;
mod window;

fn main() {
    let event_loop = EventLoop::new().expect("create event loop");
    event_loop
        .run_app(&mut Application::new())
        .expect("run event loop");

    let (fd, _pid) = unsafe {
        let res = forkpty(None, None).expect("fork pty");
        match res {
            ForkptyResult::Child => {
                let shell = c"sh";
                execvp(shell, &[shell]).expect("spawn shell");
                unreachable!();
            }
            ForkptyResult::Parent { master, child } => (master, child),
        }
    };

    let mut file: File = fd.into();

    let mut buf = [0u8; 1024];
    loop {
        match file.read(&mut buf) {
            Ok(0) => break, // EOF
            Ok(n) => println!("{}", String::from_utf8_lossy(&buf[..n])),
            Err(_) => break,
        }
    }
}
