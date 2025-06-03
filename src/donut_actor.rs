use crate::{actor_trait::ActorBase, donut_mesh::DonutMeshFactory, mesh_trait::{MeshDesc, MeshFactory}};
use camera_controllers::{
    model_view_projection
};
use vecmath::Matrix4;



extern crate piston_window;
extern crate vecmath;
extern crate camera_controllers;
extern crate gfx;
extern crate shader_version;







type Vertex = crate::donut_mesh::pipe::Data<gfx_device_gl::Resources>;
type Pipe = crate::donut_mesh::pipe::Init<'static>;

pub struct ADonut
{
    pub actor_base: crate::actor_trait::ActorBase<Vertex, Pipe>,
    pub donut_width: i32,
}





impl crate::actor_trait::Actor for ADonut
{
    fn initialize(mesh_desc: MeshDesc,
                  open_gl:   &glutin_window::OpenGL, 
                  window:    &piston_window::PistonWindow,
                  factory:   &mut gfx_device_gl::Factory) -> Self
    {
        let donut_mesh = DonutMeshFactory::new(mesh_desc.desc[0], mesh_desc.desc[1]);

        let mesh = donut_mesh.create_mesh_instance_on_gpu(open_gl, factory);
        let mesh_data = crate::donut_mesh::pipe::Data {
            vbuf: mesh.vbuf,
            u_model_view_proj: vecmath::mat4_id(),
            t_color: (mesh.texture_view, mesh.sampler),
            out_color: window.output_color.clone(),
            out_depth: window.output_stencil.clone(),
        };
        
        ADonut { 
            actor_base: (ActorBase::new(mesh_data, mesh.slice, mesh.pso)),
            donut_width: 0,
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
              camera:     &Matrix4<f32>,
              projection: &vecmath::Matrix4<f32>) 
    {
        self.actor_base.mesh_data.u_model_view_proj = model_view_projection(ADonut::get_model(&self.actor_base),
                                                                            *camera,
                                                                            *projection);

        window.encoder.draw(&self.actor_base.slice, &self.actor_base.pso, &self.actor_base.mesh_data);
    }
}
