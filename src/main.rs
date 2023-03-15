extern crate sdl2;

use std::path::Path;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::surface::Surface;
use sdl2::ttf::Font;
use std::time::Duration;
use sdl2::rect::Rect;
use sdl2::keyboard::Scancode;



fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let width = 800;
    let height = 600;

    let window = video_subsystem.window("pong", width, height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).expect("err has happened");
    let mut font = ttf_context.load_font(Path::new("./assets/fonts/elnath/ELNATH.ttf"), 100).expect("could not open ttf file");
    font.set_style(sdl2::ttf::FontStyle::BOLD);
    
    let mut surface_score_1 = text_to_surface(&font, "0".to_string());
    let mut surface_score_2 = text_to_surface(&font, "0".to_string());

    let mut texture_score_1 = texture_creator
    .create_texture_from_surface(&surface_score_1)
    .map_err(|e| e.to_string()).expect("err");
    let mut texture_score_2 =  texture_creator
        .create_texture_from_surface(&surface_score_2)
        .map_err(|e| e.to_string()).expect("err");
    
    let score_1_rect = Rect::new(60, 60, 150, 150);
    let score_2_rect = Rect::new(580, 60, 150, 150);

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let ball_x = 390;
    let ball_y = 295;
    let ball_speed = 15;
    let ball_size = 10;
    let mut ball = Ball{
            rect : Rect::new(ball_x, ball_y, ball_size, ball_size), 
            speed: SpeedVector{x:ball_speed, y:0},
            collision: Option::None
        };    
    
    let mut p1 = Rect::new(5, 250, 10, 80);
    let mut p2 = Rect::new(785, 250, 10, 80);
    let mut score_p1 = 0;
    let mut score_p2 = 0;

    let paddle_speed = 30;

    let midline = construct_midline(width, height);


    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown{ keycode: Some(Keycode::R), ..} => {
                    ball.reset(ball_x, ball_y, ball_speed);

                }, 
                _ => {}
            }
        }

        canvas.copy(&texture_score_1, None, Some(score_1_rect)).expect("err");
        canvas.copy(&texture_score_2, None, Some(score_2_rect)).expect("err");

        if event_pump.keyboard_state().is_scancode_pressed(Scancode::W){
            p1.set_y(p1.top()-&paddle_speed);
        }
        if event_pump.keyboard_state().is_scancode_pressed(Scancode::S){
            p1.set_y(p1.top()+&paddle_speed);
        }
        if event_pump.keyboard_state().is_scancode_pressed(Scancode::Up){
            p2.set_y(p2.top()-&paddle_speed);
        }
        if event_pump.keyboard_state().is_scancode_pressed(Scancode::Down){
            p2.set_y(p2.top()+&paddle_speed);
        }

        if ball.rect.x() <= 0{
            ball.reset(ball_x, ball_y, ball_speed);
            score_p1 += 1;
            println!("player 1: {}", score_p1);
        }
        if ball.rect.x() >= width as i32{
            ball.reset(ball_x, ball_y, -ball_speed);
            score_p2 += 1;
            println!("player 2: {}", score_p2);
        }

        surface_score_1 = text_to_surface(&font, score_p1.to_string());
        surface_score_2 = text_to_surface(&font, score_p2.to_string());

        texture_score_1 = texture_creator
            .create_texture_from_surface(&surface_score_1)
            .map_err(|e| e.to_string()).expect("err");
        texture_score_2 = texture_creator
            .create_texture_from_surface(&surface_score_2)
            .map_err(|e| e.to_string()).expect("err");

        canvas.copy(&texture_score_1, None, Some(score_1_rect)).expect("err");
        canvas.copy(&texture_score_2, None, Some(score_2_rect)).expect("err");
        

        p1.set_y(clamp(p1.y(), 0, 520).expect("something went wrong!"));
        p2.set_y(clamp(p2.y(), 0, 520).expect("something went wrong!"));

        ball.rect.set_x(ball.rect.x() + ball.speed.x);
        ball.rect.set_y(ball.rect.y() + ball.speed.y);

        ball.collision = ball.rect.check_collision(p1, p2);
        if ball.collision != Option::None{
            //println!("{}", ball.collision.unwrap());
            ball.speed.elastic_collision(1, ball.collision.unwrap());
        }


        if ball.rect.y() <= 0 || ball.rect.y() >= 590{
            ball.speed.elastic_collision(2, 0.);
        }
        
        canvas.set_draw_color(Color::RGB(200, 200, 200));
        canvas.fill_rects(&midline).expect("could not draw midline");
        
        
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.fill_rect(ball.rect).expect("Failed to draw ball");
        canvas.fill_rect(p1).expect("Failed to draw p1");
        canvas.fill_rect(p2).expect("Failed to draw p2");

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
}


