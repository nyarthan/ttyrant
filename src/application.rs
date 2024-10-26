use std::sync::Arc;

use winit::{
    application::ApplicationHandler, dpi::LogicalSize, event::WindowEvent, window::Window,
};

use crate::window::WindowState;

pub struct Application {
    window_state: Option<WindowState>,
}

impl Application {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Application {
    fn default() -> Self {
        Self { window_state: None }
    }
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window_state.is_some() {
            return;
        }

        let (width, height) = (800, 600);
        let window_attributes = Window::default_attributes()
            .with_inner_size(LogicalSize::new(width, height))
            .with_title("ttyrant");
        let window = Arc::new(
            event_loop
                .create_window(window_attributes)
                .expect("create window"),
        );

        self.window_state = Some(pollster::block_on(WindowState::new(window)));
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let Some(state) = &mut self.window_state else {
            return;
        };

        let WindowState {
            device,
            queue,
            surface,
            surface_config,
            window,
            ..
        } = state;

        match event {
            WindowEvent::Resized(size) => {
                surface_config.width = size.width;
                surface_config.height = size.height;
                surface.configure(&device, &surface_config);
                window.request_redraw();
            }
            WindowEvent::RedrawRequested => {}
            _ => {}
        }
    }
}
