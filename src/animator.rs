use std::rc::Rc;
use std::cell::RefCell;
use crate::{
    actor_trait::{Actor, ActorBase},
    donut_actor::ADonut,
};

type Vertex = crate::base_mesh_trait::pipe::Data<gfx_device_gl::Resources>;
type Pipe = crate::base_mesh_trait::pipe::Init<'static>;

pub struct Anmiator
{
    is_in_animation: bool,
    animated: Option<Rc<RefCell<ActorBase<Vertex, Pipe>>>>,
    starting_pos: Option<vecmath::Vector3<f32>>,
    target_pos: Option<vecmath::Vector3<f32>>,
}



impl Anmiator
{
    pub fn new() -> Self
    {
        Anmiator { 
            is_in_animation: (false),
            animated: None,
            starting_pos: None,
            target_pos: None,
        }
    }



    pub fn is_in_animation(&self) -> bool
    {
        self.is_in_animation
    }



    pub fn queue_animation(&mut self, 
                           actor:        Rc<RefCell<ActorBase<Vertex, Pipe>>>,
                           starting_pos: vecmath::Vector3<f32>,
                           target_pos:   vecmath::Vector3<f32>)
    {
        self.animated = Some(actor.clone());
        self.target_pos = Some(target_pos);
        self.starting_pos = Some(starting_pos);
        self.is_in_animation = true;
    }


    fn check_if_on_pos(&self) -> bool
    {
        let pos = <ADonut as Actor>::get_pos(&mut self.animated.clone().unwrap().borrow());
        let target = self.target_pos.unwrap();

        if !(pos[0] - 0.1 < target[0] && target[0] < pos[0] + 0.1) {
            return false
        }

        if !(pos[1] - 0.1 < target[1] && target[1] < pos[1] + 0.1) {
            return false
        }

        if !(pos[2] - 0.1 < target[2] && target[2] < pos[2] + 0.1) {
            return false
        }

        true
    }


    pub fn update(&mut self)
    {
        if !self.is_in_animation || self.target_pos.is_none() || self.animated.is_none() {
            return
        }
    
        let base = self.animated.clone().unwrap();

        let pos_diff = [ (self.target_pos.unwrap()[0] - self.starting_pos.unwrap()[0]) * 0.01,
                         (self.target_pos.unwrap()[1] - self.starting_pos.unwrap()[1]) * 0.01,
                         (self.target_pos.unwrap()[2] - self.starting_pos.unwrap()[2]) * 0.01 ];

        <ADonut as Actor>::move_position(&mut base.borrow_mut(), pos_diff);

        if self.check_if_on_pos() 
        {
            self.is_in_animation = false;
            self.animated = None;
            self.target_pos = None;
        }
    }
}

