use ggez::*;
use ggez::graphics::*;
use ggez::glam::*;

struct Current(u32, u32);

struct Previous(u32, u32);

struct Position(Current, Previous);

enum Direction{
    up,
    right,
    left,
    down,
}

struct Head(Position, Direction);

struct Snake{
    head: Head,
    body: Vec<Position>,
}




fn main() {
    let config = conf::Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("demo_ggez", "Renzo")
        .default_conf(config)
        .build()
        .unwrap();

    let state = State {
        square: graphics::Mesh::new_rectangle(
            &ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0,0.0,20.0,20.0),
            Color::WHITE,
        ).unwrap(),
    };

    println!("Hello, world!");
    event::run(ctx, event_loop, state);
}

struct State {
    square: graphics::Mesh,
}

impl ggez::event::EventHandler<GameError> for State {
  fn update(&mut self, ctx: &mut Context) -> GameResult {
        //const FPS: u32 = 60;
        //while ctx.time.check_update_time(60){

        //}
      return Ok(());
  }
  fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::BLUE);

        canvas.draw(&self.square, Vec2::new(0.0,0.0));

        canvas.finish(ctx)?;
      return Ok(());
  }
}


