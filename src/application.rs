use std::sync::Arc;

use glyphon::{Color, Resolution, TextArea, TextBounds};
use wgpu::{
    CommandEncoderDescriptor, LoadOp, Operations, RenderPassColorAttachment, RenderPassDescriptor,
    TextureViewDescriptor,
};
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
            font_system,
            swash_cache,
            viewport,
            atlas,
            window,
            text_renderer,
            text_buffer,
        } = state;

        match event {
            WindowEvent::Resized(size) => {
                surface_config.width = size.width;
                surface_config.height = size.height;
                surface.configure(&device, &surface_config);
                window.request_redraw();
            }
            WindowEvent::RedrawRequested => {
                viewport.update(
                    &queue,
                    Resolution {
                        width: surface_config.width,
                        height: surface_config.height,
                    },
                );

                text_renderer
                    .prepare(
                        device,
                        queue,
                        font_system,
                        atlas,
                        viewport,
                        [TextArea {
                            buffer: text_buffer,
                            left: 10.0,
                            top: 10.0,
                            scale: 1.0,
                            bounds: TextBounds {
                                left: 0,
                                top: 0,
                                right: 600,
                                bottom: 160,
                            },
                            default_color: Color::rgb(255, 255, 255),
                            custom_glyphs: &[],
                        }],
                        swash_cache,
                    )
                    .unwrap();

                let frame = surface.get_current_texture().unwrap();
                let view = frame.texture.create_view(&TextureViewDescriptor::default());
                let mut encoder =
                    device.create_command_encoder(&CommandEncoderDescriptor { label: None });
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

                    text_renderer.render(&atlas, &viewport, &mut pass).unwrap();
                }

                queue.submit(Some(encoder.finish()));
                frame.present();

                atlas.trim();
            }
            _ => {}
        }
    }
}
