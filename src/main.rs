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
        apple: (CurrentP(0.0, 0.0), graphics::Mesh::new_rectangle(
            &ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0,0.0,20.0,20.0),
            Color::RED,
        ).unwrap(),),
        head: (CurrentP(0.0, 0.0), graphics::Mesh::new_rectangle(
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
        const FPS: u32 = 60;
        const SPEED: f32 = -1.0;

        while ctx.time.check_update_time(FPS){
            match self.head.2 {
                Direction::Up => self.head.0.1 = self.head.0.1 + SPEED,
                Direction::Right => self.head.0.0 = self.head.0.0 + SPEED,
                Direction::Left => self.head.0.0 = self.head.0.0 - SPEED,
                Direction::Down => self.head.0.1 = self.head.0.1 - SPEED,
            }
        }
      return Ok(());
  }
  fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::BLUE);

        canvas.draw(&self.apple.1, Vec2::new(self.apple.0.0, self.apple.0.1));
        canvas.draw(&self.head.1, Vec2::new(self.head.0.0, self.head.0.1));

        canvas.finish(ctx)?;
      return Ok(());
  }
}

struct State {
    apple: (CurrentP, graphics::Mesh),
    head: (CurrentP, graphics::Mesh, Direction),
    //head: (Position, Direction, graphics::Mesh)
}

struct Snake{
    head: (Position, Direction, graphics::Mesh),
    body: Vec<Position>,
}

struct CurrentP(f32, f32);

struct PreviousP(f32, f32);

struct Position(CurrentP, PreviousP);

enum Direction{
    Up,
    Right,
    Left,
    Down,
}








