use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop}
};

struct EngineLoop {
    event_loop: EventLoop<()>,
    target_frame_time: i32,
}

impl EngineLoop {
    pub fn new() -> EngineLoop {
        let event_loop: EventLoop<()> = EventLoop::new().unwrap();
        let target_frame_time = 17;

        Self{
            event_loop,
            target_frame_time,
        };
    }

    pub async fn start(&self){
        event_loop.set_control_flow(ControlFlow::Poll);

        event_loop.run(move |event, eltw| {           
            match event {
            Event::WindowEvent { window_id, event } if window_id == state.window.id() => match event {
                WindowEvent::CloseRequested => eltw.exit(),
                _ => (),
            },
            _ => (),
            }
        });
    }

    pub async fn add_new_window(&self) {
        todo!()
    }
}