struct SpeedVector{
    x: i32,
    y: i32
}
impl SpeedVector{
    fn elastic_collision(&mut self, orientation:i32, val:f64){
        let x = self.x as f64;
        let y = self.y as f64;
        let angle = val*3.141593625;
        let temp = match orientation{
            1 => Self { 
                x: -self.x,
                y: (x*angle.cos() + y*angle.sin()) as i32 
            }, 
            2 => Self { x: self.x, y: -self.y },
            _ => Self { x: self.x, y: self.y }
        };
        self.x = temp.x;
        self.y = temp.y;
    }
}

trait Collide{
    fn check_collision(&self, rect_left: Rect, rect_right: Rect)->Option<f64>;
}

impl Collide for Rect{
    fn check_collision(&self, rect_left: Rect, rect_right: Rect)->Option<f64> {
        let self_left = self.left();
        let self_right = self.right();
        let self_top = self.top();
        let self_bottom = self.bottom();
        if self_left <= rect_left.right() && self_left >= rect_left.left(){
            if self_bottom >= rect_left.top() && self_top <= rect_left.bottom(){
                let mid = (rect_left.top() + rect_left.bottom()) as f64 / 2.;
                let res = ((self.y()as f64 - (self.height() as f64/2.))/mid) - 2.;
                return Option::Some(res);
            }
            return Option::None;

        }else if self_right >= rect_right.left() && self_right <= rect_right.right(){
            if self_bottom >= rect_right.top() && self_top <= rect_right.bottom(){
                let mid = (rect_right.top() + rect_right.bottom()) as f64 / 2.;
                let res = ((self.y()as f64 - (self.height() as f64/2.))/mid) - 2.;
                return Option::Some(res);
            }
            return Option::None;
        }else{
            return Option::None;
        }
    }
}

fn clamp(input:i32, min: i32, max: i32) -> Result<i32, ()>{
    if std::cmp::min(input, max) == std::cmp::max(input, min){
        return Result::Ok(input);
    }
    else if std::cmp::min(input, max) == input{
        return Result::Ok(min);
    }
    else if std::cmp::max(input, min) == input{
        return Result::Ok(max);
    }
    else{
        return Result::Err(());
    }
}

struct Ball{
    rect: Rect,
    speed: SpeedVector,
    collision: Option<f64>
}

impl Ball{
    fn reset(&mut self, x: i32, y:i32, speed: i32){
        self.rect.set_x(x);
        self.rect.set_y(y);
        self.speed.x = speed;
        self.speed.y = 0;
    }
}

fn construct_midline(width: u32, height: u32)->Vec<Rect>{
    let size = 10;
    let x = width / 2 - size / 2;
    let mut out = Vec::new();
    for y in (0..=height).step_by(size.try_into().expect("bop")){
        if y/size % 2 != 0{
            out.push(Rect::new(x.try_into().expect("nop"), y.try_into().expect("clop"), size, size));
        }
    }
    out

}

fn text_to_surface<'a>(font:&Font<'a, 'a>, text:String)->Surface<'a>{
    font
        .render(&text)
        .blended(Color::RGBA(55, 55, 55, 255))
        .map_err(|e| e.to_string()).expect("err")
}
