use crate::{actor_trait::ActorBase, mesh_trait::{MeshDesc, MeshFactory}, stick_mesh::*};
use camera_controllers::{
    model_view_projection
};



extern crate piston_window;
extern crate vecmath;
extern crate camera_controllers;
extern crate gfx;
extern crate shader_version;



type Vertex = pipe::Data<gfx_device_gl::Resources>;
type Pipe = pipe::Init<'static>;

pub struct AStick
{
    pub actor_base: crate::actor_trait::ActorBase<Vertex, Pipe>,
}





impl crate::actor_trait::Actor for AStick
{
    fn initialize(_mesh_desc: MeshDesc,
                  open_gl:   &glutin_window::OpenGL, 
                  window:    &piston_window::PistonWindow,
                  factory:   &mut gfx_device_gl::Factory) -> Self
    {
        let stick_mesh = StickMeshFactory::new();

        let mesh = stick_mesh.create_mesh_instance_on_gpu(open_gl, factory);
        let mesh_data = pipe::Data {
            vbuf: mesh.vbuf,
            u_model_view_proj: vecmath::mat4_id(),
            t_color: (mesh.texture_view, mesh.sampler),
            out_color: window.output_color.clone(),
            out_depth: window.output_stencil.clone(),
        };

        AStick { 
            actor_base: (ActorBase::new(mesh_data, mesh.slice, mesh.pso)),
        }
    }



    fn resize(&mut self, window: &mut piston_window::PistonWindow)
    {
        self.actor_base.mesh_data.out_depth = window.output_stencil.clone();
        self.actor_base.mesh_data.out_color = window.output_color.clone();
    }
 


    fn update(&mut self)
    {
        Self::update_actor_base(&mut self.actor_base);
    }



    fn render(&mut self,
              window:     &mut piston_window::PistonWindow,
              camera:     &vecmath::Matrix4<f32>,
              projection: &vecmath::Matrix4<f32>) 
    {
        self.actor_base.mesh_data.u_model_view_proj = model_view_projection(AStick::get_model(&self.actor_base),
                                                                            *camera,
                                                                            *projection);

        window.encoder.draw(&self.actor_base.slice, &self.actor_base.pso, &self.actor_base.mesh_data);
    }
}

