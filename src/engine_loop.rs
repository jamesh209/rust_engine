use winit::{
    error::EventLoopError, event::{Event, WindowEvent}, event_loop::{self, ControlFlow, EventLoop}, window::Window
};

use std::{task::Poll, time::Instant, vec::Vec};

use crate::engine_window::WindowContenProvider;

pub struct EngineLoop {
    event_loop: EventLoop<()>,
    target_frame_time: u128,
    windows: Vec<Box<dyn WindowContenProvider>>
}

impl EngineLoop {
    pub fn new() -> EngineLoop {
        let event_loop: EventLoop<()> = EventLoop::new().unwrap();
        let target_frame_time: u128 = 17;
        let windows = Vec::new();

        Self{
            event_loop,
            target_frame_time,
            windows
        }
    }

    pub fn add_new_window<F>(&mut self, constructor: F)
    where
        F: FnOnce(&EventLoop<()>) -> Box<dyn WindowContenProvider>
    {
        self.windows.push(constructor(&self.event_loop));
    }

    pub async fn start(self) -> Result<(), EventLoopError> {
        self.event_loop.set_control_flow(ControlFlow::Poll);

        let mut now = Instant::now();
        self.event_loop.run(move |event, eltw| {
            match event {
                Event::NewEvents(poll) => {
                    if now.elapsed().as_millis() >= self.target_frame_time {
                        now = Instant::now();
                        // perform an update
                    }
                },
                Event::WindowEvent { window_id, event } => {
                    println!("{event:?}")
                },
                _ => (),
            }
        })
    }
}