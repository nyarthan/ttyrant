use std::{
    fs::File,
    io::Read,
    os::fd::AsFd,
    sync::mpsc::{channel, Receiver, Sender},
    thread::{self, JoinHandle},
};

use nix::{
    pty::{forkpty, ForkptyResult},
    sys::select::{select, FdSet},
    unistd::execvp,
};
use winit::window::Window;

pub struct Pty {
    fd: File,
    output_rx: Receiver<String>,
    _output_thread: JoinHandle<()>,
}

impl Pty {
    pub fn new(window: std::sync::Weak<Window>) -> Self {
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

        let (tx, rx) = channel();
        let file: File = fd.into();
        let read_file = file.try_clone().expect("clone fd");

        let output_thread = thread::spawn(move || Self::read_output(read_file, tx, window));

        Self {
            fd: file,
            output_rx: rx,
            _output_thread: output_thread,
        }
    }

    fn read_output(mut file: File, tx: Sender<String>, window: std::sync::Weak<Window>) {
        let mut buf = [0u8; 1024];
        loop {
            let mut fd_set = FdSet::new();
            fd_set.insert(file.as_fd());

            match select(None, &mut fd_set, None, None, None) {
                Ok(_) => {
                    match file.read(&mut buf) {
                        Ok(0) => break, // EOF
                        Ok(n) => {
                            if let Ok(s) = String::from_utf8(buf[..n].to_vec()) {
                                if tx.send(s).is_err() {
                                    break;
                                }

                                if let Some(window) = window.upgrade() {
                                    window.request_redraw();
                                }
                            }
                        }
                        Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                        Err(_) => break,
                    }
                }
                Err(_) => break,
            }
        }
    }

    pub fn write(&mut self, data: &[u8]) -> std::io::Result<usize> {
        std::io::Write::write(&mut self.fd, data)
    }

    pub fn try_read(&self) -> Option<String> {
        self.output_rx.try_recv().ok()
    }
}
