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
use shader_version::Shaders;






pub struct MeshOnGpu<VERTEX, PIPE: gfx::pso::PipelineInit >
{
    pub vbuf:         gfx::handle::Buffer<gfx_device_gl::Resources, VERTEX>,
    pub texture_view: gfx::handle::ShaderResourceView<gfx_device_gl::Resources, [f32; 4]>, 
    pub slice:        gfx::Slice<gfx_device_gl::Resources>,
    pub pso:          gfx::PipelineState<gfx_device_gl::Resources, 
                                         <PIPE as gfx::pso::PipelineInit>::Meta>,
    pub sampler:      gfx::handle::Sampler<gfx_device_gl::Resources>
}



impl<VERTEX, PIPE: gfx::pso::PipelineInit> MeshOnGpu<VERTEX, PIPE>
{
    pub fn new_from_vertices(open_gl:       &glutin_window::OpenGL,
                             factory:       &mut gfx_device_gl::Factory,
                             pipe:          PIPE,
                             vertex_buffer: &Vec::<VERTEX>,
                             index_buffer:  &Vec::<u16>) -> MeshOnGpu<VERTEX, PIPE>

        where  VERTEX: gfx::pso::buffer::Structure<gfx::format::Format> + gfx::traits::Pod,
               PIPE: gfx::pso::PipelineInit 
               
    {   
        let index_buffer = index_buffer.into_index_buffer(factory);
        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&vertex_buffer, index_buffer);
        let fake_texels = [[0xff, 0xff, 0xff, 0x00],
                           [0xff, 0x00, 0x00, 0x00],
                           [0x00, 0xff, 0x00, 0x00],
                           [0x00, 0x00, 0xff, 0x00]];
    
        let (_, texture_view) = factory.create_texture_immutable::<gfx::format::Rgba8>(
                                                                  gfx::texture::Kind::D2(2, 
                                                                                         2,
                                                                                         gfx::texture::AaMode::Single),
                                                                  gfx::texture::Mipmap::Provided,
                                                                  &[&fake_texels]).unwrap();

        let sinfo = gfx::texture::SamplerInfo::new(gfx::texture::FilterMethod::Bilinear,
                                                   gfx::texture::WrapMode::Clamp);
        
        let glsl = open_gl.to_glsl();
        let pso = factory.create_pipeline_simple(Shaders::new()
                                                          .set(GLSL::V1_20, include_str!("../assets/cube_120.glslv"))
                                                          .set(GLSL::V1_50, include_str!("../assets/cube_150.glslv"))
                                                          .get(glsl).unwrap().as_bytes(),
                                                 Shaders::new()
                                                          .set(GLSL::V1_20, include_str!("../assets/cube_120.glslf"))
                                                          .set(GLSL::V1_50, include_str!("../assets/cube_150.glslf"))
                                                          .get(glsl).unwrap().as_bytes(),
                                                 pipe).unwrap();
    
    
        MeshOnGpu { 
            vbuf: (vbuf),
            slice: (slice),
            texture_view: (texture_view),
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


pub trait MeshFactory<VERTEX, PIPE: gfx::pso::PipelineInit >
{
    fn create_mesh_instance_on_gpu(&self, 
                                   open_gl: &glutin_window::OpenGL,
                                   factory: &mut gfx_device_gl::Factory) -> crate::mesh_trait::MeshOnGpu<VERTEX, PIPE>;
}
