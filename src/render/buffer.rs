use wgpu::util::DeviceExt;

use crate::systems::{Vec2, GameState, SCREEN_SIZE, Quad};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3]
}

const QUAD_VERTS: [Vertex; 4] =  [
    Vertex { position: [0.5, 0.5, 0.0], color: [0.0, 0.0, 0.0], }, // Top right
    Vertex { position: [-0.5, 0.5, 0.0], color: [-0.1,- -0.1, -0.1], }, // Top left
    Vertex { position: [-0.5, -0.5, 0.0], color: [-0.2, -0.2, -0.2], }, // Bottom left
    Vertex { position: [0.5, -0.5, 0.0], color: [-0.1, -0.1, -0.1], }, // Bottom right
];

const QUAD_INDIS: [u16; 6] = [
    0, 1, 2,
    0, 2, 3
];

impl Vertex {

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3
                }
            ]
        }
    }
}

const BRICK_COLORS : [[f32; 3]; 5] = [
    [0.9, 0.1, 0.1],
    [0.7, 0.5, 0.2],
    [0.2, 0.8, 0.4],
    [0.2, 0.4, 0.8],
    [0.0, 0.7, 0.8]
];

pub fn create_buffers(device: &wgpu::Device, state: &GameState) -> (Option<wgpu::Buffer>, Option<wgpu::Buffer>, usize) {

    let mut verts : Vec<Vertex> = vec![];
    let mut indis : Vec<u16> = vec![];

    create_quad(&state.player.quad, [0.8, 0.3, 0.1], &mut verts, &mut indis);
    create_quad(&state.ball.quad, [0.5, 0.5, 0.0], &mut verts, &mut indis);

    for brick in &state.bricks {
        create_quad(&brick.quad, BRICK_COLORS[(brick.health - 1) as usize], &mut verts, &mut indis)
    }

    let vertex_buffer = device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&verts),
            usage: wgpu::BufferUsages::VERTEX
        }
    );

    let index_buffer = device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indis),
            usage: wgpu::BufferUsages::INDEX
        }
    );

    (Some(vertex_buffer), Some(index_buffer), indis.len())
}

fn create_quad(quad: &Quad, color: [f32; 3], verts: &mut Vec<Vertex>, indis: &mut Vec<u16>) {


    let mut tile_verts : Vec<Vertex> = QUAD_VERTS.iter()
        .map(|v| Vertex {
            position: { 
                [((quad.pos.x + v.position[0] * quad.size.x as f32) / SCREEN_SIZE.x as f32), 
                ((quad.pos.y + v.position[1] * quad.size.y as f32) / SCREEN_SIZE.y as f32), 
                v.position[2]]
            },
            color: [
                v.color[0] + color[0],
                v.color[1] + color[1],
                v.color[2] + color[2]
            ]
        })
        .collect();

    let mut tile_indis : Vec<u16> = QUAD_INDIS.iter()
        .map(|i| i + verts.len() as u16)
        .collect();

    verts.append(&mut tile_verts);
    indis.append(&mut tile_indis);
}