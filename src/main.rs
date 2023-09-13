use ggez::*;
use ggez::graphics::*;
use ggez::glam::*;
use ggez::input::keyboard::{KeyCode, KeyMods, KeyInput};

fn main() {
    let config = conf::Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("demo_ggez", "Renzo")
        .default_conf(config)
        .build()
        .unwrap();

    let state = State {
        apple: (Vec2::new(0.0, 0.0), graphics::Mesh::new_rectangle(
            &ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0,0.0,20.0,20.0),
            Color::RED,
        ).unwrap(),),
        head: (Vec2::new(0.0, 0.0), graphics::Mesh::new_rectangle(
            &ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0,0.0,20.0,20.0),
            Color::GREEN,
        ).unwrap(), Direction::Down),
    };
    event::run(ctx, event_loop, state);
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
            let SPEED: u32 = 3;
            const DISTANCE: f32 = 20.0;

            while ctx.time.check_update_time(SPEED){
                match self.head.2 {
                    Direction::Up => self.head.0.y = self.head.0.y - DISTANCE,
                    Direction::Down => self.head.0.y = self.head.0.y + DISTANCE,                
                    Direction::Right => self.head.0.x = self.head.0.x + DISTANCE,
                    Direction::Left => self.head.0.x = self.head.0.x - DISTANCE,
                }
                if self.apple.0 == self.head.0
                {
                    self.head.2 = Direction::Right;
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
    //head: (Position, Direction, graphics::Mesh)
}

struct Snake{
    head: (PositionInfo, Direction, graphics::Mesh),
    body: Vec<PositionInfo>,
}

struct PositionInfo(Vec2, Vec2);

enum Direction{
    Up,
    Down,    
    Right,
    Left,
}








