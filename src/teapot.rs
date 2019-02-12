#[macro_use]
extern crate glium;

use glium::{glutin, Surface, VertexBuffer};

mod support;

fn main() {
    // building the display, ie. the main object
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    #[derive(Copy, Clone, Debug)]
    struct Vertex {
        position: [u8; 3],
    }
    implement_vertex!(Vertex, position);

    // building the vertex and index buffers
    let vertex_buffer = support::load_wavefront(&display, include_bytes!("support/teapot.obj"));
    const CX: usize = 16;
    const CY: usize = 16;
    const CZ: usize = 16;

    #[derive(Copy, Clone)]
    pub enum Block {
        Air,
        Dirt,
        Grass,
    }
    let mut block = [[[Block::Air; CZ]; CY]; CX];
    block[0][0][0] = Block::Dirt;
    block[0][1][0] = Block::Dirt;
    block[1][1][0] = Block::Dirt;
    block[0][1][1] = Block::Dirt;

    let mut vertices = [Vertex {
        position: [0, 0, 0],
    }; CX * CY * CZ * 6 * 6];
    let mut i: usize = 0;
    for x in 0..CX {
        for y in 0..CY {
            for z in 0..CZ {
                match block[x][y][z] {
                    Block::Air => continue,
                    _ => {
                        if x > 0 {
                            if let Block::Air = block[x - 1][y][z] {
                                eprintln!("({}, {}, {}), x>0, x-1 == Air", x, y, z);
                                vertices[i] = Vertex {
                                    position: [x as u8, y as u8, z as u8],
                                };
                                i += 1;
                                vertices[i] = Vertex {
                                    position: [x as u8, y as u8, z as u8 + 1],
                                };
                                i += 1;
                                vertices[i] = Vertex {
                                    position: [x as u8, y as u8 + 1, z as u8],
                                };
                                i += 1;
                                vertices[i] = Vertex {
                                    position: [x as u8, y as u8, z as u8 + 1],
                                };
                                i += 1;
                                vertices[i] = Vertex {
                                    position: [x as u8, y as u8 + 1, z as u8],
                                };
                                i += 1;
                                vertices[i] = Vertex {
                                    position: [x as u8, y as u8 + 1, z as u8 + 1],
                                };
                                i += 1;
                            }
                        }

                        if x + 1 < CX {
                            if let Block::Air = block[x + 1][y][z] {
                                eprintln!("({}, {}, {}), x+1<CY, x+1 == Air", x, y, z);
                                vertices[i] = Vertex {
                                    position: [x as u8 + 1, y as u8, z as u8 + 1],
                                };
                                i += 1;
                                vertices[i] = Vertex {
                                    position: [x as u8 + 1, y as u8, z as u8],
                                };
                                i += 1;
                                vertices[i] = Vertex {
                                    position: [x as u8 + 1, y as u8 + 1, z as u8 + 1],
                                };
                                i += 1;
                                vertices[i] = Vertex {
                                    position: [x as u8 + 1, y as u8, z as u8],
                                };
                                i += 1;
                                vertices[i] = Vertex {
                                    position: [x as u8 + 1, y as u8 + 1, z as u8 + 1],
                                };
                                i += 1;
                                vertices[i] = Vertex {
                                    position: [x as u8 + 1, y as u8 + 1, z as u8],
                                };
                                i += 1;
                            }
                        }

                        if y > 0 {
                            if let Block::Air = block[x][y - 1][z] {
                                eprintln!("({}, {}, {}), y>0, y-1 == Air", x, y, z);
                                vertices[i] = Vertex {
                                    position: [x as u8, y as u8, z as u8],
                                };
                                i += 1;
                                vertices[i] = Vertex {
                                    position: [x as u8 + 1, y as u8, z as u8],
                                };
                                i += 1;
                                vertices[i] = Vertex {
                                    position: [x as u8, y as u8, z as u8 + 1],
                                };
                                i += 1;
                                vertices[i] = Vertex {
                                    position: [x as u8 + 1, y as u8, z as u8],
                                };
                                i += 1;
                                vertices[i] = Vertex {
                                    position: [x as u8, y as u8, z as u8 + 1],
                                };
                                i += 1;
                                vertices[i] = Vertex {
                                    position: [x as u8 + 1, y as u8, z as u8 + 1],
                                };
                                i += 1;
                            }
                        }

                        if y + 1 < CY {
                            if let Block::Air = block[x][y + 1][z] {
                                eprintln!("({}, {}, {}), y+1<CY, y+1 == Air", x, y, z);
                                vertices[i] = Vertex {
                                    position: [x as u8, y as u8 + 1, z as u8],
                                };
                                i += 1;
                                vertices[i] = Vertex {
                                    position: [x as u8 + 1, y as u8 + 1, z as u8],
                                };
                                i += 1;
                                vertices[i] = Vertex {
                                    position: [x as u8, y as u8 + 1, z as u8 + 1],
                                };
                                i += 1;
                                vertices[i] = Vertex {
                                    position: [x as u8 + 1, y as u8 + 1, z as u8],
                                };
                                i += 1;
                                vertices[i] = Vertex {
                                    position: [x as u8, y as u8 + 1, z as u8 + 1],
                                };
                                i += 1;
                                vertices[i] = Vertex {
                                    position: [x as u8 + 1, y as u8 + 1, z as u8 + 1],
                                };
                                i += 1;
                            }
                        }

                        if z > 0 {
                            if let Block::Air = block[x][y][z - 1] {
                                eprintln!("({}, {}, {}), z>0, z-1 == Air", x, y, z);
                                vertices[i] = Vertex {
                                    position: [x as u8 + 1, y as u8, z as u8],
                                };
                                i += 1;
                                vertices[i] = Vertex {
                                    position: [x as u8, y as u8, z as u8],
                                };
                                i += 1;
                                vertices[i] = Vertex {
                                    position: [x as u8 + 1, y as u8 + 1, z as u8],
                                };
                                i += 1;
                                vertices[i] = Vertex {
                                    position: [x as u8, y as u8, z as u8],
                                };
                                i += 1;
                                vertices[i] = Vertex {
                                    position: [x as u8 + 1, y as u8 + 1, z as u8],
                                };
                                i += 1;
                                vertices[i] = Vertex {
                                    position: [x as u8, y as u8 + 1, z as u8],
                                };
                                i += 1;
                            }
                        }

                        if z + 1 < CZ {
                            if let Block::Air = block[x][y][z + 1] {
                                eprintln!("({}, {}, {}), z+1<CY, z+1 == Air", x, y, z);
                                vertices[i] = Vertex {
                                    position: [x as u8, y as u8, z as u8 + 1],
                                };
                                i += 1;
                                vertices[i] = Vertex {
                                    position: [x as u8 + 1, y as u8, z as u8 + 1],
                                };
                                i += 1;
                                vertices[i] = Vertex {
                                    position: [x as u8, y as u8 + 1, z as u8 + 1],
                                };
                                i += 1;
                                vertices[i] = Vertex {
                                    position: [x as u8 + 1, y as u8, z as u8 + 1],
                                };
                                i += 1;
                                vertices[i] = Vertex {
                                    position: [x as u8, y as u8 + 1, z as u8 + 1],
                                };
                                i += 1;
                                vertices[i] = Vertex {
                                    position: [x as u8 + 1, y as u8 + 1, z as u8 + 1],
                                };
                                i += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    let vertex_buffer = VertexBuffer::dynamic(&display, &vertices).unwrap();
    // the program
    let program = program!(&display,
        140 => {
            vertex: "
                #version 140

                uniform mat4 persp_matrix;
                uniform mat4 view_matrix;

                in vec3 position;
                out vec3 v_position;

                void main() {
                    v_position = position;
                    gl_Position = persp_matrix * view_matrix * vec4(v_position * 0.005, 1.0);
                }
            ",

            fragment: "
                #version 140

                out vec4 f_color;

                const vec3 LIGHT = vec3(-0.2, 0.8, 0.1);

                void main() {
                    vec3 color = (0.3 + 0.7) * vec3(1.0, 1.0, 1.0);
                    f_color = vec4(color, 1.0);
                }
            ",
        },
    ).unwrap();

    //
    let mut camera = support::camera::CameraState::new();
    
    // the main loop
    support::start_loop(|| {
        camera.update();

        // building the uniforms
        let uniforms = uniform! {
            persp_matrix: camera.get_perspective(),
            view_matrix: camera.get_view(),
        };

        // draw parameters
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        // drawing a frame
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
        target.draw(&vertex_buffer,
                    &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                    &program, &uniforms, &params).unwrap();
        target.finish().unwrap();

        let mut action = support::Action::Continue;

        // polling and handling the events received by the window
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => action = support::Action::Stop,
                    ev => camera.process_input(&ev),
                },
                _ => (),
            }
        });

        action
    });
}
