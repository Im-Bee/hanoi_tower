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


impl mesh_trait::IntoDesc for StickMeshFactory
{
    fn into_desc(&self) -> mesh_trait::MeshDesc 
    {
        mesh_trait::MeshDesc { 
            desc: ([ 0., 0., 0., 0. ]),
        }
    }
}


impl MeshFactory<Vertex, pipe::Init<'static>> for StickMeshFactory 
{
    fn create_mesh_instance_on_gpu(&self,
                                   open_gl: &glutin_window::OpenGL,
                                   factory: &mut gfx_device_gl::Factory) -> crate::mesh_trait::MeshOnGpu<
                                                                              Vertex,
                                                                              pipe::Init<'static>>
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

                vertex_data.push(Vertex::new([x, y, z], [u, v]));
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

        mesh_trait::MeshOnGpu::new_from_vertices(open_gl, 
                                                 factory,
                                                 pipe::new(),
                                                 &vertex_data,
                                                 &index_data)
    }
}

