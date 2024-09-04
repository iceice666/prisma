use encase::{ShaderType, StorageBuffer, UniformBuffer};
use glam::Vec3;
use tobj::LoadOptions;

use crate::primitives::{Triangle, Vertex};

use super::{Bvh, Camera, RenderContext};

#[derive(Default, ShaderType)]
struct Uniform {
    camera: Camera,
    env_map: u32,
}

#[derive(Default)]
pub struct Scene {
    uniform: Uniform,
}

impl Scene {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_camera(&mut self, camera: Camera) {
        self.uniform.camera = camera;
    }

    pub fn set_env_map(&mut self, env_map: u32) {
        self.uniform.env_map = env_map;
    }

    pub fn build(
        &mut self,
        context: &RenderContext,
    ) -> encase::internal::Result<(wgpu::BindGroupLayout, wgpu::BindGroup)> {
        let device = context.device();
        let queue = context.queue();

        let mut wgsl_bytes = UniformBuffer::new(Vec::new());
        wgsl_bytes.write(&self.uniform)?;
        let wgsl_bytes = wgsl_bytes.into_inner();

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: wgsl_bytes.len() as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
            mapped_at_creation: false,
        });
        queue.write_buffer(&uniform_buffer, 0, &wgsl_bytes);

        let (models, _materials) = tobj::load_obj(
            "models/bunny.obj",
            &LoadOptions {
                single_index: true,
                triangulate: true,
                ..Default::default()
            },
        )
        .unwrap();
        let mut vertices = Vec::new();
        let mut primitives = Vec::new();
        for model in models {
            let mesh = &model.mesh;

            for v in 0..mesh.positions.len() / 3 {
                vertices.push(Vertex {
                    pos: Vec3::new(
                        mesh.positions[3 * v],
                        mesh.positions[3 * v + 1],
                        mesh.positions[3 * v + 2],
                    ),
                    normal: Vec3::new(
                        mesh.normals[3 * v],
                        mesh.normals[3 * v + 1],
                        mesh.normals[3 * v + 2],
                    ),
                });
            }

            for i in 0..mesh.indices.len() / 3 {
                let triangle = Triangle {
                    p0: mesh.indices[3 * i],
                    p1: mesh.indices[3 * i + 1],
                    p2: mesh.indices[3 * i + 2],
                };
                primitives.push(triangle);
            }
        }

        let bvh = Bvh::new(&vertices, &mut primitives);

        let mut wgsl_bytes = StorageBuffer::new(Vec::new());
        wgsl_bytes.write(&vertices)?;
        let wgsl_bytes = wgsl_bytes.into_inner();

        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: wgsl_bytes.len() as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
            mapped_at_creation: false,
        });
        queue.write_buffer(&vertex_buffer, 0, &wgsl_bytes);

        let mut wgsl_bytes = StorageBuffer::new(Vec::new());
        wgsl_bytes.write(&primitives)?;
        let wgsl_bytes = wgsl_bytes.into_inner();

        let primitive_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: wgsl_bytes.len() as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
            mapped_at_creation: false,
        });
        queue.write_buffer(&primitive_buffer, 0, &wgsl_bytes);

        let mut wgsl_bytes = StorageBuffer::new(Vec::new());
        wgsl_bytes.write(&bvh.flatten())?;
        let wgsl_bytes = wgsl_bytes.into_inner();

        let bvh_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: wgsl_bytes.len() as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
            mapped_at_creation: false,
        });
        queue.write_buffer(&bvh_buffer, 0, &wgsl_bytes);

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: vertex_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: primitive_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: bvh_buffer.as_entire_binding(),
                },
            ],
        });

        Ok((bind_group_layout, bind_group))
    }
}
