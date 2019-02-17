use game01::*;
use std::time::{Duration, Instant};
use std::thread;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Cursor;
use glium::{glutin, implement_vertex, uniform, Display, Surface, VertexBuffer};
use serde::{Deserialize, Serialize};
use rmp_serde::{Deserializer, Serializer};


pub enum Action {
    Stop,
    Continue,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Player {
    name: String,
    pos: (f32, f32, f32),
}

pub fn start_loop<F>(mut callback: F) where F: FnMut() -> Action {
    let mut accumulator = Duration::new(0, 0);
    let mut previous_clock = Instant::now();

    loop {
        match callback() {
            Action::Stop => break,
            Action::Continue => ()
        };

        let now = Instant::now();
        accumulator += now - previous_clock;
        previous_clock = now;

        let fixed_time_stamp = Duration::new(0, 16666667);
        while accumulator >= fixed_time_stamp {
            accumulator -= fixed_time_stamp;

            // if you have a game, update the state here
            // 次のループの値や状態遷移をセット
            
        }

        thread::sleep(fixed_time_stamp - accumulator);
    }
}

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();
let image = image::load(Cursor::new(&include_bytes!("./opengl.png")[..]), image::PNG) .unwrap() .to_rgba(); let image_dimensions = image.dimensions();
    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::texture::Texture2d::new(&display, image).unwrap();

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

    let window = std::cell::Ref::clone(&display.gl_window());
    let window = window.window();
    window.hide_cursor(true);

    let mut chunk = chunk::Chunk::new(&display);
    for x in 1..15 {
        for z in 1..15 {
            let block = if (x + z) % 2 == 0 {
                chunk::Block::Dirt
            } else {
                chunk::Block::Grass
            };
            for y in 1..x {
                chunk.set(x, ((x + 15 -z) as f64/2.).floor() as usize, 15 -z, block);
            }
        }
    }
    chunk.update();

    let mut camera = camera::CameraState::new();

	let player = Player {
        pos: (0., 0., 0.),
        name: "player1".into(),
    };

    start_loop(|| {
        let mut action = Action::Continue;

        // 入力
        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => action = Action::Stop,
                glutin::WindowEvent::CursorMoved {
                    position,
                    ..
                } => {
                    let gl_window = std::cell::Ref::clone(&display.gl_window());

                    let window = gl_window.window();
                    let inner_size = window.get_inner_size().unwrap();
                    let cursor_diff = (position.x - inner_size.width / 2., position.y - inner_size.height / 2.);
                    let direction = camera.get_direction();
                    camera.set_direction((direction.0 + cursor_diff.0 as f32, direction.1 + cursor_diff.1 as f32, 1.));
                    window.set_cursor_position(glutin::dpi::LogicalPosition::new(inner_size.width / 2., inner_size.height / 2.));
                },
                ev => camera.process_input(&ev),
                _ => (),
            },
            _ => (),
        });



        // サーバーへ送信
        let mut buf = Vec::new();
        player.serialize(&mut Serializer::new(&mut buf)).unwrap();
        //dbg!(&buf);
        // send();
        
        // サーバーから受信
        // recv();
        let mut de = Deserializer::new(&buf[..]);
        //dbg!(<Player as Deserialize>::deserialize(&mut de).unwrap());

        // データ反映
        camera.update();

        // 描画
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        chunk.render(&mut target, &program, &texture, &camera); //, &uniforms);
        //for p in players {
        //    p.render();
        //}
        target.finish().unwrap();

        action
    });
}

