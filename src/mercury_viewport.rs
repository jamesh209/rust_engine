use winit::{
    event::WindowEvent,
    keyboard::{Key, NamedKey},
    platform::modifier_supplement::KeyEventExtModifierSupplement,
    window::{Window, WindowId},
};

use std::sync::Arc;

pub struct ViewportDesc<'a> {
    window: Arc<Window>,
    background: wgpu::Color,
    surface: wgpu::Surface<'a>,
}

pub struct Viewport<'a> {
    viewport_desc: ViewportDesc<'a>,
    config: wgpu::SurfaceConfiguration,
    pub should_close: bool,
}

impl<'a> ViewportDesc<'a> {
    pub fn new(window: Arc<Window>, background: wgpu::Color, instance: &wgpu::Instance) -> Self {
        let surface = instance.create_surface(window.clone()).unwrap();
        Self {
            window,
            background,
            surface,
        }
    }

    pub async fn build(self, instance: &wgpu::Instance) -> Viewport<'a> {
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&self.surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

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

        let should_close = false;

        Viewport {
            viewport_desc: self,
            config,
            should_close,
        }
    }
}

impl<'a> Viewport<'a> {
    pub fn get_window_id(&self) -> WindowId {
        self.viewport_desc.window.id()
    }
}
