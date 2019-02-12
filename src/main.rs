use game01::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Cursor;

fn main() {
    use glium::{glutin, implement_vertex, uniform, Display, Surface, VertexBuffer};
    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
        tex_coords: [f32; 2],
    }
    implement_vertex!(Vertex, position, tex_coords);

    struct Shape {
        position: [f32; 2],
        local_vertices: Vec<Vertex>,
        vertices: Vec<Vertex>,
        vertex_buffer: VertexBuffer<Vertex>,
    }

    /*
    impl Shape {
        fn new(display: &Display, position: [f32; 2], local_vertices: Vec<Vertex>) -> Self {
            let vertices = local_vertices + position;
            Shape {
                position,
                local_vertices,
                vertices,
                vertex_buffer: glium::VertexBuffer::new(display, &vertices).unwrap(),
            }
        }
        fn transfer(&mut self, (x, y): (f32, f32)) {
            self.position[0] += x;
            self.position[1] += y;
            self.vertices = self
                .local_vertices
                .into_iter()
                .map(|vertex| Vertex {
                    position: [x + self.position[0], y + self.position[1]],
                    vertex,
                })
                .collect::<Vec<_>>();
            self.vertex_buffer.write(&self.vertices);
        }
    }
    */

    let position = glutin::dpi::LogicalPosition::new(0., 0.);
    let player = player::Player { position };
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_dimensions(glutin::dpi::LogicalSize::new(1024.0, 768.0))
        .with_title("Hello, world");
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let image = image::load(Cursor::new(&include_bytes!("./opengl.png")[..]), image::PNG)
        .unwrap()
        .to_rgba();
    let image_dimensions = image.dimensions();
    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::texture::Texture2d::new(&display, image).unwrap();
    let shape = vec![
        Vertex {
            position: [-0.8, -0.8],
            tex_coords: [0.0, 0.0],
        },
        Vertex {
            position: [-0.8, 0.2],
            tex_coords: [0.0, 1.0],
        },
        Vertex {
            position: [0.2, -0.8],
            tex_coords: [1.0, 0.0],
        },
        Vertex {
            position: [0.2, 0.2],
            tex_coords: [1.0, 1.0],
        },
    ];
    let vertex_buffer1 = glium::VertexBuffer::new(&display, &shape).unwrap();
    const INDICES: [u16; 6] = [0, 1, 2, 1, 2, 3];
    let indices = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &INDICES,
    )
    .unwrap();

    let file = File::open("./src/block.vert").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut vertex_shader_src = String::new();
    buf_reader.read_to_string(&mut vertex_shader_src).unwrap();

    let file = File::open("./src/sample.frag").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut fragment_shader_src = String::new();
    buf_reader.read_to_string(&mut fragment_shader_src).unwrap();

    let program =
        glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None)
            .unwrap();

    let mut chunk = chunk::Chunk::new(&display);
    chunk.set(1, 1, 1, chunk::Block::Dirt);
    chunk.update();

    let mut camera = camera::CameraState::new();
    let mut time: f32 = -0.5;
    let mut closed = false;
    while !closed {
        camera.update();
        time += 0.0002;
        if time > 0.5 {
            time = -0.5;
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [time, 0.0, 0.0, 1.0f32],
            ],
            tex: &texture,
        };

        chunk.render(&mut target, &program, &texture, &camera); //, &uniforms);
        //target
        //    .draw(
        //        &vertex_buffer1,
        //        &indices,
        //        &program,
        //        &uniforms,
        //        &Default::default(),
        //    )
        //    .unwrap();
        //target
        //    .draw(
        //        &vertex_buffer2,
        //        &indices,
        //        &program,
        //        &uniforms,
        //        &Default::default(),
        //    )
        //    .unwrap();
        target.finish().unwrap();

        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => closed = true,
                //glutin::WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                //    Some(key) => match key {
                //        glutin::VirtualKeyCode::W => {}
                //        glutin::VirtualKeyCode::A => {}
                //        glutin::VirtualKeyCode::S => {}
                //        glutin::VirtualKeyCode::D => {
                //            /*
                //            shape
                //                .iter_mut()
                //                .map(|a| a.position[0] += 0.05)
                //                .collect::<Vec<_>>();
                //            vertex_buffer2.write(&shape);
                //            */
                //        }
                //        _ => (),
                //    },
                //    None => (),
                //},
                //glutin::WindowEvent::MouseInput {
                //    device_id,
                //    state,
                //    button,
                //    modifiers,
                //} => {
                //    //dbg!(device_id);
                //    //dbg!(state);
                //    //dbg!(button);
                //    //dbg!(modifiers);
                //}
                //glutin::WindowEvent::CursorMoved {
                //    device_id,
                //    position,
                //    modifiers,
                //} => {
                //    //dbg!(device_id);
                //    //dbg!(position);
                //    //dbg!(modifiers);
                //}
                //
                ev => camera.process_input(&ev),
                _ => (),
            },
            _ => (),
        });
    }
}
/*
 * pub fn set_cursor_position(
    &self,
    position: LogicalPosition
) -> Result<(), String>
pub fn new(x: f64, y: f64) -> LogicalPosition
*/
