use std::rc::Rc;
use std::cell::RefCell;
use crate::{actor_trait::ActorBase, base_mesh_trait::{MeshDesc, MeshFactory}, stick_mesh::*};
use camera_controllers::{
    model_view_projection
};



extern crate piston_window;
extern crate vecmath;
extern crate camera_controllers;
extern crate gfx;
extern crate shader_version;



type Vertex = crate::base_mesh_trait::pipe::Data<gfx_device_gl::Resources>;
type Pipe = crate::base_mesh_trait::pipe::Init<'static>;

pub struct AStick
{
    pub actor_base: Rc<RefCell<ActorBase<Vertex, Pipe>>>,
}





impl crate::actor_trait::Actor for AStick
{
    fn initialize(_mesh_desc: MeshDesc,
                  open_gl:   &crate::piston_window::OpenGL, 
                  window:    &piston_window::PistonWindow,
                  factory:   &mut gfx_device_gl::Factory) -> Self
    {
        let stick_mesh = StickMeshFactory::new();

        let mesh = stick_mesh.create_mesh_instance_on_gpu(open_gl, factory);
        let mesh_data = crate::base_mesh_trait::pipe::Data {
            vbuf: mesh.vbuf,
            u_model_view_proj: vecmath::mat4_id(),
            t_color: mesh.color,
            out_color: window.output_color.clone(),
            out_depth: window.output_stencil.clone(),
        };

        AStick { 
            actor_base: (Rc::new(RefCell::new(ActorBase::new(mesh_data, mesh.slice, mesh.pso)))),
        }
    }



    fn resize(&mut self, window: &mut piston_window::PistonWindow)
    {
        self.actor_base.borrow_mut().mesh_data.out_depth = window.output_stencil.clone();
        self.actor_base.borrow_mut().mesh_data.out_color = window.output_color.clone();
    }
 


    fn update(&mut self)
    {
        Self::update_actor_base(&mut self.actor_base.borrow_mut());
    }



    fn render(&mut self,
              window:     &mut piston_window::PistonWindow,
              camera:     &vecmath::Matrix4<f32>,
              projection: &vecmath::Matrix4<f32>) 
    {
        let model = AStick::get_model(&self.actor_base.borrow());
        self.actor_base.borrow_mut().mesh_data.u_model_view_proj = model_view_projection(model,
                                                                                         *camera,
                                                                                         *projection);

        window.encoder.draw(&self.actor_base.borrow().slice,
                            &self.actor_base.borrow().pso,
                            &self.actor_base.borrow().mesh_data);
    }
}

