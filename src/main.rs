#[macro_use]
extern crate clap;
#[macro_use]
extern crate gfx;

extern crate gfx_window_glutin;
extern crate glutin;

use clap::App;
use gfx::traits::FactoryExt;
use gfx::Device;
use gfx_window_glutin as gfx_glutin;
use glutin::dpi::LogicalSize;
use glutin::Api::OpenGl;
use glutin::{GlContext, GlRequest};

mod cpu;
mod font;

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [256.0, 256.0, 256.0, 1.0];

type BYTE = u8;
type WORD = u16;

gfx_defines!{ 
    vertex Vertex {
        pos: [f32; 4] = "a Pos",
        color: [f32; 3] = "a_Color",
    }

    constant Transform {
        transform: [[f32; 4]; 4] = "u_Transform",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = {},
        transform: gfx::ConstantBuffer<Transform> = "Transform",
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

fn main() {
    let yaml = load_yaml!("../res/config.yml");
    let _matches = App::from_yaml(yaml).get_matches();
    
    let mut events_loop = glutin::EventsLoop::new();
    let windowbuilder = glutin::WindowBuilder::new()
        .with_title("rust_chip8".to_string())
        .with_dimensions(LogicalSize {
            height: 512.0,
            width: 512.0,
        });
    let contextbuilder = glutin::ContextBuilder::new()
        .with_gl(GlRequest::Specific(OpenGl, (3, 2)))
        .with_vsync(true);
    let (window, mut device, mut factory, color_view, mut depth_view) =
        gfx_glutin::init::<ColorFormat, DepthFormat>(windowbuilder, contextbuilder, &events_loop);
    let pso = factory.create_pipeline_simple(
        include_bytes!("../res/shaders/shader_150.glslf"),
        include_bytes!("../res/shaders/shader_150.glslv"),
        pipe::new()).unwrap();

    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            if let glutin::Event::WindowEvent { event, .. } = event {
                match event {
                    glutin::WindowEvent::KeyboardInput {
                        input:
                            glutin::KeyboardInput {
                                virtual_keycode: Some(glutin::VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => running = false,
                    _ => {}
                }
            }
        });

        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
