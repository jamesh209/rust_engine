use wgpu::core::instance;
use winit::{
    error::EventLoopError,
    event::{Event, WindowEvent},
    event_loop::{self, ControlFlow, EventLoop},
    window::Window,
};

use std::{sync::Arc, time::Instant, vec::Vec};

use crate::mercury_viewport::Viewport;
use crate::mercury_viewport::ViewportDesc;

pub struct EngineLoop<'a> {
    event_loop: EventLoop<()>,
    target_frame_time: u128,
    viewports: Vec<Viewport<'a>>,
}

impl<'a> EngineLoop<'a> {
    pub fn new() -> EngineLoop<'a> {
        let event_loop: EventLoop<()> = EventLoop::new().unwrap();
        let target_frame_time: u128 = 17;
        let viewports = Vec::new();

        Self {
            event_loop,
            target_frame_time,
            viewports,
        }
    }

    pub async fn create_new_viewport(&mut self) {
        let window = winit::window::WindowBuilder::new()
            .with_title(format!("Viewport"))
            .with_inner_size(winit::dpi::PhysicalSize::new(800, 600))
            .build(&self.event_loop)
            .unwrap();

        let window = Arc::new(window);

        let instance = wgpu::Instance::default();

        let viewport_desc = ViewportDesc::new(window, wgpu::Color::GREEN, &instance);

        self.viewports.push(viewport_desc.build(&instance).await);
    }

    pub async fn start(mut self) -> Result<(), EventLoopError> {
        self.event_loop.set_control_flow(ControlFlow::Poll);

        let mut now = Instant::now();
        self.event_loop.run(move |event, eltw| {
            match event {
                Event::NewEvents(poll) => {
                    if now.elapsed().as_millis() >= self.target_frame_time {
                        now = Instant::now();
                        // perform an update
                    }

                    // remove viewports that want to close
                    self.viewports.retain(|viewport| !viewport.should_close);
                    if self.viewports.is_empty() {
                        eltw.exit();
                    }
                }
                Event::WindowEvent { window_id, event } => match event {
                    WindowEvent::CloseRequested => {
                        let viewport = self
                            .viewports
                            .iter_mut()
                            .find(|viewport| viewport.get_window_id() == window_id)
                            .unwrap();
                        viewport.should_close = true;
                    }
                    WindowEvent::KeyboardInput {
                        device_id,
                        event,
                        is_synthetic,
                    } => {
                        todo!("make an input system");
                    }
                    _ => (),
                },
                _ => (),
            }
        })
    }
}
