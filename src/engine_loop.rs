use wgpu::core::instance;
use winit::{
    error::EventLoopError, event::{Event, WindowEvent}, event_loop::{self, ControlFlow, EventLoop}, window::Window
};

use std::{time::Instant, vec::Vec, sync::Arc};

use crate::engine_window::{ViewportDesc, WindowContent};
use crate::engine_window::Viewport;

pub struct EngineLoop<'a> {
    event_loop: EventLoop<()>,
    target_frame_time: u128,
    viewports: Vec<Viewport<'a>>
}

impl<'a> EngineLoop<'a> {
    pub fn new() -> EngineLoop<'a> {
        let event_loop: EventLoop<()> = EventLoop::new().unwrap();
        let target_frame_time: u128 = 17;
        let mut viewports = Vec::new();

        Self{
            event_loop,
            target_frame_time,
            viewports,
        }
    }

    pub async fn create_new_viewport(&mut self)
    {
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
                    self.viewports.retain(| viewport | !viewport.should_close);
                    if self.viewports.is_empty() {
                        eltw.exit();
                    }
                },
                Event::WindowEvent { window_id, event } => {
                    match self.viewports.iter_mut().find(|viewport| viewport.get_window_id() == window_id) {
                        Some(viewport) => {
                            viewport.handle_inputs(&event);
                        }
                        _ => (),
                    }
                },  
                _ => (),
            }
        })
    }
}