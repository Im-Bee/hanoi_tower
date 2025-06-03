use crate::{math, mesh_trait::{self, MeshFactory}};

extern crate piston_window;
extern crate vecmath;
extern crate camera_controllers;
extern crate gfx;
extern crate shader_version;




gfx_vertex_struct!( 
    Vertex 
    {
        a_pos:       [f32; 4] = "a_pos",
        a_tex_coord: [f32; 2] = "a_tex_coord",
    }
);

impl Vertex 
{
    fn new(pos: [f32; 3], tc: [f32; 2]) -> Vertex 
    {
        Vertex {
            a_pos: [pos[0], pos[1], pos[2], 1.],
            a_tex_coord: tc,
        }
    }
}

gfx_pipeline!( 
    pipe 
    {
        vbuf:              gfx::VertexBuffer<Vertex>                     = (),
        u_model_view_proj: gfx::Global<[[f32; 4]; 4]>                    = "u_model_view_proj",
        t_color:           gfx::TextureSampler<[f32; 4]>                 = "t_color",
        out_color:         gfx::RenderTarget<::gfx::format::Srgba8>      = "o_Color",
        out_depth:         gfx::DepthTarget<::gfx::format::DepthStencil> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }
);




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


impl mesh_trait::IntoDesc for DonutMeshFactory
{
    fn into_desc(&self) -> mesh_trait::MeshDesc 
    {
        mesh_trait::MeshDesc { 
            desc: ([ self.major_radius, self.minor_radius, 0., 0. ]),
        }
    }
}


impl crate::mesh_trait::MeshFactory<Vertex, crate::donut_mesh::pipe::Init<'static>> for DonutMeshFactory 
{
    fn create_mesh_instance_on_gpu(&self,
                                   open_gl: &glutin_window::OpenGL,
                                   factory: &mut gfx_device_gl::Factory) -> crate::mesh_trait::MeshOnGpu<
                                                                              Vertex,
                                                                              crate::donut_mesh::pipe::Init<'static>>
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

                let u = i as f32 / segments_major as f32;
                let v = j as f32 / segments_minor as f32;

                vertex_data.push(Vertex::new([x, y, z], [u, v]));
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
    
        mesh_trait::MeshOnGpu::new_from_vertices(open_gl, 
                                                 factory,
                                                 pipe::new(),
                                                 &vertex_data,
                                                 &index_data)
    }
}
