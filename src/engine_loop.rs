use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window},
    error::EventLoopError,
};

use std::{task::Poll, time::Instant, vec::Vec};

pub struct EngineLoop {
    event_loop: EventLoop<()>,
    target_frame_time: u128,
    windows: Vec<Window>
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
                _ => (),
            }
        })
    }
}