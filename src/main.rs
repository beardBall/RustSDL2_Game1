pub mod player;
pub mod helpers;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::pixels::Color;

use crate::helpers::zz::*;
//use crate::player::Player;


fn first(){
	println!("First function");
}

fn get_age() -> i32{
 return 37;
}



    fn main() ->Result<(), String> {

	println!("Hello!");
	let ws = "Waqar";
	let num:i32 =-30000;
	println!("The number is: {} ", num);
	println!("The number");
	println!("{}", ws);

	let x = 32;
	println!("just x not defined: {}",x);
let screen_width = 800;
let screen_height = 600;

let sdl_context = sdl2::init()?;
let video_subsystem = sdl_context.video()?;
let win = video_subsystem.window("Rust win",screen_width,screen_height).build().unwrap();

let mut canvas = win.into_canvas()
.build().unwrap();

let mut running = true;
let mut event_queue = sdl_context.event_pump().unwrap();

let screen_area = Rect::new(0,0,screen_width, screen_height);

let screen_color = Color::RGB(64,192,255);
canvas.set_draw_color(screen_color);

while running {
    for event in event_queue.poll_iter(){
        
        match event{
            Event::Quit {..} =>{
                running = false;
            },

            Event::MouseMotion{
                x,y,xrel,yrel, .. } =>{
                    println!("Mouse x: {}, y:{}", x,y);
                    println!("Mouse Relative xrel: {}, yrel:{}", xrel,yrel);

                },
                _ => {}
            }
        }
     canvas.fill_rect(screen_area);
    canvas.present();
}
 
   








	first();
	println!("The age returned is: {}", get_age());

	let p1 = player::Player {
	x:15,y:15, 
	speed_x:1, speed_y:1,
	health:50.0};


	println!("Player x:{0}", p1.x);

    let my_name = helpers::zz::get_full_name("Waqar", "Shujrah");
    println!("Name: {my_name}");
    println!("The Name is :{0}", my_name);

    echo("Something");
    Ok(())
}
