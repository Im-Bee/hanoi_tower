use std::rc::Rc;
use std::cell::RefCell;
use crate::{actor_trait::ActorBase, donut_mesh::DonutMeshFactory, base_mesh_trait::{MeshDesc, MeshFactory}};
use camera_controllers::{
    model_view_projection
};
use vecmath::Matrix4;



extern crate piston_window;
extern crate vecmath;
extern crate camera_controllers;
extern crate gfx;
extern crate shader_version;







type Vertex = crate::base_mesh_trait::pipe::Data<gfx_device_gl::Resources>;
type Pipe = crate::base_mesh_trait::pipe::Init<'static>;

pub struct ADonut
{
    pub actor_base: Rc<RefCell<ActorBase<Vertex, Pipe>>>,
    pub donut_width: i32,
}





impl crate::actor_trait::Actor for ADonut
{
    fn initialize(mesh_desc: MeshDesc,
                  open_gl:   &crate::piston_window::OpenGL, 
                  window:    &piston_window::PistonWindow,
                  factory:   &mut gfx_device_gl::Factory) -> Self
    {
        let donut_mesh = DonutMeshFactory::new(mesh_desc.desc[0], mesh_desc.desc[1]);

        let mesh = donut_mesh.create_mesh_instance_on_gpu(open_gl, factory);
        let mesh_data = crate::base_mesh_trait::pipe::Data {
            vbuf: mesh.vbuf,
            u_model_view_proj: vecmath::mat4_id(),
            t_color: mesh.color,
            out_color: window.output_color.clone(),
            out_depth: window.output_stencil.clone(),
        };
        
        ADonut { 
            actor_base: (Rc::new(RefCell::new(ActorBase::new(mesh_data, mesh.slice, mesh.pso)))),
            donut_width: 0,
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
              camera:     &Matrix4<f32>,
              projection: &vecmath::Matrix4<f32>) 
    {
        let model = ADonut::get_model(&self.actor_base.borrow());
        self.actor_base.borrow_mut().mesh_data.u_model_view_proj = model_view_projection(model,
                                                                                         *camera,
                                                                                         *projection);

        window.encoder.draw(&self.actor_base.borrow().slice,
                            &self.actor_base.borrow().pso,
                            &self.actor_base.borrow().mesh_data);
    }
}
