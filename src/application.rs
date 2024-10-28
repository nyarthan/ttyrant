use std::sync::Arc;

use glyphon::{Attrs, Color, Family, Resolution, Shaping, TextArea, TextBounds};
use wgpu::{
    CommandEncoderDescriptor, LoadOp, Operations, RenderPassColorAttachment, RenderPassDescriptor,
    TextureViewDescriptor,
};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    keyboard::{Key, NamedKey},
    window::Window,
};

use crate::{pty::Pty, window::WindowState};

pub struct Application {
    window_state: Option<WindowState>,
    pty: Option<Pty>,
    content: String,
}

impl Application {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Application {
    fn default() -> Self {
        Self {
            window_state: None,
            pty: None,
            content: String::new(),
        }
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

        self.window_state = Some(pollster::block_on(WindowState::new(window.clone())));
        self.pty = Some(Pty::new(Arc::downgrade(&window)));
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let Some(state) = &mut self.window_state else {
            return;
        };

        match event {
            WindowEvent::Resized(size) => {
                state.surface_config.width = size.width;
                state.surface_config.height = size.height;
                state
                    .surface
                    .configure(&state.device, &state.surface_config);
                state.window.request_redraw();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state.is_pressed() {
                    if let Some(pty) = &mut self.pty {
                        match event.logical_key {
                            Key::Character(c) => {
                                let _ = pty.write(c.as_bytes());
                            }
                            Key::Named(NamedKey::Space) => {
                                let _ = pty.write(" ".as_bytes());
                            }
                            Key::Named(NamedKey::Enter) => {
                                let _ = pty.write(b"\r");
                            }
                            Key::Named(NamedKey::Backspace) => {
                                let _ = pty.write(b"\x7f");
                            }
                            _ => {}
                        }
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                state.viewport.update(
                    &state.queue,
                    Resolution {
                        width: state.surface_config.width,
                        height: state.surface_config.height,
                    },
                );

                let inner_size = state.window.inner_size();

                let mut content_updated = false;
                if let Some(pty) = &self.pty {
                    while let Some(output) = pty.try_read() {
                        self.content.push_str(&output);
                        content_updated = true;
                    }
                }

                if content_updated {
                    state.text_buffer.set_text(
                        &mut state.font_system,
                        &self.content,
                        Attrs::new().family(Family::Monospace),
                        Shaping::Advanced,
                    );
                    state
                        .text_buffer
                        .shape_until_scroll(&mut state.font_system, false);
                }

                state
                    .text_renderer
                    .prepare(
                        &state.device,
                        &state.queue,
                        &mut state.font_system,
                        &mut state.atlas,
                        &state.viewport,
                        [TextArea {
                            buffer: &mut state.text_buffer,
                            left: 0.0,
                            top: 0.0,
                            scale: 1.0,
                            bounds: TextBounds {
                                left: 0,
                                top: 0,
                                right: inner_size.width as i32,
                                bottom: inner_size.height as i32,
                            },
                            default_color: Color::rgb(255, 255, 255),
                            custom_glyphs: &[],
                        }],
                        &mut state.swash_cache,
                    )
                    .unwrap();

                let frame = state.surface.get_current_texture().unwrap();
                let view = frame.texture.create_view(&TextureViewDescriptor::default());
                let mut encoder = state
                    .device
                    .create_command_encoder(&CommandEncoderDescriptor { label: None });
                {
                    let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
                        label: None,
                        color_attachments: &[Some(RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: Operations {
                                load: LoadOp::Clear(wgpu::Color::BLACK),
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        depth_stencil_attachment: None,
                        timestamp_writes: None,
                        occlusion_query_set: None,
                    });

                    state
                        .text_renderer
                        .render(&state.atlas, &state.viewport, &mut pass)
                        .unwrap();
                }

                state.queue.submit(Some(encoder.finish()));
                frame.present();

                state.atlas.trim();
            }
            _ => {}
        }
    }
    fn exiting(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        self.pty = None
    }
}
