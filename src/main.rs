use camera_controllers::{FirstPerson, FirstPersonSettings};
mod base_mesh_trait;
mod actor_trait;
mod donut_mesh;
mod donut_actor;
mod stick_mesh;
mod stick_actor;
mod math;
mod game_master;
mod animator;

extern crate piston_window;
extern crate vecmath;
extern crate camera_controllers;
#[macro_use]
extern crate gfx;
extern crate shader_version;

#[cfg(feature = "include_sdl2")]
extern crate sdl2_window;
#[cfg(feature = "include_glfw")]
extern crate glfw_window;
#[cfg(feature = "include_glutin")]
extern crate glutin_window;

#[cfg(feature = "include_sdl2")]
use sdl2_window::Sdl2Window as AppWindow;
#[cfg(feature = "include_glfw")]
use glfw_window::GlfwWindow as AppWindow;
#[cfg(feature = "include_glutin")]
use glutin_window::GlutinWindow as AppWindow;

use opengl_graphics::{ GlGraphics, Texture, TextureSettings};
use graphics::{Context, Graphics, ImageSize};

fn main() 
{
    use piston_window::*;
    use camera_controllers::CameraPerspective;

    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow = WindowSettings::new("Hanoi", [640, 480])
                                                  .exit_on_esc(true)
                                                  .samples(4)
                                                  .graphics_api(opengl)
                                                  .build()
                                                  .unwrap();
    window.set_capture_cursor(false);

    let mut factory = window.factory.clone();

    let get_projection = |w: &PistonWindow| {
        let draw_size = w.window.draw_size();
        CameraPerspective {
            fov: 60.0, 
            near_clip: 0.1,
            far_clip: 1000.0,
            aspect_ratio: (draw_size.width as f32) / (draw_size.height as f32)
        }.projection()
    };

    let mut projection = get_projection(&window);


    let donuts_amount = 5;
    let mut game_master = game_master::GameMaster::new();
    game_master.initialize(donuts_amount, &opengl, &window, &mut factory);


    let mut first_person_camera = FirstPerson::new(
        [0.5, 10.5, 21.0],
        FirstPersonSettings::keyboard_wasd()
    );
    first_person_camera.pitch = 0.25;

    while let Some(e) = window.next() 
    {
        // first_person.event(&e);

        game_master.update(e.press_args());
        
        window.draw_3d(&e, | window | {
            let args = e.render_args().clone().unwrap();

            window.encoder.clear(&window.output_color, [0.0, 0.0, 0.0, 1.0]);
            window.encoder.clear_depth(&window.output_stencil, 1.0);


            game_master.render(window, &first_person_camera.camera(args.ext_dt).orthogonal(), projection);

        });

        if e.resize_args().is_some() 
        {
            projection = get_projection(&window);
            game_master.resize(&mut window);
        }
    }
}

