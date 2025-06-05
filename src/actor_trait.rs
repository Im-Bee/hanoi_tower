#![allow(dead_code)]
#![allow(nonstandard_style)]

use crate::{
    math,
    base_mesh_trait::MeshDesc
};


pub struct ActorBase<PIPE_DATA, PIPE: gfx::pso::PipelineInit>
{   
    pub mesh_data: PIPE_DATA,
    pub slice:     gfx::Slice<gfx_device_gl::Resources>,
    pub pso:       gfx::PipelineState<gfx_device_gl::Resources, 
                                      <PIPE as gfx::pso::PipelineInit>::Meta>,

    not_worthy_of_update: bool,

    model: vecmath::Matrix4<f32>,

    positon: vecmath::Vector3<f32>,

    scale: vecmath::Vector3<f32>,

    rotation_x: f32,
    rotation_y: f32,
    rotation_z: f32,
    rotation_mat_x: vecmath::Matrix4<f32>,
    rotation_mat_y: vecmath::Matrix4<f32>,
    rotation_mat_z: vecmath::Matrix4<f32>,
}



impl<PIPE_DATA, PIPE: gfx::pso::PipelineInit> ActorBase<PIPE_DATA, PIPE> 
{
    pub fn new(mesh_data: PIPE_DATA,
               slice:     gfx::Slice<gfx_device_gl::Resources>,
               pso:       gfx::PipelineState<gfx_device_gl::Resources, <PIPE as gfx::pso::PipelineInit>::Meta>) -> Self
    {
        ActorBase {
            mesh_data: mesh_data,
            slice: slice,
            pso: pso,
            not_worthy_of_update: false,
            model: (vecmath::mat4_id()),
            positon: ([ 0., 0., 0. ]), 
            scale: ([ 1., 1., 1. ]), 
            rotation_x: (0.0),
            rotation_y: (0.0),
            rotation_z: (0.0),
            rotation_mat_y: (vecmath::mat4_id()), 
            rotation_mat_x: (vecmath::mat4_id()),
            rotation_mat_z: (vecmath::mat4_id()),
        }
    }
}



#[inline]
fn add_angle_with_overflow(a: &mut f32, b: f32)
{
    *a += b;
    if *a >= math::TWO_PI {
        *a -= math::TWO_PI;
    }
}



pub trait Actor
{
    fn initialize(mesh_desc: MeshDesc,
                  open_gl:   &crate::piston_window::OpenGL, 
                  window:    &piston_window::PistonWindow,
                  factory:   &mut gfx_device_gl::Factory) -> Self;



    fn get_model<PIPE_DATA, PIPE: gfx::pso::PipelineInit>(base: &ActorBase<PIPE_DATA, PIPE>) -> vecmath::Matrix4<f32>
    {
        base.model
    }


    fn get_pos<PIPE_DATA, PIPE: gfx::pso::PipelineInit>(base: &ActorBase<PIPE_DATA, PIPE>) -> vecmath::Vector3<f32>
    {
        base.positon
    }



    fn move_position<PIPE_DATA, PIPE: gfx::pso::PipelineInit>(base: &mut ActorBase<PIPE_DATA, PIPE>, 
                                                              pos:  vecmath::Vector3<f32>)  
    {
        base.positon[0] += pos[0];
        base.positon[1] += pos[1];
        base.positon[2] += pos[2];

        base.not_worthy_of_update = false;
    }



    fn set_scale<PIPE_DATA, PIPE: gfx::pso::PipelineInit>(base: &mut ActorBase<PIPE_DATA, PIPE>, 
                                                          scale: vecmath::Vector3<f32>)  
    {
        base.scale[0] = scale[0];
        base.scale[1] = scale[1];
        base.scale[2] = scale[2];

        base.not_worthy_of_update = false;
    }


    fn set_position<PIPE_DATA, PIPE: gfx::pso::PipelineInit>(base: &mut ActorBase<PIPE_DATA, PIPE>, 
                                                             pos: vecmath::Vector3<f32>)  
    {
        base.positon[0] = pos[0];
        base.positon[1] = pos[1];
        base.positon[2] = pos[2];

        base.not_worthy_of_update = false;
    }



    fn rotate_x<PIPE_DATA, PIPE: gfx::pso::PipelineInit>(base: &mut ActorBase<PIPE_DATA, PIPE>, angle: f32)
    {
        add_angle_with_overflow(&mut base.rotation_x, angle);

        base.rotation_mat_x = crate::math::rotation_x(base.rotation_x);

        base.not_worthy_of_update = false;
    }



    fn rotate_y<PIPE_DATA, PIPE: gfx::pso::PipelineInit>(base: &mut ActorBase<PIPE_DATA, PIPE>, angle: f32)
    {
        add_angle_with_overflow(&mut base.rotation_y, angle);

        base.rotation_mat_y = crate::math::rotation_y(base.rotation_y);

        base.not_worthy_of_update = false;
    }



    fn rotate_z<PIPE_DATA, PIPE: gfx::pso::PipelineInit>(base: &mut ActorBase<PIPE_DATA, PIPE>, angle: f32)
    {
        add_angle_with_overflow(&mut base.rotation_z, angle);

        base.rotation_mat_z = crate::math::rotation_z(base.rotation_z);

        base.not_worthy_of_update = false;
    }
 


    fn update_actor_base<PIPE_DATA, PIPE: gfx::pso::PipelineInit>(base: &mut ActorBase<PIPE_DATA, PIPE>)
    {
        if base.not_worthy_of_update {
            return;
        }

        base.model = vecmath::mat4_id();
        math::scale_translation(&mut base.model, base.scale);

        base.model = vecmath::col_mat4_mul(base.model, base.rotation_mat_x);
        base.model = vecmath::col_mat4_mul(base.model, base.rotation_mat_z);
        base.model = vecmath::col_mat4_mul(base.model, base.rotation_mat_y);

        math::pos_translation(&mut base.model, base.positon);

        base.not_worthy_of_update = true;

    }



    fn update(&mut self);


    fn resize(&mut self, window: &mut piston_window::PistonWindow);


    fn render(&mut self,
              window: &mut piston_window::PistonWindow,
              camera: &vecmath::Matrix4<f32>,
              projection: &vecmath::Matrix4<f32>);
}
