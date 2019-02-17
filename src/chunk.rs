use glium::*;
const CX: usize = 16;
const CY: usize = 16;
const CZ: usize = 16;

#[derive(Copy, Clone)]
pub enum Block {
    Air,
    Dirt,
    Grass,
}

#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [u8; 3],
    tex_coords: [f32; 2],
    normal: [f32; 3],
    material: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords, normal, material);


pub struct Chunk<'a> {
    block: [[[Block; CZ]; CY]; CX],
    changed: bool,
    vertex_buffer: VertexBuffer<Vertex>,
    display: &'a Display,
}

impl<'a> Chunk<'a> {
    pub fn new(display: &'a Display) -> Self {
        Chunk {
            block: [[[Block::Air; CZ]; CY]; CX],
            changed: true,
            vertex_buffer: VertexBuffer::empty(display, 0).unwrap(),
            display,
        }
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> Block {
        self.block[x][y][z]
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, block: Block) {
        self.block[x][y][z] = block;
        self.changed = true;
    }

    pub fn update(&mut self) {
        self.changed = false;
        let mut vertices = [Vertex {
            position: [0, 0, 0],
            tex_coords: [0., 0.],
            normal: [0., 0., 0.],
            material: [0., 1.],
        }; CX * CY * CZ * 6 * 6];
        let mut i: usize = 0;
        for x in 0..CX {
            for y in 0..CY {
                for z in 0..CZ {
                    match self.block[x][y][z] {
                        Block::Air => continue,
                        _ => {
                            if x > 0 {
                                if let Block::Air = self.block[x - 1][y][z] {
                                    vertices[i] = Vertex {
                                        position: [x as u8, y as u8, z as u8],
                                        tex_coords: [0., 0.],
                                        normal: [-1., 0. ,0.],
                                        material: [(self.block[x][y][z] as u8).into(), 1.0],

                                    };
                                    i += 1;
                                    vertices[i] = Vertex {
                                        position: [x as u8, y as u8, z as u8 + 1],
                                        tex_coords: [1., 0.],
                                        normal: [-1., 0. ,0.],
                                        material: [(self.block[x][y][z] as u8).into(), 1.0],
                                    };
                                    i += 1;
                                    vertices[i] = Vertex {
                                        position: [x as u8, y as u8 + 1, z as u8],
                                        tex_coords: [0., 1.],
                                        normal: [-1., 0. ,0.],
                                        material: [(self.block[x][y][z] as u8).into(), 1.0],
                                    };
                                    i += 1;
                                    vertices[i] = Vertex {
                                        position: [x as u8, y as u8, z as u8 + 1],
                                        tex_coords: [1., 0.],
                                        normal: [-1., 0. ,0.],
                                        material: [(self.block[x][y][z] as u8).into(), 1.0],
                                    };
                                    i += 1;
                                    vertices[i] = Vertex {
                                        position: [x as u8, y as u8 + 1, z as u8 + 1],
                                        tex_coords: [1., 1.],
                                        normal: [-1., 0. ,0.],
                                        material: [(self.block[x][y][z] as u8).into(), 1.0],
                                    };
                                    i += 1;
                                    vertices[i] = Vertex {
                                        position: [x as u8, y as u8 + 1, z as u8],
                                        tex_coords: [0., 1.],
                                        normal: [-1., 0. ,0.],
                                        material: [(self.block[x][y][z] as u8).into(), 1.0],
                                    };
                                    i += 1;
                                }
                            }

                            if x + 1 < CX {
                                if let Block::Air = self.block[x + 1][y][z] {
                                    vertices[i] = Vertex {
                                        position: [x as u8 + 1, y as u8, z as u8 + 1],
                                        tex_coords: [0., 0.],
                                        normal: [1., 0. ,0.],
                                        material: [(self.block[x][y][z] as u8).into(), 1.0],
                                    };
                                    i += 1;
                                    vertices[i] = Vertex {
                                        position: [x as u8 + 1, y as u8, z as u8],
                                        tex_coords: [1., 0.],
                                        normal: [1., 0. ,0.],
                                        material: [(self.block[x][y][z] as u8).into(), 1.0],
                                    };
                                    i += 1;
                                    vertices[i] = Vertex {
                                        position: [x as u8 + 1, y as u8 + 1, z as u8 + 1],
                                        tex_coords: [0., 1.],
                                        normal: [1., 0. ,0.],
                                        material: [(self.block[x][y][z] as u8).into(), 1.0],
                                    };
                                    i += 1;
                                    vertices[i] = Vertex {
                                        position: [x as u8 + 1, y as u8, z as u8],
                                        tex_coords: [1., 0.],
                                        normal: [1., 0. ,0.],
                                        material: [(self.block[x][y][z] as u8).into(), 1.0],
                                    };
                                    i += 1;
                                    vertices[i] = Vertex {
                                        position: [x as u8 + 1, y as u8 + 1, z as u8],
                                        tex_coords: [1., 1.],
                                        normal: [1., 0. ,0.],
                                        material: [(self.block[x][y][z] as u8).into(), 1.0],
                                    };
                                    i += 1;
                                    vertices[i] = Vertex {
                                        position: [x as u8 + 1, y as u8 + 1, z as u8 + 1],
                                        tex_coords: [0., 1.],
                                        normal: [1., 0. ,0.],
                                        material: [(self.block[x][y][z] as u8).into(), 1.0],
                                    };
                                    i += 1;
                                }
                            }

                            if y > 0 {
                                if let Block::Air = self.block[x][y - 1][z] {
                                    vertices[i] = Vertex {
                                        position: [x as u8, y as u8, z as u8],
                                        tex_coords: [0., 0.],
                                        normal: [0., -1. ,0.],
                                        material: [(self.block[x][y][z] as u8).into(), 1.0],
                                    };
                                    i += 1;
                                    vertices[i] = Vertex {
                                        position: [x as u8 + 1, y as u8, z as u8],
                                        tex_coords: [1., 0.],
                                        normal: [0., -1. ,0.],
                                        material: [(self.block[x][y][z] as u8).into(), 1.0],
                                    };
                                    i += 1;
                                    vertices[i] = Vertex {
                                        position: [x as u8, y as u8, z as u8 + 1],
                                        tex_coords: [0., 1.],
                                        normal: [0., -1. ,0.],
                                        material: [(self.block[x][y][z] as u8).into(), 1.0],
                                    };
                                    i += 1;
                                    vertices[i] = Vertex {
                                        position: [x as u8 + 1, y as u8, z as u8],
                                        tex_coords: [1., 0.],
                                        normal: [0., -1. ,0.],
                                        material: [(self.block[x][y][z] as u8).into(), 1.0],
                                    };
                                    i += 1;
                                    vertices[i] = Vertex {
                                        position: [x as u8 + 1, y as u8, z as u8 + 1],
                                        tex_coords: [1., 1.],
                                        normal: [0., -1. ,0.],
                                        material: [(self.block[x][y][z] as u8).into(), 1.0],
                                    };
                                    i += 1;
                                    vertices[i] = Vertex {
                                        position: [x as u8, y as u8, z as u8 + 1],
                                        tex_coords: [0., 1.],
                                        normal: [0., -1. ,0.],
                                        material: [(self.block[x][y][z] as u8).into(), 1.0],
                                    };
                                    i += 1;
                                }
                            }

                            if y + 1 < CY {
                                if let Block::Air = self.block[x][y + 1][z] {
                                    vertices[i] = Vertex {
                                        position: [x as u8, y as u8 + 1, z as u8 + 1],
                                        tex_coords: [0., 0.],
                                        normal: [0., 1. ,0.],
                                        material: [(self.block[x][y][z] as u8).into(), 1.0],
                                    };
                                    i += 1;
                                    vertices[i] = Vertex {
                                        position: [x as u8 + 1, y as u8 + 1, z as u8 + 1],
                                        tex_coords: [1., 0.],
                                        normal: [0., 1. ,0.],
                                        material: [(self.block[x][y][z] as u8).into(), 1.0],
                                    };
                                    i += 1;
                                    vertices[i] = Vertex {
                                        position: [x as u8, y as u8 + 1, z as u8],
                                        tex_coords: [0., 1.],
                                        normal: [0., 1. ,0.],
                                        material: [(self.block[x][y][z] as u8).into(), 1.0],
                                    };
                                    i += 1;
                                    vertices[i] = Vertex {
                                        position: [x as u8 + 1, y as u8 + 1, z as u8 + 1],
                                        tex_coords: [1., 0.],
                                        normal: [0., 1. ,0.],
                                        material: [(self.block[x][y][z] as u8).into(), 1.0],
                                    };
                                    i += 1;
                                    vertices[i] = Vertex {
                                        position: [x as u8 + 1, y as u8 + 1, z as u8],
                                        tex_coords: [1., 1.],
                                        normal: [0., 1. ,0.],
                                        material: [(self.block[x][y][z] as u8).into(), 1.0],
                                    };
                                    i += 1;
                                    vertices[i] = Vertex {
                                        position: [x as u8, y as u8 + 1, z as u8],
                                        tex_coords: [0., 1.],
                                        normal: [0., 1. ,0.],
                                        material: [(self.block[x][y][z] as u8).into(), 1.0],
                                    };
                                    i += 1;
                                }
                            }

                            if z > 0 {
                                if let Block::Air = self.block[x][y][z - 1] {
                                    vertices[i] = Vertex {
                                        position: [x as u8 + 1, y as u8, z as u8],
                                        tex_coords: [0., 0.],
                                        normal: [0., 0., -1.],
                                        material: [(self.block[x][y][z] as u8).into(), 1.0],
                                    };
                                    i += 1;
                                    vertices[i] = Vertex {
                                        position: [x as u8, y as u8, z as u8],
                                        tex_coords: [1., 0.],
                                        normal: [0., 0., -1.],
                                        material: [(self.block[x][y][z] as u8).into(), 1.0],
                                    };
                                    i += 1;
                                    vertices[i] = Vertex {
                                        position: [x as u8 + 1, y as u8 + 1, z as u8],
                                        tex_coords: [0., 1.],
                                        normal: [0., 0., -1.],
                                        material: [(self.block[x][y][z] as u8).into(), 1.0],
                                    };
                                    i += 1;
                                    vertices[i] = Vertex {
                                        position: [x as u8, y as u8, z as u8],
                                        tex_coords: [1., 0.],
                                        normal: [0., 0., -1.],
                                        material: [(self.block[x][y][z] as u8).into(), 1.0],
                                    };
                                    i += 1;
                                    vertices[i] = Vertex {
                                        position: [x as u8, y as u8 + 1, z as u8],
                                        tex_coords: [1., 1.],
                                        normal: [0., 0., -1.],
                                        material: [(self.block[x][y][z] as u8).into(), 1.0],
                                    };
                                    i += 1;
                                    vertices[i] = Vertex {
                                        position: [x as u8 + 1, y as u8 + 1, z as u8],
                                        tex_coords: [0., 1.],
                                        normal: [0., 0., -1.],
                                        material: [(self.block[x][y][z] as u8).into(), 1.0],
                                    };
                                    i += 1;
                                }
                            }

                            if z + 1 < CZ {
                                if let Block::Air = self.block[x][y][z + 1] {
                                    vertices[i] = Vertex {
                                        position: [x as u8, y as u8, z as u8 + 1],
                                        tex_coords: [0., 0.],
                                        normal: [0., 0., 1.],
                                        material: [(self.block[x][y][z] as u8).into(), 1.0],
                                    };
                                    i += 1;
                                    vertices[i] = Vertex {
                                        position: [x as u8 + 1, y as u8, z as u8 + 1],
                                        tex_coords: [1., 0.],
                                        normal: [0., 0., 1.],
                                        material: [(self.block[x][y][z] as u8).into(), 1.0],
                                    };
                                    i += 1;
                                    vertices[i] = Vertex {
                                        position: [x as u8, y as u8 + 1, z as u8 + 1],
                                        tex_coords: [0., 1.],
                                        normal: [0., 0., 1.],
                                        material: [(self.block[x][y][z] as u8).into(), 1.0],
                                    };
                                    i += 1;
                                    vertices[i] = Vertex {
                                        position: [x as u8 + 1, y as u8, z as u8 + 1],
                                        tex_coords: [1., 0.],
                                        normal: [0., 0., 1.],
                                        material: [(self.block[x][y][z] as u8).into(), 1.0],
                                    };
                                    i += 1;
                                    vertices[i] = Vertex {
                                        position: [x as u8 + 1, y as u8 + 1, z as u8 + 1],
                                        tex_coords: [1., 1.],
                                        normal: [0., 0., 1.],
                                        material: [(self.block[x][y][z] as u8).into(), 1.0],
                                    };
                                    i += 1;
                                    vertices[i] = Vertex {
                                        position: [x as u8, y as u8 + 1, z as u8 + 1],
                                        tex_coords: [0., 1.],
                                        normal: [0., 0., 1.],
                                        material: [(self.block[x][y][z] as u8).into(), 1.0],
                                    };
                                    i += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        self.vertex_buffer = VertexBuffer::dynamic(self.display, &vertices).unwrap();
    }

    pub fn render<T: Surface>(
        &mut self,
        target: &mut T,
        program: &Program,
        texture: &texture::Texture2d,
        camera: &super::camera::CameraState,
    ) {
        if self.changed {
            self.update();
        }
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            ..Default::default()
        };
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let uniforms = uniform! {
            persp_matrix: camera.get_perspective(),
            view_matrix: camera.get_view(),
            tex: texture,
        };
        target
            .draw(
                &self.vertex_buffer,
                &indices,
                &program,
                &uniforms,
                //&uniforms::EmptyUniforms,
                &params,
            )
            .unwrap();
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn range() {}
}
