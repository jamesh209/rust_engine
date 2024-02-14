use winit::{
    event::{WindowEvent},
    event_loop::{EventLoop},
    window::{Window, WindowBuilder}
};

use wgpu::{
    InstanceDescriptor,
    util::DeviceExt,
};

use std::sync::Arc;

pub trait WindowContent {
    fn render(&mut self);

    fn handle_inputs(&mut self, event : &WindowEvent);

    fn resize(&mut self, device: &wgpu::Device, size: winit::dpi::PhysicalSize<u32>);
}

pub struct ViewportDesc<'a> {
    window: Arc<Window>,
    background: wgpu::Color,
    surface: wgpu::Surface<'a>,
}

pub struct Viewport<'a>{
    viewport_desc: ViewportDesc<'a>,
    config: wgpu::SurfaceConfiguration,
}

impl<'a>  ViewportDesc<'a> {
    pub fn new(window: Arc<Window>, background: wgpu::Color, instance: &wgpu::Instance) -> Self {
        let surface = instance.create_surface(window.clone()).unwrap();
        Self {
            window,
            background,
            surface,
        }
    }

    pub async fn build(self, instance: &wgpu::Instance) -> Viewport<'a> {
        
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&self.surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_defaults(),
            },
            None,
        )
        .await
        .expect("Failed to create device");      
        
        let size = self.window.inner_size();
        let config = self
            .surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();
        self.surface.configure(&device, &config);
        Viewport { viewport_desc: self, config }
    }
}

impl<'a> Viewport<'a> {
    fn resize(&mut self, device: &wgpu::Device, size: winit::dpi::PhysicalSize<u32>) {
        self.config.width = size.width;
        self.config.height = size.height;
        self.viewport_desc.surface.configure(device, &self.config);
    }
    fn get_current_texture(&mut self) -> wgpu::SurfaceTexture {
        self.viewport_desc
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture")
    }
}