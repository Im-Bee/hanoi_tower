use crate::{math, base_mesh_trait::{self, MeshFactory}};



extern crate piston_window;
extern crate vecmath;
extern crate camera_controllers;
extern crate gfx;
extern crate shader_version;



pub struct StickMeshFactory
{
}

impl StickMeshFactory 
{
    pub fn new() -> Self
    {
        StickMeshFactory {
        }
    }
}


impl crate::base_mesh_trait::IntoDesc for StickMeshFactory
{
    fn into_desc(&self) -> base_mesh_trait::MeshDesc 
    {
        base_mesh_trait::MeshDesc { 
            desc: ([ 0., 0., 0., 0. ]),
        }
    }
}


impl MeshFactory for StickMeshFactory 
{
    fn create_mesh_instance_on_gpu(&self,
                                   open_gl: &glutin_window::OpenGL,
                                   factory: &mut gfx_device_gl::Factory) -> crate::base_mesh_trait::BasicMeshOnGpu
    {
        let mut vertex_data = Vec::new();
        let mut index_data = Vec::new();

        let segments = 32; 
        let height_segments = 10; 
        let radius = 0.25;
        let height = 15.0;

        for i in 0..=height_segments 
        {
            let z = -height / 2.0 + (i as f32 / height_segments as f32) * height;

            for j in 0..=segments 
            {
                let theta = (j as f32 / segments as f32) * math::TWO_PI;
                let cos_theta = theta.cos();
                let sin_theta = theta.sin();

                let x = radius * cos_theta;
                let y = radius * sin_theta;

                let u = j as f32 / segments as f32;
                let v = i as f32 / height_segments as f32;

                vertex_data.push(base_mesh_trait::Vertex::new([x, y, z], [u, v]));
            }
        }

        for i in 0..height_segments 
        {
            for j in 0..segments 
            {
                let row1 = i * (segments + 1);
                let row2 = (i + 1) * (segments + 1);

                let a = (row1 + j) as u16;
                let b = (row2 + j) as u16;
                let c = (row2 + j + 1) as u16;
                let d = (row1 + j + 1) as u16;

                index_data.extend_from_slice(&[a, b, c, c, d, a]);
            }
        }

        base_mesh_trait::BasicMeshOnGpu::new_from_vertices(open_gl, 
                                                      factory,
                                                      base_mesh_trait::pipe::new(),
                                                      &vertex_data,
                                                      &index_data)
    }
}

