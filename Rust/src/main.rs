extern crate sdl2;

use std::{thread, time};
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use sdl2::render::Canvas;
use sdl2::EventPump;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 600;
const BORDER: u32 = 20;
const WIDTHF: f32 = WIDTH as f32;
const HEIGHTF: f32 = HEIGHT as f32;
const BORDERF: f32 = BORDER as f32;


// This is the main function
fn main() {

    let (mut canvas, mut events) = init_window(WIDTH, HEIGHT);

    let mut gs = GameState::init();

    'running: loop {
        for event in events.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } |
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: Some(Keycode::R), .. } => gs.reset(),
                _ => {}
            }
        }

        let state = events.mouse_state();

        gs.update(state.y() as f32);

        match gs.draw(&mut canvas) {
            Ok(_) => (),
            Err(e) => {println!("error drawing: {:?}", e); break},
        }
        canvas.present();
        thread::sleep(time::Duration::from_millis(10));
    }
}

fn init_window<'a>(width: u32, height: u32)-> (Canvas<Window>, EventPump){
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Pong", width, height)
        .build()
        .unwrap();
    let mut canvas = window.into_canvas()
        .build()
        .unwrap();

    canvas.set_draw_color(Color::RGB(255, 255, 255));

    let event_pump = sdl_context.event_pump().unwrap();

    return (canvas, event_pump)
}

struct GameState{
    ball : Ball,
    paddle : Paddle,
    borders : [Rectangle; 3],
    ticks : i32
}

struct Rectangle {
    top_left : Point,
    bottom_right : Point
}

#[derive(Clone)]
struct Point {
    x: f32,
    y: f32,
}

impl Rectangle {
    fn new(top_left: Point, bottom_right: Point) -> Rectangle{
        return Rectangle {top_left: top_left, bottom_right: bottom_right };
    }
}

impl Point {
    fn new(x: f32, y: f32) -> Point{
        return Point {x: x, y: y };
    }
}

impl std::ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, _rhs: Point) -> Point {
        return Point::new(self.x + _rhs.x, self.y + _rhs.y);
    }
}

struct Ball {
    pos : Point,
    speed: Point,
    size : f32
}

impl GameState {
    fn init() -> GameState{
        let start_pos: Point = Point::new(WIDTHF / 2.0, HEIGHTF / 2.0);
        let start_speed = Point::new(-5.0, 1.0);
        let ball = Ball { pos: start_pos, speed: start_speed, size: 20.0};

        let width_paddle = 20.0;
        let length_paddle = 100.0;
        let start_pos_paddle: Point = Point::new(WIDTHF - width_paddle / 2.0, HEIGHTF / 2.0);
        let paddle = Paddle {pos: start_pos_paddle, width: width_paddle, length: length_paddle};

        let top_border = Rectangle::new(Point::new(0.0,0.0), Point::new(WIDTHF, BORDERF) );
        let left_border = Rectangle::new(Point::new(0.0,0.0), Point::new(BORDERF, HEIGHTF) );
        let right_border = Rectangle::new(Point::new(0.0,(HEIGHT - BORDER) as f32), Point::new(WIDTHF, HEIGHTF) );
        
        let borders = [top_border, left_border, right_border];
        
        return GameState { ball: ball, paddle: paddle, borders: borders, ticks : 0} ;
    }

    fn reset(&mut self) {
        let start_pos: Point = Point::new(WIDTHF / 2.0, HEIGHTF / 2.0);
        let start_speed = Point::new(-5.0, 1.0);
        let ball = Ball { pos: start_pos, speed: start_speed, size: 20.0};

        self.ball = ball;
        self.ticks = 0;
    }

    fn update(&mut self, mouse_y: f32){
        self.paddle.update(mouse_y);
        self.update_ball();
    }

    fn update_ball(&mut self){
        let old = self.ball.pos.clone();
        let ball = &mut self.ball;
        let paddle = &mut self.paddle;
        ball.pos.x += ball.speed.x;
        ball.pos.y += ball.speed.y;

        if (ball.pos.x - ball.size) < BORDERF{
            let tofar = BORDERF - (ball.pos.x - ball.size);
            ball.pos.x += tofar;
            ball.speed.x = -ball.speed.x;
        }

        if (ball.pos.y - ball.size) < BORDERF{
            let tofar = BORDERF - (ball.pos.y - ball.size);
            ball.pos.y += tofar;
            ball.speed.y = -ball.speed.y;
        }

        if (ball.pos.y + ball.size) > HEIGHTF - BORDERF{
            let tofar = (ball.pos.y + ball.size) - (HEIGHTF - BORDERF);
            ball.pos.y -= tofar;
            ball.speed.y = -ball.speed.y;
        }

        let paddle_border_x = paddle.pos.x -paddle.width / 2.0;


        // If ball hits paddle
        if (old.x + ball.size) <= paddle_border_x && (ball.pos.x + ball.size) > paddle_border_x 
            && (ball.pos.y - paddle.pos.y).abs() <= paddle.length / 2.0 + ball.size
        {
            let tofar = ball.pos.x - paddle_border_x;
            ball.pos.x -= tofar;
            ball.speed.x = -ball.speed.x;
            self.ticks += 1;
            self.ball.speed.x *= 1.1;
            self.ball.speed.y *= 1.1;
        }
    }
}

