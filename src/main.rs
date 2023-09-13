use ggez::*;

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
    head: Head;
    body: Vec<Position>;
}


struct State {
    dt: std::time::Duration,
}

impl ggez::event::EventHandler<GameError> for State {
  fn update(&mut self, ctx: &mut Context) -> GameResult { //Context = ggez access to player input
      return Ok(());
  }
  fn draw(&mut self, ctx: &mut Context) -> GameResult {
      return Ok(());
  }
}

fn main(){

    let state = State {
        dt: std::time::Duration::new(0, 0),
    };
    // state = data that define the state of the game 

    let c = conf::Conf::new();
    //ctx = short for context

    let (ctx, event_loop) = ContextBuilder::new("hello_ggez", "Renzo")
    .default_conf(c)
    .build()
    .unwrap();

    event::run(ctx, event_loop, state);
}