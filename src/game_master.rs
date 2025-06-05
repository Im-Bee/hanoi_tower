#![allow(dead_code)]

use std::{collections::VecDeque, isize, usize};

use piston::{Button, Key};
use vecmath::Matrix4;

use crate::{
    actor_trait::{self, Actor},
    animator::Anmiator,
    base_mesh_trait::IntoDesc,
    donut_actor::ADonut,
    donut_mesh::DonutMeshFactory,
    stick_actor::AStick,
    stick_mesh::StickMeshFactory
};


type Stack<T> = Vec<T>;

pub struct GameMaster
{
    animator: Anmiator,

    stack_one: Stack<ADonut>,
    stack_two: Stack<ADonut>,
    stack_three: Stack<ADonut>,

    sticks: Vec::<AStick>,

    button_choice_1: Option<Key>,
    button_choice_2: Option<Key>,

    dounuts_amount: i32,

    playing: bool,

    auto_gamer_moves: VecDeque<AutoGameMove>,
    auto_gamer: bool,
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


struct AutoGameMove 
{
    from: u32,
    to: u32,
}



impl GameMaster
{
    pub fn new() -> Self
    {
        GameMaster {  
            animator: Anmiator::new(),

            stack_one: Stack::new(),
            stack_two: Stack::new(),
            stack_three: Stack::new(),
    
            sticks: Vec::new(),

            button_choice_1: None,
            button_choice_2: None,

            dounuts_amount: -1,

            playing: true,


            auto_gamer_moves: VecDeque::new(),
            auto_gamer: false,
        }
    }


   
    pub fn initialize(&mut self,
                      dounuts_amount: i32,
                      open_gl:        &glutin_window::OpenGL, 
                      window:         &piston_window::PistonWindow,
                      factory:        &mut gfx_device_gl::Factory)
    {
        self.dounuts_amount = dounuts_amount;



        let stick_factory = StickMeshFactory {};

        let stick = AStick::initialize(stick_factory.into_desc(), open_gl, window, factory);
        <AStick as Actor>::rotate_x(&mut stick.actor_base.borrow_mut(), std::f32::consts::PI * 0.5);
        <AStick as Actor>::set_position(&mut stick.actor_base.borrow_mut(), 
                                        [ -DISTANCE_BETWEEN_STICKS, 1.0, POS_FAR_STICK ]);

        self.sticks.push(stick);


        let stick = AStick::initialize(stick_factory.into_desc(), open_gl, window, factory);
        <AStick as Actor>::rotate_x(&mut stick.actor_base.borrow_mut(), std::f32::consts::PI * 0.5);
        <AStick as Actor>::set_position(&mut stick.actor_base.borrow_mut(),
                                        [ 0., 1., POS_CLOSE_STICK ]);

        self.sticks.push(stick);

        let stick = AStick::initialize(stick_factory.into_desc(), open_gl, window, factory);
        <AStick as Actor>::rotate_x(&mut stick.actor_base.borrow_mut(), std::f32::consts::PI * 0.5);
        <AStick as Actor>::set_position(&mut stick.actor_base.borrow_mut(),
                                        [ DISTANCE_BETWEEN_STICKS, 1.0, POS_FAR_STICK ]);

        self.sticks.push(stick);


        self.generate_donuts(open_gl, window, factory); 
    }

    pub fn update(&mut self, button: Option<Button>) 
    {
        if !self.playing {
            return
        }


        call_on_stack(| actor: &mut ADonut | -> () { actor.update() }, &mut self.stack_one);
        call_on_stack(| actor: &mut ADonut | -> () { actor.update() }, &mut self.stack_two);
        call_on_stack(| actor: &mut ADonut | -> () { actor.update() }, &mut self.stack_three);
        call_on_stack(| actor: &mut AStick | -> () { actor.update() }, &mut self.sticks);
    
        if self.animator.is_in_animation() {
            self.animator.update();
            return;
        }

        if self.check_win_condition() {
            self.playing = false;

            println!("You won!");
        }

        if self.auto_gamer {
            self.auto_game();
            return;
        }

        if let Some(Button::Keyboard(key)) = button
        {
            match key 
            {
                Key::D1 | Key::D2 | Key::D3 => { self.start_donut_routine(key); },
                Key::A => {



                    self.auto_gamer = true;
                },
                _ => ()
            }

            // println!("Pressed keyboard key '{:?}'", key);
        };

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

 

    pub fn check_win_condition(&mut self) -> bool
    {
        let third_stack = self.get_stack(2);
    

        if third_stack.len() >= self.dounuts_amount as usize {
            return true
        }

        false
    }



    fn generate_donuts(&mut self, 
                       open_gl: &glutin_window::OpenGL, 
                       window:  &piston_window::PistonWindow,
                       factory: &mut gfx_device_gl::Factory)
    {
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

            <ADonut as Actor>::rotate_x(&mut donut.actor_base.borrow_mut(), std::f32::consts::PI * 0.5);
            <ADonut as Actor>::set_position(&mut donut.actor_base.borrow_mut(), 
                                            [ -DISTANCE_BETWEEN_STICKS, offset, POS_FAR_STICK ]);
            donut.donut_width = (max_i - i) as i32;

            self.stack_one.push(donut);
        }
    }



    fn flush_choices(&mut self) 
    {
        self.button_choice_1 = None;
        self.button_choice_2 = None;
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

        let donut = self.get_stack(index_a).pop().unwrap();
        let stack_len = self.get_stack(index_b).len();
        let mut z = POS_FAR_STICK;

        if index_b == 1 {
            z = POS_CLOSE_STICK;
        }

        let starting_pos = <ADonut as Actor>::get_pos(&donut.actor_base.borrow());
    
        
        self.animator.queue_animation(donut.actor_base.clone(), 
                                      starting_pos,
                                      [ distance,                                          
                                        DONUT_HEIGHT * stack_len as f32 + GROUND_OFFSET, 
                                        z ]);


        self.get_stack(index_b).push(donut);
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



    fn generate_auto_gamer_moves(&mut self)
    {
        generate_moves(&mut self.auto_gamer_moves, self.dounuts_amount, 0, 2, 1);
    }



    fn auto_game(&mut self) 
    {
        if self.auto_gamer_moves.is_empty() {
            self.generate_auto_gamer_moves();
        }

        let game_move = self.auto_gamer_moves.pop_front().unwrap();
        self.button_choice_1 = Some(Key::from('1' as u32 + game_move.from));
        self.start_donut_routine(Key::from('1' as u32 + game_move.to));
    }
}

fn generate_moves(target: &mut VecDeque<AutoGameMove>, n: i32, from: u32, to: u32, aux: u32)
{
    if n == 0 {
        return;
    }

    generate_moves(target, n - 1, from, aux, to);
    target.push_back(AutoGameMove { from: (from), to: (to) });
    generate_moves(target, n - 1, aux, to, from);
}
