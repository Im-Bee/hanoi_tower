#![allow(dead_code)]

extern crate piston_window;
extern crate vecmath;
extern crate camera_controllers;
extern crate gfx;
extern crate shader_version;

use gfx::IntoIndexBuffer;

use gfx::{
    Factory,
    traits::FactoryExt,
};
use opengl_graphics::GLSL;
use rand::Rng;
use shader_version::Shaders;





gfx_vertex_struct!( 
    Vertex 
    {
        a_pos: [f32; 4] = "a_pos",
    }
);

impl Vertex 
{
    pub fn new(pos: [f32; 3], _: [f32; 2]) -> Vertex 
    {
        Vertex {
            a_pos: [pos[0], pos[1], pos[2], 1.],
        }
    }
}

gfx_pipeline!( 
    pipe 
    {
        vbuf:              gfx::VertexBuffer<Vertex>                     = (),
        u_model_view_proj: gfx::Global<[[f32; 4]; 4]>                    = "u_model_view_proj",
        t_color:           gfx::Global<[f32; 4]>                         = "t_color",
        out_color:         gfx::RenderTarget<::gfx::format::Srgba8>      = "o_Color",
        out_depth:         gfx::DepthTarget<::gfx::format::DepthStencil> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }
);




pub struct BasicMeshOnGpu
{
    pub vbuf:         gfx::handle::Buffer<gfx_device_gl::Resources, Vertex>,
    pub color:        [f32; 4], 
    pub slice:        gfx::Slice<gfx_device_gl::Resources>,
    pub pso:          gfx::PipelineState<gfx_device_gl::Resources, 
                                         pipe::Meta>,
    pub sampler:      gfx::handle::Sampler<gfx_device_gl::Resources>
}



impl BasicMeshOnGpu
{
    pub fn new_from_vertices(open_gl:       &glutin_window::OpenGL,
                             factory:       &mut gfx_device_gl::Factory,
                             pipe:          pipe::Init,
                             vertex_buffer: &Vec::<Vertex>,
                             index_buffer:  &Vec::<u16>) -> BasicMeshOnGpu
    {   
        let index_buffer = index_buffer.into_index_buffer(factory);
        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&vertex_buffer, index_buffer);
    
        let mut random = rand::thread_rng();
        let r = random.gen_range(0. .. 1.0);
        let g = random.gen_range(0. .. 1.0);
        let b = random.gen_range(0. .. 1.0);
        let color = [ r, g, b, 1.0 ];

        let sinfo = gfx::texture::SamplerInfo::new(gfx::texture::FilterMethod::Bilinear,
                                                   gfx::texture::WrapMode::Clamp);
        
        let glsl = open_gl.to_glsl();
        let pso = factory.create_pipeline_simple(Shaders::new()
                                                          .set(GLSL::V1_50, include_str!("../assets/shader_150.vert"))
                                                          .get(glsl).unwrap().as_bytes(),
                                                 Shaders::new()
                                                          .set(GLSL::V1_50, include_str!("../assets/shader_150.frag"))
                                                          .get(glsl).unwrap().as_bytes(),
                                                 pipe).unwrap();
    
    
        BasicMeshOnGpu { 
            vbuf: (vbuf),
            slice: (slice),
            color: (color),
            pso: (pso),
            sampler: (factory.create_sampler(sinfo)) 
        }
    }
}


pub struct MeshDesc
{
    pub desc: vecmath::Vector4<f32>,
}

pub trait IntoDesc
{
    fn into_desc(&self) -> MeshDesc;
}


pub trait MeshFactory
{
    fn create_mesh_instance_on_gpu(&self, 
                                   open_gl: &glutin_window::OpenGL,
                                   factory: &mut gfx_device_gl::Factory) -> BasicMeshOnGpu;
}
