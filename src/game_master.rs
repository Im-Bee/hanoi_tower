#![allow(dead_code)]

use piston::{Button, Key};
use vecmath::Matrix4;

use crate::{
    actor_trait::{self, Actor}, 
    donut_actor::ADonut,
    donut_mesh::DonutMeshFactory,
    base_mesh_trait::IntoDesc,
    stick_actor::AStick,
    stick_mesh::StickMeshFactory
};


type Stack<T> = Vec<T>;

pub struct GameMaster 
{
    stack_one: Stack<ADonut>,
    stack_two: Stack<ADonut>,
    stack_three: Stack<ADonut>,

    sticks: Vec::<AStick>,

    button_choice_1: Option<Key>,
    button_choice_2: Option<Key>,

    dounuts_amount: isize,

    playing: bool,
}


fn call_on_stack<F, ACTOR: actor_trait::Actor>(mut f: F, stack: &mut Stack<ACTOR>)
    where F: FnMut(&mut ACTOR)
{
    for mut actor in stack.iter_mut()
    {
        f(&mut actor);
    }
}


fn convert_key_to_i32(key: Key) -> i32
{
    key.code() - '1' as i32
}


const POS_SCALE: f32 = 0.45;
const GROUND_OFFSET: f32 = -6.;
const POS_FAR_STICK: f32 = -12.;
const POS_CLOSE_STICK: f32 = -10.;
const DISTANCE_BETWEEN_STICKS: f32 = 15.;
const DONUT_HEIGHT: f32 = 0.6;


impl GameMaster 
{
    pub fn new() -> Self
    {
        GameMaster {  
            stack_one: Stack::new(),
            stack_two: Stack::new(),
            stack_three: Stack::new(),
    
            sticks: Vec::new(),

            button_choice_1: None,
            button_choice_2: None,

            dounuts_amount: -1,

            playing: true,
        }
    }

    pub fn initialize(&mut self,
                      dounuts_amount: isize,
                      open_gl:        &glutin_window::OpenGL, 
                      window:         &piston_window::PistonWindow,
                      factory:        &mut gfx_device_gl::Factory)
    {
        self.dounuts_amount = dounuts_amount;



        let stick_factory = StickMeshFactory {};

        let mut stick = AStick::initialize(stick_factory.into_desc(), open_gl, window, factory);
        <AStick as Actor>::rotate_x(&mut stick.actor_base, std::f32::consts::PI * 0.5);
        <AStick as Actor>::set_position(&mut stick.actor_base, [ -DISTANCE_BETWEEN_STICKS, 1.0, POS_FAR_STICK ]);
        self.sticks.push(stick);


        let mut stick = AStick::initialize(stick_factory.into_desc(), open_gl, window, factory);
        <AStick as Actor>::rotate_x(&mut stick.actor_base, std::f32::consts::PI * 0.5);
        <AStick as Actor>::set_position(&mut stick.actor_base, [ 0., 1., POS_CLOSE_STICK ]);
        self.sticks.push(stick);

        let mut stick = AStick::initialize(stick_factory.into_desc(), open_gl, window, factory);
        <AStick as Actor>::rotate_x(&mut stick.actor_base, std::f32::consts::PI * 0.5);
        <AStick as Actor>::set_position(&mut stick.actor_base, [ DISTANCE_BETWEEN_STICKS, 1.0, POS_FAR_STICK ]);
        self.sticks.push(stick);



        let max_i = self.dounuts_amount;

        for i in 0..max_i
        {
            let i_diff = max_i - i;

            let donut_factory = DonutMeshFactory {
                major_radius: 0.40 + (i_diff as f32 * 0.2),
                minor_radius: 0.35, 
                segments_major: 28,
                segments_minor: 16,
            };


            let mut donut = ADonut::initialize(donut_factory.into_desc(), open_gl, window, factory);
            let offset = DONUT_HEIGHT * self.stack_one.len() as f32 + GROUND_OFFSET;

            <ADonut as Actor>::rotate_x(&mut donut.actor_base, std::f32::consts::PI * 0.5);
            <ADonut as Actor>::set_position(&mut donut.actor_base, [ -DISTANCE_BETWEEN_STICKS, offset, POS_FAR_STICK ]);
            donut.donut_width = (max_i - i) as i32;

            self.stack_one.push(donut);
        }
    }

