use ggez::*;
use ggez::graphics::*;
use ggez::glam::*;
use ggez::input::keyboard::{KeyCode, KeyMods, KeyInput};
use rand::Rng;

const DISTANCE: f32 = 20.0;
const RANGE_X: i32 = 96;
const RANGE_Y: i32 = 51;

fn main() {
    let config = conf::Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("demo_ggez", "Renzo")
        .default_conf(config)
        .build()
        .unwrap();
    let mut x: f32 = (rand::thread_rng().gen_range(0..RANGE_X) * DISTANCE as i32) as f32;
    let mut y: f32 = (rand::thread_rng().gen_range(0..RANGE_Y) * DISTANCE as i32) as f32;
    let state = State {
        apple: (Vec2::new(x , y), 
                graphics::Mesh::new_rectangle(
                    &ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(0.0,0.0,20.0,20.0),
                    Color::RED,
                    ).unwrap(),
                ),

        head: (Vec2::new(0.0, 0.0), 
                graphics::Mesh::new_rectangle(
                    &ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(0.0,0.0,20.0,20.0),
                    Color::GREEN,
                ).unwrap(), 
                Direction::NotSet),

        speed: 20,
    };
    event::run(ctx, event_loop, state);
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
            while ctx.time.check_update_time(self.speed){
                match self.head.2 {
                    Direction::Up => self.head.0.y = self.head.0.y - DISTANCE,
                    Direction::Down => self.head.0.y = self.head.0.y + DISTANCE,                
                    Direction::Right => self.head.0.x = self.head.0.x + DISTANCE,
                    Direction::Left => self.head.0.x = self.head.0.x - DISTANCE,
                    Direction::NotSet => self.head.0.x = self.head.0.x,
                }
                if self.apple.0 == self.head.0
                {
                    self.apple.0.x = (rand::thread_rng().gen_range(0..RANGE_X) * DISTANCE as i32) as f32;
                    self.apple.0.y = (rand::thread_rng().gen_range(0..RANGE_Y) * DISTANCE as i32) as f32;
                    self.speed = (self.speed as f32 * 1.1) as u32 ;
                }
            }
        return Ok(());
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
            let mut canvas =
                graphics::Canvas::from_frame(ctx, graphics::Color::BLUE);

            canvas.draw(&self.apple.1, self.apple.0); // mesh, Vec2
            canvas.draw(&self.head.1, self.head.0);

            canvas.finish(ctx)?;
        return Ok(());
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput,_repeat: bool) -> GameResult {
        match input.keycode {

            Some(KeyCode::W | KeyCode::Up) => {
                self.head.2 = Direction::Up;
            }
            Some(KeyCode::S | KeyCode::Down) => {
                self.head.2 = Direction::Down;
            }
            Some(KeyCode::D | KeyCode::Right) => {
                self.head.2 = Direction::Right;
            }
            Some(KeyCode::A | KeyCode::Left) => {
                self.head.2 = Direction::Left;
            }
            _ => (),
        }

        Ok(())
    }
}


struct State {
    apple: (Vec2, graphics::Mesh),
    head: (Vec2, graphics::Mesh, Direction),
    speed: u32,
    //head: (Position, Direction, graphics::Mesh)
}

struct Snake{
    head: (PositionInfo, Direction, graphics::Mesh),
    body: Vec<PositionInfo>,
}

struct PositionInfo(Vec2, Vec2);

enum Direction{
    NotSet,
    Up,
    Down,    
    Right,
    Left,
}








