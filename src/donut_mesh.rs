use crate::{math, base_mesh_trait::{self, MeshFactory}};


extern crate piston_window;
extern crate vecmath;
extern crate camera_controllers;
extern crate gfx;
extern crate shader_version;




pub struct DonutMeshFactory
{
    pub major_radius: f32,
    pub minor_radius: f32,
    pub segments_major: isize,
    pub segments_minor: isize,
}




impl DonutMeshFactory 
{
    pub fn new(major_radius: f32, minor_radius: f32) -> Self
    {
        DonutMeshFactory {
            major_radius: major_radius,
            minor_radius: minor_radius,
            segments_major: 32,
            segments_minor: 32,
        }
    }
}


impl base_mesh_trait::IntoDesc for DonutMeshFactory
{
    fn into_desc(&self) -> base_mesh_trait::MeshDesc 
    {
        base_mesh_trait::MeshDesc { 
            desc: ([ self.major_radius, self.minor_radius, 0., 0. ]),
        }
    }
}


impl crate::base_mesh_trait::MeshFactory for DonutMeshFactory 
{
    fn create_mesh_instance_on_gpu(&self,
                                   open_gl: &glutin_window::OpenGL,
                                   factory: &mut gfx_device_gl::Factory) -> crate::base_mesh_trait::BasicMeshOnGpu
    {
        let major_radius = self.major_radius;
        let minor_radius = self.minor_radius;
        let segments_major = self.segments_major;
        let segments_minor = self.segments_minor;

        let mut vertex_data = Vec::new();
        let mut index_data = Vec::new();

        for i in 0..segments_major 
        {
            let theta = (i as f32 / segments_major as f32) * math::TWO_PI;
            let cos_theta = theta.cos();
            let sin_theta = theta.sin();

            for j in 0..segments_minor 
            {
                let phi = (j as f32 / segments_minor as f32) * math::TWO_PI;
                let cos_phi = phi.cos();
                let sin_phi = phi.sin();

                let x = (major_radius + minor_radius * cos_phi) * cos_theta;
                let y = (major_radius + minor_radius * cos_phi) * sin_theta;
                let z = minor_radius * sin_phi;

                vertex_data.push(base_mesh_trait::Vertex::new([x, y, z]));
            }
        }

        for i in 0..segments_major 
        {
            for j in 0..segments_minor 
            {
                let next_i = (i + 1) % segments_major;
                let next_j = (j + 1) % segments_minor;

                let a = (i * segments_minor + j) as u16;
                let b = (next_i * segments_minor + j) as u16;
                let c = (next_i * segments_minor + next_j) as u16;
                let d = (i * segments_minor + next_j) as u16;

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