    pub fn update(&mut self, button: Option<Button>) 
    {
        if !self.playing {
            return
        }



        if let Some(Button::Keyboard(key)) = button
        {
            match key 
            {
                Key::D1 | Key::D2 | Key::D3 => { self.start_donut_routine(key); },
                _ => ()
            }

            // println!("Pressed keyboard key '{:?}'", key);
        };

        call_on_stack(| actor: &mut ADonut | -> () { actor.update() }, &mut self.stack_one);
        call_on_stack(| actor: &mut ADonut | -> () { actor.update() }, &mut self.stack_two);
        call_on_stack(| actor: &mut ADonut | -> () { actor.update() }, &mut self.stack_three);
        call_on_stack(| actor: &mut AStick | -> () { actor.update() }, &mut self.sticks);




        if self.check_win_condition() {
            self.playing = false;

            println!("You won!");
        }
    }

    pub fn render(&mut self,
                  mut window: &mut piston_window::PistonWindow,
                  camera: &Matrix4<f32>,
                  projection: vecmath::Matrix4<f32>)
    {

        call_on_stack(| actor: &mut ADonut | -> () { actor.render(&mut window, &camera, &projection) },
                      &mut self.stack_one);

        call_on_stack(| actor: &mut ADonut | -> () { actor.render(&mut window, &camera, &projection) },
                      &mut self.stack_two);

        call_on_stack(| actor: &mut ADonut | -> () { actor.render(&mut window, &camera, &projection) }, 
                      &mut self.stack_three);

        call_on_stack(| actor: &mut AStick | -> () { actor.render(&mut window, &camera, &projection) }, 
                      &mut self.sticks);
    }

    pub fn resize(&mut self, mut window: &mut piston_window::PistonWindow)
    {
        call_on_stack(| actor: &mut ADonut | -> () { actor.resize(&mut window) }, 
                      &mut self.stack_one);
        
        call_on_stack(| actor: &mut ADonut | -> () { actor.resize(&mut window) }, 
                      &mut self.stack_two);

        call_on_stack(| actor: &mut ADonut | -> () { actor.resize(&mut window) }, 
                      &mut self.stack_three);

        call_on_stack(| actor: &mut AStick | -> () { actor.resize(&mut window) }, 
                      &mut self.sticks);
    }


    fn flush_choices(&mut self) 
    {
        self.button_choice_1 = None;
        self.button_choice_2 = None;
    }

        
    fn check_win_condition(&mut self) -> bool
    {
        let third_stack = self.get_stack(2);
    

        if third_stack.len() >= self.dounuts_amount as usize {
            return true
        }

        false
    }
    

    fn check_if_move_possible(&mut self) -> bool
    {
        let index_a = convert_key_to_i32(self.button_choice_1.unwrap());
        if self.get_stack(index_a).is_empty() {
            return false
        }

        let index_b = convert_key_to_i32(self.button_choice_2.unwrap());
        if self.get_stack(index_b).is_empty() {
            return true
        }

        if self.get_stack(index_a).last().unwrap().donut_width > self.get_stack(index_b).last().unwrap().donut_width {
            return false
        }

        true
    }

    
    fn get_stack(&mut self, index: i32) -> &mut Stack<ADonut>
    {
        match index
        {
            0 => &mut self.stack_one,
            1 => &mut self.stack_two,
            2 => &mut self.stack_three,
            _ => panic!("Index outside of bounds"),
        }
    }

    
    fn finish_donut_routinge(&mut self) 
    {
        let index_a = convert_key_to_i32(self.button_choice_1.unwrap());
        let index_b = convert_key_to_i32(self.button_choice_2.unwrap());
        let distance = DISTANCE_BETWEEN_STICKS * index_b as f32 - DISTANCE_BETWEEN_STICKS;

        let mut donut = self.get_stack(index_a).pop().unwrap();
        let stack = self.get_stack(index_b);
        let mut z = POS_FAR_STICK;

        if index_b == 1 {
            z = POS_CLOSE_STICK;
        }
    
        <ADonut as Actor>::set_position(&mut donut.actor_base, [ distance,
                                                                 DONUT_HEIGHT * stack.len() as f32 + GROUND_OFFSET, 
                                                                 z ]);

        stack.push(donut);
    }

    
    fn start_donut_routine(&mut self, button: Key)
    {
        if self.button_choice_1.is_none() {
            self.button_choice_1 = Some(button);
        } else {
            self.button_choice_2 = Some(button);
        }

        if self.button_choice_2.is_none() {
            return;
        }

        println!("We want to move the donut from {:?} to {:?}", 
                 self.button_choice_1.unwrap(),
                 self.button_choice_2.unwrap());

        if !self.check_if_move_possible() {
            self.flush_choices();
            println!("The move is invalid."); 
            return;
        }

        self.finish_donut_routinge();
        self.flush_choices();
    }
}


