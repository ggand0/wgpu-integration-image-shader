use iced_widget::shader::{self, Viewport};
use iced_winit::core::{Element, Rectangle, Theme, mouse};
use iced_wgpu::wgpu;
use crate::shader_pipeline::Pipeline;

pub struct ShaderScene;

impl ShaderScene {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug)]
pub struct Primitive;

impl shader::Primitive for Primitive {
    fn prepare(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        storage: &mut shader::Storage,
        bounds: &Rectangle,
        viewport: &Viewport,
    ) {
        println!("viewport: {:?}", viewport);
        let scale_factor = viewport.scale_factor() as f32;
        let viewport_size = viewport.physical_size();
        /*let shader_size = (
            (bounds.width * viewport.scale_factor() as f32) as u32,
            (bounds.height * viewport.scale_factor() as f32) as u32,
        );
        
        let bounds_relative = (
            bounds.x as f32 / viewport_size.width as f32,
            bounds.y as f32 / viewport_size.height as f32,
            bounds.width as f32 / viewport_size.width as f32,
            bounds.height as f32 / viewport_size.height as f32,
        );*/
        let bounds_physical = (
            (bounds.x * scale_factor) as f32,
            (bounds.y * scale_factor) as f32,
            (bounds.width * scale_factor) as f32,
            (bounds.height * scale_factor) as f32,
        );
    
        let bounds_relative = (
            bounds_physical.0 / viewport_size.width as f32,
            bounds_physical.1 / viewport_size.height as f32,
            bounds_physical.2 / viewport_size.width as f32,
            bounds_physical.3 / viewport_size.height as f32,
        );
    
        if storage.has::<Pipeline>() {
            storage.store(Pipeline::new(device, queue, format, bounds_relative));

        } else {
            storage.store(Pipeline::new(device, queue, format, bounds_relative));
        }
    }
    
    



    fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        storage: &shader::Storage,
        target: &wgpu::TextureView,
        clip_bounds: &Rectangle<u32>,
    ) {
        let pipeline = storage.get::<Pipeline>().unwrap();
        pipeline.render(target, encoder, clip_bounds);
    }
}

impl<Message> shader::Program<Message> for ShaderScene {
    type State = ();
    type Primitive = Primitive;

    fn draw(
        &self,
        _state: &Self::State,
        _cursor: mouse::Cursor,
        _bounds: Rectangle,
    ) -> Self::Primitive {
        Primitive
    }
}
