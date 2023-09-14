use ggez::*;
use ggez::graphics::*;
use ggez::glam::*;
use ggez::input::keyboard::{KeyCode, KeyInput};
use rand::Rng;

const SCREEN_X: f32 = 1920.0;
const SCREEN_Y: f32 = 1080.0-60.0; // menos barra de titulo

const DISTANCE: f32 = 20.0;
const RANGE_X: i32 = SCREEN_X as i32/20;
const RANGE_Y: i32 = SCREEN_Y as i32/20;

fn main() {
    let (ctx, event_loop) = ContextBuilder::new("the_snake_case", "Renzo")
        .window_setup(ggez::conf::WindowSetup::default().title("the_snake_case"))
        .window_mode(ggez::conf::WindowMode::default().maximized(true))
        .build()
        .unwrap();

    let apple_mesh: graphics::Mesh = graphics::Mesh::new_rectangle(
        &ctx,
        graphics::DrawMode::fill(),
        graphics::Rect::new(0.0,0.0,20.0,20.0),
        Color::RED,
    ).unwrap();

    let snake_mesh: graphics::Mesh = graphics::Mesh::new_rectangle(
        &ctx,
        graphics::DrawMode::fill(),
        graphics::Rect::new(0.0,0.0,20.0,20.0),
        Color::GREEN,
    ).unwrap();

    let body_pos = Positions{
        current: Vec2::new(0.0, 0.0),
        previous: Vec2::new(0.0, 0.0),
    };

    let head_pos = Positions{
        current: Vec2::new(0.0, 0.0),
        previous: Vec2::new(0.0, 0.0),
    };

    let x: f32 = (rand::thread_rng().gen_range(0..RANGE_X) * DISTANCE as i32) as f32;
    let y: f32 = (rand::thread_rng().gen_range(0..RANGE_Y) * DISTANCE as i32) as f32;

    let state = State {
        apple: (Vec2::new(x , y), 
                apple_mesh,
                ),

        head: (head_pos, 
                snake_mesh, 
                Direction::NotSet,
            ),
        body: vec![body_pos],
        speed: 20,
        points: 0,
        over: false,
    };

    event::run(ctx, event_loop, state);
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(self.speed){
            if !self.over {
                self.head.0.previous = self.head.0.current;
                
                //updates body
                self.body[0].previous = self.body[0].current;
                self.body[0].current = self.head.0.previous;
                for i in 1..self.body.len(){
                    self.body[i].previous = self.body[i].current;
                    self.body[i].current = self.body[i - 1].previous;
                    if self.body[i].current == self.head.0.current{
                        println!("Game over");
                        self.over = true;
                    }
                }

                match self.head.2 {
                    Direction::Up => self.head.0.current.y = self.head.0.current.y - DISTANCE,
                    Direction::Down => self.head.0.current.y = self.head.0.current.y + DISTANCE,                
                    Direction::Right => self.head.0.current.x = self.head.0.current.x + DISTANCE,
                    Direction::Left => self.head.0.current.x = self.head.0.current.x - DISTANCE,
                    Direction::NotSet => self.head.0.current.x = self.head.0.current.x, // does nothing
                }
                if self.head.0.current.x > SCREEN_X 
                || self.head.0.current.x < 0.0
                || self.head.0.current.y > SCREEN_Y 
                || self.head.0.current.y < 0.0{
                    self.over = true;
                }                
                // if catches apple:
                if self.apple.0 == self.head.0.current
                {
                    self.apple.0.x = (rand::thread_rng().gen_range(0..RANGE_X) * DISTANCE as i32) as f32;
                    self.apple.0.y = (rand::thread_rng().gen_range(0..RANGE_Y) * DISTANCE as i32) as f32;
                    self.speed = self.speed + 2;
                    let pos = Positions{
                        current: self.body[self.body.len() - 1].previous,
                        previous: self.body[self.body.len() - 1].previous,
                    };
                    self.body.push(pos);
                    let pos2 = Positions{
                        current: self.body[self.body.len() - 1].previous,
                        previous: self.body[self.body.len() - 1].previous,
                    };
                    self.body.push(pos2);

                    self.points = self.points + 100;
                }
            }
        }
        return Ok(());
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
            let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLUE);


            canvas.draw(&self.apple.1, self.apple.0); // mesh, Vec2
            canvas.draw(&self.head.1, self.head.0.current);


            if self.over{
                let end_str = format!("Fim de jogo. Pontos: {}", self.points);
                let end_vec2 = Vec2::new(SCREEN_X/2.0 , SCREEN_Y/2.0);
                canvas.draw(
                    &graphics::Text::new(end_str),
                    graphics::DrawParam::from(end_vec2).color(Color::WHITE),
                );
            }  else{
                let points_str = format!("Pontos: {}", self.points);
                let points_vec2 = Vec2::new(20.0, 20.0);
                canvas.draw(
                    &graphics::Text::new(points_str),
                    graphics::DrawParam::from(points_vec2).color(Color::WHITE),
                );
            }

            for part in self.body.iter(){
                canvas.draw(&self.head.1, part.current); // smake mesh, Vec2
            }


            canvas.finish(ctx)?;
        return Ok(());
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput,_repeat: bool) -> GameResult {
        match input.keycode {

            Some(KeyCode::W | KeyCode::Up) => {
                match self.head.2{
                    Direction::Down => {},
                    _ => self.head.2 = Direction::Up,
                } 
            }
            Some(KeyCode::S | KeyCode::Down) => {
                match self.head.2{
                    Direction::Up => {},
                    _ => self.head.2 = Direction::Down,
                } 
            }
            Some(KeyCode::D | KeyCode::Right) => {
                match self.head.2{
                    Direction::Left => {},
                    _ => self.head.2 = Direction::Right,
                } 
            }
            Some(KeyCode::A | KeyCode::Left) => {
                match self.head.2{
                    Direction::Right => {},
                    _ => self.head.2 = Direction::Left,
                } 
            }
            _ => (),
        }
        return Ok(());
    }
}

struct Positions{
    current: Vec2,
    previous: Vec2,
}

enum Direction{
    NotSet,
    Up,
    Down,    
    Right,
    Left,
}

struct State {
    apple: (Vec2, graphics::Mesh),
    head: (Positions, graphics::Mesh, Direction),
    body: Vec<Positions>,
    speed: u32,
    points: u32,
    over: bool,
    //head: (Position, Direction, graphics::Mesh)
}












