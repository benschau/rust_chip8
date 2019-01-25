extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};

// Change this to OpenGL::V2_1 if not working.
const OPENGL_VERSION: OpenGL = OpenGL::V3_2;

pub struct App {
    pub window: Window,
    pub gl: GlGraphics,
    pub rotation: f64
}

pub struct GraphicsConfig {
    pub name: String,
    pub dim: (u32, u32),
}

impl GraphicsConfig {
    pub fn new(name: &str, dim: (u32, u32)) -> GraphicsConfig {
        GraphicsConfig {
            name: String::from(name),
            dim: dim
        }
    }
}

impl App {
    pub fn new(conf: GraphicsConfig) -> App {
        // Create an Glutin window.
        let mut window: Window = WindowSettings::new(
                conf.name,
                conf.dim
            )
            .opengl(OPENGL_VERSION)
            .exit_on_esc(true)
            .build()
            .unwrap();

        App { 
            window: window,
            gl: GlGraphics::new(OPENGL_VERSION),
            rotation: 0.0
        }    
    }

    pub fn input(&mut self, button: &Button) {
        if let Button::Keyboard(key) = *button {
            /* // use this here for mapping to keyboard data in Cpu.rs
            match key {
                Key::Up => self.
            
            } */
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 5.0);
        let rotation = self.rotation;
        let (x, y) = ((args.width / 2) as f64,
                      (args.height / 2) as f64);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c.transform.trans(x, y)
                                       .rot_rad(rotation)
                                       .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}