struct Paddle {
    pos : Point,
    width: f32,
    length: f32
}

impl Paddle {
    fn update(&mut self, mouse_y: f32) {
        if mouse_y < BORDERF + self.length / 2.0{
            self.pos.y = BORDERF + self.length / 2.0;
        } else
        if mouse_y > HEIGHTF - BORDERF - self.length / 2.0{
            self.pos.y = HEIGHTF - BORDERF - self.length / 2.0;
        } else{
            self.pos.y = mouse_y;
        }
    }
}


trait Drawable {
    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(),String>;
}

impl Drawable for GameState{
    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(),String>{
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        self.ball.draw(canvas)?;
        self.paddle.draw(canvas)?;
        for b in &self.borders{
            b.draw(canvas)?;
        }

        let offset = BORDER as i32 + 1;
        draw_number(canvas, self.ticks, 5, offset, offset)?;

        return Ok(());
    }
}

impl Drawable for Ball{
    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(),String>{
        let x0 = self.pos.x as i32;
        let y0 = self.pos.y as i32;
        let size = self.size as i32;
        
        //TODO: Use a midpoint_cirlce algorithm?
        // See https://en.wikipedia.org/wiki/Midpoint_circle_algorithm#C_Example
        for i in -size..size {
            for j in -size..size {
                if i*i + j*j <= size * size{
                    let p = sdl2::rect::Point::new(x0 + i,y0 + j);
                    canvas.draw_point(p)?;
                }
            }
        }
        return Ok(());
    }
}


impl Drawable for Paddle{
    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(),String>{
        let topleft_x = (self.pos.x - self.width / 2.0) as i32;
        let topleft_y = (self.pos.y - self.length / 2.0) as i32;
        let width = self.width as u32;
        let length = self.length as u32;

        return canvas.fill_rect(Rect::new(topleft_x, topleft_y, width, length))
    }
}

impl Drawable for Rectangle{
    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(),String>{
        let topleft_x = self.top_left.x as i32;
        let topleft_y = self.top_left.y as i32;
        let width = (self.bottom_right.x - self.top_left.x) as u32;
        let length = (self.bottom_right.y - self.top_left.y) as u32;

        return canvas.fill_rect(Rect::new(topleft_x, topleft_y, width, length))
    }
}

fn draw_number(canvas: &mut Canvas<Window>, n: i32, size: i32, offset_x:i32, offset_y:i32) -> Result<(),String>{
    if n < 0{
        return Err("Incorrect number".to_string());
    }

    if n == 0 {
        return draw_digit(canvas, n, size, offset_x, offset_y);
    }

    let mut d = n;
    let total = (n as f32 + 0.1).log10().floor() as i32;
    let mut off_x = offset_x + total * 5 * size;
    while d > 0{
        let i = d % 10;
        draw_digit(canvas, i, size, off_x, offset_y)?;
        off_x -= size * 5;

        d = d / 10;
    }

    return Ok(());
}


fn draw_digit(canvas: &mut Canvas<Window>, n: i32, size: i32, offset_x:i32, offset_y:i32) -> Result<(),String>{

    let pixels : [i32; 15];
    pixels = match n {
        0 => [1,1,1
             ,1,0,1
             ,1,0,1
             ,1,0,1
             ,1,1,1],
        1 => [0,0,1
             ,0,0,1
             ,0,0,1
             ,0,0,1
             ,0,0,1],
        2 => [1,1,1
             ,0,0,1
             ,1,1,1
             ,1,0,0
             ,1,1,1],
        3 => [1,1,1
             ,0,0,1
             ,0,1,1
             ,0,0,1
             ,1,1,1],
        4 => [1,0,1
             ,1,0,1
             ,1,1,1
             ,0,0,1
             ,0,0,1],
        5 => [1,1,1
             ,1,0,0
             ,1,1,1
             ,0,0,1
             ,1,1,1],
        6 => [1,1,1
             ,1,0,0
             ,1,1,1
             ,1,0,1
             ,1,1,1],
        7 => [1,1,1
             ,0,0,1
             ,0,0,1
             ,0,0,1
             ,0,0,1],
        8 => [1,1,1
             ,1,0,1
             ,1,1,1
             ,1,0,1
             ,1,1,1],
        9 => [1,1,1
             ,1,0,1
             ,1,1,1
             ,0,0,1
             ,1,1,1],
        _ => return Err("Incorrect number".to_string()),
    };
    let mut column = 0;
    let mut row = 0;

    for i in &pixels{
         
        if i == &1 {
            let topleft_x = offset_x + column*size;
            let topleft_y = offset_y + row*size;
            canvas.fill_rect(Rect::new(topleft_x, topleft_y, size as u32, size as u32))?;
        }
        

        column += 1;
        if column == 3{
            column = 0;
            row += 1;
        }
    }

    return Ok(());
}