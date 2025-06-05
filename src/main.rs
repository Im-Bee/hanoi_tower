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
extern crate freetype as ft;
#[macro_use]
extern crate gfx;
extern crate shader_version;


#[cfg(feature = "include_glfw")]
use opengl_graphics::{ GlGraphics, Texture, TextureSettings};
use graphics::{Context, Graphics, ImageSize};

fn glyphs(face: &mut ft::Face, text: &str) -> Vec<(Texture, [f64; 2])> 
{
    let mut x = 10;
    let mut y = 0;
    let mut res = vec![];
    for ch in text.chars() {
        face.load_char(ch as usize, ft::face::LoadFlag::RENDER).unwrap();
        let g = face.glyph();

        let bitmap = g.bitmap();
        let texture = Texture::from_memory_alpha(
            bitmap.buffer(),
            bitmap.width() as u32,
            bitmap.rows() as u32,
            &TextureSettings::new()
        ).unwrap();
        res.push((texture, [(x + g.bitmap_left()) as f64, (y - g.bitmap_top()) as f64]));

        x += (g.advance().x >> 6) as i32;
        y += (g.advance().y >> 6) as i32;
    }
    res
}


fn render_text<G, T>(glyphs: &[(T, [f64; 2])], c: &Context, gl: &mut G)
    where G: Graphics<Texture = T>, T: ImageSize
{
    for &(ref texture, [x, y]) in glyphs {
        use graphics::*;

        Image::new_color(color::BLACK).draw(
            texture,
            &c.draw_state,
            c.transform.trans(x, y),
            gl
        );
    }
}


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
    window.set_capture_cursor(true);

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

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let freetype = ft::Library::init().unwrap();
    let font = assets.join("FiraSans-Regular.ttf");
    let mut face = freetype.new_face(&font, 0).unwrap();
    face.set_pixel_sizes(0, 48).unwrap();
    let ref mut gl = GlGraphics::new(opengl);

    let glyphs = glyphs(&mut face, "You won!");

    while let Some(e) = window.next() 
    {
        // first_person.event(&e);

        game_master.update(e.press_args());
        
        if !game_master.check_win_condition() 
        {
            window.draw_3d(&e, | window | {
                let args = e.render_args().clone().unwrap();

                window.encoder.clear(&window.output_color, [0.0, 0.0, 0.0, 1.0]);
                window.encoder.clear_depth(&window.output_stencil, 1.0);


                game_master.render(window, &first_person_camera.camera(args.ext_dt).orthogonal(), projection);

            });
        }
        else if let Some(args) = e.render_args() 
        {
            gl.draw(args.viewport(), |c, gl| {
                clear(color::WHITE, gl);
                let size = window.size();
                render_text(&glyphs, &c.trans(size.width / 2.5, size.height / 2.1), gl);
            });
        }

        if e.resize_args().is_some() 
        {
            projection = get_projection(&window);
            game_master.resize(&mut window);
        }
    }
}

